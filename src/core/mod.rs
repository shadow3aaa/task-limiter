mod android_utils;
mod info;

use android_utils::*;
use info::*;

use crate::{info_sync::InfoSync, Config};

use crate::lru_info::*;

use std::sync::Arc;
use std::time::Duration;

const MSG_TIMER: Duration = Duration::from_secs(30);
const MSG_TIME_LEN: Duration = Duration::from_secs(1);

pub async fn process(conf: Arc<InfoSync<Config>>) {
    let third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(30)).arc();
    let simp_apps = simple_apps(conf.clone(), third_apps.clone());
    let msg_apps = msg_apps(conf.clone(), third_apps.clone());

    loop {
        let lru_info = LruDumper::dump(third_apps.try_get().unwrap_or_default());
    }
}
