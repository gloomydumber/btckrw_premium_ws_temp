mod adapter;
mod application;
mod domain;

use tokio::sync::mpsc;
use tokio::task;
use crate::adapter::inbound::krw_websocket::KRWWebSocketClient;
use crate::adapter::inbound::usd_websocket::USDWebSocketClient;
use crate::adapter::outbound::websocket_server::WebSocketServer;
use crate::application::service::message_handler::MessageHandler;
use crate::application::service::combine_service::CombineService;
use crate::application::port::outbound::exchange_client::ExchangeClientPort;

#[tokio::main]
async fn main() {
    // Channels for KRW and USD messages
    let (krw_tx, krw_rx) = mpsc::channel(100);
    let (usd_tx, usd_rx) = mpsc::channel(100);

    // Start WebSocket subscription tasks
    task::spawn(async move {
        KRWWebSocketClient::subscribe(krw_tx).await;
    });
    task::spawn(async move {
        USDWebSocketClient::subscribe(usd_tx).await;
    });

    // Initialize services and message handler
    let combine_service: CombineService = CombineService;
    let websocket_server: WebSocketServer = WebSocketServer;
    let message_handler: MessageHandler<CombineService, WebSocketServer> = MessageHandler::new(combine_service, websocket_server);

    // Start handling messages
    message_handler.handle_messages(krw_rx, usd_rx).await;
}
