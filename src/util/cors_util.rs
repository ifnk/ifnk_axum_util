use tower_http::cors::{Any, CorsLayer};
// 允许所有跨域
pub fn allow_all_cors() ->CorsLayer {
    // 允许所有跨域
    CorsLayer::new()
        .allow_methods(Any) // 允许所有方法
        .allow_headers(Any) // 允许所有header
        .allow_origin(Any) // 允许所有origin
}