mod android_utils;

use crate::{info_sync::InfoSync, Config};

use crate::lru_info::*;
use android_utils::*;

use std::time::Duration;

const MSG_TIMER: Duration = Duration::from_secs(30);
const MSG_TIME_LEN: Duration = Duration::from_secs(1);

pub async fn process(_conf: &mut InfoSync<Config>) {
    let mut third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(10));

    loop {
        let third_apps = third_apps.get().unwrap_or_default();
        let lru_info = LruDumper::dump(third_apps);
    }
}
