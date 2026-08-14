
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fwdgt",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "fwdgt_ctl",
                    description: None,
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
                    description: None,
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
                    description: None,
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
                    description: None,
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
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "fwdgt_ctl_cmd",
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
            ],
        },
        FieldSet {
            name: "FwdgtPsc",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "fwdgt_psc_psc",
                    description: None,
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
            name: "FwdgtRld",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "fwdgt_rld_rld",
                    description: None,
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
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "fwdgt_stat_pud",
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
                    name: "fwdgt_stat_rud",
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
            ],
        },
    ],
    enums: &[],
};
                