
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
                    name: "ctl0",
                    description: Some(
                        "control register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "swt",
                    description: Some(
                        "software trigger register",
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
                    name: "out0_r12dh",
                    description: Some(
                        "DAC_OUT0 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out0R12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out0_l12dh",
                    description: Some(
                        "DACx_OUT0 12-bit left-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out0L12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out0_r8dh",
                    description: Some(
                        "DACx_OUT0 8-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out0R8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out0_do",
                    description: Some(
                        "DACx_OUT0 data output register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Out0Do",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "status register 0",
                    ),
                    array: None,
                    byte_offset: 0x34,
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
            name: "Ctl0",
            extends: None,
            description: Some(
                "control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den0",
                    description: Some(
                        "DAC0 enable",
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
                    name: "dboff0",
                    description: Some(
                        "DAC0 output buffer turn off",
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
                    name: "dten0",
                    description: Some(
                        "DAC0 trigger enable",
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
                    name: "dtsel0",
                    description: Some(
                        "DAC0 trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddmaen0",
                    description: Some(
                        "DAC0 DMA enable",
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
                    name: "ddudrie0",
                    description: Some(
                        "DAC0 DMA Underrun Interrupt enable",
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
            name: "Out0Do",
            extends: None,
            description: Some(
                "DACx_OUT0 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_do",
                    description: Some(
                        "DACx_OUT0 12-bit output data",
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
            name: "Out0L12dh",
            extends: None,
            description: Some(
                "DACx_OUT0 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DACx_OUT0 12-bit left-aligned data.",
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
            name: "Out0R12dh",
            extends: None,
            description: Some(
                "DAC_OUT0 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DAC_OUT0 12-bit right-aligned data",
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
            name: "Out0R8dh",
            extends: None,
            description: Some(
                "DACx_OUT0 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DACx_OUT0 8-bit right-aligned data",
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
                "status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ddudr0",
                    description: Some(
                        "DACx_OUT0 DMA underrun flag",
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
                "software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtr0",
                    description: Some(
                        "DAC0 software trigger",
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
                