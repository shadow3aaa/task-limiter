# task-limiter

### 这是Task limiter模块的源码仓库

* 介绍
Task Limiter 是一个实现类似墓碑效果的Magisk模块

特点是大量采用异步运行，理论上需要墓碑的应用多了(非常多的时候)速度有优势

~~完全开源也算优点吗~~


* 编译:
```shell
# 先clone仓库(省略)
cd task_limiter
bash build.sh
```
ps: 在termux编译，只需要此依赖
```shell
apt install rust
```
其它地方编译到 aarch64-linux-android 比较复杂，至少我的archlinux on wsa没成功()

因此作为代替，如果到aarch64-linux-android 失败，build.sh 会尝试编译为 aarch64-unknown-linux-musl

特点是内存占用极小，不过性能应该没有aarch64-linux-android好

因此你可能需要这样安装 aarch64-unknown-linux-musl target (如果还没有)
```shell
rustup target add aarch64-unknown-linux-musl
```

* 打包模块
复制编译好的二进制到 MagiskModule 目录，然后把里面的文件全选，压缩为zip即可
