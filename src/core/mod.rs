mod android_utils;
mod pid_utils;

use crate::{info_sync::*, misc, Config};

use android_utils::*;
use pid_utils::*;

use std::collections::HashSet;
use std::time::Duration;

use cpulimiter::{CpuLimit, Pid};
use rayon::prelude::*;

#[allow(dead_code)]
pub(crate) const BG_CTL: &str = "/dev/cpuctl/background/cgroup.procs";
#[allow(dead_code)]
pub(crate) const BG_SET: &str = "/dev/cpuset/background/cgroup.procs";
const LIMIT_PERCENTAGE: f64 = 3.0;
const MSG_TIMER: Duration = Duration::from_secs(30);
const MSG_TIME_LEN: Duration = Duration::from_secs(6);

type Limiters = Vec<CpuLimit>;
type PidSet = HashSet<PidType>;
pub async fn process(mut conf: InfoSync<Config>) {
    let mut limiters = Limiters::new();
    let mut third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(10));

    loop {
        let do_lim = limiters_process(&mut conf, limiters, &mut third_apps);
        // 用inotify堵塞循环直到更新
        misc::inotify_block([BG_SET]).expect("Failed to block by inotify");
        limiters = do_lim.await;
    }
}

async fn limiters_process(conf: &mut InfoSync<Config>, mut limiters: Limiters, third_apps: &mut InfoSync<Vec<String>>) -> Limiters {
// 读取pid，并且过滤
    let pids = read_bg_pids();
    // println!("pids_ori: {}", pids.len());

    let conf = match conf.get() {
        Some(o) => o,
        None => return limiters,
    };
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
    // println!("pids after filter: {:?}", &pids);

    // 首先drop不再需要的
    // CpuLimiter crate原版没有自定义drop行为
    // 因此用原版直接drop会产生垃圾线程，而且并不能停止限制
    // 所以这里用的是fork来的CpuLimiter，提供了一些原版没有的特性
    limiters = limiters
        .into_par_iter()
        .filter(|limiter| {
            if !limiter.alive() || !limiter.pid().alive() {
                // println!("limiter/process dead!");
                false
            } else {
                let lim_pid = limiter.pid().as_u32();
                pids.is_empty() || pids.par_iter().any(|pid| lim_pid == pid.as_u32())
            }
        })
        .collect();
    // println!("limiters count: {}", limiters.len());

    // Pidset中重过滤掉不需要动的那些
    let pids = pids
        .into_par_iter()
        .filter(|pid| {
            limiters.is_empty() || !limiters
                .par_iter()
                .any(|lim| pid.as_u32() == lim.pid().as_u32())
        })
        .collect::<PidSet>();
    // println!("pids after filter twice: {:?}", &pids);

    // 从剩下的PidSet创建新的CpuLimit
    let new_limiters = pids
        .into_par_iter()
        .filter_map(|pid| {
            let limiter = CpuLimit::new(Pid::from(pid.as_u32()), LIMIT_PERCENTAGE).ok()?;
            if let PidType::MsgApp(_) = pid {
                let _ = limiter.set_slice(Duration::from_millis(500));
                Some(limiter.with_timer_suspend(MSG_TIMER, MSG_TIME_LEN))
            } else {
                let _ = limiter.set_slice(Duration::from_millis(400));
                Some(limiter)
            }
        })
        .collect::<Limiters>();
    limiters.par_extend(new_limiters);
    
    // println!("limiter count: {}", limiters.len());
    // limiters.par_iter().for_each(|lim| println!("app: {}, ", read_comm(lim.pid().as_u32()).unwrap()));
    limiters
}