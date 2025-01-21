use http::StatusCode;
use axum::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct CusResponse<T> {
    pub code: i32,
    pub msg: String,
    pub detail: Option<String>,  // 新增 detail 字段
    pub data: Option<T>,
}

// success response
pub fn success<T>(data: T) -> CusResponse<T> {
    CusResponse {
        code: 0,
        msg: "成功".to_string(),
        detail: None,  // 成功时 detail 为 None
        data: Some(data),
    }
}

// failed response
pub fn failed<T>(msg: String, detail: Option<String>) -> CusResponse<T> {
    CusResponse {
        code: 500,
        msg,
        detail,  // 失败时传入 detail
        data: None,
    }
}

pub fn json_success<T: Serialize>(data: T) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!(success(data))))
}

pub fn json_failed(msg: String, detail: Option<String>) -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!(failed::<()>(msg, detail))))
}