use std::env::args;
use std::fs;
use std::process::exit;

use task_limiter::{config, core, info_sync::*, misc};

#[tokio::main]
async fn main() {
    // 从第一个参数获取配置路径
    let path = match args().nth(1) {
        Some(o) => o,
        None => {
            eprintln!("Pleas specify the configuration path in arg");
            exit(2);
        }
    };

    // 读取 & 解析配置
    let conf_raw = fs::read_to_string(&path).expect("Parse config failed");
    let conf = InfoSync::new_blocker(move || {
        misc::inotify_block([&path]).expect("Failed to block by inotify");
        config::get_config(&conf_raw)
    });

    // 把配置传给执行函数
    tokio::spawn(core::process(conf.into())).await.unwrap();
}
