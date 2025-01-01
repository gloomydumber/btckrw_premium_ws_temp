use crate::domain::message::CombinedMessage;
use crate::domain::error::DomainError;

pub trait WebSocketServerPort {
    fn publish_message(&self, message: CombinedMessage) -> Result<(), DomainError>;
}