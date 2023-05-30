use libc::{kill, SIGCONT, SIGSTOP};

#[derive(Copy, Clone)]
pub enum Signal {
    Stop,
    Continue,
    Alive, // 信号0，kill发送这个返回true代表pid存活
}

pub fn spawn_killer(pid: u32, sign: Signal) {
    tokio::spawn(async move {
        killer(pid, sign);
    });
}

pub fn killer(pid: u32, sign: Signal) -> bool {
    let sign = match sign {
        Signal::Stop => SIGSTOP,
        Signal::Continue => SIGCONT,
        Signal::Alive => 0,
    };
    0 == unsafe { kill(pid as _, sign) }
}
