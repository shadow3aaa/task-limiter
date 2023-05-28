mod processes_group;

use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use rayon::prelude::*;

use self::processes_group::AppProcessGroup;

pub struct NapGroup {
    group: Arc<AppProcessGroup>,
    sleeply_handle: JoinHandle<()>,
}

impl NapGroup {
    pub fn new(nap_time: Duration, awaken_time: Duration) -> Self
    {
        let group = Arc::new(AppProcessGroup::new());
        let group_handle = group.clone();
        let sleeply_handle = thread::spawn(move || {
            let group = group_handle;
            loop {
                group.nap();
                thread::sleep(nap_time);
                group.awake();
                thread::sleep(awaken_time);
            }
        });
        Self { group, sleeply_handle }
    }

    #[allow(dead_code)]
    pub fn extend<T>(&mut self, other: T) 
    where
        T: IntoIterator<Item = u32>,
    {
        self.group.extend(other)
    }

    #[allow(dead_code)]
    pub fn par_extend<T>(&mut self, other: T) 
    where
        T: IntoParallelIterator<Item = u32>,
    {
        self.group.par_extend(other)
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: u32) {
        self.group.add(other);
    }
}