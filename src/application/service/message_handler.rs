use tokio::sync::mpsc::Receiver;
use crate::application::port::inbound::combine::CombineMessagesPort;
use crate::application::port::outbound::websocket_server::WebSocketServerPort;
use crate::domain::message::ExchangeMessage;

pub struct MessageHandler<T: CombineMessagesPort, U: WebSocketServerPort> {
    combine_service: T,
    websocket_server: U,
}

impl<T: CombineMessagesPort, U: WebSocketServerPort> MessageHandler<T, U> {
    pub fn new(combine_service: T, websocket_server: U) -> Self {
        Self {
            combine_service,
            websocket_server,
        }
    }

    pub async fn handle_messages(
        &self,
        mut krw_rx: Receiver<ExchangeMessage>,
        mut usd_rx: Receiver<ExchangeMessage>,
    ) {
        loop {
            tokio::select! {
                Some(krw_message) = krw_rx.recv() => {
                    if let Some(usd_message) = usd_rx.recv().await {
                        if let Ok(combined_message) = self.combine_service.combine_messages(&krw_message, &usd_message) {
                            self.websocket_server.publish_message(combined_message).unwrap_or_else(|err| {
                                eprintln!("Failed to publish combined message: {:?}", err);
                            });
                        }
                    }
                }
                Some(usd_message) = usd_rx.recv() => {
                    if let Some(krw_message) = krw_rx.recv().await {
                        if let Ok(combined_message) = self.combine_service.combine_messages(&krw_message, &usd_message) {
                            self.websocket_server.publish_message(combined_message).unwrap_or_else(|err| {
                                eprintln!("Failed to publish combined message: {:?}", err);
                            });
                        }
                    }
                }
            }
        }
    }
}