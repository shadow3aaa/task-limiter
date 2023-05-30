mod group;
mod killer;

use parking_lot::RwLock;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

use group::AppProcessGroup;
use killer::*;

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
    group.read().nap();
}

async fn retian_alive(group: Arc<RwLock<AppProcessGroup>>) {
    let new_group = AppProcessGroup::from(
        group
            .read()
            .processes
            .par_iter()
            .filter(|pid| killer(**pid, Signal::Alive))
            .map(|pid| *pid)
            .collect::<HashSet<u32>>(),
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
                let nap_task = awake_for_dur(group.clone(), nap_time, awaken_time);
                let alive_task = retian_alive(group.clone());
                nap_task.await;
                alive_task.await;
            }
        });
        self
    }

    #[inline(always)]
    pub fn put_them_nap<T>(&self, other: T)
    where
        T: IntoIterator<Item = u32>,
    {
        self.group.write().processes.extend(other)
    }

    pub async fn wake_them_up<T>(&self, remove: &mut T)
    where
        T: IntoIterator<Item = u32> + Iterator,
        <T as Iterator>::Item: PartialEq<u32>,
    {
        self.group.write().processes.retain(|process| {
            if remove.any(|pid| pid == *process) {
                spawn_killer(*process, Signal::Continue);
                false
            } else {
                true
            }
        });
    }
}
