mod parse;

use parse::*;
use std::fs;
// 解析配置，返回应用列表
pub fn get_config(path: &str) -> (Vec<String>, Vec<String>){
    let config = fs::read_to_string(path).expect("Parse config failed");
    check(&config).expect("The configuration is malformed");
    let (simp_app, msg_app) = fliter_conf(config);
    (parse(simp_app), parse(msg_app))
}