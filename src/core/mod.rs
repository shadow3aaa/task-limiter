mod android_utils;
mod info;

use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use crate::info_sync::InfoSync;
use crate::lru_info::*;
use crate::nap_group::*;
use crate::Config;
use android_utils::*;
use info::*;

use log::{debug, error, info};
use tokio::time::sleep;

// Todo: 可配置(目前无必要)
const SLEEP_TIME: Duration = Duration::from_millis(220);
const SIMP_NAP_TIME: Duration = Duration::from_secs(1);
const SIMP_AWAKE_TIME: Duration = Duration::from_millis(5);
const MSG_NAP_TIME: Duration = Duration::from_secs(1);
const MSG_AWAKE_TIME: Duration = Duration::from_millis(10);

pub async fn process(conf: Arc<InfoSync<Config>>) {
    // 非堵塞的自动更新信息
    info!("Creating a third-party app list auto-updater");
    let third_apps = InfoSync::new_timer(get_third_party_apps, Duration::from_secs(30)).arc();
    info!("Creating a common app list auto-updater");
    let simp_apps = simple_apps(conf.clone(), third_apps.clone()).arc();
    info!("Creating a instant messaging app list auto-updater");
    let msg_apps = msg_apps(conf.clone(), third_apps.clone()).arc();

    // 创建休眠组
    info!("Creating a normal app nap group");
    let simp_group = NapGroup::new(MSG_NAP_TIME, MSG_AWAKE_TIME).arc();
    info!("Creating instant messaging a normal app nap group");
    let msg_group = NapGroup::new(SIMP_NAP_TIME, SIMP_AWAKE_TIME).arc();

    loop {
        let sleeper = sleep(SLEEP_TIME);
        // 调用 dumpsys 读取应用 lru(last recent used) 信息
        debug!("Call dumpsys to read LRU (last recently used) apps information");
        let lru_info = tokio::spawn(LruDumper::dump());
        // 获取从通过配置和第三方应用列表整合的可控列表
        debug!("Get controllable lists integrated from configurations and third-party app lists");
        let simp_apps = Arc::new(simp_apps.clone().try_get().unwrap_or_default());
        let msg_apps = Arc::new(msg_apps.clone().try_get().unwrap_or_default());

        // 创建异步分析信息任务
        debug!("Create an asynchronous Analyze Information task");
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
        debug!("Wait for the analysis result");
        let (simp_nap, msg_nap, simp_wake, msg_wake) =
            match tokio::try_join!(simp_nap, msg_nap, simp_wake, msg_wake) {
                Ok(o) => o,
                Err(e) => {
                    error!("Analysis failed! Cause of the error:");
                    error!("{}", e);
                    exit(1);
                }
            };

        // 异步地执行休眠和唤醒
        debug!("process nap and wake asynchronously");
        if let Err(e) = tokio::try_join!(
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
        ) {
            error!("Run-time error:");
            error!("{}", e);
            exit(1);
        }
        sleeper.await;
    }
}
