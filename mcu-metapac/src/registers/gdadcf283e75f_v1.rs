
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
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
                    name: "sampt0",
                    description: Some(
                        "Sample time register 0",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sampt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sampt1",
                    description: Some(
                        "Sample time register 1",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sampt1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ioff0",
                    description: Some(
                        "Inserted channel data offset register 0",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                        "Inserted channel data offset register 1",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                        "Inserted channel data offset register 2",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                        "Inserted channel data offset register 3",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "wd0ht",
                    description: Some(
                        "Watchdog 0 high threshold register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd0ht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd0lt",
                    description: Some(
                        "Watchdog 0 low threshold register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd0lt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq0",
                    description: Some(
                        "Routine sequence register 0",
                    ),
                    array: None,
                    byte_offset: 0x2c,
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
                        "Routine sequence register 1",
                    ),
                    array: None,
                    byte_offset: 0x30,
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
                        "Routine sequence register 2",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
                    name: "isq",
                    description: Some(
                        "Inserted sequence register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Isq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idata0",
                    description: Some(
                        "Inserted data register 0",
                    ),
                    array: None,
                    byte_offset: 0x3c,
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
                        "Inserted data register 1",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                        "Inserted data register 2",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                        "Inserted data register 3",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                        "Routine data register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                    name: "wd1sr",
                    description: Some(
                        "Watchdog 1 channel selection register",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                        "Watchdog 2 channel selection register",
                    ),
                    array: None,
                    byte_offset: 0x54,
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
                    name: "wd1ht",
                    description: Some(
                        "Watchdog 1 high threshold register1",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd1ht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd1lt",
                    description: Some(
                        "Watchdog 1 low threshold register1",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd1lt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd2ht",
                    description: Some(
                        "Watchdog 2 high threshold register2",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd2ht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wd2lt",
                    description: Some(
                        "Watchdog 2 low threshold register2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wd2lt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ovsampctl",
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
                                "Ovsampctl",
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
                        "Analog watchdog 0 channel select",
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
                    name: "wd0eie",
                    description: Some(
                        "Interrupt enable for WD0E",
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
                        "When in scan mode, analog watchdog 0 is effective on a single channel.",
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
                        "Inserted sequence convert automatically",
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
                        "Discontinuous mode on routine sequence",
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
                        "Discontinuous mode on inserted sequence",
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
                        "Inserted channel analog watchdog 0 enable",
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
                        "Routine channel analog watchdog 0 enable",
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
                        "ADC resolution",
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
                    name: "wd1eie",
                    description: Some(
                        "Interrupt enable for WD1E",
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
                    name: "wd2eie",
                    description: Some(
                        "Interrupt enable for WD2E",
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
                    name: "etsic",
                    description: Some(
                        "External trigger selection for inserted sequence",
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
                    name: "eteic",
                    description: Some(
                        "External trigger enable for inserted sequence",
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
                    name: "etsrc",
                    description: Some(
                        "External trigger selection for routine sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eterc",
                    description: Some(
                        "External trigger enable for routine sequence",
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
                    name: "swicst",
                    description: Some(
                        "Software start on inserted sequence.",
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
                    name: "swrcst",
                    description: Some(
                        "Software start of routine sequence",
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
                    name: "tsven",
                    description: Some(
                        "Temperature sensor output voltage channel (VSENSE) enable",
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
                    name: "inrefen",
                    description: Some(
                        "Internal reference voltage channel (VREFINT) enable",
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
            name: "Idata0",
            extends: None,
            description: Some(
                "Inserted data register 0",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata1",
            extends: None,
            description: Some(
                "Inserted data register 1",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata2",
            extends: None,
            description: Some(
                "Inserted data register 2",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Idata3",
            extends: None,
            description: Some(
                "Inserted data register 3",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff0",
            extends: None,
            description: Some(
                "Inserted channel data offset register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff1",
            extends: None,
            description: Some(
                "Inserted channel data offset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff2",
            extends: None,
            description: Some(
                "Inserted channel data offset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ioff3",
            extends: None,
            description: Some(
                "Inserted channel data offset register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ioff",
                    description: Some(
                        "Data offset for inserted channel x",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isq",
            extends: None,
            description: Some(
                "Inserted sequence register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "isq0",
                    description: Some(
                        "The channel number is written to these bits to select a channel at the nth conversion in the inserted sequence.",
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
                    name: "isq1",
                    description: Some(
                        "Refer to ISQ0[3:0] description",
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
                    name: "isq2",
                    description: Some(
                        "Refer to ISQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isq3",
                    description: Some(
                        "Refer to ISQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "il",
                    description: Some(
                        "Inserted sequence length",
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
            name: "Ovsampctl",
            extends: None,
            description: Some(
                "Oversample control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ovsen",
                    description: Some(
                        "Oversampler enable",
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
                    name: "ovsr",
                    description: Some(
                        "Oversampling ratio",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
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
                        "Triggered oversampling",
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
            ],
        },
        FieldSet {
            name: "Rdata",
            extends: None,
            description: Some(
                "Routine data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata",
                    description: Some(
                        "Routine channel data",
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
            ],
        },
        FieldSet {
            name: "Rsq0",
            extends: None,
            description: Some(
                "Routine sequence register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq12",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq13",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq14",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq15",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rl",
                    description: Some(
                        "Routine sequence length",
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
                "Routine sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq6",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq7",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq8",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq9",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq10",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq11",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rsq2",
            extends: None,
            description: Some(
                "Routine sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq0",
                    description: Some(
                        "The channel number is written to these bits to select a channel as the nth conversion in the routine sequence.",
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
                    name: "rsq1",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq2",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq3",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq4",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
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
                    name: "rsq5",
                    description: Some(
                        "Refer to RSQ0[3:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sampt0",
            extends: None,
            description: Some(
                "Sample time register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spt10",
                    description: Some(
                        "Channel sample time",
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
                    name: "spt11",
                    description: Some(
                        "Refer to SPT10[2:0] description.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt12",
                    description: Some(
                        "Refer to SPT10[2:0] description.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt13",
                    description: Some(
                        "Refer to SPT10[2:0] description.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt14",
                    description: Some(
                        "Refer to SPT10[2:0] description.",
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
                    name: "spt15",
                    description: Some(
                        "Refer to SPT10[2:0] description.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sampt1",
            extends: None,
            description: Some(
                "Sample time register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spt0",
                    description: Some(
                        "Channel sample time",
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
                    name: "spt1",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt2",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt3",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt4",
                    description: Some(
                        "Refer to SPT0[2:0] description",
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
                    name: "spt5",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt6",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt7",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spt8",
                    description: Some(
                        "Refer to SPT0[2:0] description",
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
                    name: "spt9",
                    description: Some(
                        "Refer to SPT0[2:0] description",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 3,
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
                    name: "wd0e",
                    description: Some(
                        "Analog watchdog 0 event flag",
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
                        "End flag of sequence conversion",
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
                        "End flag of inserted sequence conversion",
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
                        "Start flag of inserted sequence",
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
                        "Start flag of routine sequence",
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
                    name: "wd1e",
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
                    name: "wd2e",
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
            name: "Wd0ht",
            extends: None,
            description: Some(
                "Watchdog 0 high threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd0ht",
                    description: Some(
                        "High threshold for analog watchdog 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd0lt",
            extends: None,
            description: Some(
                "Watchdog 0 low threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd0lt",
                    description: Some(
                        "Low threshold for analog watchdog 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd1ht",
            extends: None,
            description: Some(
                "Watchdog 1 high threshold register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd1ht",
                    description: Some(
                        "High threshold for analog watchdog 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd1lt",
            extends: None,
            description: Some(
                "Watchdog 1 low threshold register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd1lt",
                    description: Some(
                        "Low threshold for analog watchdog 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd1sr",
            extends: None,
            description: Some(
                "Watchdog 1 channel selection register",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd2ht",
            extends: None,
            description: Some(
                "Watchdog 2 high threshold register2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd2ht",
                    description: Some(
                        "High threshold for analog watchdog 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd2lt",
            extends: None,
            description: Some(
                "Watchdog 2 low threshold register2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wd2lt",
                    description: Some(
                        "Low threshold for analog watchdog 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wd2sr",
            extends: None,
            description: Some(
                "Watchdog 2 channel selection register",
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
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                