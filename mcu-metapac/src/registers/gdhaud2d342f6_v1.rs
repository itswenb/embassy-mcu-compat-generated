
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hau",
            extends: None,
            description: Some(
                "Hash Acceleration Unit",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "HAU control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "di",
                    description: Some(
                        "HAU data input register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Di",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "HAU configuration register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do0",
                    description: Some(
                        "HAU data output register 0",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do1",
                    description: Some(
                        "HAU data output register 1",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do2",
                    description: Some(
                        "HAU data output register 2",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do3",
                    description: Some(
                        "HAU data output register 3",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do4",
                    description: Some(
                        "HAU data output register 4",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "HAU interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "HAU status and flag register",
                    ),
                    array: None,
                    byte_offset: 0x24,
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
                    name: "ctxs0",
                    description: Some(
                        "Context switch register 0",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs1",
                    description: Some(
                        "Context switch register 1",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs2",
                    description: Some(
                        "Context switch register 2",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs3",
                    description: Some(
                        "Context switch register 3",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs4",
                    description: Some(
                        "Context switch register 4",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs5",
                    description: Some(
                        "Context switch register 5",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs6",
                    description: Some(
                        "Context switch register 6",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs7",
                    description: Some(
                        "Context switch register 7",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs8",
                    description: Some(
                        "Context switch register 8",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs9",
                    description: Some(
                        "Context switch register 9",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs10",
                    description: Some(
                        "Context switch register 10",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs11",
                    description: Some(
                        "Context switch register 11",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs12",
                    description: Some(
                        "Context switch register 12",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs13",
                    description: Some(
                        "Context switch register 13",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs14",
                    description: Some(
                        "Context switch register 14",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs15",
                    description: Some(
                        "Context switch register 15",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs16",
                    description: Some(
                        "Context switch register 16",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs17",
                    description: Some(
                        "Context switch register 17",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs18",
                    description: Some(
                        "Context switch register 18",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs19",
                    description: Some(
                        "Context switch register 19",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs20",
                    description: Some(
                        "Context switch register 20",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs21",
                    description: Some(
                        "Context switch register 21",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs22",
                    description: Some(
                        "Context switch register 22",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs23",
                    description: Some(
                        "Context switch register 23",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs24",
                    description: Some(
                        "Context switch register 24",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs25",
                    description: Some(
                        "Context switch register 25",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs26",
                    description: Some(
                        "Context switch register 26",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs27",
                    description: Some(
                        "Context switch register 27",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs28",
                    description: Some(
                        "Context switch register 28",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs29",
                    description: Some(
                        "Context switch register 29",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs30",
                    description: Some(
                        "Context switch register 30",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs31",
                    description: Some(
                        "Context switch register 31",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs32",
                    description: Some(
                        "Context switch register 32",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs32",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs33",
                    description: Some(
                        "Context switch register 33",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs33",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs34",
                    description: Some(
                        "Context switch register 34",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs34",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs35",
                    description: Some(
                        "Context switch register 35",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs35",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs36",
                    description: Some(
                        "Context switch register 36",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs36",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs37",
                    description: Some(
                        "Context switch register 37",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs37",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs38",
                    description: Some(
                        "Context switch register 38",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs38",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs39",
                    description: Some(
                        "Context switch register 39",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs39",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs40",
                    description: Some(
                        "Context switch register 40",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs40",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs41",
                    description: Some(
                        "Context switch register 41",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs41",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs42",
                    description: Some(
                        "Context switch register 42",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs42",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs43",
                    description: Some(
                        "Context switch register 43",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs43",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs44",
                    description: Some(
                        "Context switch register 44",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs44",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs45",
                    description: Some(
                        "Context switch register 45",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs45",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs46",
                    description: Some(
                        "Context switch register 46",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs46",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs47",
                    description: Some(
                        "Context switch register 47",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs47",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs48",
                    description: Some(
                        "Context switch register 48",
                    ),
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs48",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs49",
                    description: Some(
                        "Context switch register 49",
                    ),
                    array: None,
                    byte_offset: 0x1bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs49",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs50",
                    description: Some(
                        "Context switch register 50",
                    ),
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs50",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs51",
                    description: Some(
                        "Context switch register 51",
                    ),
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs51",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs52",
                    description: Some(
                        "Context switch register 52",
                    ),
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs52",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctxs53",
                    description: Some(
                        "Context switch register 53",
                    ),
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctxs53",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do5",
                    description: Some(
                        "HAU data output register 5",
                    ),
                    array: None,
                    byte_offset: 0x324,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do6",
                    description: Some(
                        "HAU data output register 6",
                    ),
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do7",
                    description: Some(
                        "HAU data output register 7",
                    ),
                    array: None,
                    byte_offset: 0x32c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do7",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some(
                "HAU configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbl",
                    description: Some(
                        "Valid bits length in the last word",
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
                    name: "calen",
                    description: Some(
                        "Digest calculation enable",
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
            name: "Ctl",
            extends: None,
            description: Some(
                "HAU control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "Start the digest calculation",
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
                    name: "dmae",
                    description: Some(
                        "DMA enable",
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
                    name: "datam",
                    description: Some(
                        "Data type mode",
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
                    name: "hms",
                    description: Some(
                        "HAU mode selection, must be changed when no computation is processing",
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
                    name: "algm_0",
                    description: Some(
                        "Algorithm selection bit 0",
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
                    name: "nwif",
                    description: Some(
                        "Number of words in the input FIFO",
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
                    name: "dine",
                    description: Some(
                        "DI register not empty",
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
                    name: "mds",
                    description: Some(
                        "Multiple DMA Selection",
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
                    name: "klm",
                    description: Some(
                        "Key length mode",
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
                    name: "algm_1",
                    description: Some(
                        "Algorithm selection bit 1",
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
            ],
        },
        FieldSet {
            name: "Ctxs0",
            extends: None,
            description: Some(
                "Context switch register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs1",
            extends: None,
            description: Some(
                "Context switch register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs10",
            extends: None,
            description: Some(
                "Context switch register 10",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs11",
            extends: None,
            description: Some(
                "Context switch register 11",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs12",
            extends: None,
            description: Some(
                "Context switch register 12",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs13",
            extends: None,
            description: Some(
                "Context switch register 13",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs14",
            extends: None,
            description: Some(
                "Context switch register 14",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs15",
            extends: None,
            description: Some(
                "Context switch register 15",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs16",
            extends: None,
            description: Some(
                "Context switch register 16",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs17",
            extends: None,
            description: Some(
                "Context switch register 17",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs18",
            extends: None,
            description: Some(
                "Context switch register 18",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs19",
            extends: None,
            description: Some(
                "Context switch register 19",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs2",
            extends: None,
            description: Some(
                "Context switch register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs20",
            extends: None,
            description: Some(
                "Context switch register 20",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs21",
            extends: None,
            description: Some(
                "Context switch register 21",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs22",
            extends: None,
            description: Some(
                "Context switch register 22",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs23",
            extends: None,
            description: Some(
                "Context switch register 23",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs24",
            extends: None,
            description: Some(
                "Context switch register 24",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs25",
            extends: None,
            description: Some(
                "Context switch register 25",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs26",
            extends: None,
            description: Some(
                "Context switch register 26",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs27",
            extends: None,
            description: Some(
                "Context switch register 27",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs28",
            extends: None,
            description: Some(
                "Context switch register 28",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs29",
            extends: None,
            description: Some(
                "Context switch register 29",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs3",
            extends: None,
            description: Some(
                "Context switch register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs30",
            extends: None,
            description: Some(
                "Context switch register 30",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs31",
            extends: None,
            description: Some(
                "Context switch register 31",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs32",
            extends: None,
            description: Some(
                "Context switch register 32",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs33",
            extends: None,
            description: Some(
                "Context switch register 33",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs34",
            extends: None,
            description: Some(
                "Context switch register 34",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs35",
            extends: None,
            description: Some(
                "Context switch register 35",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs36",
            extends: None,
            description: Some(
                "Context switch register 36",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs37",
            extends: None,
            description: Some(
                "Context switch register 37",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs38",
            extends: None,
            description: Some(
                "Context switch register 38",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs39",
            extends: None,
            description: Some(
                "Context switch register 39",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs4",
            extends: None,
            description: Some(
                "Context switch register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs40",
            extends: None,
            description: Some(
                "Context switch register 40",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs41",
            extends: None,
            description: Some(
                "Context switch register 41",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs42",
            extends: None,
            description: Some(
                "Context switch register 42",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs43",
            extends: None,
            description: Some(
                "Context switch register 43",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs44",
            extends: None,
            description: Some(
                "Context switch register 44",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs45",
            extends: None,
            description: Some(
                "Context switch register 45",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs46",
            extends: None,
            description: Some(
                "Context switch register 46",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs47",
            extends: None,
            description: Some(
                "Context switch register 47",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs48",
            extends: None,
            description: Some(
                "Context switch register 48",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs49",
            extends: None,
            description: Some(
                "Context switch register 49",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs5",
            extends: None,
            description: Some(
                "Context switch register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs50",
            extends: None,
            description: Some(
                "Context switch register 50",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs51",
            extends: None,
            description: Some(
                "Context switch register 51",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs52",
            extends: None,
            description: Some(
                "Context switch register 52",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs53",
            extends: None,
            description: Some(
                "Context switch register 53",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs6",
            extends: None,
            description: Some(
                "Context switch register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs7",
            extends: None,
            description: Some(
                "Context switch register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs8",
            extends: None,
            description: Some(
                "Context switch register 8",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Ctxs9",
            extends: None,
            description: Some(
                "Context switch register 9",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx",
                    description: Some(
                        "The complete internal status of the HAU core.",
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
            name: "Di",
            extends: None,
            description: Some(
                "HAU data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "di",
                    description: Some(
                        "Message data input",
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
            name: "Do0",
            extends: None,
            description: Some(
                "HAU data output register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do0",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do1",
            extends: None,
            description: Some(
                "HAU data output register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do1",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do2",
            extends: None,
            description: Some(
                "HAU data output register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do2",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do3",
            extends: None,
            description: Some(
                "HAU data output register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do3",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do4",
            extends: None,
            description: Some(
                "HAU data output register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do4",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do5",
            extends: None,
            description: Some(
                "HAU data output register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do5",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do6",
            extends: None,
            description: Some(
                "HAU data output register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do6",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Do7",
            extends: None,
            description: Some(
                "HAU data output register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do7",
                    description: Some(
                        "Messagedigest result of hash algorithm",
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
            name: "Inten",
            extends: None,
            description: Some(
                "HAU interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "diie",
                    description: Some(
                        "Data input interrupt enable",
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
                    name: "ccie",
                    description: Some(
                        "Calculation completion interrupt enable",
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
            name: "Stat",
            extends: None,
            description: Some(
                "HAU status and flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dif",
                    description: Some(
                        "Data input flag",
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
                    name: "ccf",
                    description: Some(
                        "Digest calculation completion flag",
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
                    name: "dmas",
                    description: Some(
                        "DMA status",
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
                    name: "busy",
                    description: Some(
                        "Busy bit",
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
            ],
        },
    ],
    enums: &[],
};
                