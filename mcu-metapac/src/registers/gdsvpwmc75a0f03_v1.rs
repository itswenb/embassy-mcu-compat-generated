
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Svpwm",
            extends: None,
            description: Some(
                "Space Vector Pulse Width Modulation",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "Control register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "ualpha",
                    description: Some(
                        "Ualpha register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ualpha",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ubeta",
                    description: Some(
                        "Ubeta register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ubeta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "car",
                    description: Some(
                        "Counter auto reload register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Car",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ta",
                    description: Some(
                        "Ta register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ta",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tb",
                    description: Some(
                        "Tb register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tc",
                    description: Some(
                        "Tc register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sec",
                    description: Some(
                        "Sector register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Car",
            extends: None,
            description: Some(
                "Counter auto reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "car",
                    description: Some(
                        "This bit-field indicates the period of TIMER",
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
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sm",
                    description: Some(
                        "Switch mode",
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
                    name: "type_",
                    description: Some(
                        "Five-segment SVPWM and Seven-segment SVPWM select flag bit",
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
                    name: "cen",
                    description: Some(
                        "SVPWM peripheral enable",
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
            name: "Sec",
            extends: None,
            description: Some(
                "Sector register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sec",
                    description: Some(
                        "Sector value to the SVPWM module",
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
                    name: "oal",
                    description: Some(
                        "The duty cycle of A-phase is less than 0 overflow flag",
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
                    name: "obl",
                    description: Some(
                        "The duty cycle of B-phase is less than 0 overflow flag",
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
                    name: "ocl",
                    description: Some(
                        "The duty cycle of C-phase is less than 0 overflow flag",
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
                    name: "osf",
                    description: Some(
                        "SVPWM operation status flag bit",
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
                    name: "oah",
                    description: Some(
                        "The duty cycle of A-phase is greater than the COUNT overflow flag",
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
                    name: "obh",
                    description: Some(
                        "The duty cycle of B-phase is greater than the COUNT overflow flag",
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
                    name: "och",
                    description: Some(
                        "The duty cycle of C-phase is greater than the COUNT overflow flag",
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
            name: "Ta",
            extends: None,
            description: Some(
                "Ta register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ta",
                    description: Some(
                        "This data represents the TA output to the SVPWM module",
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
            name: "Tb",
            extends: None,
            description: Some(
                "Tb register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tb",
                    description: Some(
                        "This data represents the TB output to the SVPWM module",
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
            name: "Tc",
            extends: None,
            description: Some(
                "Tc register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tc",
                    description: Some(
                        "This data represents the TC output to the SVPWM module",
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
            name: "Ualpha",
            extends: None,
            description: Some(
                "Ualpha register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ualpha",
                    description: Some(
                        "This data represents the UALPHA input to the SVPWM module",
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
            name: "Ubeta",
            extends: None,
            description: Some(
                "Ubeta register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ubeta",
                    description: Some(
                        "This data represents the UBETA input to the SVPWM module",
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
    ],
    enums: &[],
};
                