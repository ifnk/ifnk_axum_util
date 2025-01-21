use std::any::Any;
use http::Response;
use axum::body::Body;
use axum::Json;
use axum::response::IntoResponse;
use crate::util_entity::response::{CusResponse, failed};

pub fn handle_exception(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let msg = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };
    let details = if std::env::var("ENV").unwrap_or_default() == "production" {
        format!("Error occurred at {}:{}", file!(), line!())
    } else {
        format!(
            "Error occurred at {}:{}\nStack trace: {:?}", 
            file!(), 
            line!(),
            backtrace::Backtrace::new()
        )
    };

    let cus_response: CusResponse<()> = failed(msg, Some(details));
    let response = Json(cus_response);

    response.into_response()
}
