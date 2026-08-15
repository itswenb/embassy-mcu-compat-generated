
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
                    name: "cfg1",
                    description: Some(
                        "System configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "cfg2",
                    description: Some(
                        "System configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "cfg3",
                    description: Some(
                        "System configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg4",
                    description: Some(
                        "System configuration register 4",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg5",
                    description: Some(
                        "System configuration register 5",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tcmsramcs",
                    description: Some(
                        "SYSCFG TCMSRAM control and status register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcmsramcs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tcmsramkey",
                    description: Some(
                        "SYSCFG TCMSRAM key register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcmsramkey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tcmsramwp",
                    description: Some(
                        "SYSCFG TCMSRAM write protection register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tcmsramwp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpsctl",
                    description: Some(
                        "I/O Compensation cell control/status register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpsctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel0",
                    description: Some(
                        "Timer input selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel1",
                    description: Some(
                        "Timer input selection register 1",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel2",
                    description: Some(
                        "Timer input selection register 2",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel3",
                    description: Some(
                        "Timer input selection register 3",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel4",
                    description: Some(
                        "Timer input selection register 4",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg0",
                    description: Some(
                        "TIMER0 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x100,
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
                        "TIMER1 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x104,
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
                    name: "timer0cfg2",
                    description: Some(
                        "TIMERx configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg0",
                    description: Some(
                        "TIMERx configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg1",
                    description: Some(
                        "TIMERx configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg2",
                    description: Some(
                        "TIMERx configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg2",
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
                        "Boot mode",
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
                    name: "fmc_swp",
                    description: Some(
                        "FMC memory mapping swap",
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
                    name: "pb6fmpen",
                    description: Some(
                        "I2C Fm+ mode on PB6 pin enable",
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
                        "I2C Fm+ mode on PB7 pin enable",
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
                        "I2C Fm+ mode on PB8 pin enable",
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
                        "I2C Fm+ mode on PB9 pin enable",
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
                    name: "i2c0fmpen",
                    description: Some(
                        "I2C0 Fm+ mode enable",
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
                    name: "i2c1fmpen",
                    description: Some(
                        "I2C1 Fm+ mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2fmpen",
                    description: Some(
                        "I2C2 Fm+ mode enable",
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
                    name: "i2c3fmpen",
                    description: Some(
                        "I2C3 Fm+ mode enable",
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
                    name: "iopie",
                    description: Some(
                        "FPU invalid operation interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dzie",
                    description: Some(
                        "FPU divide by 0 interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufie",
                    description: Some(
                        "FPU underflow interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovfie",
                    description: Some(
                        "FPU overflow interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idie",
                    description: Some(
                        "FPU input denormal interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ixie",
                    description: Some(
                        "FPU inexact interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
                "System configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex-M33 lockup (Hardfault) output enable bit",
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
                    name: "sram0_ecc_lock",
                    description: Some(
                        "SRAM0 ECC lockup enable bit",
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
                    name: "lvd_lock",
                    description: Some(
                        "LVD lcokup enable bit",
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
                    name: "flash_ecc_lock",
                    description: Some(
                        "Flash ECC lockup enable bit",
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
                    name: "sram1_ecc_lock",
                    description: Some(
                        "SRAM1 ECC lock enable bit",
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
                    name: "tcmsram_ecc_lock",
                    description: Some(
                        "TCMSRAM ECC lock enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pin_rstmd",
                    description: Some(
                        "Pin reset mode",
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
                Field {
                    name: "trgsel_rstmd",
                    description: Some(
                        "Trigsel reset mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfg3",
            extends: None,
            description: Some(
                "System configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram0eccmeie",
                    description: Some(
                        "SRAM0 multi-bits (two bits) non-correction error NMI interrupt enable",
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
                    name: "sram0eccseie",
                    description: Some(
                        "SRAM0 single bit correction error interrupt enable",
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
                    name: "flasheccie",
                    description: Some(
                        "Flash ECC NMI interrupt flag",
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
                    name: "ckmnmiie",
                    description: Some(
                        "HXTAL clock moniotor NMI interrupt enable",
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
                    name: "nmipinie",
                    description: Some(
                        "NMI pin interrupt enable",
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
                    name: "sram0eccserrbits",
                    description: Some(
                        "Which one bit has SRAM0 ECC single-bit correctable error",
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
                    name: "sram0ecceaddr",
                    description: Some(
                        "Record the faulting system address (SRAM0ECCEADDR = SRAM0 address[16:0] >> 2) where the last SRAM0 ECC event on SRAM occurred",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfg4",
            extends: None,
            description: Some(
                "System configuration register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sram1eccmeie",
                    description: Some(
                        "SRAM1 multi-bits (two bits) non-correction error event flag",
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
                    name: "sram1eccseie",
                    description: Some(
                        "SRAM1 single bit correction error event flag",
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
                    name: "sram1eccserrbits",
                    description: Some(
                        "Which one bit has SRAM1 ECC single-bit correctable error",
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
                    name: "sram1ecceaddr",
                    description: Some(
                        "Record the faulting system address (SRAM1ECCEADDR = SRAM1 address[13:0] >> 2) where the last SRAM1 ECC event on SRAM occurred",
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
            name: "Cfg5",
            extends: None,
            description: Some(
                "System configuration register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcmsrameccmeie",
                    description: Some(
                        "TCMSRAM multi-bits (two bits) non-correction error event flag",
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
                    name: "tcmsrameccseie",
                    description: Some(
                        "TCMSRAM single bit correction error event flag",
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
                    name: "tcmsrameccserrbits",
                    description: Some(
                        "Which one bit has TCMSRAM ECC single-bit correctable error",
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
                    name: "tcmsramecceaddr",
                    description: Some(
                        "Record the faulting system address (TCMSRAMECCEADDR = TCM SRAM address[14:0] >> 2) where the last TCMSRAM ECC event on SRAM occurred",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpsctl",
            extends: None,
            description: Some(
                "I/O Compensation cell control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cps_en",
                    description: Some(
                        "I/O compensation cell enable",
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
                    name: "cps_rdy",
                    description: Some(
                        "Compensation cell ready flag",
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    bit_size: 4,
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
                    name: "sram0eccmeif",
                    description: Some(
                        "SRAM0 multi-bits (two bits) non-correction interrupt flag",
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
                    name: "sram0eccseif",
                    description: Some(
                        "SRAM0 single bit correction interrupt flag",
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
                    name: "flasheccif",
                    description: Some(
                        "Flash ECC NMI interrupt flag",
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
                    name: "ckmnmiif",
                    description: Some(
                        "HXTAL clock moniotor NMI interrupt flag",
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
                    name: "nmipinif",
                    description: Some(
                        "Interrupt flag from NMI pin",
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
                    name: "sram1eccmeif",
                    description: Some(
                        "SRAM1 multi-bits (two bits) non-correction interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1eccseif",
                    description: Some(
                        "SRAM1 single bit correction interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcmsrameccmeif",
                    description: Some(
                        "TCMSRAM multi-bits (two bits) non-correction interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tcmsrameccseif",
                    description: Some(
                        "TCMSRAM single bit correction interrupt flag",
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
            ],
        },
        FieldSet {
            name: "Tcmsramcs",
            extends: None,
            description: Some(
                "SYSCFG TCMSRAM control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcmsramers",
                    description: Some(
                        "TCMSRAM erase",
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
                    name: "tcmsrambsy",
                    description: Some(
                        "TCMSRAM busy flag by erase operation",
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
            name: "Tcmsramkey",
            extends: None,
            description: Some(
                "SYSCFG TCMSRAM key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp_key",
                    description: Some(
                        "TCMSRAM write protection key",
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
            name: "Tcmsramwp",
            extends: None,
            description: Some(
                "SYSCFG TCMSRAM write protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "p0wpen",
                    description: Some(
                        "TCMSRAM page 0 write protection enable bit",
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
                    name: "p1wpen",
                    description: Some(
                        "TCMSRAM page 1 write protection enable bit",
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
                    name: "p2wpen",
                    description: Some(
                        "TCMSRAM page 2 write protection enable bit",
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
                    name: "p3wpen",
                    description: Some(
                        "TCMSRAM page 3 write protection enable bit",
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
                    name: "p4wpen",
                    description: Some(
                        "TCMSRAM page 4 write protection enable bit",
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
                    name: "p5wpen",
                    description: Some(
                        "TCMSRAM page 5 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p6wpen",
                    description: Some(
                        "TCMSRAM page 6 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p7wpen",
                    description: Some(
                        "TCMSRAM page 7 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p8wpen",
                    description: Some(
                        "TCMSRAM page 8 write protection enable bit",
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
                    name: "p9wpen",
                    description: Some(
                        "TCMSRAM page 9 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p10wpen",
                    description: Some(
                        "TCMSRAM page 10 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p11wpen",
                    description: Some(
                        "TCMSRAM page 11 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p12wpen",
                    description: Some(
                        "TCMSRAM page 12 write protection enable bit",
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
                Field {
                    name: "p13wpen",
                    description: Some(
                        "TCMSRAM page 13 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p14wpen",
                    description: Some(
                        "TCMSRAM page 14 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p15wpen",
                    description: Some(
                        "TCMSRAM page 15 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p16wpen",
                    description: Some(
                        "TCMSRAM page 16 write protection enable bit",
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
                    name: "p17wpen",
                    description: Some(
                        "TCMSRAM page 17 write protection enable bit",
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
                    name: "p18wpen",
                    description: Some(
                        "TCMSRAM page 18 write protection enable bit",
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
                    name: "p19wpen",
                    description: Some(
                        "TCMSRAM page 19 write protection enable bit",
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
                    name: "p20wpen",
                    description: Some(
                        "TCMSRAM page 20 write protection enable bit",
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
                    name: "p21wpen",
                    description: Some(
                        "TCMSRAM page 21 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p22wpen",
                    description: Some(
                        "TCMSRAM page 22 write protection enable bit",
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
                    name: "p23wpen",
                    description: Some(
                        "TCMSRAM page 23 write protection enable bit",
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
                    name: "p24wpen",
                    description: Some(
                        "TCMSRAM page 24 write protection enable bit",
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
                Field {
                    name: "p25wpen",
                    description: Some(
                        "TCMSRAM page 25 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p26wpen",
                    description: Some(
                        "TCMSRAM page 26 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p27wpen",
                    description: Some(
                        "TCMSRAM page 27 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p28wpen",
                    description: Some(
                        "TCMSRAM page 28 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p29wpen",
                    description: Some(
                        "TCMSRAM page 29 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p30wpen",
                    description: Some(
                        "TCMSRAM page 30 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "p31wpen",
                    description: Some(
                        "TCMSRAM page 31 write protection enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
                "TIMER0 configuration register 0",
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
                    bit_size: 5,
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
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg1",
            extends: None,
            description: Some(
                "TIMER1 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart + event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Pause + restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg10",
                    description: Some(
                        "Decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg11",
                    description: Some(
                        "Decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg2",
            extends: None,
            description: Some(
                "TIMERx configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg12",
                    description: Some(
                        "Decoder mode 3 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg13",
                    description: Some(
                        "Quadrature decoder mode 3 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg14",
                    description: Some(
                        "Quadrature decoder mode 4 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg0",
            extends: None,
            description: Some(
                "TIMERx configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
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
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg1",
            extends: None,
            description: Some(
                "TIMERx configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart + event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg2",
            extends: None,
            description: Some(
                "TIMERx configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel0",
            extends: None,
            description: Some(
                "Timer input selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer7_ci0_sel",
                    description: Some(
                        "Selects TIMER7_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci1_sel",
                    description: Some(
                        "Selects TIMER7_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci2_sel",
                    description: Some(
                        "Selects TIMER7_CI2 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci3_sel",
                    description: Some(
                        "Selects TIMER7_CI3 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci0_sel",
                    description: Some(
                        "Selects TIMER0_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci1_sel",
                    description: Some(
                        "Selects TIMER0_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci2_sel",
                    description: Some(
                        "Selects TIMER0_CI2 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci3_sel",
                    description: Some(
                        "Selects TIMER0_CI3 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel1",
            extends: None,
            description: Some(
                "Timer input selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer19_ci0_sel",
                    description: Some(
                        "Selects TIMER19_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer19_ci1_sel",
                    description: Some(
                        "Selects TIMER19_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer19_ci2_sel",
                    description: Some(
                        "Selects TIMER19_CI2 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer19_ci3_sel",
                    description: Some(
                        "Selects TIMER19_CI3 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel2",
            extends: None,
            description: Some(
                "Timer input selection register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer2_ci0_sel",
                    description: Some(
                        "TIMER2_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci1_sel",
                    description: Some(
                        "TIMER2_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci2_sel",
                    description: Some(
                        "TIMER2_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci3_sel",
                    description: Some(
                        "TIMER2_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci0_sel",
                    description: Some(
                        "TIMER1_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci1_sel",
                    description: Some(
                        "TIMER1_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci2_sel",
                    description: Some(
                        "TIMER1_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci3_sel",
                    description: Some(
                        "TIMER1_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel3",
            extends: None,
            description: Some(
                "Timer input selection register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer4_ci0_sel",
                    description: Some(
                        "TIMER4_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci1_sel",
                    description: Some(
                        "TIMER4_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci2_sel",
                    description: Some(
                        "TIMER4_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci3_sel",
                    description: Some(
                        "TIMER4_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci0_sel",
                    description: Some(
                        "TIMER3_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci1_sel",
                    description: Some(
                        "TIMER3_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci2_sel",
                    description: Some(
                        "TIMER3_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci3_sel",
                    description: Some(
                        "TIMER3_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel4",
            extends: None,
            description: Some(
                "Timer input selection register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer14_ci0_sel",
                    description: Some(
                        "Selects TIMER14_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer14_ci1_sel",
                    description: Some(
                        "Selects TIMER14_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer15_ci0_sel",
                    description: Some(
                        "Selects TIMER15_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer16_ci0_sel",
                    description: Some(
                        "Selects TIMER16_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                