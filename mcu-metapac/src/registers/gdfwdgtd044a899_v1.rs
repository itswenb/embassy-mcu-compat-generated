
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fwdgt",
            extends: None,
            description: Some(
                "Free watchdog timer",
            ),
            items: &[
                BlockItem {
                    name: "fwdgt_ctl",
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
                                "FwdgtCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fwdgt_psc",
                    description: Some(
                        "Prescaler register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FwdgtPsc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fwdgt_rld",
                    description: Some(
                        "Reload register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FwdgtRld",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fwdgt_stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FwdgtStat",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "FwdgtCtl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmd",
                    description: Some(
                        "Key value",
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
            name: "FwdgtPsc",
            extends: None,
            description: Some(
                "Prescaler register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "psc",
                    description: Some(
                        "Free watchdog timer prescaler selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FwdgtRld",
            extends: None,
            description: Some(
                "Reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rld",
                    description: Some(
                        "Free watchdog timer counter reload value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FwdgtStat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pud",
                    description: Some(
                        "Free watchdog timer prescaler value update",
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
                    name: "rud",
                    description: Some(
                        "Free watchdog timer counter reload value update",
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
                