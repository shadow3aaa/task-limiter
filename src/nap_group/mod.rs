mod group;
mod killer;

use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::RwLock;
use rayon::prelude::*;

use group::AppProcessGroup;
use killer::*;

pub struct NapGroup {
    group: Arc<RwLock<AppProcessGroup>>,
}

impl NapGroup {
    pub fn new(_nap_time: Duration, _awaken_time: Duration) -> Self {
        let group = Arc::new(RwLock::new(AppProcessGroup::new()));
        group.read().nap();
        Self { group }
    }

    #[inline(always)]
    pub fn put_them_nap<T>(&mut self, other: T)
    where
        T: IntoIterator<Item = u32>,
    {
        self.group.write().processes.extend(other)
    }

    pub fn wake_them_up<T>(&mut self, remove: &mut T)
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

    pub async fn awake_for_dur(&self, awaken_time: Duration) {
        self.group.read().awake();
        thread::sleep(awaken_time);
        self.group.read().nap();
    }

    pub async fn retian_alive(&self) {
        let new_group = AppProcessGroup::from(
            self.group
                .read()
                .processes
                .par_iter()
                .filter(|pid| killer(**pid, Signal::Alive))
                .map(|pid| *pid)
                .collect::<HashSet<u32>>(),
        );
        *self.group.write() = new_group;
    }
}
