use tokio::sync::mpsc::Sender;
use crate::application::port::outbound::exchange_client::ExchangeClientPort;
use crate::domain::message::ExchangeMessage;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use serde::Serialize;

pub struct USDWebSocketClient;

#[derive(Serialize)]
struct SubscribeMessage {
    #[serde(rename = "type")]
    message_type: String, // `type` is a reserved keyword in Rust
    product_ids: Vec<String>,
    channels: Vec<String>,
}

impl ExchangeClientPort for USDWebSocketClient {
    async fn subscribe(sender: Sender<ExchangeMessage>) {
        let url = "wss://ws-feed.exchange.coinbase.com";

        // Establish a secure WebSocket connection
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

        // Construct the subscription message
        let subscribe_message = SubscribeMessage {
            message_type: "subscribe".to_string(),
            product_ids: vec!["BTC-USD".to_string()], // Add desired trading pairs
            channels: vec!["ticker".to_string()],    // Subscribe to ticker channel
        };

        // Serialize the subscription messages
        let subscribe_message_json = match serde_json::to_string(&subscribe_message) {
            Ok(json) => json,
            Err(err) => {
                eprintln!("Failed to serialize subscription message: {:?}", err);
                return;
            }
        };

        // Send the subscription message
        if let Err(err) = write
            .send(Message::Text(subscribe_message_json.clone())) // Clone for logging
            .await
        {
            eprintln!("Failed to send subscription message: {:?}", err);
            return;
        }

        println!("Subscription message sent: {}", subscribe_message_json);

        // Read and forward incoming messages
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received Text message: {}", text); // Log the received message
                    process_message(&text, &sender).await; // Process and send the message
                }
                Ok(Message::Binary(binary_data)) => {
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
                    break;
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
