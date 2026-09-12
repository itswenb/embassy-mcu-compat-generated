# 上游来源与许可声明

本文件记录交付所用来源，不为任何第三方代码重新授权。各文件原有的版权、许可和来源标记应保持不变。

## 兼容PAC与metadata

基础生成产物来自 `embassy-rs/stm32-data-generated` 提交 `ac8849c0c0941fdf46901d26b8d68d723db307eb`。其 `stm32-metapac/Cargo.toml` 声明包名stm32-metapac、版本21.0.0、许可 `MIT OR Apache-2.0`。本批次保留该声明，未以本文件替换或扩大其适用范围。

Flash XL所引用的官方PAC来自同仓库提交 `4e2c788baad277419b2571dba360b76960b36b00`，对应stm32-data提交 `464ea6903382eed594e4cf2ff8aaf4537c74f493`。

本批次对metadata、外设名称与有证据的行为差异进行了适配，不是未经修改的官方stm32-metapac发行物。具体修改由generation及Flash、TIMER、USBFS、ENET等适配档案追溯；不因此宣称官方支持GD32。

## HAL依赖

用户应用依赖官方 `embassy-rs/embassy` 提交 `bd35bd7a7d19147c8bcae9c2eda99b25e597294c`，本PAC包不包含或替换整个HAL仓库。该仓库LICENSE-MIT与LICENSE-APACHE正文使用署名 `Copyright (c) Embassy project contributors`；这条记录针对该仓库，不把其署名直接套用于所有PAC或厂商输入。

## 厂商资料

GigaDevice手册、Firmware、Builder资源及其他原始输入仅用于分析硬件事实和建立来源证据；不作为本兼容PAC交付包的原始源码附带分发。相应版本、定义和哈希保留在工具库批次来源档案。

## 许可材料核对状态

已检查的锁定stm32-data-generated树、相邻stm32-data树及本地注册表stm32-metapac-21.0.0包未找到独立LICENSE/COPYING文本，但有上述包许可声明。此处明确区分“来源未附正文”和“来源未声明许可”，不把二者混写；不编造缺失的版权持有人或声称已经取得额外授权。

正式打包仍须保留原有许可字段和文件内声明，并核对最终文件清单中是否引入了需要单独处理许可的其他来源。此记录不代替法律意见。
