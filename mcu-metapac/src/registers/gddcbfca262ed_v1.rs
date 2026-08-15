
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dcb",
            extends: None,
            description: Some(
                "Debug Control Block",
            ),
            items: &[
                BlockItem {
                    name: "dscsr",
                    description: Some(
                        "Debug Security Control and Status Register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dscsr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Dscsr",
            extends: None,
            description: Some(
                "Debug Security Control and Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cds",
                    description: Some(
                        "Current domain Secure",
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
    ],
    enums: &[],
};
                