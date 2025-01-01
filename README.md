# btckrw-premium-ws-temp

for hex arch practice

```
btckrw-premium-ws-temp
├─ Cargo.toml
└─ src
   ├─ adapter
   │  ├─ inbound
   │  │  ├─ krw_websocket.rs
   │  │  ├─ mod.rs
   │  │  └─ usd_websocket.rs
   │  ├─ mod.rs
   │  └─ outbound
   │     ├─ mod.rs
   │     └─ websocket_server.rs
   ├─ application
   │  ├─ mod.rs
   │  ├─ port
   │  │  ├─ inbound
   │  │  │  ├─ combine.rs
   │  │  │  └─ mod.rs
   │  │  ├─ mod.rs
   │  │  └─ outbound
   │  │     ├─ exchange_client.rs
   │  │     ├─ mod.rs
   │  │     └─ websocket_server.rs
   │  └─ service
   │     ├─ combine_service.rs
   │     ├─ message_handler.rs
   │     └─ mod.rs
   ├─ domain
   │  ├─ error.rs
   │  ├─ message.rs
   │  └─ mod.rs
   └─ main.rs
```