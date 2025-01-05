use crate::application::port::inbound::combine::CombineMessagesPort;
use crate::domain::message::{CombinedMessage, ExchangeMessage};
use crate::domain::error::DomainError;

pub struct CombineService;

// Upbit Websocket -> domain : A
// Binance Websocket -> domain : B

// domain service(A, B) -> combine C

// C -> adapter
// service(C) -> D
// D -> adapter

// A, B한테 100원 보내줘
// application service              send(a, b, amount)
// send(A, B, 100)
// 통장잔고어댑터.잔고(A) -> 1000
// 통장잔고어댑터.잔고(B) -> 1200
// domain service
// A 잔고 > 100                                   isMorethan()
// A 잔고 - 100 => 900, B 잔고 + 100 => 1300        update()
// 통장잔고어댑터.잔고업데이트(A, 900)
// 통장잔고어댑터.잔고업데이트(B, 1300)
// return 성공했다 

// 계좌 { 주인, 잔고 }

// application 

// 시나리오

// Application service: 시나리오 1개
// DB 조회 -> 내부 로직 -> DB 저장

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