use crate::util_entity::config::{AppSettings, Config};

pub fn read_toml_config() -> anyhow::Result<(AppSettings, Config)> {
    // 读取 appsettings.toml 文件
    let appsettings_content = std::fs::read_to_string("appsettings.toml")?;
    // 将字符串解析为 AppSettings 结构体
    let appsettings = toml::from_str::<AppSettings>(&appsettings_content)?;
    let appsettings_clone = appsettings.clone();

    // 读取配置文件的内容
    let localhost_content = std::fs::read_to_string(format!("{}.toml", appsettings.Addr))?;
    // 将字符串解析为 config 结构体
    let config = toml::from_str::<Config>(&localhost_content)?;

    Ok((appsettings_clone,config))
}
