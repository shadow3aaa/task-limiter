mod inotify_utils;

pub use inotify_utils::*;

// 把自身线程绑定到小核
#[allow(dead_code)]
pub fn set_self_sched() {
    let self_pid = &std::process::id().to_string();
    write_file(self_pid, "/dev/cpuset/background/tasks");
}

#[allow(dead_code)]
pub fn exec_cmd(command: &str, args: &[&str]) -> Result<String, i32> {
    use std::process::Command;
    let output = Command::new(command).args(args).output();

    match output {
        Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
        Err(e) => {
            eprintln!("{}", e);
            Err(-1)
        }
    }
}

pub fn write_file(content: &str, path: &str) {
    use std::{
        fs::{set_permissions, OpenOptions},
        io::Write,
        os::unix::fs::PermissionsExt,
    };

    // debug
    // println!("path: {}, value: {}", path, content);

    match set_permissions(path, PermissionsExt::from_mode(0o644)) {
        Ok(()) => {
            match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)
            {
                Ok(mut file) => match file.write_all(content.as_bytes()) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Write failed: {}", e),
                },
                Err(e) => eprintln!("Open failed: {}", e),
            }
        }
        Err(e) => eprintln!("Set permissions failed: {}", e),
    }
}
