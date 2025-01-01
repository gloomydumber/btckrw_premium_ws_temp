use tokio::sync::mpsc::Sender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use serde::Serialize;

use crate::application::port::outbound::exchange_client::ExchangeClientPort;
use crate::domain::message::ExchangeMessage;

pub struct KRWWebSocketClient;

#[derive(Serialize)]
struct TicketMessage {
    ticket: String,
}

#[derive(Serialize)]
struct TickerSubscription {
    #[serde(rename = "type")]
    subscription_type: String,
    codes: Vec<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum SubscribeMessage {
    Ticket(TicketMessage),
    Subscription(TickerSubscription),
}

impl ExchangeClientPort for KRWWebSocketClient {
    async fn subscribe(sender: Sender<ExchangeMessage>) {
        let url = "wss://api.upbit.com/websocket/v1";

        // Establish a WebSocket connection
        let (ws_stream, _) = match connect_async(url).await {
            Ok(stream) => {
                println!("Connected to WebSocket: {}", url); // Print success message
                stream
            }
            Err(err) => {
                eprintln!("Failed to connect to WebSocket: {:?}", err);
                return;
            }
        };

        let (mut write, mut read) = ws_stream.split();

        // Construct subscription messages
        let messages = vec![
            SubscribeMessage::Ticket(TicketMessage {
                ticket: "unique-ticket".to_string(),
            }),
            SubscribeMessage::Subscription(TickerSubscription {
                subscription_type: "ticker".to_string(),
                codes: vec!["KRW-BTC".to_string()],
            }),
        ];

        // Serialize the subscription message
        let subscribe_message_json = match serde_json::to_string(&messages) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Failed to serialize subscription message: {:?}", err);
                return;
            }
        };

        // Send the subscription message
        if let Err(err) = write
            .send(Message::Text(subscribe_message_json.clone())) // Clone here to reuse the variable
            .await
        {
            eprintln!("Failed to send subscription message: {:?}", err);
            return;
        }

        println!("Subscription message sent: {}", subscribe_message_json);

        // Handle incoming messages
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received Text message: {}", text);
                    process_message(&text, &sender).await;
                }
                Ok(Message::Binary(binary_data)) => {
                    // Decode binary data to UTF-8 text
                    match String::from_utf8(binary_data) {
                        Ok(decoded_text) => {
                            println!("Received Binary message (decoded): {}", decoded_text);
                            process_message(&decoded_text, &sender).await;
                        }
                        Err(err) => eprintln!("Failed to decode binary message: {:?}", err),
                    }
                }
                Ok(other) => {
                    println!("Received unexpected message: {:?}", other);
                }
                Err(err) => {
                    eprintln!("Failed to read from WebSocket: {:?}", err);
                    break; // Exit the loop on error
                }
            }
        }

        println!("WebSocket connection closed.");
    }
}

// Process and forward the message
async fn process_message(message: &str, sender: &Sender<ExchangeMessage>) {
    match serde_json::from_str::<ExchangeMessage>(message) {
        Ok(exchange_message) => {
            if let Err(err) = sender.send(exchange_message).await {
                eprintln!("Failed to forward message: {:?}", err);
            } else {
                println!("Message forwarded to receiver.");
            }
        }
        Err(err) => {
            eprintln!("Failed to parse message into ExchangeMessage: {:?}", err);
        }
    }
}

