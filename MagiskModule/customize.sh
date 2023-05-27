SKIPUNZIP=0

ui_print "请等待…"
ui_print "Please wait…"

# permission
chmod a+x "$MODPATH/task_limiter"

# config
config_dir=/sdcard/Android/TaskLimiter
mkdir -p "$config_dir"
[ ! -f "$config_dir/config.toml" ] &&
	cp "$MODPATH/config_ori.toml" "$config_dir/config.toml"
