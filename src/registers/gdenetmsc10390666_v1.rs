
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EnetMsc",
            extends: None,
            description: Some(
                "Ethernet: MAC statistics counters",
            ),
            items: &[
                BlockItem {
                    name: "msc_ctl",
                    description: Some(
                        "Ethernet MSC control register (MSC_CTL)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MscCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_rintf",
                    description: Some(
                        "Ethernet MSC receive interrupt flag register (MSC_RINTF)",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscRintf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_tintf",
                    description: Some(
                        "Ethernet MSC transmit interrupt flag register (MSC_TINTF)",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscTintf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_rintmsk",
                    description: Some(
                        "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MscRintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_tintmsk",
                    description: Some(
                        "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MscTintmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_sccnt",
                    description: Some(
                        "Ethernet MSC transmitted good frames after a single collision counter",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscSccnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_msccnt",
                    description: Some(
                        "Ethernet MSC transmitted good frames after more than a single collision",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscMsccnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_tgfcnt",
                    description: Some(
                        "Ethernet MSC transmitted good frames counter register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscTgfcnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_rfcecnt",
                    description: Some(
                        "Ethernet MSC received frames with CRC error counter register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscRfcecnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_rfaecnt",
                    description: Some(
                        "Ethernet MSC received frames with alignment error counter register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscRfaecnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "msc_rgufcnt",
                    description: Some(
                        "MSC received good unicast frames counter register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MscRgufcnt",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "MscCtl",
            extends: None,
            description: Some(
                "Ethernet MSC control register (MSC_CTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctr",
                    description: Some(
                        "Counter reset",
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
                    name: "ctsr",
                    description: Some(
                        "Counter stop rollover",
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
                    name: "rtor",
                    description: Some(
                        "Reset on read",
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
                    name: "mcfz",
                    description: Some(
                        "MSC counter freeze",
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
                    name: "pmc",
                    description: Some(
                        "Preset MSC counter",
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
                    name: "afhpm",
                    description: Some(
                        "Almost full or half preset mode",
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
            name: "MscMsccnt",
            extends: None,
            description: Some(
                "Ethernet MSC transmitted good frames after more than a single collision",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mscc",
                    description: Some(
                        "Transmitted good frames after more than a single collision counter",
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
            name: "MscRfaecnt",
            extends: None,
            description: Some(
                "Ethernet MSC received frames with alignment error counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfaer",
                    description: Some(
                        "Received frames with alignment error counter",
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
            name: "MscRfcecnt",
            extends: None,
            description: Some(
                "Ethernet MSC received frames with CRC error counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfcer",
                    description: Some(
                        "Received frames with CRC error counter",
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
            name: "MscRgufcnt",
            extends: None,
            description: Some(
                "MSC received good unicast frames counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rguf",
                    description: Some(
                        "Received good unicast frames counter",
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
            name: "MscRintf",
            extends: None,
            description: Some(
                "Ethernet MSC receive interrupt flag register (MSC_RINTF)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfce",
                    description: Some(
                        "Received frames CRC error",
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
                Field {
                    name: "rfae",
                    description: Some(
                        "Received frames alignment error",
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
                    name: "rguf",
                    description: Some(
                        "Received Good Unicast Frames",
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
            ],
        },
        FieldSet {
            name: "MscRintmsk",
            extends: None,
            description: Some(
                "Ethernet MSC receive interrupt mask register (MSC_RINTMSK)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfceim",
                    description: Some(
                        "Received frame CRC error interrupt mask",
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
                Field {
                    name: "rfaeim",
                    description: Some(
                        "Received frames alignment error interrupt mask",
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
                    name: "rgufim",
                    description: Some(
                        "Received good unicast frames interrupt mask",
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
            ],
        },
        FieldSet {
            name: "MscSccnt",
            extends: None,
            description: Some(
                "Ethernet MSC transmitted good frames after a single collision counter",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scc",
                    description: Some(
                        "Transmitted good frames after a single collision counter",
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
            name: "MscTgfcnt",
            extends: None,
            description: Some(
                "Ethernet MSC transmitted good frames counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgf",
                    description: Some(
                        "Transmitted good frames counter",
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
            name: "MscTintf",
            extends: None,
            description: Some(
                "Ethernet MSC transmit interrupt flag register (MSC_TINTF)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfsc",
                    description: Some(
                        "Transmitted good frames single collision",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgfmsc",
                    description: Some(
                        "Transmitted good frames more single collision",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgf",
                    description: Some(
                        "Transmitted good frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MscTintmsk",
            extends: None,
            description: Some(
                "Ethernet MSC transmit interrupt mask register (MSC_TINTMSK)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tgfscim",
                    description: Some(
                        "Transmitted good frames single collision interrupt mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgfmscim",
                    description: Some(
                        "Transmitted good frames more single interrupt collision mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tgfim",
                    description: Some(
                        "Transmitted good frames interrupt mask",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
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
