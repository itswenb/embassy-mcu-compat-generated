
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Vref",
            extends: None,
            description: Some(
                "VREF",
            ),
            items: &[
                BlockItem {
                    name: "cs",
                    description: Some(
                        "Control and status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "calib",
                    description: Some(
                        "Calibration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calib",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Calib",
            extends: None,
            description: Some(
                "Calibration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vrefcal",
                    description: Some(
                        "VREF calibration",
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
            name: "Cs",
            extends: None,
            description: Some(
                "Control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREF enable",
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
                    name: "hipm",
                    description: Some(
                        "High impedance mode",
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
                    name: "vrefrdy",
                    description: Some(
                        "VREF ready",
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
