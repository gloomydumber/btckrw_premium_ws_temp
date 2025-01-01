use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeMessage {
    pub exchange: String,
    pub price: f64,
}

pub struct CombinedMessage {
    pub krw_message: ExchangeMessage,
    pub usd_message: ExchangeMessage,
}