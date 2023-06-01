use std::env::args;
use std::fs;
use std::io::Write;
use std::process::exit;

use task_limiter::config;
use task_limiter::core;
use task_limiter::info_sync::*;
use task_limiter::misc;

use chrono::prelude::*;
use log::LevelFilter;
use log::{debug, error, info};

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} at {}] : {}",
                record.level(),
                Local::now().format("%H:%M"),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    info!("Log initialization is complete");

    // 从第一个参数获取配置路径
    let path = match args().nth(1) {
        Some(o) => o,
        None => {
            error!("Pleas specify the configuration path in the first arg");
            exit(2);
        }
    };
    info!("Try read profile at {}", &path);

    // 读取 & 解析配置
    let conf_raw = match fs::read_to_string(&path) {
        Ok(o) => o,
        Err(_) => {
            error!("Fail to read config");
            exit(1);
        }
    };
    info!("Successfully readed profile at {}", &path);
    debug!("Config Raw : {}", &conf_raw);

    let conf = InfoSync::new_blocker(move || {
        if misc::inotify_block([&path]).is_err() {
            error!("Failed to block config file by using inotify");
            exit(1)
        }
        info!("Configuration updates, reparsing");
        config::get_config(&conf_raw)
    });
    info!("Create a configuration monitoring thread");

    // 把配置传给执行函数
    debug!("Switch to the process function");
    tokio::spawn(core::process(conf.into())).await.unwrap();
}
