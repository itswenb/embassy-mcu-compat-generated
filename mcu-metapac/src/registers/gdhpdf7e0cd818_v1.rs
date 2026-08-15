
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hpdf",
            extends: None,
            description: Some(
                "High Performance Digital Filter",
            ),
            items: &[
                BlockItem {
                    name: "ch0ctl",
                    description: Some(
                        "Channel 0 control regist",
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
                        "Channel 0 configuration register",
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
                        "Channel 0 configuration register",
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
                        "Channel 0 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Channel 0 parallel data input regist",
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
                        "Channel 0 pulse skip regist",
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
                        "Channel 1 control regist",
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
                        "Channel 1 configuration register",
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
                    name: "ch1cfg1",
                    description: Some(
                        "Channel 1 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1tmfdt",
                    description: Some(
                        "Channel 1 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Channel 1 parallel data input regist",
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
                        "Channel 1 pulse skip regist",
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
                    name: "ch2ctl",
                    description: Some(
                        "Channel 2 control regist",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2cfg0",
                    description: Some(
                        "Channel 2 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2cfg1",
                    description: Some(
                        "Channel 2 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2tmfdt",
                    description: Some(
                        "Channel 2 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2pdi",
                    description: Some(
                        "Channel 2 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2ps",
                    description: Some(
                        "Channel 2 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3ctl",
                    description: Some(
                        "Channel 3 control regist",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3cfg0",
                    description: Some(
                        "Channel 3 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3cfg1",
                    description: Some(
                        "Channel 3 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3tmfdt",
                    description: Some(
                        "Channel 3 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3pdi",
                    description: Some(
                        "Channel 3 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3ps",
                    description: Some(
                        "Channel 3 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4ctl",
                    description: Some(
                        "Channel 4 control regist",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4cfg0",
                    description: Some(
                        "Channel 4 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4cfg1",
                    description: Some(
                        "Channel 4 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4tmfdt",
                    description: Some(
                        "Channel 4 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4pdi",
                    description: Some(
                        "Channel 4 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4ps",
                    description: Some(
                        "Channel 4 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5ctl",
                    description: Some(
                        "Channel 5 control regist",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5cfg0",
                    description: Some(
                        "Channel 5 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5cfg1",
                    description: Some(
                        "Channel 5 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5tmfdt",
                    description: Some(
                        "Channel 5 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5pdi",
                    description: Some(
                        "Channel 5 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5ps",
                    description: Some(
                        "Channel 5 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6ctl",
                    description: Some(
                        "Channel 6 control regist",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6cfg0",
                    description: Some(
                        "Channel 6 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6cfg1",
                    description: Some(
                        "Channel 6 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6tmfdt",
                    description: Some(
                        "Channel 6 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6pdi",
                    description: Some(
                        "Channel 6 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6ps",
                    description: Some(
                        "Channel 6 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7ctl",
                    description: Some(
                        "Channel 7 control regist",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7cfg0",
                    description: Some(
                        "Channel 7 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7cfg1",
                    description: Some(
                        "Channel 7 configuration register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7tmfdt",
                    description: Some(
                        "Channel 7 threshold monitor filter data regist",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7tmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7pdi",
                    description: Some(
                        "Channel 7 parallel data input regist",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7pdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7ps",
                    description: Some(
                        "Channel 7 pulse skip regist",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7ps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0ctl0",
                    description: Some(
                        "Filter 0 control register",
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
                        "Filter 0 control register",
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
                        "Filter 0 status regist",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 0 interrupt flag clear regist",
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
                        "Filter 0 inserted channel group selection regist",
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
                    name: "flt0sfcfg",
                    description: Some(
                        "Filter 0 sinc filter configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0sfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0idata",
                    description: Some(
                        "Filter 0 inserted group conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 0 regular channel conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 0 threshold monitor high threshold regist",
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
                        "Filter 0 threshold monitor low threshold regist",
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
                        "Filter 0 threshold monitor status regist",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 0 threshold monitor flag clear regist",
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
                        "Filter 0 extremes monitor maximum regist",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 0 extremes monitor minimum regist",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0emmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt0ct",
                    description: Some(
                        "Filter 0 conversion timer register",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt0ct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1ctl0",
                    description: Some(
                        "Filter 1 control register",
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
                        "Filter 1 control register",
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
                        "Filter 1 status regist",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 1 interrupt flag clear regist",
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
                        "Filter 1 inserted channel group selection regist",
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
                    name: "flt1sfcfg",
                    description: Some(
                        "Filter 1 sinc filter configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1sfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1idata",
                    description: Some(
                        "Filter 1 inserted group conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 1 regular channel conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 1 threshold monitor high threshold regist",
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
                        "Filter 1 threshold monitor low threshold regist",
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
                        "Filter 1 threshold monitor status regist",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 1 threshold monitor flag clear regist",
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
                        "Filter 1 extremes monitor maximum regist",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "Filter 1 extremes monitor minimum regist",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1emmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt1ct",
                    description: Some(
                        "Filter 1 conversion timer register",
                    ),
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt1ct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2ctl0",
                    description: Some(
                        "Filter 2 control register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2ctl1",
                    description: Some(
                        "Filter 2 control register",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2stat",
                    description: Some(
                        "Filter 2 status regist",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2intc",
                    description: Some(
                        "Filter 2 interrupt flag clear regist",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2icgs",
                    description: Some(
                        "Filter 2 inserted channel group selection regist",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2icgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2sfcfg",
                    description: Some(
                        "Filter 2 sinc filter configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2sfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2idata",
                    description: Some(
                        "Filter 2 inserted group conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2idata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2rdata",
                    description: Some(
                        "Filter 0 regular channel conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2rdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2tmht",
                    description: Some(
                        "Filter 2 threshold monitor high threshold regist",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2tmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2tmlt",
                    description: Some(
                        "Filter 2 threshold monitor low threshold regist",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2tmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2tmstat",
                    description: Some(
                        "Filter 2 threshold monitor status regist",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2tmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2tmfc",
                    description: Some(
                        "Filter 2 threshold monitor flag clear regist",
                    ),
                    array: None,
                    byte_offset: 0x22c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2tmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2emmax",
                    description: Some(
                        "Filter 2 extremes monitor maximum regist",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2emmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2emmin",
                    description: Some(
                        "Filter 2 extremes monitor minimum regist",
                    ),
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2emmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt2ct",
                    description: Some(
                        "Filter 2 conversion timer register",
                    ),
                    array: None,
                    byte_offset: 0x238,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt2ct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3ctl0",
                    description: Some(
                        "Filter 3 control register",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3ctl1",
                    description: Some(
                        "Filter 2 control register",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3stat",
                    description: Some(
                        "Filter 3 status regist",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3intc",
                    description: Some(
                        "Filter 3 interrupt flag clear regist",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3icgs",
                    description: Some(
                        "Filter 3 inserted channel group selection regist",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3icgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3sfcfg",
                    description: Some(
                        "Filter 3 sinc filter configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3sfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3idata",
                    description: Some(
                        "Filter 3 inserted group conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3idata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3rdata",
                    description: Some(
                        "Filter 3 regular channel conversion data regist",
                    ),
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3rdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3tmht",
                    description: Some(
                        "Filter 3 threshold monitor high threshold regist",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3tmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3tmlt",
                    description: Some(
                        "Filter 3 threshold monitor low threshold regist",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3tmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3tmfc",
                    description: Some(
                        "Filter 3 threshold monitor flag clear regist",
                    ),
                    array: None,
                    byte_offset: 0x2ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3tmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3emmax",
                    description: Some(
                        "Filter 3 extremes monitor maximum regist",
                    ),
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3emmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3emmin",
                    description: Some(
                        "Filter 3 extremes monitor minimum regist",
                    ),
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3emmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3ct",
                    description: Some(
                        "Filter 3 conversion timer register",
                    ),
                    array: None,
                    byte_offset: 0x2b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3ct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "flt3tmstat",
                    description: Some(
                        "Filter 3 threshold monitor status regist",
                    ),
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Flt3tmstat",
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
                "Channel 0 configuration register",
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
                "Channel 0 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
                "Channel 0 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
                "Channel 0 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
                "Channel 0 pulse skip regist",
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
                "Channel 0 threshold monitor filter data regist",
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
                "Channel 1 configuration register",
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
            name: "Ch1cfg1",
            extends: None,
            description: Some(
                "Channel 1 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
                "Channel 1 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
                "Channel 1 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
                "Channel 1 pulse skip regist",
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
                "Channel 1 threshold monitor filter data regist",
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
            name: "Ch2cfg0",
            extends: None,
            description: Some(
                "Channel 2 configuration register",
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
            name: "Ch2cfg1",
            extends: None,
            description: Some(
                "Channel 2 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch2ctl",
            extends: None,
            description: Some(
                "Channel 2 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch2pdi",
            extends: None,
            description: Some(
                "Channel 2 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch2ps",
            extends: None,
            description: Some(
                "Channel 2 pulse skip regist",
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
            name: "Ch2tmfdt",
            extends: None,
            description: Some(
                "Channel 2 threshold monitor filter data regist",
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
            name: "Ch3cfg0",
            extends: None,
            description: Some(
                "Channel 3 configuration register",
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
            name: "Ch3cfg1",
            extends: None,
            description: Some(
                "Channel 3 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch3ctl",
            extends: None,
            description: Some(
                "Channel 3 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch3pdi",
            extends: None,
            description: Some(
                "Channel 3 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch3ps",
            extends: None,
            description: Some(
                "Channel 3 pulse skip regist",
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
            name: "Ch3tmfdt",
            extends: None,
            description: Some(
                "Channel 3 threshold monitor filter data regist",
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
            name: "Ch4cfg0",
            extends: None,
            description: Some(
                "Channel 4 configuration register",
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
            name: "Ch4cfg1",
            extends: None,
            description: Some(
                "Channel 4 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch4ctl",
            extends: None,
            description: Some(
                "Channel 4 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch4pdi",
            extends: None,
            description: Some(
                "Channel 4 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch4ps",
            extends: None,
            description: Some(
                "Channel 4 pulse skip regist",
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
            name: "Ch4tmfdt",
            extends: None,
            description: Some(
                "Channel 4 threshold monitor filter data regist",
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
            name: "Ch5cfg0",
            extends: None,
            description: Some(
                "Channel 5 configuration register",
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
            name: "Ch5cfg1",
            extends: None,
            description: Some(
                "Channel 5 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch5ctl",
            extends: None,
            description: Some(
                "Channel 5 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch5pdi",
            extends: None,
            description: Some(
                "Channel 5 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch5ps",
            extends: None,
            description: Some(
                "Channel 5 pulse skip regist",
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
            name: "Ch5tmfdt",
            extends: None,
            description: Some(
                "Channel 5 threshold monitor filter data regist",
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
            name: "Ch6cfg0",
            extends: None,
            description: Some(
                "Channel 6 configuration register",
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
            name: "Ch6cfg1",
            extends: None,
            description: Some(
                "Channel 6 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch6ctl",
            extends: None,
            description: Some(
                "Channel 6 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch6pdi",
            extends: None,
            description: Some(
                "Channel 6 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch6ps",
            extends: None,
            description: Some(
                "Channel 6 pulse skip regist",
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
            name: "Ch6tmfdt",
            extends: None,
            description: Some(
                "Channel 6 threshold monitor filter data regist",
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
            name: "Ch7cfg0",
            extends: None,
            description: Some(
                "Channel 7 configuration register",
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
            name: "Ch7cfg1",
            extends: None,
            description: Some(
                "Channel 7 configuration register",
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
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmfor",
                    description: Some(
                        "Threshold monitor filter oversampling rate (decimation rate)",
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
            name: "Ch7ctl",
            extends: None,
            description: Some(
                "Channel 7 control regist",
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
                        "Channel x enable",
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
                        "Channel x multiplexer select input data source",
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
                        "Data packing mode for HPDF_CHxPDI register",
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
            name: "Ch7pdi",
            extends: None,
            description: Some(
                "Channel 7 parallel data input regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "datain0",
                    description: Some(
                        "Data input for channel x",
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
                        "Data input for channel x or channel x+1",
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
            name: "Ch7ps",
            extends: None,
            description: Some(
                "Channel 7 pulse skip regist",
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
            name: "Ch7tmfdt",
            extends: None,
            description: Some(
                "Channel 7 threshold monitor filter data regist",
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
            name: "Flt0ct",
            extends: None,
            description: Some(
                "Filter 0 conversion timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctcnt",
                    description: Some(
                        "Extremes monitor minimum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0ctl0",
            extends: None,
            description: Some(
                "Filter 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "HPDF_FLTy enable",
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
                        "Inserted conversion synchronously with the HPDF_FLT0 SICC trigger",
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
                        "Regular conversion synchronously with HPDF_FLT0",
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
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode for regular conversions",
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
                "Filter 0 control register",
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
                    name: "icdoie",
                    description: Some(
                        "Inserted conversion data overflow interrupt enable",
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
                    name: "rcdoie",
                    description: Some(
                        "Regular conversion data overflow interrupt enable",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0emmax",
            extends: None,
            description: Some(
                "Filter 0 extremes monitor maximum regist",
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
                    bit_size: 3,
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
                "Filter 0 extremes monitor minimum regist",
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
                    bit_size: 3,
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
                "Filter 0 inserted channel group selection regist",
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0idata",
            extends: None,
            description: Some(
                "Filter 0 inserted group conversion data regist",
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
                    bit_size: 3,
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
                "Filter 0 interrupt flag clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icdofc",
                    description: Some(
                        "Clear the inserted conversion data overflow flag",
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
                    name: "rcdofc",
                    description: Some(
                        "Clear the regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0rdata",
            extends: None,
            description: Some(
                "Filter 0 regular channel conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt0sfcfg",
            extends: None,
            description: Some(
                "Filter 0 sinc filter configuration regist",
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
                        "Sinc filter oversampling ratio (decimation rate)",
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
                "Filter 0 status regist",
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
                    name: "icdof",
                    description: Some(
                        "Inserted conversion data overflow flag",
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
                    name: "rcdof",
                    description: Some(
                        "Regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0tmfc",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor flag clear regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt0tmht",
            extends: None,
            description: Some(
                "Filter 0 threshold monitor high threshold regist",
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
                    bit_size: 4,
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
                "Filter 0 threshold monitor low threshold regist",
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
                    bit_size: 4,
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
                "Filter 0 threshold monitor status regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1ct",
            extends: None,
            description: Some(
                "Filter 1 conversion timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctcnt",
                    description: Some(
                        "Extremes monitor minimum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1ctl0",
            extends: None,
            description: Some(
                "Filter 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "HPDF_FLTy enable",
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
                        "Inserted conversion synchronously with the HPDF_FLT0 SICC trigger",
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
                        "Regular conversion synchronously with HPDF_FLT0",
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
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode for regular conversions",
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
                "Filter 1 control register",
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
                    name: "icdoie",
                    description: Some(
                        "Inserted conversion data overflow interrupt enable",
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
                    name: "rcdoie",
                    description: Some(
                        "Regular conversion data overflow interrupt enable",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1emmax",
            extends: None,
            description: Some(
                "Filter 1 extremes monitor maximum regist",
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
                    bit_size: 3,
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
                "Filter 1 extremes monitor minimum regist",
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
                    bit_size: 3,
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
                "Filter 1 inserted channel group selection regist",
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1idata",
            extends: None,
            description: Some(
                "Filter 1 inserted group conversion data regist",
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
                    bit_size: 3,
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
                "Filter 1 interrupt flag clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icdofc",
                    description: Some(
                        "Clear the inserted conversion data overflow flag",
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
                    name: "rcdofc",
                    description: Some(
                        "Clear the regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1rdata",
            extends: None,
            description: Some(
                "Filter 1 regular channel conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt1sfcfg",
            extends: None,
            description: Some(
                "Filter 1 sinc filter configuration regist",
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
                        "Sinc filter oversampling ratio (decimation rate)",
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
                "Filter 1 status regist",
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
                    name: "icdof",
                    description: Some(
                        "Inserted conversion data overflow flag",
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
                    name: "rcdof",
                    description: Some(
                        "Regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1tmfc",
            extends: None,
            description: Some(
                "Filter 1 threshold monitor flag clear regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt1tmht",
            extends: None,
            description: Some(
                "Filter 1 threshold monitor high threshold regist",
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
                    bit_size: 4,
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
                "Filter 1 threshold monitor low threshold regist",
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
                    bit_size: 4,
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
                "Filter 1 threshold monitor status regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2ct",
            extends: None,
            description: Some(
                "Filter 2 conversion timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctcnt",
                    description: Some(
                        "Extremes monitor minimum value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2ctl0",
            extends: None,
            description: Some(
                "Filter 2 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "HPDF_FLTy enable",
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
                        "Inserted conversion synchronously with the HPDF_FLT0 SICC trigger",
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
                        "Regular conversion synchronously with HPDF_FLT0",
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
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode for regular conversions",
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
            name: "Flt2ctl1",
            extends: None,
            description: Some(
                "Filter 2 control register",
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
                    name: "icdoie",
                    description: Some(
                        "Inserted conversion data overflow interrupt enable",
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
                    name: "rcdoie",
                    description: Some(
                        "Regular conversion data overflow interrupt enable",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2emmax",
            extends: None,
            description: Some(
                "Filter 2 extremes monitor maximum regist",
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
                    bit_size: 3,
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
            name: "Flt2emmin",
            extends: None,
            description: Some(
                "Filter 2 extremes monitor minimum regist",
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
                    bit_size: 3,
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
            name: "Flt2icgs",
            extends: None,
            description: Some(
                "Filter 2 inserted channel group selection regist",
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2idata",
            extends: None,
            description: Some(
                "Filter 2 inserted group conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt2intc",
            extends: None,
            description: Some(
                "Filter 2 interrupt flag clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icdofc",
                    description: Some(
                        "Clear the inserted conversion data overflow flag",
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
                    name: "rcdofc",
                    description: Some(
                        "Clear the regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2rdata",
            extends: None,
            description: Some(
                "Filter 0 regular channel conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt2sfcfg",
            extends: None,
            description: Some(
                "Filter 2 sinc filter configuration regist",
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
                        "Sinc filter oversampling ratio (decimation rate)",
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
            name: "Flt2stat",
            extends: None,
            description: Some(
                "Filter 2 status regist",
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
                    name: "icdof",
                    description: Some(
                        "Inserted conversion data overflow flag",
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
                    name: "rcdof",
                    description: Some(
                        "Regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2tmfc",
            extends: None,
            description: Some(
                "Filter 2 threshold monitor flag clear regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt2tmht",
            extends: None,
            description: Some(
                "Filter 2 threshold monitor high threshold regist",
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
                    bit_size: 4,
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
            name: "Flt2tmlt",
            extends: None,
            description: Some(
                "Filter 2 threshold monitor low threshold regist",
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
                    bit_size: 4,
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
            name: "Flt2tmstat",
            extends: None,
            description: Some(
                "Filter 2 threshold monitor status regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3ct",
            extends: None,
            description: Some(
                "Filter 3 conversion timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctcnt",
                    description: Some(
                        "Timer counting conversion time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3ctl0",
            extends: None,
            description: Some(
                "Filter 3 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flten",
                    description: Some(
                        "HPDF_FLTy enable",
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
                        "Inserted conversion synchronously with the HPDF_FLT0 SICC trigger",
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
                        "Regular conversion synchronously with HPDF_FLT0",
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
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fast",
                    description: Some(
                        "Fast conversion mode for regular conversions",
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
            name: "Flt3ctl1",
            extends: None,
            description: Some(
                "Filter 2 control register",
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
                    name: "icdoie",
                    description: Some(
                        "Inserted conversion data overflow interrupt enable",
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
                    name: "rcdoie",
                    description: Some(
                        "Regular conversion data overflow interrupt enable",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3emmax",
            extends: None,
            description: Some(
                "Filter 3 extremes monitor maximum regist",
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
                    bit_size: 3,
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
            name: "Flt3emmin",
            extends: None,
            description: Some(
                "Filter 3 extremes monitor minimum regist",
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
                    bit_size: 3,
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
            name: "Flt3icgs",
            extends: None,
            description: Some(
                "Filter 3 inserted channel group selection regist",
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3idata",
            extends: None,
            description: Some(
                "Filter 3 inserted group conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt3intc",
            extends: None,
            description: Some(
                "Filter 3 interrupt flag clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "icdofc",
                    description: Some(
                        "Clear the inserted conversion data overflow flag",
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
                    name: "rcdofc",
                    description: Some(
                        "Clear the regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3rdata",
            extends: None,
            description: Some(
                "Filter 3 regular channel conversion data regist",
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
                    bit_size: 3,
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
            name: "Flt3sfcfg",
            extends: None,
            description: Some(
                "Filter 3 sinc filter configuration regist",
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
                        "Sinc filter oversampling ratio (decimation rate)",
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
            name: "Flt3stat",
            extends: None,
            description: Some(
                "Filter 3 status regist",
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
                    name: "icdof",
                    description: Some(
                        "Inserted conversion data overflow flag",
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
                    name: "rcdof",
                    description: Some(
                        "Regular conversion data overflow flag",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3tmfc",
            extends: None,
            description: Some(
                "Filter 3 threshold monitor flag clear regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Flt3tmht",
            extends: None,
            description: Some(
                "Filter 3 threshold monitor high threshold regist",
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
                    bit_size: 4,
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
            name: "Flt3tmlt",
            extends: None,
            description: Some(
                "Filter 3 threshold monitor low threshold regist",
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
                    bit_size: 4,
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
            name: "Flt3tmstat",
            extends: None,
            description: Some(
                "Filter 3 threshold monitor status regist",
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
                    bit_size: 8,
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
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                