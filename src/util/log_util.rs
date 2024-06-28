use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, Layer, Registry};
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;

// 设置 文件日志和控制台日志同时输出
// todo 后面 日志等级啊 ，输出目录通过读取配置文件来搞
pub fn setup_logs() -> anyhow::Result<WorkerGuard> {
    let file_appender = rolling::daily("./logs", "log.log");
    let (nb, wg) = tracing_appender::non_blocking(file_appender);
    // file layer with custom formatter
    let file_layer = fmt::layer()
        .with_writer(move || nb.clone())
        // .event_format(MyFormatter)
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));
    // terminal layer
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_thread_names(true)
        .with_line_number(true)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    // Combine layers into a single subscriber and set global default
    let subscriber = Registry::default().with(stdout_layer).with(file_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    Ok(wg)
}
