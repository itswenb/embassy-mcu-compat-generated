# MCU 兼容生成包

这是统一生成仓库中的 Rust PAC 包，当前开放 GD32F30x 理论集成驱动，不保留ST全型号数据。Cargo包名暂保留 `stm32-metapac` 以匹配官方Embassy的依赖；目录名为 `mcu-metapac`，无需修改官方HAL。

## 安装

将本仓库与应用放在同一父目录。应用（或 workspace 根）的 Cargo.toml 中配置：

```toml
[dependencies]
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
embassy-stm32 = { git = "https://github.com/embassy-rs/embassy", rev = "bd35bd7a7d19147c8bcae9c2eda99b25e597294c", features = ["memory-x", "rt", "exti"] }

[patch.crates-io]
stm32-metapac = { path = "../embassy-mcu-compat-generated/mcu-metapac" }

[patch."https://github.com/embassy-rs/stm32-data-generated"]
stm32-metapac = { path = "../embassy-mcu-compat-generated/mcu-metapac" }
```

相对路径以应用 Cargo.toml 所在目录为准。保留应用已有依赖、链接配置和 Cargo.lock。构建必须指定真实型号及该型号对应的 Embassy STM32 feature，例如：

```sh
EMBASSY_MCU_COMPAT_CHIP=gd32f307vg CARGO_ENCODED_RUSTFLAGS=-Clink-arg=-Tlink.x cargo build --release --locked --target thumbv7em-none-eabihf --features embassy-stm32/stm32f103vf
```

已有链接参数的工程应合并 `-Tlink.x`，不要覆盖原有配置。STM32 feature 必须与 `compat.rs` 中该真实型号的兼容 profile 一致；真实芯片由 `EMBASSY_MCU_COMPAT_CHIP` 决定。用 `cargo metadata --locked --format-version 1` 核对实际解析到本包及上述官方提交。

## 设备与兼容profile

| 真实设备 | 兼容 feature |
| --- | --- |
| `gd32f303cb` | `stm32f103vc` |
| `gd32f303cc` | `stm32f103vc` |
| `gd32f303ce` | `stm32f103vc` |
| `gd32f303cg` | `stm32f103vf` |
| `gd32f303rb` | `stm32f103vc` |
| `gd32f303rc` | `stm32f103vc` |
| `gd32f303re` | `stm32f103vc` |
| `gd32f303rg` | `stm32f103vf` |
| `gd32f303ri` | `stm32f103vf` |
| `gd32f303rk` | `stm32f103vf` |
| `gd32f303vb` | `stm32f103vc` |
| `gd32f303vc` | `stm32f103vc` |
| `gd32f303ve` | `stm32f103vc` |
| `gd32f303vg` | `stm32f103vf` |
| `gd32f303vi` | `stm32f103vf` |
| `gd32f303vk` | `stm32f103vf` |
| `gd32f303zc` | `stm32f103vc` |
| `gd32f303ze` | `stm32f103vc` |
| `gd32f303zg` | `stm32f103vf` |
| `gd32f303zi` | `stm32f103vf` |
| `gd32f303zk` | `stm32f103vf` |
| `gd32f305rb` | `stm32f103vf` |
| `gd32f305rc` | `stm32f103vf` |
| `gd32f305re` | `stm32f103vf` |
| `gd32f305rg` | `stm32f103vf` |
| `gd32f305vc` | `stm32f103vf` |
| `gd32f305ve` | `stm32f103vf` |
| `gd32f305vg` | `stm32f103vf` |
| `gd32f305zc` | `stm32f103vf` |
| `gd32f305ze` | `stm32f103vf` |
| `gd32f305zg` | `stm32f103vf` |
| `gd32f307rc` | `stm32f103vf` |
| `gd32f307re` | `stm32f103vf` |
| `gd32f307rg` | `stm32f103vf` |
| `gd32f307vc` | `stm32f103vf` |
| `gd32f307ve` | `stm32f103vf` |
| `gd32f307vg` | `stm32f103vf` |
| `gd32f307zc` | `stm32f103vf` |
| `gd32f307ze` | `stm32f103vf` |
| `gd32f307zg` | `stm32f103vf` |

## 验证与限制

当前 GD32F30x 的指定用例完成806项正向release编译链接及6项预期编译拒绝；报告、原始日志、来源审计和生成记录保留在工具库及独立验证库，不在生成仓库分发。所有实机验证均为未执行。

- RTC仅提供真实PAC，锁定Embassy不提供rtc_v1 HAL。
- USBFS设备模式存在PHY bit6手册/Firmware冲突及GHWCFG4读取待验证风险；不宣称Host、OTG角色切换或低功耗已验证。
- Flash主机模型和擦写接口编译不等于实机擦写；测试前须备份并预留独立擦除区域。
- TIMER实例检查侧重默认CH1 PWM和BASIC计数，不涵盖全部重映射、互补输出、死区或输入捕获。
- ADC/DAC精度、波形、ADC DMA扫描和内部通道未完整验证；EXMC检查不等于外部存储读写通过。
- SPI/I2C/UART其他路由、模式和故障恢复，以及CAN/ENET/SDIO通信、看门狗复位时序仍需按实际硬件验证。
- 无控制器或封装未引出所需引脚的项不计成功。任何单型号结果都不自动推广到全部模式。

来源与许可见仓库根目录的 [UPSTREAM.md](../UPSTREAM.md)。
