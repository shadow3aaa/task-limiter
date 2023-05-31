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
use tokio::time::sleep;

const SLEEP_TIME: Duration = Duration::from_millis(300);
const SIMP_NAP_TIME: Duration = Duration::from_secs(1);
const SIMP_AWAKE_TIME: Duration = Duration::from_millis(10);
const MSG_NAP_TIME: Duration = Duration::from_secs(3);
const MSG_AWAKE_TIME: Duration = Duration::from_millis(500);

pub async fn process(conf: Arc<InfoSync<Config>>) {
    // 非堵塞的自动更新信息
    let third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(30)).arc();
    let simp_apps = simple_apps(conf.clone(), third_apps.clone()).arc();
    let msg_apps = msg_apps(conf.clone(), third_apps.clone()).arc();

    // 创建休眠池
    let simp_group = NapGroup::new(MSG_NAP_TIME, MSG_AWAKE_TIME).arc();
    let msg_group = NapGroup::new(SIMP_NAP_TIME, SIMP_AWAKE_TIME).arc();

    loop {
        let sleeper = sleep(SLEEP_TIME);
        // 调用 dumpsys 读取应用 lru(last recent used) 信息
        let lru_info = tokio::spawn(LruDumper::dump());
        // 获取访问通过配置和第三方应用列表整合的可控列表
        let simp_apps = Arc::new(simp_apps.clone().try_get().unwrap_or_default());
        let msg_apps = Arc::new(msg_apps.clone().try_get().unwrap_or_default());

        // 创建异步分析信息任务
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
        // 等待分析结果
        let (simp_nap, msg_nap, simp_wake, msg_wake) =
            tokio::try_join!(simp_nap, msg_nap, simp_wake, msg_wake).unwrap();

        // 异步地执行休眠和唤醒
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
            }),
        )
        .unwrap();
        sleeper.await;
    }
}
