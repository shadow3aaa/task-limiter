#!/system/bin/sh
MODDIR=${0%/*}
config_dir=/sdcard/Android/TaskLimiter

wait_until_login() {
	while [ "$(getprop sys.boot_completed)" != "1" ]; do
		sleep 1
	done
	while [ ! -d "/sdcard/Android" ]; do
		sleep 1
	done
}
wait_until_login

sleep 30s

chmod a+x "$MODDIR/task_limiter"
nohup "$MODDIR/task_limiter" "$config_dir/config.toml" >"$config_dir/Output" 2>&1 &
