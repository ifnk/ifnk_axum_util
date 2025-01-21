use anyhow::{anyhow, Error};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::{json, Value};

use crate::util_entity::response::{failed, json_failed, success};

pub type JsonResult<T> = Result<Json<T>, AnyhowError>;

pub struct AnyhowError {
    error: Error,
    detail: Option<String>,  // 新增 detail 字段
}

impl AnyhowError {
    pub fn new(error: Error, detail: Option<String>) -> Self {
        Self { error, detail }
    }
}

pub fn json_success_response<T: Serialize>(data: T) -> Json<Value> {
    Json(json!(success(data)))
}

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        let error = self.error;
        let detail = self.detail;
        json_failed(error.to_string(), detail).into_response()
    }
}

impl From<serde_json::Error> for AnyhowError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(anyhow!(err.to_string()), Some(format!("行号: {}", err.line())))
    }
}

impl From<String> for AnyhowError {
    fn from(err: String) -> Self {
        Self::new(anyhow!(err), None)
    }
}

impl From<anyhow::Error> for AnyhowError {
    fn from(err: anyhow::Error) -> Self {
        Self::new(err, None)
    }
}