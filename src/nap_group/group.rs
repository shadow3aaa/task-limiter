use std::collections::HashMap;

use super::killer::*;

use rayon::prelude::*;

type PidApp = HashMap<u32, String>;

pub struct AppProcessGroup {
    pub processes: PidApp,
}

unsafe impl Send for AppProcessGroup {}
unsafe impl Sync for AppProcessGroup {}

impl AppProcessGroup {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    fn kill_with(&self, sign: Signal) {
        self.processes.par_iter().for_each(|(pid, app)| {
            promised_app_killer(*pid, app, sign);
        });
    }

    pub fn awake(&self) {
        self.kill_with(Signal::Continue);
    }

    pub fn nap(&self) {
        self.kill_with(Signal::Stop);
    }
}

impl From<PidApp> for AppProcessGroup {
    fn from(hash_map: PidApp) -> Self {
        AppProcessGroup {
            processes: hash_map,
        }
    }
}
