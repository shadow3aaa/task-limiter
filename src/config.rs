use crate::Config;
use toml::Value;

// 解析配置，返回应用列表
pub fn get_config(config: String) -> Config {
    let config = config.parse::<Value>().expect("Failed to parse TOML");
    let white_list = parse_toml(&config, "White_List_Apps");
    let msg_list = parse_toml(&config, "Msg_Apps");
    let force_list = parse_toml(&config, "Force_Apps");
    Config {
        white_list,
        msg_list,
        force_list,
    }
}

pub fn parse_toml(config: &Value, flag: &'static str) -> Vec<String> {
    config[flag]["list"]
        .as_array()
        .unwrap_or_else(|| panic!("Err : The {} is not parsed", flag))
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

#[test]
fn test_config() {
    // 测试配置是乱写的
    let config = r#"[White_List_Apps]
    list = [
        "good"
    ]
    [Msg_Apps]
    list = [
        "orange",
        "mango"
    ]
    [Force_Apps]
    list = [
        "apple.com",
        "baidu.com"
    ]
    "#;
    let config = get_config(config.to_string());
    println!("{:?}", config);
}
