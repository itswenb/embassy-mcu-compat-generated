
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mdma",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "mdma_gintf",
                    description: None,
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaGintf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_0",
                    description: None,
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_0",
                    description: None,
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_0",
                    description: None,
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_0",
                    description: None,
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_0",
                    description: None,
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_0",
                    description: None,
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_0",
                    description: None,
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_0",
                    description: None,
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_0",
                    description: None,
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_0",
                    description: None,
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_0",
                    description: None,
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_0",
                    description: None,
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_0",
                    description: None,
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_1",
                    description: None,
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_1",
                    description: None,
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_1",
                    description: None,
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_1",
                    description: None,
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_1",
                    description: None,
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_1",
                    description: None,
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_1",
                    description: None,
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_1",
                    description: None,
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_1",
                    description: None,
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_1",
                    description: None,
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_1",
                    description: None,
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_1",
                    description: None,
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_1",
                    description: None,
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_2",
                    description: None,
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_2",
                    description: None,
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_2",
                    description: None,
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_2",
                    description: None,
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_2",
                    description: None,
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_2",
                    description: None,
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_2",
                    description: None,
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_2",
                    description: None,
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_2",
                    description: None,
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_2",
                    description: None,
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_2",
                    description: None,
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_2",
                    description: None,
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_2",
                    description: None,
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_3",
                    description: None,
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_3",
                    description: None,
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_3",
                    description: None,
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_3",
                    description: None,
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_3",
                    description: None,
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_3",
                    description: None,
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_3",
                    description: None,
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_3",
                    description: None,
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_3",
                    description: None,
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_3",
                    description: None,
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_3",
                    description: None,
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_3",
                    description: None,
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_3",
                    description: None,
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_4",
                    description: None,
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_4",
                    description: None,
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_4",
                    description: None,
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_4",
                    description: None,
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_4",
                    description: None,
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_4",
                    description: None,
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_4",
                    description: None,
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_4",
                    description: None,
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_4",
                    description: None,
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_4",
                    description: None,
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_4",
                    description: None,
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_4",
                    description: None,
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_4",
                    description: None,
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_5",
                    description: None,
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_5",
                    description: None,
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_5",
                    description: None,
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_5",
                    description: None,
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_5",
                    description: None,
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_5",
                    description: None,
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_5",
                    description: None,
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_5",
                    description: None,
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_5",
                    description: None,
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_5",
                    description: None,
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_5",
                    description: None,
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_5",
                    description: None,
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_5",
                    description: None,
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_6",
                    description: None,
                    array: None,
                    byte_offset: 0x1f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_7",
                    description: None,
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_7",
                    description: None,
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_7",
                    description: None,
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_7",
                    description: None,
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_7",
                    description: None,
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_7",
                    description: None,
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_7",
                    description: None,
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_7",
                    description: None,
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_7",
                    description: None,
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_7",
                    description: None,
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_7",
                    description: None,
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_7",
                    description: None,
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_7",
                    description: None,
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_8",
                    description: None,
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_8",
                    description: None,
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_8",
                    description: None,
                    array: None,
                    byte_offset: 0x248,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_8",
                    description: None,
                    array: None,
                    byte_offset: 0x24c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_8",
                    description: None,
                    array: None,
                    byte_offset: 0x250,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_8",
                    description: None,
                    array: None,
                    byte_offset: 0x254,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_8",
                    description: None,
                    array: None,
                    byte_offset: 0x258,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_8",
                    description: None,
                    array: None,
                    byte_offset: 0x25c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_8",
                    description: None,
                    array: None,
                    byte_offset: 0x260,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_8",
                    description: None,
                    array: None,
                    byte_offset: 0x264,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_8",
                    description: None,
                    array: None,
                    byte_offset: 0x268,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_8",
                    description: None,
                    array: None,
                    byte_offset: 0x270,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_8",
                    description: None,
                    array: None,
                    byte_offset: 0x274,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_9",
                    description: None,
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_9",
                    description: None,
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_9",
                    description: None,
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_9",
                    description: None,
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_9",
                    description: None,
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_9",
                    description: None,
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_9",
                    description: None,
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_9",
                    description: None,
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_9",
                    description: None,
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_9",
                    description: None,
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_9",
                    description: None,
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_9",
                    description: None,
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_9",
                    description: None,
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_10",
                    description: None,
                    array: None,
                    byte_offset: 0x2f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_11",
                    description: None,
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_11",
                    description: None,
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_11",
                    description: None,
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_11",
                    description: None,
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_11",
                    description: None,
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_11",
                    description: None,
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_11",
                    description: None,
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_11",
                    description: None,
                    array: None,
                    byte_offset: 0x31c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_11",
                    description: None,
                    array: None,
                    byte_offset: 0x320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_11",
                    description: None,
                    array: None,
                    byte_offset: 0x324,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_11",
                    description: None,
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_11",
                    description: None,
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_11",
                    description: None,
                    array: None,
                    byte_offset: 0x334,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_12",
                    description: None,
                    array: None,
                    byte_offset: 0x340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_12",
                    description: None,
                    array: None,
                    byte_offset: 0x344,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_12",
                    description: None,
                    array: None,
                    byte_offset: 0x348,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_12",
                    description: None,
                    array: None,
                    byte_offset: 0x34c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_12",
                    description: None,
                    array: None,
                    byte_offset: 0x350,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_12",
                    description: None,
                    array: None,
                    byte_offset: 0x354,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_12",
                    description: None,
                    array: None,
                    byte_offset: 0x358,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_12",
                    description: None,
                    array: None,
                    byte_offset: 0x35c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_12",
                    description: None,
                    array: None,
                    byte_offset: 0x360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_12",
                    description: None,
                    array: None,
                    byte_offset: 0x364,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_12",
                    description: None,
                    array: None,
                    byte_offset: 0x368,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_12",
                    description: None,
                    array: None,
                    byte_offset: 0x370,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_12",
                    description: None,
                    array: None,
                    byte_offset: 0x374,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_13",
                    description: None,
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_13",
                    description: None,
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_13",
                    description: None,
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_13",
                    description: None,
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_13",
                    description: None,
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_13",
                    description: None,
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_13",
                    description: None,
                    array: None,
                    byte_offset: 0x398,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_13",
                    description: None,
                    array: None,
                    byte_offset: 0x39c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_13",
                    description: None,
                    array: None,
                    byte_offset: 0x3a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_13",
                    description: None,
                    array: None,
                    byte_offset: 0x3a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_13",
                    description: None,
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_13",
                    description: None,
                    array: None,
                    byte_offset: 0x3b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_13",
                    description: None,
                    array: None,
                    byte_offset: 0x3b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_14",
                    description: None,
                    array: None,
                    byte_offset: 0x3f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat0_15",
                    description: None,
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstatc_15",
                    description: None,
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstatc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxstat1_15",
                    description: None,
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxstat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl0_15",
                    description: None,
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxcfg_15",
                    description: None,
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxbtcfg_15",
                    description: None,
                    array: None,
                    byte_offset: 0x414,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxbtcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxsaddr_15",
                    description: None,
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxsaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxdaddr_15",
                    description: None,
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxdaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmbaddru_15",
                    description: None,
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmbaddru",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxladdr_15",
                    description: None,
                    array: None,
                    byte_offset: 0x424,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxladdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxctl1_15",
                    description: None,
                    array: None,
                    byte_offset: 0x428,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmaddr_15",
                    description: None,
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdma_chxmdata_15",
                    description: None,
                    array: None,
                    byte_offset: 0x434,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MdmaChxmdata",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "MdmaChxbtcfg",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxbtcfg_tbnum",
                    description: None,
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
                    name: "mdma_chxbtcfg_saddrum",
                    description: None,
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
                    name: "mdma_chxbtcfg_daddrum",
                    description: None,
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
                    name: "mdma_chxbtcfg_brnum",
                    description: None,
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
            name: "MdmaChxcfg",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxcfg_simod",
                    description: None,
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
                    name: "mdma_chxcfg_dimod",
                    description: None,
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
                    name: "mdma_chxcfg_swidth",
                    description: None,
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
                    name: "mdma_chxcfg_dwidth",
                    description: None,
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
                    name: "mdma_chxcfg_sios",
                    description: None,
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
                    name: "mdma_chxcfg_dios",
                    description: None,
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
                    name: "mdma_chxcfg_sburst",
                    description: None,
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
                    name: "mdma_chxcfg_dburst",
                    description: None,
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
                    name: "mdma_chxcfg_btlen",
                    description: None,
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
                    name: "mdma_chxcfg_pken",
                    description: None,
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
                    name: "mdma_chxcfg_pamod",
                    description: None,
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
                    name: "mdma_chxcfg_trigmod",
                    description: None,
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
                    name: "mdma_chxcfg_swreqmod",
                    description: None,
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
                    name: "mdma_chxcfg_bwmod",
                    description: None,
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
            name: "MdmaChxctl0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxctl0_chen",
                    description: None,
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
                    name: "mdma_chxctl0_errie",
                    description: None,
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
                    name: "mdma_chxctl0_chtcie",
                    description: None,
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
                    name: "mdma_chxctl0_mbtcie",
                    description: None,
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
                    name: "mdma_chxctl0_btcie",
                    description: None,
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
                    name: "mdma_chxctl0_tcie",
                    description: None,
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
                    name: "mdma_chxctl0_prio",
                    description: None,
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
                    name: "mdma_chxctl0_smoden",
                    description: None,
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
                    name: "mdma_chxctl0_bes",
                    description: None,
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
                    name: "mdma_chxctl0_hwes",
                    description: None,
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
                    name: "mdma_chxctl0_wes",
                    description: None,
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
                    name: "mdma_chxctl0_swreq",
                    description: None,
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
            name: "MdmaChxctl1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxctl1_trigsel",
                    description: None,
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
                    name: "mdma_chxctl1_sbsel",
                    description: None,
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
                    name: "mdma_chxctl1_dbsel",
                    description: None,
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
            name: "MdmaChxdaddr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxdaddr_daddr",
                    description: None,
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
            name: "MdmaChxladdr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxladdr_laddr",
                    description: None,
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
            name: "MdmaChxmaddr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxmaddr_maddr",
                    description: None,
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
            name: "MdmaChxmbaddru",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxmbaddru_saddruv",
                    description: None,
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
                    name: "mdma_chxmbaddru_daddruv",
                    description: None,
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
            name: "MdmaChxmdata",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxmdata_mdata",
                    description: None,
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
            name: "MdmaChxsaddr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxsaddr_saddr",
                    description: None,
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
            name: "MdmaChxstat0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxstat0_err",
                    description: None,
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
                    name: "mdma_chxstat0_chtcf",
                    description: None,
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
                    name: "mdma_chxstat0_mbtcf",
                    description: None,
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
                    name: "mdma_chxstat0_btcf",
                    description: None,
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
                    name: "mdma_chxstat0_tcf",
                    description: None,
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
                    name: "mdma_chxstat0_reqaf",
                    description: None,
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
            name: "MdmaChxstat1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxstat1_erraddr",
                    description: None,
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
                    name: "mdma_chxstat1_terrd",
                    description: None,
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
                    name: "mdma_chxstat1_ldterr",
                    description: None,
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
                    name: "mdma_chxstat1_mdterr",
                    description: None,
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
                    name: "mdma_chxstat1_aserr",
                    description: None,
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
                    name: "mdma_chxstat1_bzerr",
                    description: None,
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
            name: "MdmaChxstatc",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_chxstatc_errc",
                    description: None,
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
                    name: "mdma_chxstatc_chtcfc",
                    description: None,
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
                    name: "mdma_chxstatc_mbtcfc",
                    description: None,
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
                    name: "mdma_chxstatc_btcfc",
                    description: None,
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
                    name: "mdma_chxstatc_tcfc",
                    description: None,
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
            name: "MdmaGintf",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdma_gintf_gif0",
                    description: None,
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
                    name: "mdma_gintf_gif1",
                    description: None,
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
                    name: "mdma_gintf_gif2",
                    description: None,
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
                    name: "mdma_gintf_gif3",
                    description: None,
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
                    name: "mdma_gintf_gif4",
                    description: None,
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
                    name: "mdma_gintf_gif5",
                    description: None,
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
                    name: "mdma_gintf_gif6",
                    description: None,
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
                    name: "mdma_gintf_gif7",
                    description: None,
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
                    name: "mdma_gintf_gif8",
                    description: None,
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
                    name: "mdma_gintf_gif9",
                    description: None,
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
                    name: "mdma_gintf_gif10",
                    description: None,
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
                    name: "mdma_gintf_gif11",
                    description: None,
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
                    name: "mdma_gintf_gif12",
                    description: None,
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
                    name: "mdma_gintf_gif13",
                    description: None,
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
                    name: "mdma_gintf_gif14",
                    description: None,
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
                    name: "mdma_gintf_gif15",
                    description: None,
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
                