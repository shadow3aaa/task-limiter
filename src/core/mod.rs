mod android_utils;

use crate::{info_sync::InfoSync, Config};

use crate::lru_info::*;
use android_utils::*;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

const MSG_TIMER: Duration = Duration::from_secs(30);
const MSG_TIME_LEN: Duration = Duration::from_secs(1);

pub async fn process(conf: Arc<InfoSync<Config>>) {
    let mut third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(30)).arc();

    let mut simp_apps = InfoSync::new_blocker(move || {
        let conf = conf.get().unwrap();
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
    });

    loop {
        let third_apps = third_apps.try_get().unwrap_or_default();
        let lru_info = LruDumper::dump(third_apps);
    }
}
