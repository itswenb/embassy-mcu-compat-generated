
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Edout",
            extends: None,
            description: Some(
                "En-coder Divided-Output controller",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "Control register",
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
                    name: "enable",
                    description: Some(
                        "Enable register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Enable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "loc",
                    description: Some(
                        "Location register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Loc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ocnt",
                    description: Some(
                        "Output counter register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ocnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcnt",
                    description: Some(
                        "Location counter register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "zcr",
                    description: Some(
                        "Z-PHASE configure register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Zcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pol",
                    description: Some(
                        "The active polarity of the B-phase output signal selection",
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
            ],
        },
        FieldSet {
            name: "Enable",
            extends: None,
            description: Some(
                "Enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edouten",
                    description: Some(
                        "EDOUT enable bit",
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
            ],
        },
        FieldSet {
            name: "Lcnt",
            extends: None,
            description: Some(
                "Location counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "loccnt",
                    description: Some(
                        "These bits are used to set the current position value",
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
            name: "Loc",
            extends: None,
            description: Some(
                "Location register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "locmax",
                    description: Some(
                        "This bits set the maximum location value for one rotation",
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
            name: "Ocnt",
            extends: None,
            description: Some(
                "Output counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "edgc",
                    description: Some(
                        "These bits set the number of edges of the A-phase signal and the B-phase signal for the next carrier cycle",
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
                    name: "pdc",
                    description: Some(
                        "These bits set the phase difference between the A-phase signal and the B-phase signal for the next carrier cycle",
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
            name: "Zcr",
            extends: None,
            description: Some(
                "Z-PHASE configure register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "zosp",
                    description: Some(
                        "Z-PHASE output start position",
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
                    name: "zowh",
                    description: Some(
                        "Z-PHASE output width",
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
                    name: "zomd",
                    description: Some(
                        "Z-PHASE output mode",
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
    ],
    enums: &[],
};
