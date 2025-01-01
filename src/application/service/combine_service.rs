use crate::application::port::inbound::combine::CombineMessagesPort;
use crate::domain::message::{CombinedMessage, ExchangeMessage};
use crate::domain::error::DomainError;

pub struct CombineService;

impl CombineMessagesPort for CombineService {
    fn combine_messages(
        &self,
        krw_message: &ExchangeMessage,
        usd_message: &ExchangeMessage,
    ) -> Result<CombinedMessage, DomainError> {
        Ok(CombinedMessage {
            krw_message: krw_message.clone(),
            usd_message: usd_message.clone(),
            // krw_message,
            // usd_message,
        })
    }
}