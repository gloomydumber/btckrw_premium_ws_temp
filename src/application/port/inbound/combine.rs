use crate::domain::message::{ExchangeMessage, CombinedMessage};
use crate::domain::error::DomainError;

pub trait CombineMessagesPort {
    fn combine_messages(
        &self,
        krw_message: &ExchangeMessage,
        usd_message: &ExchangeMessage,
    ) -> Result<CombinedMessage, DomainError>;
}