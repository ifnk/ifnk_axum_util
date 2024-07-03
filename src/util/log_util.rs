use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, Layer, Registry};
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
fn create_log_layers(file_appender: rolling::RollingFileAppender) -> (impl Layer<Registry> + Send + Sync, WorkerGuard) {
    let (nb, wg) = tracing_appender::non_blocking(file_appender);
    // file layer with custom formatter
    let file_layer = fmt::layer()
        .with_writer(move || nb.clone())
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    // terminal layer
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .pretty()
        .with_thread_names(true)
        .with_line_number(true)
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    // Combine layers
    let combined_layer = stdout_layer.and_then(file_layer);

    (combined_layer, wg)
}

fn set_global_subscriber(layer: impl Layer<Registry> + Send + Sync) {
    let subscriber = Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("设置全局日志订阅失败");
}

pub fn setup_logs() -> anyhow::Result<WorkerGuard> {
    let file_appender = rolling::daily("./logs", "log.log");
    let (layer, wg) = create_log_layers(file_appender);
    set_global_subscriber(layer);
    Ok(wg)
}

pub fn setup_logs_with_path(directory: String, file_name: String) -> anyhow::Result<WorkerGuard> {
    let file_appender = rolling::daily(directory, file_name);
    let (layer, wg) = create_log_layers(file_appender);
    set_global_subscriber(layer);
    Ok(wg)
}

// // 设置 文件日志和控制台日志同时输出
// pub fn setup_logs() -> anyhow::Result<WorkerGuard> {
//     let file_appender = rolling::daily("./logs", "log.log");
//     let (nb, wg) = tracing_appender::non_blocking(file_appender);
//     // file layer with custom formatter
//     let file_layer = fmt::layer()
//         .with_writer(move || nb.clone())
//         // .event_format(MyFormatter)
//         .with_ansi(false)
//         .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));
//     // terminal layer
//     let stdout_layer = fmt::layer()
//         .with_writer(std::io::stdout)
//         .pretty()
//         .with_thread_names(true)
//         .with_line_number(true)
//         .with_ansi(false)
//         .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));
//
//     // Combine layers into a single subscriber and set global default
//     let subscriber = Registry::default().with(stdout_layer).with(file_layer);
//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Failed to set global default subscriber");
//
//     Ok(wg)
// }
//
// pub fn setup_logs_with_path(directory:String, file_name:String) -> anyhow::Result<WorkerGuard> {
//     let file_appender = rolling::daily(directory, file_name);
//     let (nb, wg) = tracing_appender::non_blocking(file_appender);
//     // file layer with custom formatter
//     let file_layer = fmt::layer()
//         .with_writer(move || nb.clone())
//         // .event_format(MyFormatter)
//         .with_ansi(false)
//         .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));
//     // terminal layer
//     let stdout_layer = fmt::layer()
//         .with_writer(std::io::stdout)
//         .pretty()
//         .with_thread_names(true)
//         .with_line_number(true)
//         .with_ansi(false)
//         .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));
//
//     // Combine layers into a single subscriber and set global default
//     let subscriber = Registry::default().with(stdout_layer).with(file_layer);
//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Failed to set global default subscriber");
//
//     Ok(wg)
// }