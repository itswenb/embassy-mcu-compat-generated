
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EnetPtp",
            extends: None,
            description: Some(
                "Ethernet: Precision time protocol",
            ),
            items: &[
                BlockItem {
                    name: "ptp_tsctl",
                    description: Some(
                        "Ethernet PTP time stamp control register (PTP_TSCTL)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_ssinc",
                    description: Some(
                        "Ethernet PTP subsecond increment register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpSsinc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsh",
                    description: Some(
                        "Ethernet PTP time stamp high register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsl",
                    description: Some(
                        "Ethernet PTP time stamp low register (PTP_TSL)",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsuh",
                    description: Some(
                        "Ethernet PTP time stamp high update register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsuh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsul",
                    description: Some(
                        "Ethernet PTP time stamp low update register (PTP_TSUL)",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsul",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsaddend",
                    description: Some(
                        "Ethernet PTP time stamp addend register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsaddend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_eth",
                    description: Some(
                        "Ethernet PTP expected time high register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_etl",
                    description: Some(
                        "Ethernet PTP expected time low register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpEtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_tsf",
                    description: Some(
                        "Ethernet PTP time stamp flag register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpTsf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ptp_ppsctl",
                    description: Some(
                        "Ethernet PTP PPS control register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PtpPpsctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "PtpEth",
            extends: None,
            description: Some(
                "Ethernet PTP expected time high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etsh",
                    description: Some(
                        "Expected time stamp high",
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
            name: "PtpEtl",
            extends: None,
            description: Some(
                "Ethernet PTP expected time low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "etsl",
                    description: Some(
                        "Expected time stamp low",
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
            name: "PtpPpsctl",
            extends: None,
            description: Some(
                "Ethernet PTP PPS control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppsofc",
                    description: Some(
                        "PPS output frequency configure",
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
            name: "PtpSsinc",
            extends: None,
            description: Some(
                "Ethernet PTP subsecond increment register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stmssi",
                    description: Some(
                        "System time subsecond increment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTsaddend",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp addend register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmsa",
                    description: Some(
                        "Time stamp addend",
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
            name: "PtpTsctl",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp control register (PTP_TSCTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmsen",
                    description: Some(
                        "Time stamp enable",
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
                    name: "tmsfcu",
                    description: Some(
                        "Time stamp fine or coarse update",
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
                    name: "tmssti",
                    description: Some(
                        "Time stamp system time initialize",
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
                    name: "tmsstu",
                    description: Some(
                        "Time stamp system time update",
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
                    name: "tmsiten",
                    description: Some(
                        "Time stamp interrupt trigger enable",
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
                    name: "tmsaru",
                    description: Some(
                        "Time stamp addend register update",
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
                    name: "arfsen",
                    description: Some(
                        "All received frames snapshot enable",
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
                    name: "scrom",
                    description: Some(
                        "Subsecond counter rollover mode",
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
                    name: "pfsv",
                    description: Some(
                        "PTP frame snooping version",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "esen",
                    description: Some(
                        "Received Ethernet snapshot enable",
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
                Field {
                    name: "ip6sen",
                    description: Some(
                        "Received IPv6 snapshot enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ip4sen",
                    description: Some(
                        "Received IPv4 snapshot enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etmsen",
                    description: Some(
                        "Received event type message snapshot enable",
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
                    name: "mnmsen",
                    description: Some(
                        "Received master node message snapshot enable",
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
                    name: "cknt",
                    description: Some(
                        "Clock node type for time stamp",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mafen",
                    description: Some(
                        "MAC address filter enable for PTP frame",
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
            name: "PtpTsf",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tssco",
                    description: Some(
                        "Timestamp second counter overflow",
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
                    name: "ttm",
                    description: Some(
                        "Target time match",
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
        FieldSet {
            name: "PtpTsh",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stms",
                    description: Some(
                        "System time second",
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
            name: "PtpTsl",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low register (PTP_TSL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stmss",
                    description: Some(
                        "System time subseconds",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sts",
                    description: Some(
                        "System time sign",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PtpTsuh",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp high update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmsus",
                    description: Some(
                        "Time stamp update second",
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
            name: "PtpTsul",
            extends: None,
            description: Some(
                "Ethernet PTP time stamp low update register (PTP_TSUL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmsuss",
                    description: Some(
                        "Time stamp update subseconds",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 31,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmsupns",
                    description: Some(
                        "Time stamp update positive or negative sign",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
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
                