mod android_utils;

use crate::info_sync::*;
use crate::misc;
use crate::Config;
use cpulimiter::{CpuLimit, Pid};
use rayon::prelude::*;
use std::collections::HashSet;

use std::time::Duration;

pub(crate) const BG_CTL: &str = "/dev/cpuctl/background/cgroup.procs";
pub(crate) const BG_SET: &str = "/dev/cpuset/background/cgroup.procs";
const LIMIT_PERCENTAGE: f64 = 3.0;
const MSG_TIMER: Duration = Duration::from_secs(30);
const MSG_TIME_LEN: Duration = Duration::from_secs(6);

#[derive(Hash, Eq, PartialEq)]
enum PidType {
    SimpApp(u32),
    MsgApp(u32),
}

impl PidType {
    pub fn as_u32(&self) -> u32 {
        match self {
            PidType::SimpApp(o) => *o,
            PidType::MsgApp(o) => *o,
        }
    }
}

type Limiters = Vec<CpuLimit>;
type PidSet = HashSet<PidType>;
pub fn process(conf: Config) {
    let mut limiters = Limiters::new();
    let mut third_apps =
        InfoSync::new(android_utils::get_third_party_apps, Duration::from_secs(10));

    loop {
        // 用inotify堵塞循环直到更新
        misc::inotify_block([BG_CTL, BG_SET]).expect("inotify failed to monitor procs");

        // 读取pid，并且过滤
        let pids = android_utils::read_bg_pids();
        let third_apps = third_apps.get().unwrap_or_default();
        let white_list = &conf.white_list;
        let msg_list = &conf.msg_list;
        let force_list = &conf.force_list;

        // 过滤
        let pids = pids
            .into_par_iter()
            .filter_map(|pid| {
                let comm = match read_comm(pid) {
                    Some(o) => o,
                    None => return None,
                };
                if white_list.par_iter().any(|app| same_app(&comm, app)) {
                    None
                } else if msg_list.par_iter().any(|app| same_app(&comm, app)) {
                    Some(PidType::MsgApp(pid))
                } else if force_list.par_iter().any(|app| same_app(&comm, app))
                    || third_apps.par_iter().any(|app| same_app(&comm, app))
                {
                    Some(PidType::SimpApp(pid))
                } else {
                    None
                }
            })
            .collect::<PidSet>();

        // 首先drop不再需要的
        // CpuLimiter crate原版没有自定义drop行为
        // 因此用原版直接drop会产生垃圾线程，而且并不能停止限制
        // 所以这里用的是fork来的CpuLimiter，提供了一些原版没有的特性
        limiters = limiters
            .into_par_iter()
            .filter(|limiter| {
                if !limiter.alive() {
                    false
                } else {
                    let lim_pid = limiter.pid().as_u32();
                    pids.par_iter().any(|pid| lim_pid == pid.as_u32())
                }
            })
            .collect();

        // Pidset中重过滤掉不需要动的那些
        let pids = pids
            .into_par_iter()
            .filter(|pid| {
                limiters
                    .par_iter()
                    .any(|lim| pid.as_u32() != lim.pid().as_u32())
            })
            .collect::<PidSet>();

        // 从剩下的PidSet创建新的CpuLimit
        let new_limiters = pids
            .into_par_iter()
            .filter_map(|pid| {
                let limiter = CpuLimit::new(Pid::from(pid.as_u32()), LIMIT_PERCENTAGE).ok()?;
                if let PidType::MsgApp(_p) = pid {
                    Some(limiter.with_timer_suspend(MSG_TIMER, MSG_TIME_LEN))
                } else {
                    Some(limiter)
                }
            })
            .collect::<Limiters>();
        limiters.par_extend(new_limiters);
    }
}

fn read_comm(pid: u32) -> Option<String> {
    use std::fs;
    let comm = format!("/proc/{}/comm", pid);
    let comm = fs::read_to_string(comm).ok()?;
    Some(comm.trim().into())
}

fn same_app(comm: &str, app: &str) -> bool {
    if comm == app {
        true
    } else if let Some(o) = comm.split(':').next() {
        o == app
    } else {
        false
    }
}