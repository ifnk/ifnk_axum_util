use axum::{extract::{Request, State}, Json, middleware::Next, response::{IntoResponse, Response}};
use crate::util_entity::response::failed;



pub async fn machine_index_header_middleware(request: Request, next: Next) -> Response {
    let path = request.uri().path();

    // 以api 开头的请求需要检查header中的Machineindex
    if path.starts_with("/api") {
        let machine_index = request.headers().get("Machineindex");
        if machine_index.is_none() {
            return Json(failed::<()>("请在header传Machineindex".to_string(),None)).into_response();
        }

        let machine_index_str = machine_index.unwrap().to_str();

        let machine_index_str = match machine_index_str {
            Ok(v) => v,
            Err(_) => return Json(failed::<()>("Machineindex必须为字符串".to_string(),None)).into_response(),
        };

        // parse to int
        let machine_index = match machine_index_str.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Json(failed::<()>("Machineindex必须为数字".to_string(),None)).into_response(),
        };

        if machine_index == 0 {
            return Json(failed::<()>("Machineindex不能为0".to_string(),None)).into_response();
        }
    }

    next.run(request).await
}
