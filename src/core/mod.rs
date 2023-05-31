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
    let simp_apps = simple_apps(conf.clone(), third_apps.clone()).arc();
    let msg_apps = msg_apps(conf.clone(), third_apps.clone()).arc();

    let simp_group = NapGroup::new(MSG_NAP_TIME, MSG_AWAKE_TIME).arc();
    let msg_group = NapGroup::new(SIMP_NAP_TIME, SIMP_AWAKE_TIME).arc();

    loop {
        let lru_info = tokio::spawn(LruDumper::dump());
        let simp_apps = Arc::new(simp_apps.clone().try_get().unwrap_or_default());
        let msg_apps = Arc::new(msg_apps.clone().try_get().unwrap_or_default());

        let lru_info = lru_info.await.unwrap().arc();
        let simp_nap = tokio::spawn({
            let lru = lru_info.clone();
            let simp_apps = simp_apps.clone();
            async move { lru.need_nap_in(&simp_apps) }
        });
        let msg_nap = tokio::spawn({
            let lru = lru_info.clone();
            let msg_apps = msg_apps.clone();
            async move { lru.need_nap_in(&msg_apps) }
        });
        let simp_wake = tokio::spawn({
            let lru = lru_info.clone();
            let simp_apps = simp_apps.clone();
            async move { lru.need_awake_in(&simp_apps) }
        });
        let msg_wake = tokio::spawn({
            let lru = lru_info.clone();
            let msg_apps = msg_apps.clone();
            async move { lru.need_awake_in(&msg_apps) }
        });
        let (simp_nap, msg_nap, simp_wake, msg_wake) =
            tokio::try_join!(simp_nap, msg_nap, simp_wake, msg_wake).unwrap();

        tokio::try_join!(
            tokio::spawn({
                let simp_group = simp_group.clone();
                async move { simp_group.put_them_nap(simp_nap) }
            }),
            tokio::spawn({
                let simp_group = simp_group.clone();
                async move { simp_group.wake_them_up(simp_wake) }
            }),
            tokio::spawn({
                let msg_group = msg_group.clone();
                async move { msg_group.put_them_nap(msg_nap) }
            }),
            tokio::spawn({
                let msg_group = msg_group.clone();
                async move { msg_group.wake_them_up(msg_wake) }
            })
        )
        .unwrap();
    }
}
