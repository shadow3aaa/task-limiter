use std::collections::HashSet;

use rayon::prelude::*;
use libc::{kill, SIGCONT, SIGSTOP};

pub struct AppProcessGroup {
    processes: HashSet<u32>,
}

enum Signal {
    Stop,
    Continue,
}

impl AppProcessGroup {
    pub fn new() -> Self {
        Self { processes: HashSet::new() }
    }

    fn kill_with(&self, sign: Signal) {
        let sign = match sign {
            Signal::Stop => SIGSTOP,
            Signal::Continue => SIGCONT,
        };

        self.processes.par_iter().for_each(|pid| {
            unsafe { kill(*pid as _, sign) };
        });
    }

    pub fn awake(&self) {
        self.kill_with(Signal::Continue);
    }

    pub fn nap(&self) {
        self.kill_with(Signal::Stop);
    }

    #[allow(dead_code)]
    pub fn extend<T>(&mut self, other: T) 
    where
        T: IntoIterator<Item = u32>,
    {
        self.processes.extend(other)
    }

    #[allow(dead_code)]
    pub fn par_extend<T>(&mut self, other: T) 
    where
        T: IntoParallelIterator<Item = u32>,
    {
        self.processes.par_extend(other)
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: u32) {
        self.processes.insert(other);
    }
}