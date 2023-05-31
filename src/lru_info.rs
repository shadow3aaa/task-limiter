use std::collections::HashSet;
use std::sync::Arc;

use rayon::prelude::*;

use crate::misc::exec_cmd;

pub struct LruDumper {
    original_dump: String,
}

unsafe impl Send for LruDumper {}
unsafe impl Sync for LruDumper {}

impl LruDumper {
    pub async fn dump() -> Self {
        let lru = exec_cmd("dumpsys", &["activity", "lru"]).expect("Failed to get the lru list");
        Self { original_dump: lru }
    }

    pub fn need_nap_in(&self, list: &HashSet<String>) -> HashSet<u32> {
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

                if !list.contains(pkg) {
                    return None;
                }
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }

    pub fn need_awake_in(&self, list: &HashSet<String>) -> HashSet<u32> {
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

                if !list.contains(pkg) {
                    return None;
                }
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
