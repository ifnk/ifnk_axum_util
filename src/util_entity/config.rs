use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Config {
    pub cfg: Vec<Cfg>,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct Cfg {
    pub mysql: String,
    pub machineIndex: u32,
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
    pub getNewValUrl: String,
    pub webUrl: String,
    pub runtimePath: String,
}

#[derive(Debug,Serialize, Deserialize,Clone)]
pub struct AppSettings {
    pub env: String,
    // 开发/生产 环境
    pub Port: u32,
    // 端口
    // #[serde(default = "default_addr")] // toml 解析给默认值
    pub Addr: String, // 电厂环境
}
