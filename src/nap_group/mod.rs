mod group;
mod killer;

use std::collections::HashMap;
use std::sync::Arc;

use group::AppProcessGroup;
use killer::*;

use parking_lot::RwLock;
use rayon::prelude::*;
use tokio::time::{sleep, Duration};

type PidApp = HashMap<u32, String>;

pub struct NapGroup {
    group: Arc<RwLock<AppProcessGroup>>,
}

async fn awake_for_dur(
    group: Arc<RwLock<AppProcessGroup>>,
    nap_time: Duration,
    awaken_time: Duration,
) {
    sleep(nap_time).await;
    group.read().awake();
    sleep(awaken_time).await;
    if let Some(group) = group.try_read() {
        group.nap();
    }
}

async fn retian_alive(group: Arc<RwLock<AppProcessGroup>>) {
    let new_group = AppProcessGroup::from(
        group
            .read()
            .processes
            .par_iter()
            .filter(|(pid, app)| {
                if !promised_app_killer(**pid, app, Signal::Alive) {
                    killer(**pid, Signal::Continue);
                    false
                } else {
                    true
                }
            })
            .map(|(pid, app)| (*pid, app.to_string()))
            .collect::<PidApp>(),
    );
    *group.write() = new_group;
}

impl NapGroup {
    pub fn new(nap_time: Duration, awaken_time: Duration) -> Self {
        let group = Arc::new(RwLock::new(AppProcessGroup::new()));
        group.read().nap();
        Self { group }.init(nap_time, awaken_time)
    }

    fn init(self, nap_time: Duration, awaken_time: Duration) -> Self {
        let group = self.group.clone();
        tokio::spawn(async move {
            loop {
                let nap_task = tokio::spawn(awake_for_dur(group.clone(), nap_time, awaken_time));
                let alive_task = tokio::spawn(retian_alive(group.clone()));
                let (_, _) = tokio::join!(nap_task, alive_task);
            }
        });
        self
    }

    #[inline(always)]
    pub fn put_them_nap(&self, other: PidApp) {
        if other.is_empty() {
            return;
        }
        if let Some(mut group) = self.group.try_write() {
            group.processes.extend(other);
        }
    }

    pub fn wake_them_up(&self, remove: PidApp) {
        if remove.is_empty() {
            return;
        }
        self.group.write().processes.retain(|pid, _| {
            if remove.contains_key(pid) {
                killer(*pid, Signal::Continue);
                false
            } else {
                true
            }
        });
    }

    pub fn arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
