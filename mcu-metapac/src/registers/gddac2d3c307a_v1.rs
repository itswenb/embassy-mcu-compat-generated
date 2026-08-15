
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
                        "control register",
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
                    name: "dac0_r12dh",
                    description: Some(
                        "DAC0 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0R12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0_l12dh",
                    description: Some(
                        "DAC0 12-bit left-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0L12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0_r8dh",
                    description: Some(
                        "DAC0 8-bit right aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0R8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac1_r12dh",
                    description: Some(
                        "DAC1 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac1R12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac1_l12dh",
                    description: Some(
                        "DAC1 12-bit left aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac1L12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac1_r8dh",
                    description: Some(
                        "DAC1 8-bit right aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac1R8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacc_r12dh",
                    description: Some(
                        "DAC concurrent mode 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DaccR12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacc_l12dh",
                    description: Some(
                        "DAC concurrent mode 12-bit left aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DaccL12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacc_r8dh",
                    description: Some(
                        "DAC concurrent mode 8-bit right aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DaccR8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0_do",
                    description: Some(
                        "DAC0 data output register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0Do",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac1_do",
                    description: Some(
                        "DAC1 data output register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac1Do",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "status register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
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
                "control register",
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
                    name: "dwm0",
                    description: Some(
                        "DAC0 noise wave mode",
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
                    name: "dwbw0",
                    description: Some(
                        "DAC0 noise wave bit width",
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
                Field {
                    name: "den1",
                    description: Some(
                        "DAC1 enable",
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
                Field {
                    name: "dboff1",
                    description: Some(
                        "DAC1 output buffer turn off",
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
                Field {
                    name: "dten1",
                    description: Some(
                        "DAC1 trigger enable",
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
                Field {
                    name: "dtsel1",
                    description: Some(
                        "DAC1 trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwm1",
                    description: Some(
                        "DAC1 noise wave mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwbw1",
                    description: Some(
                        "DAC1 noise wave bit width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddmaen1",
                    description: Some(
                        "DAC1 DMA enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ddudrie1",
                    description: Some(
                        "DAC1 DMA Underrun Interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dac0Do",
            extends: None,
            description: Some(
                "DAC0 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_do",
                    description: Some(
                        "DAC0 data output",
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
            name: "Dac0L12dh",
            extends: None,
            description: Some(
                "DAC0 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 12-bit left-aligned data",
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
            name: "Dac0R12dh",
            extends: None,
            description: Some(
                "DAC0 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 12-bit right-aligned data",
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
            name: "Dac0R8dh",
            extends: None,
            description: Some(
                "DAC0 8-bit right aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 8-bit right-aligned data",
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
            name: "Dac1Do",
            extends: None,
            description: Some(
                "DAC1 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac1_do",
                    description: Some(
                        "DAC1 data output",
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
            name: "Dac1L12dh",
            extends: None,
            description: Some(
                "DAC1 12-bit left aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 12-bit left-aligned data",
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
            name: "Dac1R12dh",
            extends: None,
            description: Some(
                "DAC1 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 12-bit right-aligned data",
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
            name: "Dac1R8dh",
            extends: None,
            description: Some(
                "DAC1 8-bit right aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 8-bit right-aligned data",
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
            name: "DaccL12dh",
            extends: None,
            description: Some(
                "DAC concurrent mode 12-bit left aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 12-bit left-aligned data",
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
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 12-bit left-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DaccR12dh",
            extends: None,
            description: Some(
                "DAC concurrent mode 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 12-bit right-aligned data",
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
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 12-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DaccR8dh",
            extends: None,
            description: Some(
                "DAC concurrent mode 8-bit right aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dac0_dh",
                    description: Some(
                        "DAC0 8-bit right-aligned data",
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
                Field {
                    name: "dac1_dh",
                    description: Some(
                        "DAC1 8-bit right-aligned data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Stat",
            extends: None,
            description: Some(
                "status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ddudr0",
                    description: Some(
                        "DAC0 DMA underrun flag",
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
                    name: "ddudr1",
                    description: Some(
                        "DAC1 DMA underrun flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
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
                Field {
                    name: "swtr1",
                    description: Some(
                        "DAC1 software trigger",
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
                