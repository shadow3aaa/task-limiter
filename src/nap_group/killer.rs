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
