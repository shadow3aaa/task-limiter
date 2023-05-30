use std::collections::HashSet;

use rayon::prelude::*;

use crate::misc::exec_cmd;

pub struct LruDumper {
    original_dump: String,
    third_apps: HashSet<String>,
}

impl LruDumper {
    pub fn dump(third_apps: HashSet<String>) -> Self {
        let lru = exec_cmd("dumpsys", &["activity", "lru"]).expect("Failed to get the lru list");
        Self {
            original_dump: lru,
            third_apps,
        }
    }

    pub fn filter_nap(&self) -> HashSet<u32> {
        self.original_dump
            .par_lines()
            .filter_map(|lru| {
                if lru.contains("TOP")
                    || lru.contains("FGS")
                    || lru.contains("LCMN")
                    || lru.contains("IMPF")
                {
                    return None;
                }
                lru.split_whitespace().nth(4)
            })
            .filter_map(|info| {
                let info = info.split('/').next().unwrap_or(info);
                let pid = info.split(':').next()?.parse::<u32>().ok()?;
                let pkg = info.split(':').nth(1)?;

                if !self.third_apps.contains(pkg) {
                    return None;
                }
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }

    pub fn filter_awake(&self) -> HashSet<u32> {
        self.original_dump
            .par_lines()
            .filter_map(|lru| {
                if !(lru.contains("TOP")
                    || lru.contains("FGS")
                    || lru.contains("LCMN")
                    || lru.contains("IMPF"))
                {
                    return None;
                }
                lru.split_whitespace().nth(4)
            })
            .filter_map(|info| {
                let info = info.split('/').next().unwrap_or(info);
                let pid = info.split(':').next()?.parse::<u32>().ok()?;
                let pkg = info.split(':').nth(1)?;

                if !self.third_apps.contains(pkg) {
                    return None;
                }
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }
}
