use std::collections::HashSet;
use std::process::exit;
use std::sync::Arc;

use crate::misc::exec_cmd;

use log::{debug, error};
use rayon::prelude::*;

pub struct LruDumper {
    original_dump: String,
}

unsafe impl Send for LruDumper {}
unsafe impl Sync for LruDumper {}

impl LruDumper {
    pub async fn dump() -> Self {
        let lru = match exec_cmd("dumpsys", &["activity", "lru"]) {
            Ok(o) => {
                debug!("Successfully called dumpsys activity lru");
                o
            }
            Err(_) => {
                error!("Fail to dumpsys activity lru");
                exit(1);
            }
        };
        Self { original_dump: lru }
    }

    pub fn need_nap_in(&self, list: &HashSet<String>) -> HashSet<u32> {
        if list.is_empty() {
            debug!("Nothing add to nap list");
            return HashSet::new();
        }
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
                debug!("Add {}:{} to nap list", &pkg, &pid);
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }

    pub fn need_awake_in(&self, list: &HashSet<String>) -> HashSet<u32> {
        if list.is_empty() {
            debug!("Nothing to wake up");
            return HashSet::new();
        }
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
                debug!("Wake {}:{} up", &pkg, &pid);
                Some(pid)
            })
            .collect::<HashSet<u32>>()
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
