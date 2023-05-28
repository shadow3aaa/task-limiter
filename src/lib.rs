pub mod config;
pub mod core;
pub mod info_sync;
pub mod blocker;
pub mod misc;
pub mod nap_group;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub white_list: Vec<String>,
    pub msg_list: Vec<String>,
    pub force_list: Vec<String>,
}
