use std::fs;

use libc::{kill, SIGCONT, SIGSTOP};

#[derive(Copy, Clone, Debug)]
pub enum Signal {
    Stop,
    Continue,
    Alive, // 信号0，kill发送这个返回true代表pid存活
}

pub fn killer(pid: u32, sign: Signal) -> bool {
    // println!("pid: {}, sign: {:?}", pid, sign);
    let sign = match sign {
        Signal::Stop => SIGSTOP,
        Signal::Continue => SIGCONT,
        Signal::Alive => 0,
    };
    0 == unsafe { kill(pid as _, sign) }
}

pub fn promised_app_killer(pid: u32, app: &str, sign: Signal) -> bool {
    let cmd = format!("/proc/{}/cmdline", &pid);
    let cmd = match fs::read_to_string(cmd) {
        Ok(o) => o,
        Err(_) => return false,
    };
    if !cmd.trim().trim_matches('\0').contains(app) {
        return false;
    }
    killer(pid, sign)
}
