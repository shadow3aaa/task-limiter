mod android_utils;
mod info;

use crate::info_sync::InfoSync;
use crate::lru_info::*;
use crate::nap_group::*;
use crate::Config;
use android_utils::*;
use info::*;

use std::sync::Arc;
use std::time::Duration;

const SIMP_NAP_TIME: Duration = Duration::from_secs(30);
const SIMP_AWAKE_TIME: Duration = Duration::from_secs(1);
const MSG_NAP_TIME: Duration = Duration::from_secs(30);
const MSG_AWAKE_TIME: Duration = Duration::from_secs(3);

pub async fn process(conf: Arc<InfoSync<Config>>) {
    let third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(30)).arc();
    let simp_apps = simple_apps(conf.clone(), third_apps.clone());
    let msg_apps = msg_apps(conf.clone(), third_apps.clone());

    let simp_group = NapGroup::new(MSG_NAP_TIME, MSG_AWAKE_TIME);
    let msg_group = NapGroup::new(SIMP_NAP_TIME, SIMP_AWAKE_TIME);

    loop {
        let lru_info = LruDumper::dump(third_apps.try_get().unwrap_or_default());
    }
}
