use task_limiter::{config, core};
use std::env::args;
use std::process::exit;
fn main() {
    let conf_path = match args().next() {
        Some(o) => o,
        None => {
            eprintln!("Specify the configuration path");
            exit(-1);
        }
    };
    let apps = config::get_config(&conf_path);
    core::process(apps);
}
