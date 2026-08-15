
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dac",
            extends: None,
            description: Some(
                "Digital-to-analog converter",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
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
                                "Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swt",
                    description: Some(
                        "Software trigger register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Swt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_r12dh",
                    description: Some(
                        "DAC_OUT 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutR12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_l12dh",
                    description: Some(
                        "DAC_OUT 12-bit left-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutL12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_r8dh",
                    description: Some(
                        "DAC_OUT 8-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "OutR8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out_do",
                    description: Some(
                        "DAC_OUT data output register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "OutDo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "DAC Status register 0",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat0",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den",
                    description: Some(
                        "DAC_OUT enable",
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
                    name: "dboff",
                    description: Some(
                        "DAC_OUT output buffer turn off",
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
                    name: "dten",
                    description: Some(
                        "DAC_OUT trigger enable",
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
                    name: "dtsel",
                    description: Some(
                        "DAC_OUT trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwm",
                    description: Some(
                        "DAC_OUT noise wave mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwbw",
                    description: Some(
                        "DAC_OUT noise wave bit width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddmaen",
                    description: Some(
                        "DAC_OUT DMA enable",
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
                    name: "ddudrie",
                    description: Some(
                        "DAC_OUT DMA underrun interrupt enable",
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
                    name: "ddisc",
                    description: Some(
                        "DAC_OUT connect GPIO selection",
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
            ],
        },
        FieldSet {
            name: "OutDo",
            extends: None,
            description: Some(
                "DAC_OUT data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out_do",
                    description: Some(
                        "DAC_OUT data output",
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
            name: "OutL12dh",
            extends: None,
            description: Some(
                "DAC_OUT 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out_dh",
                    description: Some(
                        "DAC_OUT 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OutR12dh",
            extends: None,
            description: Some(
                "DAC_OUT 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out_dh",
                    description: Some(
                        "DAC_OUT 12-bit right-aligned data",
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
            name: "OutR8dh",
            extends: None,
            description: Some(
                "DAC_OUT 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out_dh",
                    description: Some(
                        "DAC_OUT 8-bit right-aligned data",
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
            name: "Stat0",
            extends: None,
            description: Some(
                "DAC Status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ddudr",
                    description: Some(
                        "DAC_OUT DMA underrun flag",
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
            ],
        },
        FieldSet {
            name: "Swt",
            extends: None,
            description: Some(
                "Software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtr",
                    description: Some(
                        "DAC_OUT software trigger, cleared by hardware",
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
            ],
        },
    ],
    enums: &[],
};
                