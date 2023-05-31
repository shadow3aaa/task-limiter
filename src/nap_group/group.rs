use std::collections::HashSet;

use super::killer::{self, Signal};

use rayon::prelude::*;

pub struct AppProcessGroup {
    pub processes: HashSet<u32>,
}

unsafe impl Send for AppProcessGroup {}
unsafe impl Sync for AppProcessGroup {}

impl AppProcessGroup {
    pub fn new() -> Self {
        Self {
            processes: HashSet::new(),
        }
    }

    fn kill_with(&self, sign: Signal) {
        self.processes.par_iter().for_each(|pid| {
            killer::killer(*pid, sign);
        });
    }

    pub fn awake(&self) {
        self.kill_with(Signal::Continue);
    }

    pub fn nap(&self) {
        self.kill_with(Signal::Stop);
    }
}

impl From<HashSet<u32>> for AppProcessGroup {
    fn from(hash_set: HashSet<u32>) -> Self {
        AppProcessGroup {
            processes: hash_set,
        }
    }
}
