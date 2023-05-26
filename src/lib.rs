pub mod config;
pub mod core;
pub mod info_sync;
pub mod misc;

#[derive(Debug)]
pub struct Config {
    pub white_list: Vec<String>,
    pub msg_list: Vec<String>,
    pub force_list: Vec<String>,
}
