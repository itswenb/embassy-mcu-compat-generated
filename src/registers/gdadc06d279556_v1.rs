
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc0",
            extends: None,
            description: Some(
                "Analog-to-digital converter",
            ),
            items: &[
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "ctl0",
                    description: Some(
                        "Control register 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioff0",
                    description: Some(
                        "Inserted channel data offset registe 0",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioff0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioff1",
                    description: Some(
                        "Inserted channel data offset registe 1",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioff1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioff2",
                    description: Some(
                        "Inserted channel data offset registe 2",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioff2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioff3",
                    description: Some(
                        "Inserted channel data offset registe 3",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ioff3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdht0",
                    description: Some(
                        "Watchdog high threshold register0",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdht0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdlt0",
                    description: Some(
                        "Watchdog low threshold register0",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdlt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq0",
                    description: Some(
                        "Regular sequence register 0",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq1",
                    description: Some(
                        "Regular sequence register 1",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq2",
                    description: Some(
                        "Regular sequence register 2",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq3",
                    description: Some(
                        "Regular sequence register 3",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq4",
                    description: Some(
                        "Regular sequence register 4",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq5",
                    description: Some(
                        "Regular sequence register 5",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq6",
                    description: Some(
                        "Regular sequence register 6",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq7",
                    description: Some(
                        "Regular sequence register 7",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq8",
                    description: Some(
                        "Regular sequence register 8",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rsq8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isq0",
                    description: Some(
                        "Inserted sequence register 0",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isq0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isq1",
                    description: Some(
                        "Inserted sequence register 1",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isq1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isq2",
                    description: Some(
                        "Inserted sequence register 2",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isq2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idata0",
                    description: Some(
                        "Inserted data registe 0",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idata1",
                    description: Some(
                        "Inserted data registe 1",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idata2",
                    description: Some(
                        "Inserted data registe 2",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idata3",
                    description: Some(
                        "Inserted data registe 3",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Idata3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdata",
                    description: Some(
                        "Regular data register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ovscr",
                    description: Some(
                        "Oversample control register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ovscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd1sr",
                    description: Some(
                        "Watchdog 1 Channel Selection Register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd1sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd2sr",
                    description: Some(
                        "Watchdog 2 Channel Selection Register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd2sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdht1",
                    description: Some(
                        "Watchdog high threshold register1",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdht1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdlt1",
                    description: Some(
                        "Watchdog low threshold register1",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdlt1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdht2",
                    description: Some(
                        "Watchdog high threshold register2",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdht2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdlt2",
                    description: Some(
                        "Watchdog low threshold register2",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdlt2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "difctl",
                    description: Some(
                        "Differential mode control registe",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Difctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sstat",
                    description: Some(
                        "Summary status register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "syncctl",
                    description: Some(
                        "Sync control register",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Syncctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "syncdata0",
                    description: Some(
                        "Sync regular data register0",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Syncdata0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "syncdata1",
                    description: Some(
                        "Sync regular data register 1",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Syncdata1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd0chsel",
                    description: Some(
                        "Analog watchdog0 channel select",
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
                    name: "eocie",
                    description: Some(
                        "Interrupt enable for EOC",
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
                    name: "wde0ie",
                    description: Some(
                        "Interrupt enable for WDE0",
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
                    name: "eoicie",
                    description: Some(
                        "Interrupt enable for EOIC",
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
                    name: "sm",
                    description: Some(
                        "Scan mode",
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
                    name: "wd0sc",
                    description: Some(
                        "When in scan mode, analog watchdog0 is effective on a single channel",
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
                    name: "ica",
                    description: Some(
                        "Inserted channel group convert automatically",
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
                    name: "disrc",
                    description: Some(
                        "Discontinuous mode on regular channels",
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
                    name: "disic",
                    description: Some(
                        "Discontinuous mode on inserted channels",
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
                    name: "disnum",
                    description: Some(
                        "Number of conversions in discontinuous mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwd0en",
                    description: Some(
                        "Inserted channel analog watchdog0 enable",
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
                    name: "rwd0en",
                    description: Some(
                        "Regular channel analog watchdog0 enable",
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
                    name: "dres",
                    description: Some(
                        "ADC data resolution for ADC0/ADC1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rovfie",
                    description: Some(
                        "Interrupt enable for ROVF",
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
                    name: "wde1ie",
                    description: Some(
                        "Interrupt enable for WDE1",
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
                    name: "wde2ie",
                    description: Some(
                        "Interrupt enable for WDE2",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcon",
                    description: Some(
                        "ADC ON",
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
                    name: "ctn",
                    description: Some(
                        "Continuous mode",
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
                    name: "clb",
                    description: Some(
                        "ADC calibration",
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
                    name: "rstclb",
                    description: Some(
                        "Reset calibration",
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
                    name: "calnum",
                    description: Some(
                        "Calibration Times",
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
                    name: "dma",
                    description: Some(
                        "DMA request enable",
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
                    name: "ddm",
                    description: Some(
                        "DMA disable mode",
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
                    name: "eocm",
                    description: Some(
                        "End of conversion mode",
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
                    name: "dal",
                    description: Some(
                        "Data alignment",
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
                    name: "hpdfcfg",
                    description: Some(
                        "HPDF mode configuration",
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
                    name: "etmic",
                    description: Some(
                        "External trigger mode for inserted channel",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swicst",
                    description: Some(
                        "Software start on inserted channel",
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
                    name: "calmod",
                    description: Some(
                        "ADC calibration mode (for ADC0/1)",
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
                    name: "etmrc",
                    description: Some(
                        "External trigger mode for regular channel",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swrcst",
                    description: Some(
                        "Software start on regular channel",
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
            ],
        },
        FieldSet {
            name: "Difctl",
            extends: None,
            description: Some(
                "Differential mode control registe",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "difctl",
                    description: Some(
                        "Differential mode for channel 21",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata0",
            extends: None,
            description: Some(
                "Inserted data registe 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idatan",
                    description: Some(
                        "Inserted number n conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata1",
            extends: None,
            description: Some(
                "Inserted data registe 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idatan",
                    description: Some(
                        "Inserted number n conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata2",
            extends: None,
            description: Some(
                "Inserted data registe 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idatan",
                    description: Some(
                        "Inserted number n conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata3",
            extends: None,
            description: Some(
                "Inserted data registe 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idatan",
                    description: Some(
                        "Inserted number n conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff0",
            extends: None,
            description: Some(
                "Inserted channel data offset registe 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x, For ADC0/ADC1 are IOFF[23:0], for ADC2 is IOFF[11:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff1",
            extends: None,
            description: Some(
                "Inserted channel data offset registe 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x, For ADC0/ADC1 are IOFF[23:0], for ADC2 is IOFF[11:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff2",
            extends: None,
            description: Some(
                "Inserted channel data offset registe 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x, For ADC0/ADC1 are IOFF[23:0], for ADC2 is IOFF[11:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff3",
            extends: None,
            description: Some(
                "Inserted channel data offset registe 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x, For ADC0/ADC1 are IOFF[23:0], for ADC2 is IOFF[11:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isq0",
            extends: None,
            description: Some(
                "Inserted sequence register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "isq3",
                    description: Some(
                        "refer to ISQ0[4:0] description",
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
                    name: "ismp3",
                    description: Some(
                        "Inserted channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "il",
                    description: Some(
                        "Inserted channel group length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isq1",
            extends: None,
            description: Some(
                "Inserted sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "isq1",
                    description: Some(
                        "refer to ISQ0[4:0] description",
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
                    name: "ismp1",
                    description: Some(
                        "Inserted channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isq2",
                    description: Some(
                        "refer to ISQ0[4:0] description",
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
                    name: "ismp2",
                    description: Some(
                        "Inserted channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isq2",
            extends: None,
            description: Some(
                "Inserted sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "isq0",
                    description: Some(
                        "The channel number (0",
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
                    name: "ismp0",
                    description: Some(
                        "Inserted channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ovscr",
            extends: None,
            description: Some(
                "Oversample control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovsen",
                    description: Some(
                        "Oversampler Enable",
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
                    name: "ovss",
                    description: Some(
                        "Oversampling shift",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tovs",
                    description: Some(
                        "Triggered Oversampling",
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
                    name: "ovsr",
                    description: Some(
                        "Oversampling ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdata",
            extends: None,
            description: Some(
                "Regular data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "Regular channel data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq0",
            extends: None,
            description: Some(
                "Regular sequence register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq15",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp15",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rl",
                    description: Some(
                        "Regular channel group length",
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
            ],
        },
        FieldSet {
            name: "Rsq1",
            extends: None,
            description: Some(
                "Regular sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq13",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp13",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq14",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp14",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq2",
            extends: None,
            description: Some(
                "Regular sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq11",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp11",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq12",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp12",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq3",
            extends: None,
            description: Some(
                "Regular sequence register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq9",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp9",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq10",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp10",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq4",
            extends: None,
            description: Some(
                "Regular sequence register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq7",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp7",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq8",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp8",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq5",
            extends: None,
            description: Some(
                "Regular sequence register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq5",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp5",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq6",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp6",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq6",
            extends: None,
            description: Some(
                "Regular sequence register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq3",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp3",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq4",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp4",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq7",
            extends: None,
            description: Some(
                "Regular sequence register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq1",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp1",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq2",
                    description: Some(
                        "refer to RSQ0[4:0] description",
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
                    name: "rsmp2",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq8",
            extends: None,
            description: Some(
                "Regular sequence register 8",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq0",
                    description: Some(
                        "The channel number (0",
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
                    name: "rsmp0",
                    description: Some(
                        "Regular channel sample time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sstat",
            extends: None,
            description: Some(
                "Summary status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc0_wde0",
                    description: Some(
                        "This bit is the mirror image of the WDE0 bit of ADC0",
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
                    name: "adc0_wde1",
                    description: Some(
                        "This bit is the mirror image of the WDE1 bit of ADC0",
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
                    name: "adc0_wde2",
                    description: Some(
                        "This bit is the mirror image of the WDE2 bit of ADC0",
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
                    name: "adc0_eoc",
                    description: Some(
                        "This bit is the mirror image of the EOC bit of ADC0",
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
                    name: "adc0_strc",
                    description: Some(
                        "This bit is the mirror image of the STRC bit of ADC0",
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
                    name: "adc0_rovf",
                    description: Some(
                        "This bit is the mirror image of the ROVF bit of ADC0",
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
                    name: "adc1_wde0",
                    description: Some(
                        "This bit is the mirror image of the WDE0 bit of ADC1",
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
                    name: "adc1_wde1",
                    description: Some(
                        "This bit is the mirror image of the WDE1 bit of ADC1",
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
                    name: "adc1_wde2",
                    description: Some(
                        "This bit is the mirror image of the WDE2 bit of ADC1",
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
                    name: "adc1_eoc",
                    description: Some(
                        "This bit is the mirror image of the EOC bit of ADC1",
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
                    name: "adc1_strc",
                    description: Some(
                        "This bit is the mirror image of the STRC bit of ADC1",
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
                    name: "adc1_rovf",
                    description: Some(
                        "This bit is the mirror image of the ROVF bit of ADC1",
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
                    name: "adc2_wde0",
                    description: Some(
                        "This bit is the mirror image of the WDE0 bit of ADC2",
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
                    name: "adc2_wde1",
                    description: Some(
                        "This bit is the mirror image of the WDE1 bit of ADC2",
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
                    name: "adc2_wde2",
                    description: Some(
                        "This bit is the mirror image of the WDE2 bit of ADC2",
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
                    name: "adc2_stic",
                    description: Some(
                        "This bit is the mirror image of the STIC bit of ADC2",
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
                    name: "adc2_strc",
                    description: Some(
                        "This bit is the mirror image of the STRC bit of ADC2",
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
                    name: "adc2_rovf",
                    description: Some(
                        "This bit is the mirror image of the ROVF bit of ADC2",
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
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wde0",
                    description: Some(
                        "Analog watchdog0 event flag",
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
                    name: "eoc",
                    description: Some(
                        "End of group conversion flag",
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
                    name: "eoic",
                    description: Some(
                        "End of inserted group conversion flag",
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
                    name: "stic",
                    description: Some(
                        "Start flag of inserted channel group",
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
                    name: "strc",
                    description: Some(
                        "Start flag of regular channel group",
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
                    name: "rovf",
                    description: Some(
                        "Regular data register overflow",
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
                    name: "wde1",
                    description: Some(
                        "Analog watchdog 1 event flag",
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
                    name: "wde2",
                    description: Some(
                        "Analog watchdog 2 event flag",
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
            name: "Syncctl",
            extends: None,
            description: Some(
                "Sync control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncm",
                    description: Some(
                        "ADC sync mode",
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
                    name: "syncdly",
                    description: Some(
                        "ADC sync delay",
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
                    name: "syncddm",
                    description: Some(
                        "ADC sync DMA disable mode",
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
                    name: "syncdma",
                    description: Some(
                        "ADC sync DMA mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adcsck",
                    description: Some(
                        "ADC sync clock mode",
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
                    name: "adcck",
                    description: Some(
                        "ADC clock prescaler",
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
            ],
        },
        FieldSet {
            name: "Syncdata0",
            extends: None,
            description: Some(
                "Sync regular data register0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncdata0",
                    description: Some(
                        "Regular data0 (master adc regular data) in ADC sync mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syncdata1",
                    description: Some(
                        "Regular data1(slave adc regular data) in ADC sync mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Syncdata1",
            extends: None,
            description: Some(
                "Sync regular data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syncdata",
                    description: Some(
                        "which is selected from the regular data(master/slave) of the ADCs in turn",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd1sr",
            extends: None,
            description: Some(
                "Watchdog 1 Channel Selection Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd1cs",
                    description: Some(
                        "Analog watchdog 1 channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd2sr",
            extends: None,
            description: Some(
                "Watchdog 2 Channel Selection Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd2cs",
                    description: Some(
                        "Analog watchdog 2 channel selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdht0",
            extends: None,
            description: Some(
                "Watchdog high threshold register0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdht0",
                    description: Some(
                        "Analog watchdog0 high threshold, For ADC0/ADC1 are WDHT0[23:0], for ADC2 is WDHT0[11:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdht1",
            extends: None,
            description: Some(
                "Watchdog high threshold register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdht1",
                    description: Some(
                        "Analog watchdog 1 high threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdht2",
            extends: None,
            description: Some(
                "Watchdog high threshold register2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdht2",
                    description: Some(
                        "Analog watchdog 2 high threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdlt0",
            extends: None,
            description: Some(
                "Watchdog low threshold register0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdlt0",
                    description: Some(
                        "Analog watchdog low threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdlt1",
            extends: None,
            description: Some(
                "Watchdog low threshold register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdlt1",
                    description: Some(
                        "Analog watchdog 1 low threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdlt2",
            extends: None,
            description: Some(
                "Watchdog low threshold register2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdlt2",
                    description: Some(
                        "Analog watchdog 2 low threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
