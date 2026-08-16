
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Enet1MacFcth",
            extends: None,
            description: Some(
                "MAC flow control threshold register",
            ),
            items: &[
                BlockItem {
                    name: "mac_fcth",
                    description: Some(
                        "Ethernet MAC flow control threshold register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacFcth",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "MacFcth",
            extends: None,
            description: Some(
                "Ethernet MAC flow control threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfa",
                    description: Some(
                        "Threshold of active flow control",
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
                Field {
                    name: "rfd",
                    description: Some(
                        "Threshold of deactive flow control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                