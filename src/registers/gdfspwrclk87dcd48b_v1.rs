
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "FsPwrclk",
            extends: None,
            description: Some(
                "USB on the go full speed",
            ),
            items: &[
                BlockItem {
                    name: "pwrclkctl",
                    description: Some(
                        "power and clock gating control register (PWRCLKCTL)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwrclkctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Pwrclkctl",
            extends: None,
            description: Some(
                "power and clock gating control register (PWRCLKCTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "suclk",
                    description: Some(
                        "Stop the USB clock",
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
                    name: "shclk",
                    description: Some(
                        "Stop HCLK",
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
    ],
    enums: &[],
};
                