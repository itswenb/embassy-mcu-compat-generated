
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rameccmu0",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "rameccmu_mxctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxstat_0",
                    description: None,
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfaddr_0",
                    description: None,
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdh_0",
                    description: None,
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfecode_0",
                    description: None,
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxstat_1",
                    description: None,
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfaddr_1",
                    description: None,
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdh_1",
                    description: None,
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfecode_1",
                    description: None,
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxstat_2",
                    description: None,
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfaddr_2",
                    description: None,
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdh_2",
                    description: None,
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfecode_2",
                    description: None,
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxstat_3",
                    description: None,
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfaddr_3",
                    description: None,
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdh_3",
                    description: None,
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfecode_3",
                    description: None,
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxstat_4",
                    description: None,
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfaddr_4",
                    description: None,
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdl_4",
                    description: None,
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfdh_4",
                    description: None,
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rameccmu_mxfecode_4",
                    description: None,
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RameccmuMxfecode",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "RameccmuMxctl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxctl_eccserrie",
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
                    name: "rameccmu_mxctl_eccderrie",
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
                    name: "rameccmu_mxctl_eccderrbwie",
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
                    name: "rameccmu_mxctl_eccerrlaten",
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
            ],
        },
        FieldSet {
            name: "RameccmuMxfaddr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxfaddr_eccfaddr",
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
            name: "RameccmuMxfdh",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxfdh_eccfdh",
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
            name: "RameccmuMxfdl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxfdl_eccfdl",
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
            name: "RameccmuMxfecode",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxfecode_eccfecode",
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
            name: "RameccmuMxstat",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "rameccmu_mxstat_eccserrdcf",
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
                    name: "rameccmu_mxstat_eccderrdf",
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
                    name: "rameccmu_mxstat_eccderrbwdf",
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
            ],
        },
    ],
    enums: &[],
};
                