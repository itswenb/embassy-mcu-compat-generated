# 统一生成仓库

- 目录固定为 `data/{chips,registers}` 和统一的 `mcu-metapac/`；不是按系列拆包，也不把Cargo包放仓库根。
- 只保留GD及后续明确支持的兼容MCU数据，不复制ST全型号数据；必要的上游共享定义和许可声明保留。
- Cargo包名 `stm32-metapac` 是官方Embassy patch的依赖契约，不代表目录名或ST支持范围。修改此契约须先说明对上游依赖的影响。
- 当前仅开放GD32F30x理论集成，硬件未验证；其他系列保留待支持状态，不伪造支持。
- 仅交付生成数据、源码、构建入口、用户说明、支持清单和许可信息。报告、候选标记、日志、生成过程清单留在工具库或验证库。
- 发布文件不得含本机绝对路径。禁止复制厂商原始源码，控制缓存，不重复整包和日志副本。
- 简体中文；Shell命令使用rtk，编辑使用apply_patch。保留用户修改，不改上游Embassy/stm32-data。
- 只在embassy-mcu-compat-validation验证，不操作imu-matrix-new、不自动烧录。GitHub自动更新保持关闭。
