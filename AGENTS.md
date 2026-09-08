# 批次交付仓库

- 本仓库只接收已验收系列批次，解析与适配在相邻 embassy-mcu-compat 工具库进行。
- 操作前先读取工具库 AGENTS.md 和 batches/<系列>/ 档案；简体中文，命令使用 rtk，编辑使用 apply_patch。
- 不恢复历史全厂商混合根包、原生 PAC 或实验候选作为正式交付。
- 未支持设备保留在状态清单，不删除型号掩盖失败；每次只更新指定批次与 README。
- GitHub 自动更新保持关闭；不得发布原始厂商源码，不修改上游 Embassy/stm32-data。
- 验证只使用 embassy-mcu-compat-validation，不使用 imu-matrix-new。
- 用户于2026-09-08限定授权相邻本地 Embassy 的 Flash bank 修复；需要该修复的用法须显式 Cargo patch 到本地路径并标注修改版依赖，不宣称未修改官方 HAL 已支持。不因本地专项通过提前发布整个批次。
