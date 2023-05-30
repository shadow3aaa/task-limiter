use crate::misc::exec_cmd;

use rayon::prelude::*;

use std::collections::HashSet;

pub fn get_third_party_apps() -> HashSet<String> {
    let apps = exec_cmd("pm", &["list", "packages", "-3"]).expect("Failed to get list of apps");
    apps.par_lines()
        .filter_map(|l| l.split(':').nth(1))
        .map(|s| s.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn get_system_apps() -> HashSet<String> {
    let apps = exec_cmd("pm", &["list", "packages", "-s"]).expect("Failed to get list of apps");
    apps.par_lines()
        .filter_map(|l| l.split(':').nth(1))
        .map(|s| s.to_string())
        .collect()
}
