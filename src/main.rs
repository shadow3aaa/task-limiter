use std::env::args;
use std::fs;
use std::process::exit;
use task_limiter::{config, core, misc::set_self_sched};

fn main() {
    set_self_sched();
    let path = match args().nth(1) {
        Some(o) => o,
        None => {
            eprintln!("Pleas specify the configuration path in arg");
            exit(2);
        }
    };
    let conf = fs::read_to_string(path).expect("Parse config failed");
    let apps = config::get_config(conf);
    core::process(apps);
}
