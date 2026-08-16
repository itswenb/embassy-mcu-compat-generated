
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rameccmu1",
            extends: None,
            description: Some(
                "RAM ECC monitor unit 1",
            ),
            items: &[
                BlockItem {
                    name: "int",
                    description: Some(
                        "RAMECCMU global interruput register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Int",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0ctl",
                    description: Some(
                        "monitor 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M0ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0stat",
                    description: Some(
                        "monitor 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M0stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0faddr",
                    description: Some(
                        "monitor 0 failing address register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M0faddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0fdl",
                    description: Some(
                        "monitor 0 failing data low register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M0fdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0fdh",
                    description: Some(
                        "monitor 0 failing data high register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M0fdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m0fecode",
                    description: Some(
                        "monitor 0 failing ECC error code register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M0fecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1ctl",
                    description: Some(
                        "monitor 1 control register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M1ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1stat",
                    description: Some(
                        "monitor 1 status register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M1stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1faddr",
                    description: Some(
                        "monitor 1 failing address register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M1faddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1fdl",
                    description: Some(
                        "monitor 1 failing data low register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M1fdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1fdh",
                    description: Some(
                        "monitor 0 failing data high register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M1fdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m1fecode",
                    description: Some(
                        "monitor 1 failing ECC error code register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M1fecode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2ctl",
                    description: Some(
                        "monitor 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2stat",
                    description: Some(
                        "monitor 0 status register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "M2stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2faddr",
                    description: Some(
                        "monitor 0 failing address register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M2faddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2fdl",
                    description: Some(
                        "monitor 0 failing data low register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M2fdl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2fdh",
                    description: Some(
                        "monitor 0 failing data high register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M2fdh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "m2fecode",
                    description: Some(
                        "monitor 0 failing ECC error code register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "M2fecode",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Int",
            extends: None,
            description: Some(
                "RAMECCMU global interruput register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "geie",
                    description: Some(
                        "Global ECC interrupt enable",
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
                    name: "geserrie",
                    description: Some(
                        "Global ECC single error interrupt enable",
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
                    name: "gederrie",
                    description: Some(
                        "Global ECC double error interrupt enable",
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
                    name: "gederrbwie",
                    description: Some(
                        "Global ECC double error on byte write interrupt enable",
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
        FieldSet {
            name: "M0ctl",
            extends: None,
            description: Some(
                "monitor 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrie",
                    description: Some(
                        "ECC single error interrupt enable",
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
                    name: "eccderrie",
                    description: Some(
                        "ECC double error interrupt enable",
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
                    name: "eccderrbwie",
                    description: Some(
                        "ECC double error on byte write interrupt enable",
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
                    name: "eccerrlaten",
                    description: Some(
                        "ECC error latching enable",
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
            ],
        },
        FieldSet {
            name: "M0faddr",
            extends: None,
            description: Some(
                "monitor 0 failing address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfaddr",
                    description: Some(
                        "ECC error failing address",
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
            name: "M0fdh",
            extends: None,
            description: Some(
                "monitor 0 failing data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdh",
                    description: Some(
                        "ECC failing data high bitss",
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
            name: "M0fdl",
            extends: None,
            description: Some(
                "monitor 0 failing data low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdl",
                    description: Some(
                        "ECC failing data low bits",
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
            name: "M0fecode",
            extends: None,
            description: Some(
                "monitor 0 failing ECC error code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfecode",
                    description: Some(
                        "ECC failing error code",
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
            name: "M0stat",
            extends: None,
            description: Some(
                "monitor 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrdcf",
                    description: Some(
                        "ECC single error detected and corrected flag",
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
                    name: "eccderrdf",
                    description: Some(
                        "ECC double error detected flag",
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
                    name: "eccderrbwdf",
                    description: Some(
                        "ECC double error on byte write detected flag",
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
            ],
        },
        FieldSet {
            name: "M1ctl",
            extends: None,
            description: Some(
                "monitor 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrie",
                    description: Some(
                        "ECC single error interrupt enable",
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
                    name: "eccderrie",
                    description: Some(
                        "ECC double error interrupt enable",
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
                    name: "eccderrbwie",
                    description: Some(
                        "ECC double error on byte write interrupt enable",
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
                    name: "eccerrlaten",
                    description: Some(
                        "ECC error latching enable",
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
            ],
        },
        FieldSet {
            name: "M1faddr",
            extends: None,
            description: Some(
                "monitor 1 failing address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfaddr",
                    description: Some(
                        "ECC error failing address",
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
            name: "M1fdh",
            extends: None,
            description: Some(
                "monitor 0 failing data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdh",
                    description: Some(
                        "ECC failing data high bitss",
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
            name: "M1fdl",
            extends: None,
            description: Some(
                "monitor 1 failing data low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdl",
                    description: Some(
                        "ECC failing data low bits",
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
            name: "M1fecode",
            extends: None,
            description: Some(
                "monitor 1 failing ECC error code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfecode",
                    description: Some(
                        "ECC failing error code",
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
            name: "M1stat",
            extends: None,
            description: Some(
                "monitor 1 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrdcf",
                    description: Some(
                        "ECC single error detected and corrected flag",
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
                    name: "eccderrdf",
                    description: Some(
                        "ECC double error detected flag",
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
                    name: "eccderrbwdf",
                    description: Some(
                        "ECC double error on byte write detected flag",
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
            ],
        },
        FieldSet {
            name: "M2ctl",
            extends: None,
            description: Some(
                "monitor 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrie",
                    description: Some(
                        "ECC single error interrupt enable",
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
                    name: "eccderrie",
                    description: Some(
                        "ECC double error interrupt enable",
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
                    name: "eccderrbwie",
                    description: Some(
                        "ECC double error on byte write interrupt enable",
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
                    name: "eccerrlaten",
                    description: Some(
                        "ECC error latching enable",
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
            ],
        },
        FieldSet {
            name: "M2faddr",
            extends: None,
            description: Some(
                "monitor 0 failing address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfaddr",
                    description: Some(
                        "ECC error failing address",
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
            name: "M2fdh",
            extends: None,
            description: Some(
                "monitor 0 failing data high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdh",
                    description: Some(
                        "ECC failing data high bitss",
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
            name: "M2fdl",
            extends: None,
            description: Some(
                "monitor 0 failing data low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfdl",
                    description: Some(
                        "ECC failing data low bits",
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
            name: "M2fecode",
            extends: None,
            description: Some(
                "monitor 0 failing ECC error code register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccfecode",
                    description: Some(
                        "ECC failing error code",
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
            name: "M2stat",
            extends: None,
            description: Some(
                "monitor 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccserrdcf",
                    description: Some(
                        "ECC single error detected and corrected flag",
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
                    name: "eccderrdf",
                    description: Some(
                        "ECC double error detected flag",
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
                    name: "eccderrbwdf",
                    description: Some(
                        "ECC double error on byte write detected flag",
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
            ],
        },
    ],
    enums: &[],
};
                