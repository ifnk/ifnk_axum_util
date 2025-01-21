use anyhow::{anyhow, Error};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};
use tracing::error;

use crate::util_entity::response::{failed, json_failed, success};

use super::response::json_failed_with_details;

pub type JsonResult<T> = Result<Json<T>, AnyhowError>;

pub struct AnyhowError {
    error: Error,
    detail: Option<String>, // 新增 detail 字段
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
        json_failed_with_details(error.to_string(), detail).into_response()
    }
}

impl From<serde_json::Error> for AnyhowError {
    fn from(err: serde_json::Error) -> Self {
        let detail = format!(
            "Error occurred at {}:{}\nStack trace: {:?}",
            file!(),
            line!(),
            backtrace::Backtrace::new()
        );

        Self::new(
            anyhow!(err.to_string()),
            Some(detail),
        )
    }
}

impl From<String> for AnyhowError {
    fn from(err: String) -> Self {
        Self::new(anyhow!(err), None)
    }
}

impl From<anyhow::Error> for AnyhowError {
    fn from(err: anyhow::Error) -> Self {
        error!("{:#?}", err);
        let backtrace = backtrace::Backtrace::new();
        let filtered_trace: Vec<String> = backtrace.frames()
            .iter()
            .filter(|frame| {
                if let Some(symbols) = frame.symbols().first() {
                    if let Some(filename) = symbols.filename() {
                        return filename.to_string_lossy().contains("src/");
                    }
                }
                false
            })
            .map(|frame| {
                if let Some(symbols) = frame.symbols().first() {
                    format!(
                        "{}:{} - {}",
                        symbols.filename().map(|f| f.to_string_lossy()).unwrap_or_default(),
                        symbols.lineno().unwrap_or(0),
                        symbols.name().map(|n| n.to_string()).unwrap_or_default()
                    )
                } else {
                    String::new()
                }
            })
            .collect();

        let detail = format!(
            "\n 错误堆栈:\n{}",
            filtered_trace.join("\n")
        );

        Self::new(err, Some(detail))
    }
}
