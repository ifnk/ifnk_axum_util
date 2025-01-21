use anyhow::{anyhow, Error};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::{json, Value};

use crate::util_entity::response::{failed, json_failed, success};

pub type JsonResult<T> = Result<Json<T>, AnyhowError>;

pub struct AnyhowError {
    error: Error,
}

impl AnyhowError {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}

pub fn json_success_response<T: Serialize>(data: T) -> Json<Value> {
    Json(json!(success(data)))
}

impl IntoResponse for AnyhowError {
    fn into_response(self) -> Response {
        let error = self.error;
        json_failed(error.to_string()).into_response()
    }
}

impl From<serde_json::Error> for AnyhowError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(anyhow!(err.to_string()))
    }
}

impl From<String> for AnyhowError {
    fn from(err: String) -> Self {
        Self::new(anyhow!(err))
    }
}

impl From<anyhow::Error> for AnyhowError {
    fn from(err: anyhow::Error) -> Self {
        Self::new(err)
    }
}