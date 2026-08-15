
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mdma",
            extends: None,
            description: Some(
                "Master direct memory access controller",
            ),
            items: &[
                BlockItem {
                    name: "gintf",
                    description: Some(
                        "Global interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0stat0",
                    description: Some(
                        "Channel 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0statc",
                    description: Some(
                        "Channel 0 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0stat1",
                    description: Some(
                        "Channel 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0ctl0",
                    description: Some(
                        "Channel 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0cfg",
                    description: Some(
                        "Channel 0 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0btcfg",
                    description: Some(
                        "Channel 0 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0saddr",
                    description: Some(
                        "Channel 0 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0daddr",
                    description: Some(
                        "Channel 0 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0mbaddru",
                    description: Some(
                        "Channel 0 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0laddr",
                    description: Some(
                        "Channel 0 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0ctl1",
                    description: Some(
                        "Channel 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0maddr",
                    description: Some(
                        "Channel 0 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0mdata",
                    description: Some(
                        "Channel 0 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1stat0",
                    description: Some(
                        "Channel 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1statc",
                    description: Some(
                        "Channel 1 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1stat1",
                    description: Some(
                        "Channel 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1ctl0",
                    description: Some(
                        "Channel 1 control register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1cfg",
                    description: Some(
                        "Channel 1 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1btcfg",
                    description: Some(
                        "Channel 1 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1saddr",
                    description: Some(
                        "Channel 1 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1daddr",
                    description: Some(
                        "Channel 1 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1mbaddru",
                    description: Some(
                        "Channel 1 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1laddr",
                    description: Some(
                        "Channel 1 link address regist",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1ctl1",
                    description: Some(
                        "Channel 1 control register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1maddr",
                    description: Some(
                        "Channel 1 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1mdata",
                    description: Some(
                        "Channel 1 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2stat0",
                    description: Some(
                        "Channel 2 status register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2statc",
                    description: Some(
                        "Channel 2 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2stat1",
                    description: Some(
                        "Channel 2 status register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2ctl0",
                    description: Some(
                        "Channel 2 control register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2cfg",
                    description: Some(
                        "Channel 2 configure regist",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2btcfg",
                    description: Some(
                        "Channel 2 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2saddr",
                    description: Some(
                        "Channel 2 source address regist",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2daddr",
                    description: Some(
                        "Channel 2 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2mbaddru",
                    description: Some(
                        "Channel 2 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2laddr",
                    description: Some(
                        "Channel 2 link address regist",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2ctl1",
                    description: Some(
                        "Channel 2 control register",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2maddr",
                    description: Some(
                        "Channel 2 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2mdata",
                    description: Some(
                        "Channel 2 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3stat0",
                    description: Some(
                        "Channel 3 status register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3statc",
                    description: Some(
                        "Channel 3 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3stat1",
                    description: Some(
                        "Channel 3 status register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3ctl0",
                    description: Some(
                        "Channel 3 control register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3cfg",
                    description: Some(
                        "Channel 3 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3btcfg",
                    description: Some(
                        "Channel 3 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3saddr",
                    description: Some(
                        "Channel 3 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3daddr",
                    description: Some(
                        "Channel 3 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3mbaddru",
                    description: Some(
                        "Channel 3 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3laddr",
                    description: Some(
                        "Channel 3 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3ctl1",
                    description: Some(
                        "Channel 3 control register",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3maddr",
                    description: Some(
                        "Channel 3 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3mdata",
                    description: Some(
                        "Channel 3 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4stat0",
                    description: Some(
                        "Channel 4 status register",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4statc",
                    description: Some(
                        "Channel 4 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4stat1",
                    description: Some(
                        "Channel 4 status register",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4ctl0",
                    description: Some(
                        "Channel 4 control register",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4cfg",
                    description: Some(
                        "Channel 4 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4btcfg",
                    description: Some(
                        "Channel 4 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4saddr",
                    description: Some(
                        "Channel 4 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4daddr",
                    description: Some(
                        "Channel 4 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4mbaddru",
                    description: Some(
                        "Channel 4 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4laddr",
                    description: Some(
                        "Channel 4 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4ctl1",
                    description: Some(
                        "Channel 4 control register",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4maddr",
                    description: Some(
                        "Channel 4 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4mdata",
                    description: Some(
                        "Channel 4 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5stat0",
                    description: Some(
                        "Channel 5 status register",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5statc",
                    description: Some(
                        "Channel 5 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5stat1",
                    description: Some(
                        "Channel 5 status register",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5ctl0",
                    description: Some(
                        "Channel 5 control register",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5cfg",
                    description: Some(
                        "Channel 5 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5btcfg",
                    description: Some(
                        "Channel 5 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5saddr",
                    description: Some(
                        "Channel 5 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5daddr",
                    description: Some(
                        "Channel 5 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5mbaddru",
                    description: Some(
                        "Channel 5 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5laddr",
                    description: Some(
                        "Channel 5 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5ctl1",
                    description: Some(
                        "Channel 5 control register",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5maddr",
                    description: Some(
                        "Channel 5 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5mdata",
                    description: Some(
                        "Channel 5 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6stat0",
                    description: Some(
                        "Channel 6 status register",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6statc",
                    description: Some(
                        "Channel 6 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6stat1",
                    description: Some(
                        "Channel 6 status register",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6ctl0",
                    description: Some(
                        "Channel 6 control register",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6cfg",
                    description: Some(
                        "Channel 6 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x1d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6btcfg",
                    description: Some(
                        "Channel 6 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x1d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6saddr",
                    description: Some(
                        "Channel 6 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x1d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6daddr",
                    description: Some(
                        "Channel 6 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x1dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6mbaddru",
                    description: Some(
                        "Channel 6 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x1e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6laddr",
                    description: Some(
                        "Channel 6 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x1e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6ctl1",
                    description: Some(
                        "Channel 6 control register",
                    ),
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6maddr",
                    description: Some(
                        "Channel 6 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6mdata",
                    description: Some(
                        "Channel 6 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7stat0",
                    description: Some(
                        "Channel 7 status register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7statc",
                    description: Some(
                        "Channel 7 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7stat1",
                    description: Some(
                        "Channel 7 status register",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7ctl0",
                    description: Some(
                        "Channel 7 control register",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7cfg",
                    description: Some(
                        "Channel 7 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7btcfg",
                    description: Some(
                        "Channel 7 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7saddr",
                    description: Some(
                        "Channel 7 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7daddr",
                    description: Some(
                        "Channel 7 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7mbaddru",
                    description: Some(
                        "Channel 7 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7laddr",
                    description: Some(
                        "Channel 7 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7ctl1",
                    description: Some(
                        "Channel 7 control register",
                    ),
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7maddr",
                    description: Some(
                        "Channel 7 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch7mdata",
                    description: Some(
                        "Channel 7 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch7mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8stat0",
                    description: Some(
                        "Channel 8 status register",
                    ),
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8statc",
                    description: Some(
                        "Channel 8 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8stat1",
                    description: Some(
                        "Channel 8 status register",
                    ),
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8ctl0",
                    description: Some(
                        "Channel 8 control register",
                    ),
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8cfg",
                    description: Some(
                        "Channel 8 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8btcfg",
                    description: Some(
                        "Channel 8 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8saddr",
                    description: Some(
                        "Channel 8 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x258,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8daddr",
                    description: Some(
                        "Channel 8 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x25c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8mbaddru",
                    description: Some(
                        "Channel 8 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8laddr",
                    description: Some(
                        "Channel 8 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8ctl1",
                    description: Some(
                        "Channel 8 control register",
                    ),
                    array: None,
                    byte_offset: 0x268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8maddr",
                    description: Some(
                        "Channel 8 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x270,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch8mdata",
                    description: Some(
                        "Channel 8 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x274,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch8mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9stat0",
                    description: Some(
                        "Channel 9 status register",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9statc",
                    description: Some(
                        "Channel 9 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9stat1",
                    description: Some(
                        "Channel 9 status register",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9ctl0",
                    description: Some(
                        "Channel 9 control register",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9cfg",
                    description: Some(
                        "Channel 9 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9btcfg",
                    description: Some(
                        "Channel 9 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9saddr",
                    description: Some(
                        "Channel 9 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9daddr",
                    description: Some(
                        "Channel 9 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9mbaddru",
                    description: Some(
                        "Channel 9 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9laddr",
                    description: Some(
                        "Channel 9 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9ctl1",
                    description: Some(
                        "Channel 9 control register",
                    ),
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9maddr",
                    description: Some(
                        "Channel 9 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch9mdata",
                    description: Some(
                        "Channel 9 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch9mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10stat0",
                    description: Some(
                        "Channel 10 status register",
                    ),
                    array: None,
                    byte_offset: 0x2c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10statc",
                    description: Some(
                        "Channel 10 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x2c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10stat1",
                    description: Some(
                        "Channel 10 status register",
                    ),
                    array: None,
                    byte_offset: 0x2c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10ctl0",
                    description: Some(
                        "Channel 10 control register",
                    ),
                    array: None,
                    byte_offset: 0x2cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10cfg",
                    description: Some(
                        "Channel 10 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x2d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10btcfg",
                    description: Some(
                        "Channel 10 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x2d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10saddr",
                    description: Some(
                        "Channel 10 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x2d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10daddr",
                    description: Some(
                        "Channel 10 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x2dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10mbaddru",
                    description: Some(
                        "Channel 10 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x2e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10laddr",
                    description: Some(
                        "Channel 10 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x2e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10ctl1",
                    description: Some(
                        "Channel 10 control register",
                    ),
                    array: None,
                    byte_offset: 0x2e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10maddr",
                    description: Some(
                        "Channel 10 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x2f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch10mdata",
                    description: Some(
                        "Channel 10 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x2f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch10mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11stat0",
                    description: Some(
                        "Channel 11 status register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11statc",
                    description: Some(
                        "Channel 11 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11stat1",
                    description: Some(
                        "Channel 11 status register",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11ctl0",
                    description: Some(
                        "Channel 11 control register",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11cfg",
                    description: Some(
                        "Channel 11 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11btcfg",
                    description: Some(
                        "Channel 11 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11saddr",
                    description: Some(
                        "Channel 11 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11daddr",
                    description: Some(
                        "Channel 11 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x31c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11mbaddru",
                    description: Some(
                        "Channel 11 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11laddr",
                    description: Some(
                        "Channel 11 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x324,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11ctl1",
                    description: Some(
                        "Channel 11 control register",
                    ),
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11maddr",
                    description: Some(
                        "Channel 11 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch11mdata",
                    description: Some(
                        "Channel 11 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x334,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch11mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12stat0",
                    description: Some(
                        "Channel 12 status register",
                    ),
                    array: None,
                    byte_offset: 0x340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12statc",
                    description: Some(
                        "Channel 12 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12stat1",
                    description: Some(
                        "Channel 12 status register",
                    ),
                    array: None,
                    byte_offset: 0x348,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12ctl0",
                    description: Some(
                        "Channel 12 control register",
                    ),
                    array: None,
                    byte_offset: 0x34c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12cfg",
                    description: Some(
                        "Channel 12 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x350,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12btcfg",
                    description: Some(
                        "Channel 12 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x354,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12saddr",
                    description: Some(
                        "Channel 12 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x358,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12daddr",
                    description: Some(
                        "Channel 12 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x35c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12mbaddru",
                    description: Some(
                        "Channel 12 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12laddr",
                    description: Some(
                        "Channel 12 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x364,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12ctl1",
                    description: Some(
                        "Channel 12 control register",
                    ),
                    array: None,
                    byte_offset: 0x368,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12maddr",
                    description: Some(
                        "Channel 12 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x370,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch12mdata",
                    description: Some(
                        "Channel 12 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x374,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch12mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13stat0",
                    description: Some(
                        "Channel 13 status register",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13statc",
                    description: Some(
                        "Channel 13 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13stat1",
                    description: Some(
                        "Channel 13 status register",
                    ),
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13ctl0",
                    description: Some(
                        "Channel 13 control register",
                    ),
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13cfg",
                    description: Some(
                        "Channel 13 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13btcfg",
                    description: Some(
                        "Channel 13 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13saddr",
                    description: Some(
                        "Channel 13 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x398,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13daddr",
                    description: Some(
                        "Channel 13 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x39c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13mbaddru",
                    description: Some(
                        "Channel 13 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x3a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13laddr",
                    description: Some(
                        "Channel 13 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x3a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13ctl1",
                    description: Some(
                        "Channel 13 control register",
                    ),
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13maddr",
                    description: Some(
                        "Channel 13 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x3b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch13mdata",
                    description: Some(
                        "Channel 13 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x3b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch13mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14stat0",
                    description: Some(
                        "Channel 14 status register",
                    ),
                    array: None,
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14statc",
                    description: Some(
                        "Channel 14 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x3c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14stat1",
                    description: Some(
                        "Channel 14 status register",
                    ),
                    array: None,
                    byte_offset: 0x3c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14ctl0",
                    description: Some(
                        "Channel 14 control register",
                    ),
                    array: None,
                    byte_offset: 0x3cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14cfg",
                    description: Some(
                        "Channel 14 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x3d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14btcfg",
                    description: Some(
                        "Channel 14 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x3d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14saddr",
                    description: Some(
                        "Channel 14 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x3d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14daddr",
                    description: Some(
                        "Channel 14 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14mbaddru",
                    description: Some(
                        "Channel 14 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x3e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14laddr",
                    description: Some(
                        "Channel 14 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x3e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14ctl1",
                    description: Some(
                        "Channel 14 control register",
                    ),
                    array: None,
                    byte_offset: 0x3e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14maddr",
                    description: Some(
                        "Channel 14 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x3f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch14mdata",
                    description: Some(
                        "Channel 14 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x3f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch14mdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15stat0",
                    description: Some(
                        "Channel 15 status register",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15statc",
                    description: Some(
                        "Channel 15 status clear regist",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15statc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15stat1",
                    description: Some(
                        "Channel 15 status register",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15ctl0",
                    description: Some(
                        "Channel 15 control register",
                    ),
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15cfg",
                    description: Some(
                        "Channel 15 configure regist",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15btcfg",
                    description: Some(
                        "Channel 15 block transfer configure regist",
                    ),
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15btcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15saddr",
                    description: Some(
                        "Channel 15 source address regist",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15daddr",
                    description: Some(
                        "Channel 15 destination address regist",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15daddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15mbaddru",
                    description: Some(
                        "Channel 15 multi-block address update regist",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15mbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15laddr",
                    description: Some(
                        "Channel 15 link address regist",
                    ),
                    array: None,
                    byte_offset: 0x424,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15laddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15ctl1",
                    description: Some(
                        "Channel 15 control register",
                    ),
                    array: None,
                    byte_offset: 0x428,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15maddr",
                    description: Some(
                        "Channel 15 mask address regist",
                    ),
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15maddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch15mdata",
                    description: Some(
                        "Channel 15 mask data regist",
                    ),
                    array: None,
                    byte_offset: 0x434,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch15mdata",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ch0btcfg",
            extends: None,
            description: Some(
                "Channel 0 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch0cfg",
            extends: None,
            description: Some(
                "Channel 0 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch0ctl0",
            extends: None,
            description: Some(
                "Channel 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch0ctl1",
            extends: None,
            description: Some(
                "Channel 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch0daddr",
            extends: None,
            description: Some(
                "Channel 0 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch0laddr",
            extends: None,
            description: Some(
                "Channel 0 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch0maddr",
            extends: None,
            description: Some(
                "Channel 0 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch0mbaddru",
            extends: None,
            description: Some(
                "Channel 0 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch0mdata",
            extends: None,
            description: Some(
                "Channel 0 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch0saddr",
            extends: None,
            description: Some(
                "Channel 0 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch0stat0",
            extends: None,
            description: Some(
                "Channel 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch0stat1",
            extends: None,
            description: Some(
                "Channel 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch0statc",
            extends: None,
            description: Some(
                "Channel 0 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch10btcfg",
            extends: None,
            description: Some(
                "Channel 10 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch10cfg",
            extends: None,
            description: Some(
                "Channel 10 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch10ctl0",
            extends: None,
            description: Some(
                "Channel 10 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch10ctl1",
            extends: None,
            description: Some(
                "Channel 10 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch10daddr",
            extends: None,
            description: Some(
                "Channel 10 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch10laddr",
            extends: None,
            description: Some(
                "Channel 10 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch10maddr",
            extends: None,
            description: Some(
                "Channel 10 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch10mbaddru",
            extends: None,
            description: Some(
                "Channel 10 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch10mdata",
            extends: None,
            description: Some(
                "Channel 10 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch10saddr",
            extends: None,
            description: Some(
                "Channel 10 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch10stat0",
            extends: None,
            description: Some(
                "Channel 10 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch10stat1",
            extends: None,
            description: Some(
                "Channel 10 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch10statc",
            extends: None,
            description: Some(
                "Channel 10 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch11btcfg",
            extends: None,
            description: Some(
                "Channel 11 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch11cfg",
            extends: None,
            description: Some(
                "Channel 11 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch11ctl0",
            extends: None,
            description: Some(
                "Channel 11 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch11ctl1",
            extends: None,
            description: Some(
                "Channel 11 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch11daddr",
            extends: None,
            description: Some(
                "Channel 11 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch11laddr",
            extends: None,
            description: Some(
                "Channel 11 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch11maddr",
            extends: None,
            description: Some(
                "Channel 11 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch11mbaddru",
            extends: None,
            description: Some(
                "Channel 11 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch11mdata",
            extends: None,
            description: Some(
                "Channel 11 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch11saddr",
            extends: None,
            description: Some(
                "Channel 11 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch11stat0",
            extends: None,
            description: Some(
                "Channel 11 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch11stat1",
            extends: None,
            description: Some(
                "Channel 11 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch11statc",
            extends: None,
            description: Some(
                "Channel 11 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch12btcfg",
            extends: None,
            description: Some(
                "Channel 12 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch12cfg",
            extends: None,
            description: Some(
                "Channel 12 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch12ctl0",
            extends: None,
            description: Some(
                "Channel 12 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch12ctl1",
            extends: None,
            description: Some(
                "Channel 12 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch12daddr",
            extends: None,
            description: Some(
                "Channel 12 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch12laddr",
            extends: None,
            description: Some(
                "Channel 12 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch12maddr",
            extends: None,
            description: Some(
                "Channel 12 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch12mbaddru",
            extends: None,
            description: Some(
                "Channel 12 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch12mdata",
            extends: None,
            description: Some(
                "Channel 12 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch12saddr",
            extends: None,
            description: Some(
                "Channel 12 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch12stat0",
            extends: None,
            description: Some(
                "Channel 12 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch12stat1",
            extends: None,
            description: Some(
                "Channel 12 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch12statc",
            extends: None,
            description: Some(
                "Channel 12 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch13btcfg",
            extends: None,
            description: Some(
                "Channel 13 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch13cfg",
            extends: None,
            description: Some(
                "Channel 13 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch13ctl0",
            extends: None,
            description: Some(
                "Channel 13 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch13ctl1",
            extends: None,
            description: Some(
                "Channel 13 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch13daddr",
            extends: None,
            description: Some(
                "Channel 13 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch13laddr",
            extends: None,
            description: Some(
                "Channel 13 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch13maddr",
            extends: None,
            description: Some(
                "Channel 13 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch13mbaddru",
            extends: None,
            description: Some(
                "Channel 13 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch13mdata",
            extends: None,
            description: Some(
                "Channel 13 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch13saddr",
            extends: None,
            description: Some(
                "Channel 13 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch13stat0",
            extends: None,
            description: Some(
                "Channel 13 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch13stat1",
            extends: None,
            description: Some(
                "Channel 13 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch13statc",
            extends: None,
            description: Some(
                "Channel 13 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch14btcfg",
            extends: None,
            description: Some(
                "Channel 14 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch14cfg",
            extends: None,
            description: Some(
                "Channel 14 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch14ctl0",
            extends: None,
            description: Some(
                "Channel 14 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch14ctl1",
            extends: None,
            description: Some(
                "Channel 14 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch14daddr",
            extends: None,
            description: Some(
                "Channel 14 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch14laddr",
            extends: None,
            description: Some(
                "Channel 14 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch14maddr",
            extends: None,
            description: Some(
                "Channel 14 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch14mbaddru",
            extends: None,
            description: Some(
                "Channel 14 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch14mdata",
            extends: None,
            description: Some(
                "Channel 14 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch14saddr",
            extends: None,
            description: Some(
                "Channel 14 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch14stat0",
            extends: None,
            description: Some(
                "Channel 14 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch14stat1",
            extends: None,
            description: Some(
                "Channel 14 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch14statc",
            extends: None,
            description: Some(
                "Channel 14 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch15btcfg",
            extends: None,
            description: Some(
                "Channel 15 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch15cfg",
            extends: None,
            description: Some(
                "Channel 15 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch15ctl0",
            extends: None,
            description: Some(
                "Channel 15 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch15ctl1",
            extends: None,
            description: Some(
                "Channel 15 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch15daddr",
            extends: None,
            description: Some(
                "Channel 15 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch15laddr",
            extends: None,
            description: Some(
                "Channel 15 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch15maddr",
            extends: None,
            description: Some(
                "Channel 15 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch15mbaddru",
            extends: None,
            description: Some(
                "Channel 15 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch15mdata",
            extends: None,
            description: Some(
                "Channel 15 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch15saddr",
            extends: None,
            description: Some(
                "Channel 15 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch15stat0",
            extends: None,
            description: Some(
                "Channel 15 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch15stat1",
            extends: None,
            description: Some(
                "Channel 15 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch15statc",
            extends: None,
            description: Some(
                "Channel 15 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch1btcfg",
            extends: None,
            description: Some(
                "Channel 1 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch1cfg",
            extends: None,
            description: Some(
                "Channel 1 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch1ctl0",
            extends: None,
            description: Some(
                "Channel 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch1ctl1",
            extends: None,
            description: Some(
                "Channel 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch1daddr",
            extends: None,
            description: Some(
                "Channel 1 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch1laddr",
            extends: None,
            description: Some(
                "Channel 1 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch1maddr",
            extends: None,
            description: Some(
                "Channel 1 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch1mbaddru",
            extends: None,
            description: Some(
                "Channel 1 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch1mdata",
            extends: None,
            description: Some(
                "Channel 1 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch1saddr",
            extends: None,
            description: Some(
                "Channel 1 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch1stat0",
            extends: None,
            description: Some(
                "Channel 1 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch1stat1",
            extends: None,
            description: Some(
                "Channel 1 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch1statc",
            extends: None,
            description: Some(
                "Channel 1 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch2btcfg",
            extends: None,
            description: Some(
                "Channel 2 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch2cfg",
            extends: None,
            description: Some(
                "Channel 2 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch2ctl0",
            extends: None,
            description: Some(
                "Channel 2 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch2ctl1",
            extends: None,
            description: Some(
                "Channel 2 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch2daddr",
            extends: None,
            description: Some(
                "Channel 2 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch2laddr",
            extends: None,
            description: Some(
                "Channel 2 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch2maddr",
            extends: None,
            description: Some(
                "Channel 2 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch2mbaddru",
            extends: None,
            description: Some(
                "Channel 2 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch2mdata",
            extends: None,
            description: Some(
                "Channel 2 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch2saddr",
            extends: None,
            description: Some(
                "Channel 2 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch2stat0",
            extends: None,
            description: Some(
                "Channel 2 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch2stat1",
            extends: None,
            description: Some(
                "Channel 2 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch2statc",
            extends: None,
            description: Some(
                "Channel 2 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch3btcfg",
            extends: None,
            description: Some(
                "Channel 3 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch3cfg",
            extends: None,
            description: Some(
                "Channel 3 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch3ctl0",
            extends: None,
            description: Some(
                "Channel 3 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch3ctl1",
            extends: None,
            description: Some(
                "Channel 3 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch3daddr",
            extends: None,
            description: Some(
                "Channel 3 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch3laddr",
            extends: None,
            description: Some(
                "Channel 3 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch3maddr",
            extends: None,
            description: Some(
                "Channel 3 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch3mbaddru",
            extends: None,
            description: Some(
                "Channel 3 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch3mdata",
            extends: None,
            description: Some(
                "Channel 3 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch3saddr",
            extends: None,
            description: Some(
                "Channel 3 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch3stat0",
            extends: None,
            description: Some(
                "Channel 3 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch3stat1",
            extends: None,
            description: Some(
                "Channel 3 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch3statc",
            extends: None,
            description: Some(
                "Channel 3 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch4btcfg",
            extends: None,
            description: Some(
                "Channel 4 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch4cfg",
            extends: None,
            description: Some(
                "Channel 4 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch4ctl0",
            extends: None,
            description: Some(
                "Channel 4 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch4ctl1",
            extends: None,
            description: Some(
                "Channel 4 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch4daddr",
            extends: None,
            description: Some(
                "Channel 4 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch4laddr",
            extends: None,
            description: Some(
                "Channel 4 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch4maddr",
            extends: None,
            description: Some(
                "Channel 4 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch4mbaddru",
            extends: None,
            description: Some(
                "Channel 4 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch4mdata",
            extends: None,
            description: Some(
                "Channel 4 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch4saddr",
            extends: None,
            description: Some(
                "Channel 4 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch4stat0",
            extends: None,
            description: Some(
                "Channel 4 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch4stat1",
            extends: None,
            description: Some(
                "Channel 4 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch4statc",
            extends: None,
            description: Some(
                "Channel 4 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch5btcfg",
            extends: None,
            description: Some(
                "Channel 5 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch5cfg",
            extends: None,
            description: Some(
                "Channel 5 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch5ctl0",
            extends: None,
            description: Some(
                "Channel 5 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch5ctl1",
            extends: None,
            description: Some(
                "Channel 5 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch5daddr",
            extends: None,
            description: Some(
                "Channel 5 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch5laddr",
            extends: None,
            description: Some(
                "Channel 5 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch5maddr",
            extends: None,
            description: Some(
                "Channel 5 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch5mbaddru",
            extends: None,
            description: Some(
                "Channel 5 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch5mdata",
            extends: None,
            description: Some(
                "Channel 5 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch5saddr",
            extends: None,
            description: Some(
                "Channel 5 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch5stat0",
            extends: None,
            description: Some(
                "Channel 5 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch5stat1",
            extends: None,
            description: Some(
                "Channel 5 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch5statc",
            extends: None,
            description: Some(
                "Channel 5 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch6btcfg",
            extends: None,
            description: Some(
                "Channel 6 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch6cfg",
            extends: None,
            description: Some(
                "Channel 6 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch6ctl0",
            extends: None,
            description: Some(
                "Channel 6 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch6ctl1",
            extends: None,
            description: Some(
                "Channel 6 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch6daddr",
            extends: None,
            description: Some(
                "Channel 6 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch6laddr",
            extends: None,
            description: Some(
                "Channel 6 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch6maddr",
            extends: None,
            description: Some(
                "Channel 6 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch6mbaddru",
            extends: None,
            description: Some(
                "Channel 6 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch6mdata",
            extends: None,
            description: Some(
                "Channel 6 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch6saddr",
            extends: None,
            description: Some(
                "Channel 6 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch6stat0",
            extends: None,
            description: Some(
                "Channel 6 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch6stat1",
            extends: None,
            description: Some(
                "Channel 6 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch6statc",
            extends: None,
            description: Some(
                "Channel 6 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch7btcfg",
            extends: None,
            description: Some(
                "Channel 7 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch7cfg",
            extends: None,
            description: Some(
                "Channel 7 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch7ctl0",
            extends: None,
            description: Some(
                "Channel 7 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch7ctl1",
            extends: None,
            description: Some(
                "Channel 7 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch7daddr",
            extends: None,
            description: Some(
                "Channel 7 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch7laddr",
            extends: None,
            description: Some(
                "Channel 7 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch7maddr",
            extends: None,
            description: Some(
                "Channel 7 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch7mbaddru",
            extends: None,
            description: Some(
                "Channel 7 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch7mdata",
            extends: None,
            description: Some(
                "Channel 7 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch7saddr",
            extends: None,
            description: Some(
                "Channel 7 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch7stat0",
            extends: None,
            description: Some(
                "Channel 7 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch7stat1",
            extends: None,
            description: Some(
                "Channel 7 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch7statc",
            extends: None,
            description: Some(
                "Channel 7 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch8btcfg",
            extends: None,
            description: Some(
                "Channel 8 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch8cfg",
            extends: None,
            description: Some(
                "Channel 8 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch8ctl0",
            extends: None,
            description: Some(
                "Channel 8 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch8ctl1",
            extends: None,
            description: Some(
                "Channel 8 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch8daddr",
            extends: None,
            description: Some(
                "Channel 8 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch8laddr",
            extends: None,
            description: Some(
                "Channel 8 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch8maddr",
            extends: None,
            description: Some(
                "Channel 8 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch8mbaddru",
            extends: None,
            description: Some(
                "Channel 8 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch8mdata",
            extends: None,
            description: Some(
                "Channel 8 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch8saddr",
            extends: None,
            description: Some(
                "Channel 8 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch8stat0",
            extends: None,
            description: Some(
                "Channel 8 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch8stat1",
            extends: None,
            description: Some(
                "Channel 8 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch8statc",
            extends: None,
            description: Some(
                "Channel 8 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Ch9btcfg",
            extends: None,
            description: Some(
                "Channel 9 block transfer configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tbnum",
                    description: Some(
                        "Transfer byte number in block",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 17,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saddrum",
                    description: Some(
                        "Multi-block source address update mode",
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
                    name: "daddrum",
                    description: Some(
                        "Multi-block destination address update mode",
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
                    name: "brnum",
                    description: Some(
                        "Multi-block number",
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
            name: "Ch9cfg",
            extends: None,
            description: Some(
                "Channel 9 configure regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "simod",
                    description: Some(
                        "Source increment mode",
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
                    name: "dimod",
                    description: Some(
                        "Destination increment mode",
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
                    name: "swidth",
                    description: Some(
                        "Data size of source",
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
                    name: "dwidth",
                    description: Some(
                        "Data size of destination",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sios",
                    description: Some(
                        "Offset size of source increment",
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
                    name: "dios",
                    description: Some(
                        "Offset size of destination increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sburst",
                    description: Some(
                        "Transfer burst type of source",
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
                    name: "dburst",
                    description: Some(
                        "Transfer burst type of destination",
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
                    name: "btlen",
                    description: Some(
                        "Buffer transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pken",
                    description: Some(
                        "Pack enable",
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
                    name: "pamod",
                    description: Some(
                        "Padding and alignement mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trigmod",
                    description: Some(
                        "Trigger mode",
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
                    name: "swreqmod",
                    description: Some(
                        "Software request mode",
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
                    name: "bwmod",
                    description: Some(
                        "Bufferable write mode",
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
            name: "Ch9ctl0",
            extends: None,
            description: Some(
                "Channel 9 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chen",
                    description: Some(
                        "Channel enable",
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
                    name: "errie",
                    description: Some(
                        "Transfer error interrupt enable",
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
                    name: "chtcie",
                    description: Some(
                        "Channel transfer complete interrupt enable",
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
                    name: "mbtcie",
                    description: Some(
                        "Multi-block transfer complete interrupt enable",
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
                    name: "btcie",
                    description: Some(
                        "Block transfer complete interrupt enable",
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
                    name: "tcie",
                    description: Some(
                        "Buffer transfer complete interrupt enable",
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
                    name: "prio",
                    description: Some(
                        "Priority level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "smoden",
                    description: Some(
                        "Secure mode enable",
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
                    name: "bes",
                    description: Some(
                        "Byte endianess swapping in half word",
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
                    name: "hwes",
                    description: Some(
                        "Half word endianess swapping in word",
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
                    name: "wes",
                    description: Some(
                        "Word endianess swapping in double word",
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
                    name: "swreq",
                    description: Some(
                        "Software request",
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
            ],
        },
        FieldSet {
            name: "Ch9ctl1",
            extends: None,
            description: Some(
                "Channel 9 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
                Field {
                    name: "sbsel",
                    description: Some(
                        "Source bus select",
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
                    name: "dbsel",
                    description: Some(
                        "Destination bus select",
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
            ],
        },
        FieldSet {
            name: "Ch9daddr",
            extends: None,
            description: Some(
                "Channel 9 destination address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "daddr",
                    description: Some(
                        "Destination address",
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
            name: "Ch9laddr",
            extends: None,
            description: Some(
                "Channel 9 link address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "laddr",
                    description: Some(
                        "Link address",
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
            name: "Ch9maddr",
            extends: None,
            description: Some(
                "Channel 9 mask address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "maddr",
                    description: Some(
                        "Mask address",
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
            name: "Ch9mbaddru",
            extends: None,
            description: Some(
                "Channel 9 multi-block address update regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddruv",
                    description: Some(
                        "Source address update value",
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
                    name: "daddruv",
                    description: Some(
                        "Destination address update value",
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
            name: "Ch9mdata",
            extends: None,
            description: Some(
                "Channel 9 mask data regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdata",
                    description: Some(
                        "Mask data",
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
            name: "Ch9saddr",
            extends: None,
            description: Some(
                "Channel 9 source address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "saddr",
                    description: Some(
                        "Source address",
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
            name: "Ch9stat0",
            extends: None,
            description: Some(
                "Channel 9 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "err",
                    description: Some(
                        "Channel x transfer error flag",
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
                    name: "chtcf",
                    description: Some(
                        "Channel x channel transfer complete flag",
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
                    name: "mbtcf",
                    description: Some(
                        "Channel x multi-block transfer complete flag",
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
                    name: "btcf",
                    description: Some(
                        "Channel x block transfer complete flag",
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
                    name: "tcf",
                    description: Some(
                        "Channel x buffer transfer complete flag",
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
                    name: "reqaf",
                    description: Some(
                        "Channel x request active flag",
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
            ],
        },
        FieldSet {
            name: "Ch9stat1",
            extends: None,
            description: Some(
                "Channel 9 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "erraddr",
                    description: Some(
                        "Transfer error address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terrd",
                    description: Some(
                        "Transfer error direction",
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
                    name: "ldterr",
                    description: Some(
                        "Link data transfer error flag in the last transfer of the channel",
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
                    name: "mdterr",
                    description: Some(
                        "Mask data error flag",
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
                    name: "aserr",
                    description: Some(
                        "Address and size error flag",
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
                    name: "bzerr",
                    description: Some(
                        "Block size error flag",
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
            ],
        },
        FieldSet {
            name: "Ch9statc",
            extends: None,
            description: Some(
                "Channel 9 status clear regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "errc",
                    description: Some(
                        "Channel x transfer error flag clear",
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
                    name: "chtcfc",
                    description: Some(
                        "Channel x channel transfer complete flag clear",
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
                    name: "mbtcfc",
                    description: Some(
                        "Channel x buffer multi-block transfer complete flag clear",
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
                    name: "btcfc",
                    description: Some(
                        "Channel x buffer block transfer complete flag clear",
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
                    name: "tcfc",
                    description: Some(
                        "Channel x buffer transfer complete flag clear",
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
            name: "Gintf",
            extends: None,
            description: Some(
                "Global interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gif0",
                    description: Some(
                        "Global interrupt flag of channel 0",
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
                    name: "gif1",
                    description: Some(
                        "Global interrupt flag of channel 1",
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
                    name: "gif2",
                    description: Some(
                        "Global interrupt flag of channel 2",
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
                    name: "gif3",
                    description: Some(
                        "Global interrupt flag of channel 3",
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
                    name: "gif4",
                    description: Some(
                        "Global interrupt flag of channel 4",
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
                    name: "gif5",
                    description: Some(
                        "Global interrupt flag of channel 5",
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
                    name: "gif6",
                    description: Some(
                        "Global interrupt flag of channel 6",
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
                    name: "gif7",
                    description: Some(
                        "Global interrupt flag of channel 7",
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
                    name: "gif8",
                    description: Some(
                        "Global interrupt flag of channel 8",
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
                    name: "gif9",
                    description: Some(
                        "Global interrupt flag of channel 9",
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
                    name: "gif10",
                    description: Some(
                        "Global interrupt flag of channel 10",
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
                    name: "gif11",
                    description: Some(
                        "Global interrupt flag of channel 11",
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
                    name: "gif12",
                    description: Some(
                        "Global interrupt flag of channel 12",
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
                    name: "gif13",
                    description: Some(
                        "Global interrupt flag of channel 13",
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
                    name: "gif14",
                    description: Some(
                        "Global interrupt flag of channel 14",
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
                    name: "gif15",
                    description: Some(
                        "Global interrupt flag of channel 15",
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
            ],
        },
    ],
    enums: &[],
};
                