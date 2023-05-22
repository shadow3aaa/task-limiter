use std::env::args;
use std::fs;
use std::process::exit;
use task_limiter::{config, core};

fn main() {
    let path = match args().next() {
        Some(o) => o,
        None => {
            eprintln!("Pleas specify the configuration path in arg");
            exit(-1);
        }
    };
    let conf = fs::read_to_string(path).expect("Parse config failed");
    let apps = config::get_config(conf);
    core::process(apps);
}
