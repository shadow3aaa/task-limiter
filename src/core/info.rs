use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use crate::{info_sync::InfoSync, Config};

type Info<T> = Arc<InfoSync<T>>;
type AppList = HashSet<String>;

pub fn simple_apps(conf: Info<Config>, third_apps: Info<AppList>) -> InfoSync<AppList> {
    InfoSync::new_timer(
        move || {
            let conf = conf.try_get().unwrap_or_default();
            third_apps
                .try_get()
                .unwrap_or_default()
                .iter()
                .filter(|app| {
                    !(conf.white_list.contains(app) || conf.msg_list.contains(app))
                        || conf.force_list.contains(app)
                })
                .map(|s| s.to_string())
                .collect::<HashSet<String>>()
        },
        Duration::from_secs(10),
    )
}

pub fn msg_apps(conf: Info<Config>, third_apps: Info<AppList>) -> InfoSync<AppList> {
    InfoSync::new_timer(
        move || {
            let conf = conf.get().unwrap();
            third_apps
                .try_get()
                .unwrap_or_default()
                .iter()
                .filter(|app| conf.msg_list.contains(app))
                .map(|s| s.to_string())
                .collect::<HashSet<String>>()
        },
        Duration::from_secs(10),
    )
}
