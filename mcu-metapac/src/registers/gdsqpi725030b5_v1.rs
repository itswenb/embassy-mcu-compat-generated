
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Sqpi",
            extends: None,
            description: Some(
                "Serial/Quad Parallel Interface",
            ),
            items: &[
                BlockItem {
                    name: "init",
                    description: Some(
                        "SQPI Initial Register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Init",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcmd",
                    description: Some(
                        "SQPI Read Command Register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wcmd",
                    description: Some(
                        "Write Command Register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wcmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idl",
                    description: Some(
                        "ID Low Register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "idh",
                    description: Some(
                        "ID High Register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Idh",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Idh",
            extends: None,
            description: Some(
                "ID High Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idh",
                    description: Some(
                        "ID High Data saved for SQPI read ID command",
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
            name: "Idl",
            extends: None,
            description: Some(
                "ID Low Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idl",
                    description: Some(
                        "ID Low Data saved for SQPI Read ID Command",
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
            name: "Init",
            extends: None,
            description: Some(
                "SQPI Initial Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdbit",
                    description: Some(
                        "Bit number of SQPI controller command phase",
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
                    name: "clkdiv",
                    description: Some(
                        "Clock divider for SQPI output clock",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addrbit",
                    description: Some(
                        "Bit number of SPI PSRAM address phase.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idlen",
                    description: Some(
                        "SQPI controller external memory ID length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pl",
                    description: Some(
                        "Read data sample polarity.",
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
            name: "Rcmd",
            extends: None,
            description: Some(
                "SQPI Read Command Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcmd",
                    description: Some(
                        "SQPI read command for AHB read transfer",
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
                Field {
                    name: "rwaitcycle",
                    description: Some(
                        "SQPI read command waitcycle number after address phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rmode",
                    description: Some(
                        "SQPI controller read command mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rid",
                    description: Some(
                        "Send read ID command",
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
            name: "Wcmd",
            extends: None,
            description: Some(
                "Write Command Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wcmd",
                    description: Some(
                        "SQPI write command for AHB write transfer",
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
                Field {
                    name: "wwaitcycle",
                    description: Some(
                        "SQPI write command waitcycle number after address phase",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wmode",
                    description: Some(
                        "SQPI controller write command mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scmd",
                    description: Some(
                        "Send special command",
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
                