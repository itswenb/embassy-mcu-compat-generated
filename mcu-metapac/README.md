# mcu-metapac

这是由 `embassy-mcu-compat` 确定性生成的厂商无关 MCU 外设访问包。

当前版本包含 680 个原生 MCU feature，GigaDevice 是第一个接入厂商。每个 feature
使用真实厂商寄存器、中断和内存事实生成；支持状态与来源缺口以源仓库中的机器可读报告为准。

本包提供 PAC 与 metadata，不代表所有型号已经通过 `embassy-stm32` 兼容门或硬件验证。
