use std::env::args;
use std::fs;
use std::process::exit;
use std::sync::Arc;

use task_limiter::{config, core, info_sync::*, misc};

#[tokio::main]
async fn main() {
    misc::set_self_sched();
    let path = match args().nth(1) {
        Some(o) => o,
        None => {
            eprintln!("Pleas specify the configuration path in arg");
            exit(2);
        }
    };
    let conf_raw = fs::read_to_string(&path).expect("Parse config failed");
    let mut conf = InfoSync::new_blocker(move || {
        misc::inotify_block([&path]).expect("Failed to block by inotify");
        config::get_config(&conf_raw)
    });
    core::process(conf.into()).await;
}
