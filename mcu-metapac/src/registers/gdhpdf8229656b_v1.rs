
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hpdf",
            extends: None,
            description: Some(
                "High-Performance Digital Filter",
            ),
            items: &[
                BlockItem {
                    name: "ch0ctl",
                    description: Some(
                        "Channel 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0cfg0",
                    description: Some(
                        "Channel 0 configuration 0 register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0cfg1",
                    description: Some(
                        "Channel 0 configuration 1 register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0tmfdt",
                    description: Some(
                        "Channel 0 threshold monitor filter data register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0pdi",
                    description: Some(
                        "Channel 0 parallel data input register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0ps",
                    description: Some(
                        "Channel 0 pulse skip register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1ctl",
                    description: Some(
                        "Channel 1 control register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1cfg0",
                    description: Some(
                        "Channel 1 configuration 0 register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1cgf1",
                    description: Some(
                        "Channel 1 configuration 1 register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cgf1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1tmfdt",
                    description: Some(
                        "Channel 0 threshold monitor filter data register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1pdi",
                    description: Some(
                        "Channel 1 parallel data input register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1ps",
                    description: Some(
                        "Channel 1 pulse skip register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0ctl0",
                    description: Some(
                        "Filter 0 control register 0",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0ctl1",
                    description: Some(
                        "Filter 0 control register 1",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0stat",
                    description: Some(
                        "Filter 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0intc",
                    description: Some(
                        "Filter 0 interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0icgs",
                    description: Some(
                        "Filter 0 inserted channel group selection register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0icgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0sfctl",
                    description: Some(
                        "Filter 0 sinc filter control register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0sfctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0idata",
                    description: Some(
                        "Filter 0 inserted group data register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0idata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0rdata",
                    description: Some(
                        "Filter 0 regular channel data register",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0rdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0tmht",
                    description: Some(
                        "Filter 0 threshold monitor high threshold register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0tmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0tmlt",
                    description: Some(
                        "Filter 0 threshold monitor low threshold register",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0tmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0tmstat",
                    description: Some(
                        "Filter 0 threshold monitor status register",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0tmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0tmfc",
                    description: Some(
                        "Filter 0 threshold monitor flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0tmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0emmax",
                    description: Some(
                        "Filter 0 extremes monitor maximum register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0emmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0emmin",
                    description: Some(
                        "Filter 0 extremes monitor minimum register",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0emmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1ctl0",
                    description: Some(
                        "Filter 1 control register 0",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1ctl1",
                    description: Some(
                        "Filter 1 control register 1",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1stat",
                    description: Some(
                        "Filter 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1intc",
                    description: Some(
                        "Filter 1 interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1icgs",
                    description: Some(
                        "Filter 1 inserted channel group selection register",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1icgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1sfctl",
                    description: Some(
                        "Filter 1 sinc filter control register",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1sfctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1idata",
                    description: Some(
                        "Filter 1 inserted group data register",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1idata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1rdata",
                    description: Some(
                        "Filter 1 regular channel data register",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1rdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1tmht",
                    description: Some(
                        "Filter 1 threshold monitor high threshold register",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1tmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1tmlt",
                    description: Some(
                        "Filter 1 threshold monitor low threshold register",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1tmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1tmstat",
                    description: Some(
                        "Filter 0 threshold monitor status register",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1tmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1tmfc",
                    description: Some(
                        "Filter 1 threshold monitor flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1tmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1emmax",
                    description: Some(
                        "Filter 1 extremes monitor maximum register",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1emmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1emmin",
                    description: Some(
                        "Filter 1 extremes monitor minimum register",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1emmin",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ch0cfg0",
            extends: None,
            description: Some(
                "Channel 0 configuration 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrs",
                    description: Some(
                        "Data right bit-shift",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "caloff",
                    description: Some(
                        "24-bit calibration offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0cfg1",
            extends: None,
            description: Some(
                "Channel 0 configuration 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmct",
                    description: Some(
                        "Malfunction monitor counter threshold",
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
                Field {
                    name: "mmbsd",
                    description: Some(
                        "Malfunction monitor break signal distribution",
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
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate",
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
                    name: "tmsfo",
                    description: Some(
                        "Threshold monitor Sinc filter order selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0ctl",
            extends: None,
            description: Some(
                "Channel 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sityp",
                    description: Some(
                        "Serial interface type",
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
                    name: "spickss",
                    description: Some(
                        "SPI clock source select",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmen",
                    description: Some(
                        "Malfunction monitor enable",
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
                    name: "cklen",
                    description: Some(
                        "Clock loss detector enable",
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
                    name: "chen",
                    description: Some(
                        "Channel 0 enable",
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
                    name: "chpinsel",
                    description: Some(
                        "Channel inputs pins selection",
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
                    name: "cmsd",
                    description: Some(
                        "Channel 0 multiplexer select input data source",
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
                Field {
                    name: "dpm",
                    description: Some(
                        "Data packing mode",
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
                    name: "ckoutdiv",
                    description: Some(
                        "Serial clock output divider",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckoutdm",
                    description: Some(
                        "Serial clock output duty mode",
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
                    name: "ckoutsel",
                    description: Some(
                        "Serial clock output source selection",
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
                    name: "hpdfen",
                    description: Some(
                        "Global enable for HPDF interface",
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
            name: "Ch0pdi",
            extends: None,
            description: Some(
                "Channel 0 parallel data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input 0",
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
                    name: "datain1",
                    description: Some(
                        "Data input 1",
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
            name: "Ch0ps",
            extends: None,
            description: Some(
                "Channel 0 pulse skip register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plsk",
                    description: Some(
                        "Pulses to skip for input data skipping function",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0tmfdt",
            extends: None,
            description: Some(
                "Channel 0 threshold monitor filter data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmdata",
                    description: Some(
                        "Threshold monitor data",
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
            name: "Ch1cfg0",
            extends: None,
            description: Some(
                "Channel 1 configuration 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dtrs",
                    description: Some(
                        "Data right bit-shift",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "caloff",
                    description: Some(
                        "24-bit calibration offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1cgf1",
            extends: None,
            description: Some(
                "Channel 1 configuration 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmct",
                    description: Some(
                        "Malfunction monitor counter threshold",
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
                Field {
                    name: "mmbsd",
                    description: Some(
                        "Malfunction monitor break signal distribution",
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
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate",
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
                    name: "tmsfo",
                    description: Some(
                        "Threshold monitor Sinc filter order selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1ctl",
            extends: None,
            description: Some(
                "Channel 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sityp",
                    description: Some(
                        "Serial interface type",
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
                    name: "spickss",
                    description: Some(
                        "SPI clock source select",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmen",
                    description: Some(
                        "Malfunction monitor enable",
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
                    name: "cklen",
                    description: Some(
                        "Clock loss detector enable",
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
                    name: "chen",
                    description: Some(
                        "Channel 0 enable",
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
                    name: "chpinsel",
                    description: Some(
                        "Channel inputs pins selection",
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
                    name: "cmsd",
                    description: Some(
                        "Channel 0 multiplexer select input data source",
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
                Field {
                    name: "dpm",
                    description: Some(
                        "Data packing mode",
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
            ],
        },
        FieldSet {
            name: "Ch1pdi",
            extends: None,
            description: Some(
                "Channel 1 parallel data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input 0",
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
                    name: "datain1",
                    description: Some(
                        "Data input 1",
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
            name: "Ch1ps",
            extends: None,
            description: Some(
                "Channel 1 pulse skip register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plsk",
                    description: Some(
                        "Pulses to skip for input data skipping function",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1tmfdt",
            extends: None,
            description: Some(
                "Channel 0 threshold monitor filter data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmdata",
                    description: Some(
                        "Threshold monitor data",
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
            name: "Flt0ctl0",
            extends: None,
            description: Some(
                "Filter 0 control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "Inserted conversions trigger signal selection",
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
                    name: "sicc",
                    description: Some(
                        "Start inserted group channel conversion",
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
                    name: "icsyn",
                    description: Some(
                        "Inserted conversion synchronously",
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
                    name: "scmod",
                    description: Some(
                        "Scan conversion mode of inserted conversions",
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
                    name: "icdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the inserted channel group",
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
                    name: "ictssel",
                    description: Some(
                        "Inserted conversions trigger signal selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icteen",
                    description: Some(
                        "Inserted conversions trigger edge enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcs",
                    description: Some(
                        "Start regular channel conversion by software",
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
                    name: "rccm",
                    description: Some(
                        "Regular conversions continuous mode",
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
                    name: "rcsyn",
                    description: Some(
                        "Regular conversion synchronously",
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
                    name: "rcdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the regular conversion",
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
                    name: "rcs",
                    description: Some(
                        "Regular conversion channel selection",
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
                    name: "fast",
                    description: Some(
                        "Fast conversion mode enable for regular conversions",
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
                    name: "tmfm",
                    description: Some(
                        "Threshold monitor fast mode",
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
            name: "Flt0ctl1",
            extends: None,
            description: Some(
                "Filter 0 control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iceie",
                    description: Some(
                        "Inserted conversion end interrupt enable",
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
                    name: "rceie",
                    description: Some(
                        "Regular conversion end interrupt enable",
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
                    name: "idovrie",
                    description: Some(
                        "Inserted data overflow interrupt enable",
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
                    name: "rdovrie",
                    description: Some(
                        "Regular data overflow interrupt enable",
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
                    name: "tmie",
                    description: Some(
                        "Threshold monitor interrupt enable",
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
                    name: "mmie",
                    description: Some(
                        "Malfunction monitor interrupt enable",
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
                    name: "cklie",
                    description: Some(
                        "Clock loss interrupt enable",
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
                    name: "emcs",
                    description: Some(
                        "Extremes monitor channel selection",
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
                    name: "tmchen",
                    description: Some(
                        "Threshold monitor channel enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0emmax",
            extends: None,
            description: Some(
                "Filter 0 extremes monitor maximum register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maxdc",
                    description: Some(
                        "Extremes monitor maximum data channel",
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
                    name: "maxval",
                    description: Some(
                        "Extremes monitor maximum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0emmin",
            extends: None,
            description: Some(
                "Filter 0 extremes monitor minimum register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mindc",
                    description: Some(
                        "Extremes monitor minimum data channel",
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
                    name: "minval",
                    description: Some(
                        "Extremes monitor minimum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0icgs",
            extends: None,
            description: Some(
                "Filter 0 inserted channel group selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icgsel",
                    description: Some(
                        "Inserted channel group selection",
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
            ],
        },
        FieldSet {
            name: "Flt0idata",
            extends: None,
            description: Some(
                "Filter 0 inserted group data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icch",
                    description: Some(
                        "Inserted channel most recently converted",
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
                    name: "idata",
                    description: Some(
                        "Inserted group conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0intc",
            extends: None,
            description: Some(
                "Filter 0 interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icofc",
                    description: Some(
                        "Clear the inserted conversion overflow flag",
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
                    name: "rcofc",
                    description: Some(
                        "Clear the regular conversion overflow flag",
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
                    name: "cklfc",
                    description: Some(
                        "Clear the clock loss flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmfc",
                    description: Some(
                        "Clear the malfunction monitor flag",
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
            ],
        },
        FieldSet {
            name: "Flt0rdata",
            extends: None,
            description: Some(
                "Filter 0 regular channel data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcch",
                    description: Some(
                        "Regular channel most recently converted",
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
                    name: "rchpdt",
                    description: Some(
                        "Regular channel pending data",
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
                    name: "rdata",
                    description: Some(
                        "Regular channel conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0sfctl",
            extends: None,
            description: Some(
                "Filter 0 sinc filter control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ior",
                    description: Some(
                        "Integrator oversampling ratio",
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
                Field {
                    name: "sfor",
                    description: Some(
                        "Sinc filter oversampling ratio",
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
                Field {
                    name: "sfo",
                    description: Some(
                        "Sinc filter order",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0stat",
            extends: None,
            description: Some(
                "Filter 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icef",
                    description: Some(
                        "Inserted conversion end flag",
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
                    name: "rcef",
                    description: Some(
                        "Regular conversion end flag",
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
                    name: "icof",
                    description: Some(
                        "Inserted conversion overflow flag",
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
                    name: "rcof",
                    description: Some(
                        "Regular conversion end flag",
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
                    name: "tmeof",
                    description: Some(
                        "Threshold monitor event occurred flag",
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
                    name: "icpf",
                    description: Some(
                        "Inserted conversion in progress flag",
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
                    name: "rcpf",
                    description: Some(
                        "Regular conversion in progress flag",
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
                    name: "cklf",
                    description: Some(
                        "Clock loss flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmf",
                    description: Some(
                        "Malfunction monitor flag",
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
            ],
        },
        FieldSet {
            name: "Flt0tmfc",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltfc",
                    description: Some(
                        "Clear the threshold monitor low threshold flag",
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
                    name: "htfc",
                    description: Some(
                        "Clear the threshold monitor high threshold flag",
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
            ],
        },
        FieldSet {
            name: "Flt0tmht",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor high threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htbsd",
                    description: Some(
                        "High threshold event break signal distribution",
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
                    name: "htval",
                    description: Some(
                        "Threshold monitor high threshold value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0tmlt",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor low threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltbsd",
                    description: Some(
                        "Low threshold event break signal distribution",
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
                    name: "ltval",
                    description: Some(
                        "Threshold monitor low threshold value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0tmstat",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltf",
                    description: Some(
                        "Threshold monitor low threshold flag",
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
                    name: "htf",
                    description: Some(
                        "Threshold monitor high threshold flag",
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
            ],
        },
        FieldSet {
            name: "Flt1ctl0",
            extends: None,
            description: Some(
                "Filter 1 control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "Inserted conversions trigger signal selection",
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
                    name: "sicc",
                    description: Some(
                        "Start inserted group channel conversion",
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
                    name: "icsyn",
                    description: Some(
                        "Inserted conversion synchronously",
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
                    name: "scmod",
                    description: Some(
                        "Scan conversion mode of inserted conversions",
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
                    name: "icdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the inserted channel group",
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
                    name: "ictssel",
                    description: Some(
                        "Inserted conversions trigger signal selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icteen",
                    description: Some(
                        "Inserted conversions trigger edge enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "srcs",
                    description: Some(
                        "Start regular channel conversion by software",
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
                    name: "rccm",
                    description: Some(
                        "Regular conversions continuous mode",
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
                    name: "rcsyn",
                    description: Some(
                        "Regular conversion synchronously",
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
                    name: "rcdmaen",
                    description: Some(
                        "DMA channel enabled to read data for the regular conversion",
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
                    name: "rcs",
                    description: Some(
                        "Regular conversion channel selection",
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
                    name: "fast",
                    description: Some(
                        "Fast conversion mode enable for regular conversions",
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
                    name: "tmfm",
                    description: Some(
                        "Threshold monitor fast mode",
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
            name: "Flt1ctl1",
            extends: None,
            description: Some(
                "Filter 1 control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iceie",
                    description: Some(
                        "Inserted conversion end interrupt enable",
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
                    name: "rceie",
                    description: Some(
                        "Regular conversion end interrupt enable",
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
                    name: "idovrie",
                    description: Some(
                        "Inserted data overflow interrupt enable",
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
                    name: "rdovrie",
                    description: Some(
                        "Regular data overflow interrupt enable",
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
                    name: "tmie",
                    description: Some(
                        "Threshold monitor interrupt enable",
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
                    name: "mmie",
                    description: Some(
                        "Malfunction monitor interrupt enable",
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
                    name: "emcs",
                    description: Some(
                        "Extremes monitor channel selection",
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
                    name: "tmchen",
                    description: Some(
                        "Threshold monitor channel enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1emmax",
            extends: None,
            description: Some(
                "Filter 1 extremes monitor maximum register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maxdc",
                    description: Some(
                        "Extremes monitor maximum data channel",
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
                    name: "maxval",
                    description: Some(
                        "Extremes monitor maximum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1emmin",
            extends: None,
            description: Some(
                "Filter 1 extremes monitor minimum register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mindc",
                    description: Some(
                        "Extremes monitor minimum data channel",
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
                    name: "minval",
                    description: Some(
                        "Extremes monitor minimum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1icgs",
            extends: None,
            description: Some(
                "Filter 1 inserted channel group selection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icgsel",
                    description: Some(
                        "Inserted channel group selection",
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
            ],
        },
        FieldSet {
            name: "Flt1idata",
            extends: None,
            description: Some(
                "Filter 1 inserted group data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icch",
                    description: Some(
                        "Inserted channel most recently converted",
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
                    name: "idata",
                    description: Some(
                        "Inserted group conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1intc",
            extends: None,
            description: Some(
                "Filter 1 interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icofc",
                    description: Some(
                        "Clear the inserted conversion overflow flag",
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
                    name: "rcofc",
                    description: Some(
                        "Clear the regular conversion overflow flag",
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
                    name: "mmfc",
                    description: Some(
                        "Clear the short-circuit detector flag",
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
            ],
        },
        FieldSet {
            name: "Flt1rdata",
            extends: None,
            description: Some(
                "Filter 1 regular channel data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcch",
                    description: Some(
                        "Regular channel most recently converted",
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
                    name: "rchpdt",
                    description: Some(
                        "Regular channel pending data",
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
                    name: "rdata",
                    description: Some(
                        "Regular channel conversion data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1sfctl",
            extends: None,
            description: Some(
                "Filter 1 sinc filter control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ior",
                    description: Some(
                        "Integrator oversampling ratio",
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
                Field {
                    name: "sfor",
                    description: Some(
                        "Sinc filter oversampling ratio",
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
                Field {
                    name: "sfo",
                    description: Some(
                        "Sinc filter order",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1stat",
            extends: None,
            description: Some(
                "Filter 1 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icef",
                    description: Some(
                        "Inserted conversion end flag",
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
                    name: "rcef",
                    description: Some(
                        "Regular conversion end flag",
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
                    name: "icof",
                    description: Some(
                        "Inserted conversion overflow flag",
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
                    name: "rcof",
                    description: Some(
                        "Regular conversion end flag",
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
                    name: "tmeof",
                    description: Some(
                        "Analog watchdog event occurred flag",
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
                    name: "icpf",
                    description: Some(
                        "Inserted conversion in progress flag",
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
                    name: "rcpf",
                    description: Some(
                        "Regular conversion in progress flag",
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
                    name: "cklf",
                    description: Some(
                        "Clock loss flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mmf",
                    description: Some(
                        "Malfunction monitor flag",
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
            ],
        },
        FieldSet {
            name: "Flt1tmfc",
            extends: None,
            description: Some(
                "Filter 1 threshold monitor flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltfc",
                    description: Some(
                        "Clear the threshold monitor low threshold flag",
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
                    name: "htfc",
                    description: Some(
                        "Clear the threshold monitor high threshold flag",
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
            ],
        },
        FieldSet {
            name: "Flt1tmht",
            extends: None,
            description: Some(
                "Filter 1 threshold monitor high threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "htbsd",
                    description: Some(
                        "High threshold event break signal distribution",
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
                    name: "htval",
                    description: Some(
                        "Threshold monitor high threshold value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1tmlt",
            extends: None,
            description: Some(
                "Filter 1 threshold monitor low threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltbsd",
                    description: Some(
                        "Low threshold event break signal distribution",
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
                    name: "ltval",
                    description: Some(
                        "Threshold monitor low threshold value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1tmstat",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltf",
                    description: Some(
                        "Threshold monitor low threshold flag",
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
                    name: "htf",
                    description: Some(
                        "Threshold monitor high threshold flag",
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
            ],
        },
    ],
    enums: &[],
};
                