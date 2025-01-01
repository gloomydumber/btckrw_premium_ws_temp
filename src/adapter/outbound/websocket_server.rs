use crate::application::port::outbound::websocket_server::WebSocketServerPort;
use crate::domain::message::CombinedMessage;
use crate::domain::error::DomainError;

pub struct WebSocketServer;

impl WebSocketServerPort for WebSocketServer {
    fn publish_message(&self, message: CombinedMessage) -> Result<(), DomainError> {
        // todo!()
        // Err(DomainError::InvalidMessage("temp".to_string()))
        if message.krw_message.price > 0.0 {
            Ok(())
        } else {
            Err(DomainError::InvalidMessage("Invalid Message".to_owned()))
        }
    }
}