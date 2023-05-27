#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PidType {
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

pub fn read_comm(pid: u32) -> Option<String> {
    use std::fs;
    let comm = format!("/proc/{}/cmdline", pid);
    let comm = fs::read_to_string(comm).ok()?;
    Some(comm.trim().trim_matches('\0').into())
}

pub fn same_app(comm: &str, app: &str) -> bool {
    if comm == app {
        true
    } else if let Some(o) = comm.split(':').next() {
        o == app
    } else {
        false
    }
}
