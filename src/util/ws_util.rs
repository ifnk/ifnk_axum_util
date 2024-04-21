use axum::extract::{State, WebSocketUpgrade};
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::response::Response;
use axum::extract::ws::{Message, WebSocket};
use chrono::Local;
use serde::{Deserialize, Serialize};
use tracing::info;
// use crate::app_state::AppState;
// use crate::controller::ws_controller::zoom;
// use crate::repo::pcm_repo::PcmData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSData {
    // 裁剪数据
    pub key: String,
    pub data: String, // 数据json
}

pub async fn handler(
    ws: WebSocketUpgrade,
    // State(app_state): State<Arc<Mutex<AppState>>>,
) -> Response {
    ws.on_upgrade(|socket| {
        handle_socket(socket,
                      // app_state
        )
    })
}

async fn handle_socket(mut socket: WebSocket,
                       // state: Arc<Mutex<AppState>>,
) {
    loop {
        let msg_option = socket.recv().await;
        match msg_option {
            None => {
                // 没有更多的消息，continue
                info!("ws 客户端没有更多消息");
                continue;
            }
            Some(msg) => {
                // 在这里处理消息
                let nmsg = match msg {
                    Ok(msg) => msg,
                    Err(_) => {
                        // client disconnected
                        info!("ws 客户端断开连接");
                        return;
                    }
                };
                let content = match nmsg {
                    Message::Text(text) => text,
                    Message::Close(close) => {
                        info!("ws 客户端关闭连接");
                        return;
                    }
                    _ => {
                        info!("ws 客户端其他消息不处理 ");
                        continue;
                    }
                };
                // 解析消息
                // 反序列化
                let ws_data: WSData = match serde_json::from_str(&content) {
                    Ok(data) => data,
                    Err(e) => {
                        info!("ws 客户端消息解析失败 {:?}", e);
                        continue;
                    }
                };
                // 根据 ws_data.key 处理不同的消息
                // match ws_data.key.as_str() {
                    // // 处理 "KEY1"
                    // "zoom" => {
                    //     match zoom(ws_data.data, state.clone(), &mut socket).await {
                    //         Ok(_) => {}
                    //         Err(err) => {
                    //             info!("ws 客户端消息处理{}失败 {:?}", ws_data.key, err);
                    //         }
                    //     }
                    // }
                    // // 处理 "KEY2"
                    // "key2" => {}
                    // _ => {
                    //     info!("ws 客户端消息不处理 key {:?}", ws_data.key);
                    // }
                // }
            }
        }
    }
}