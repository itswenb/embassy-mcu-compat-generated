
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System and memory architectur",
            ),
            items: &[
                BlockItem {
                    name: "cfg0",
                    description: Some(
                        "System configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss0",
                    description: Some(
                        "EXTI sources selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss1",
                    description: Some(
                        "EXTI sources selection register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss2",
                    description: Some(
                        "EXTI sources selection register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss3",
                    description: Some(
                        "EXTI sources selection register 3",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg1",
                    description: Some(
                        "System configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "System status register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg2",
                    description: Some(
                        "SYSCFG configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpu_irq_lat",
                    description: Some(
                        "IRQ latency register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CpuIrqLat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg0",
                    description: Some(
                        "TIMERx configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg1",
                    description: Some(
                        "TIMERx configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2cfg0",
                    description: Some(
                        "TIMER2 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2cfg1",
                    description: Some(
                        "TIMER2 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2cfg1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg0",
            extends: None,
            description: Some(
                "System configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_mode",
                    description: Some(
                        "Boot mode (Refer to Boot configuration for details)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa11_rmp",
                    description: Some(
                        "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa12_rmp",
                    description: Some(
                        "PA12 pin remapping",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6fmpen",
                    description: Some(
                        "I2C Fm+ mode on PB6 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7fmpen",
                    description: Some(
                        "I2C Fm+ mode on PB7 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8fmpen",
                    description: Some(
                        "I2C Fm+ mode on PB8 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9fmpen",
                    description: Some(
                        "I2C Fm+ mode on PB9 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c0_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for I2C0 (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa9fmpen",
                    description: Some(
                        "I2C Fm+ mode on PA9 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa10fmpen",
                    description: Some(
                        "I2C Fm+ mode on PA10 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pc14fmpen",
                    description: Some(
                        "I2C Fm+ mode on PC14 pin enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfg1",
            extends: None,
            description: Some(
                "System configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex-M23 LOCKUP output lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram_ecc_lock",
                    description: Some(
                        "SRAM ECC check error lock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfg2",
            extends: None,
            description: Some(
                "SYSCFG configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lxtalcss_ie",
                    description: Some(
                        "LXTAL css interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hxtalcss_ie",
                    description: Some(
                        "HXTAL css interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccmeie",
                    description: Some(
                        "Multi-bits (two bits) non-correction error NMI interrupt enable (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccseie",
                    description: Some(
                        "Single bit correction error interrupt enable(for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccserrbits",
                    description: Some(
                        "Indicates the error bit(for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ecceaddr",
                    description: Some(
                        "Indicates the last ECC event on SRAM occurred. (for GD32C231xx / GD32C221xx series)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CpuIrqLat",
            extends: None,
            description: Some(
                "IRQ latency register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "irq_latency",
                    description: Some(
                        "IRQ_LATENCY specifies the minimum number of cycles between an interrupt that becomes pended in the NVIC, and the vector fetch for that interrupt being issued on the AHB-Lite interface.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss0",
            extends: None,
            description: Some(
                "EXTI sources selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti0_ss",
                    description: Some(
                        "EXTI 0 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti1_ss",
                    description: Some(
                        "EXTI 1 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti2_ss",
                    description: Some(
                        "EXTI 2 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti3_ss",
                    description: Some(
                        "EXTI 3 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss1",
            extends: None,
            description: Some(
                "EXTI sources selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti4_ss",
                    description: Some(
                        "EXTI 4 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti5_ss",
                    description: Some(
                        "EXTI 5 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti6_ss",
                    description: Some(
                        "EXTI 6 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti7_ss",
                    description: Some(
                        "EXTI 7 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss2",
            extends: None,
            description: Some(
                "EXTI sources selection register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti8_ss",
                    description: Some(
                        "EXTI 8 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti9_ss",
                    description: Some(
                        "EXTI 9 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti10_ss",
                    description: Some(
                        "EXTI 10 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti11_ss",
                    description: Some(
                        "EXTI 11 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss3",
            extends: None,
            description: Some(
                "EXTI sources selection register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti12_ss",
                    description: Some(
                        "EXTI 12 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti13_ss",
                    description: Some(
                        "EXTI 13 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti14_ss",
                    description: Some(
                        "EXTI 14 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti15_ss",
                    description: Some(
                        "EXTI 15 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "System status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccmeif",
                    description: Some(
                        "SRAM two bits non-correction event flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccseif",
                    description: Some(
                        "SRAM single bit correction event flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg0",
            extends: None,
            description: Some(
                "TIMERx configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg1",
            extends: None,
            description: Some(
                "TIMERx configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer2cfg0",
            extends: None,
            description: Some(
                "TIMER2 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer2cfg1",
            extends: None,
            description: Some(
                "TIMER2 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                