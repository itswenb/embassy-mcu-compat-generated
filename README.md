# stm32-metapac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for STMicroelectronics STM32 microcontrollers.

This crate has been automatically generated based on data in the [`stm32-data` project](https://github.com/embassy-rs/stm32-data), and is used for the [`embassy-stm32`](github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the STM32 microcontrollers.

## Metadata

This PAC additionally exports "metadata" about the chips. To use it, enable the `metadata` feature and access it at `stm32_metapac::METADATA`. It is intended to be consumed from `build.rs` scripts or code-generation tools running on PCs, not from the firmware itself.

The metadata includes the following info:

- Memory maps for RAM, flash.
- Interrupts
- GPIO Alternate Function mappings
- Interrupt -> peripheral mappings
- DMA channel -> peripehral mappings
- RCC clock tree information for each peripheral (what clocks does it receive, which RCC registers to poke to enable, reset, or choose the clock)

## Supported chips

This PAC aims to support all STM32 chip families:

- STM32F0
- STM32F1
- STM32F2
- STM32F3
- STM32F4
- STM32F7
- STM32C0
- STM32G0
- STM32G4
- STM32H5
- STM32H7
- STM32H7RS
- STM32L0
- STM32L1
- STM32L4
- STM32L5
- STM32U0
- STM32U5
- STM32WB
- STM32WBA
- STM32WL

## Embassy STM32 零修改兼容入口

根 `stm32-metapac` 包允许未修改的 `embassy-stm32` 使用 662 个 Cortex-M
GD32 真实型号。应用选择一个合适的 STM32 feature，并通过环境变量选择真实芯片：

```toml
[dependencies]
embassy-stm32 = { version = "...", features = ["stm32f303cb"] }

[patch."https://github.com/embassy-rs/stm32-data-generated"]
stm32-metapac = { git = "https://github.com/itswenb/embassy-mcu-compat-generated", rev = "<固定提交>" }
```

```toml
[env]
EMBASSY_MCU_COMPAT_CHIP = "gd32f303cb"
```

真实型号与 STM32 feature 不做固定映射；使用者负责选择架构及外设拓扑相近的 feature。

## 原生 GD32 PAC

本仓库还发布 workspace 包 [`mcu-metapac`](mcu-metapac/README.md)，当前包含 680
个由真实厂商数据生成并通过编译门的 GD32 feature。依赖时选择一个真实型号：

```toml
[dependencies]
mcu-metapac = { git = "https://github.com/itswenb/embassy-mcu-compat-generated", rev = "<固定提交>", features = ["gd32f103c8", "pac", "metadata"] }
```

这条原生 PAC 路径不等于所有型号已经通过 `embassy-stm32` 或实机验证。
