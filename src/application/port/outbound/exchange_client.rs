use tokio::sync::mpsc::Sender;
use crate::domain::message::ExchangeMessage;

pub trait ExchangeClientPort {
    // fn fetch_latest_message(&self) -> Result<ExchangeMessage, DomainError>;
    async fn subscribe(sender: Sender<ExchangeMessage>);
}