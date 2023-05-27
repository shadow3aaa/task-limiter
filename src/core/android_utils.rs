#[allow(unused_imports)]
use super::{BG_CTL, BG_SET};

use crate::misc::exec_cmd;

use rayon::prelude::*;

use std::{collections::HashSet, fs};

#[allow(dead_code)]
pub fn get_top_apps() -> Vec<String> {
    let top_apps = exec_cmd("dumpsys", &["window", "visible-apps"])
        .expect("Failed to get the top-level app list");
    top_apps
        .lines()
        .filter(|l| l.contains("Window #"))
        .filter_map(|l| l.split_whitespace().nth(4).unwrap().split('/').next())
        .map(|s| s.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn get_third_party_apps() -> Vec<String> {
    let apps = exec_cmd("pm", &["list", "packages", "-3"]).expect("Failed to get list of apps");
    apps.par_lines()
        .filter_map(|l| l.split(':').nth(1))
        .map(|s| s.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn get_system_apps() -> Vec<String> {
    let apps = exec_cmd("pm", &["list", "packages", "-s"]).expect("Failed to get list of apps");
    apps.par_lines()
        .filter_map(|l| l.split(':').nth(1))
        .map(|s| s.to_string())
        .collect()
}

pub fn read_bg_pids() -> HashSet<u32> {
    let mut bg_pids: HashSet<u32> = HashSet::new();
    [BG_SET].iter().for_each(|procs| {
        if let Ok(o) = fs::read_to_string(procs) {
            bg_pids.par_extend(o.par_lines().filter_map(|l| l.parse::<u32>().ok()));
        }
    });
    bg_pids
}
