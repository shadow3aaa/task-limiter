use super::exec_cmd;

pub fn get_top_apps() -> Vec<String> {
    let top_apps = exec_cmd("dumpsys", &["window", "visible-apps"])
        .expect("Failed to get the top-level app list");
    top_apps
        .lines()
        .filter(|l| l.contains("Window #"))
        .map(|a| {
            a.split_whitespace()
                .nth(4)
                .unwrap()
                .split('/')
                .nth(0)
                .unwrap()
                .to_string()
        })
        .collect()
}

pub fn get_third_party_apps() -> String {
    let apps = exec_cmd("pm", &["list", "packages", "-3"]).expect("Failed to get list of apps");
    apps.lines().map(|l| l.split(':').nth(1).unwrap()).collect()
}

pub fn get_system_apps() -> String {
    let apps = exec_cmd("pm", &["list", "packages", "-s"]).expect("Failed to get list of apps");
    apps.lines().map(|l| l.split(':').nth(1).unwrap()).collect()
}
