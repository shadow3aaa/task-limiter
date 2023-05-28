#[allow(unused_imports)]
use super::{BG_CTL, BG_SET};

use crate::misc::exec_cmd;

use rayon::prelude::*;

use std::{collections::HashSet, fs};

pub fn get_lru() -> String {
    let lru = exec_cmd("dumpsys", &["activity", "lru"])
        .expect("Failed to get the lru list");
    lru
}

#[allow(dead_code)]
pub fn get_top_apps() -> Vec<String> {
    let top_apps = get_lru();
    let mut result: Vec<String> = top_apps
        .par_lines()
        .filter(|l| l.contains("TOP"))
        .filter_map(|line| {
            let line = line.split_whitespace().nth(4)?.split(':').nth(1)?;
            match line.split('/').nth(0) {
                Some(o) => Some(o),
                None => Some(line),
            }
        })
        .map(|s| s.to_string())
        .collect();
    result.sort();
    result
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
    [BG_SET, BG_CTL].iter().for_each(|procs| {
        if let Ok(o) = fs::read_to_string(procs) {
            bg_pids.par_extend(o.par_lines().filter_map(|l| l.parse::<u32>().ok()));
        }
    });
    bg_pids
}
