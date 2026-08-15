
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc1",
            extends: None,
            description: Some(
                "Analog to digital converter",
            ),
            items: &[
                BlockItem {
                    name: "stat",
                    description: Some(
                        "status register",
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
                        "control register 0",
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
                        "control register 1",
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
                    name: "wdht",
                    description: Some(
                        "watchdog higher threshold register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdlt",
                    description: Some(
                        "watchdog lower threshold register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rsq0",
                    description: Some(
                        "regular sequence register 0",
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
                        "regular sequence register 1",
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
                        "regular sequence register 2",
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
                            access: Access::Read,
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
                            access: Access::Read,
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
                            access: Access::Read,
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
                            access: Access::Read,
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
                        "regular data register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdata",
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
                "control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdchsel",
                    description: Some(
                        "Analog watchdog channel select",
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
                    name: "wdeie",
                    description: Some(
                        "Interrupt enable for WDE",
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
                    name: "wdsc",
                    description: Some(
                        "When in scan mode, analog watchdog \t is effective on a single channel",
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
                        "Inserted channel group convert \t automatically",
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
                        "Discontinuous mode on \t inserted channels",
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
                        "Number of conversions in \t discontinuous mode",
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
                    name: "syncm",
                    description: Some(
                        "sync mode selection",
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
                    name: "iwden",
                    description: Some(
                        "Inserted channel analog watchdog \t enable",
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
                    name: "rwden",
                    description: Some(
                        "Regular channel analog watchdog \t enable",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcon",
                    description: Some(
                        "ADC on",
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
                        "External trigger select for inserted channel",
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
                    name: "etsrc",
                    description: Some(
                        "External trigger select for regular channel",
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
                        "External trigger enable for regular channel",
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
                        "Start on inserted channel",
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
                        "Start on regular channel",
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
                    name: "tsvren",
                    description: Some(
                        "Channel 16 and 17 enable of ADC0",
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
                        "Data offset for inserted channel 0",
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
                        "Data offset for inserted channel 1",
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
                        "Data offset for inserted channel 2",
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
                        "Data offset for inserted channel 3",
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
                        "1st conversion in inserted sequence",
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
                    name: "isq1",
                    description: Some(
                        "2nd conversion in inserted sequence",
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
                    name: "isq2",
                    description: Some(
                        "3rd conversion in inserted sequence",
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
                    name: "isq3",
                    description: Some(
                        "4th conversion in inserted sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
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
                        "Oversampling Enable",
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
                    name: "dres",
                    description: Some(
                        "ADC resolution",
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
            name: "Rdata",
            extends: None,
            description: Some(
                "regular data register",
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
                "regular sequence register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq12",
                    description: Some(
                        "13th conversion in regular sequence",
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
                    name: "rsq13",
                    description: Some(
                        "14th conversion in regular sequence",
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
                    name: "rsq14",
                    description: Some(
                        "15th conversion in regular sequence",
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
                    name: "rsq15",
                    description: Some(
                        "16th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
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
                "regular sequence register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq6",
                    description: Some(
                        "7th conversion in regular sequence",
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
                    name: "rsq7",
                    description: Some(
                        "8th conversion in regular sequence",
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
                    name: "rsq8",
                    description: Some(
                        "9th conversion in regular sequence",
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
                    name: "rsq9",
                    description: Some(
                        "10th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq10",
                    description: Some(
                        "11th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq11",
                    description: Some(
                        "12th conversion in regular sequence",
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
            name: "Rsq2",
            extends: None,
            description: Some(
                "regular sequence register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rsq0",
                    description: Some(
                        "1st conversion in regular sequence",
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
                    name: "rsq1",
                    description: Some(
                        "2nd conversion in regular sequence",
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
                    name: "rsq2",
                    description: Some(
                        "3rd conversion in regular sequence",
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
                    name: "rsq3",
                    description: Some(
                        "4th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq4",
                    description: Some(
                        "5th conversion in regular sequence",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rsq5",
                    description: Some(
                        "6th conversion in regular sequence",
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
                        "Channel 10 sample time selection",
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
                        "Channel 11 sample time selection",
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
                        "Channel 12 sample time selection",
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
                        "Channel 13 sample time selection",
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
                        "Channel 14 sample time selection",
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
                        "Channel 15 sample time selection",
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
                    name: "spt16",
                    description: Some(
                        "Channel 16 sample time selection",
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
                    name: "spt17",
                    description: Some(
                        "Channel 17 sample time selection",
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
                        "Channel 0 sample time selection",
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
                        "Channel 1 sample time selection",
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
                        "Channel 2 sample time selection",
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
                        "Channel 3 sample time selection",
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
                        "Channel 4 sample time selection",
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
                        "Channel 5 sample time selection",
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
                        "Channel 6 sample time selection",
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
                        "Channel 7 sample time selection",
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
                        "Channel 8 sample time selection",
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
                        "Channel 9 sample time selection",
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
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wde",
                    description: Some(
                        "Analog watchdog event flag",
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
            ],
        },
        FieldSet {
            name: "Wdht",
            extends: None,
            description: Some(
                "watchdog higher threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdht",
                    description: Some(
                        "Analog watchdog higher threshold",
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
            name: "Wdlt",
            extends: None,
            description: Some(
                "watchdog lower threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdlt",
                    description: Some(
                        "Analog watchdog lower threshold",
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
    ],
    enums: &[],
};
                