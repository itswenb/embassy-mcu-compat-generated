
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
                        "DACx control register 0",
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
                        "DACx software trigger register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "DACx_OUT0 12-bit right-aligned data holding register",
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
                    name: "out1_r12dh",
                    description: Some(
                        "DACx_OUT1 12-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out1R12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out1_l12dh",
                    description: Some(
                        "DACx_OUT1 12-bit left-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out1L12dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out1_r8dh",
                    description: Some(
                        "DACx_OUT1 8-bit right-aligned data holding register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out1R8dh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dacc_r12dh",
                    description: Some(
                        "DACx concurrent mode 12-bit right-aligned data holding register",
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
                        "DACx concurrent mode 12-bit left-aligned data holding register",
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
                        "DACx concurrent mode 8-bit right-aligned data holding register",
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
                    name: "out0_do",
                    description: Some(
                        "DACx_OUT0 data output register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out0Do",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "out1_do",
                    description: Some(
                        "DACx_OUT1 data output register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Out1Do",
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
                "DACx control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den0",
                    description: Some(
                        "DACx_OUT0 enable",
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
                        "DACx_OUT0 output buffer turn off",
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
                        "DACx_OUT0 trigger enable",
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
                        "DACx_OUT0 trigger selection",
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
                        "DACx_OUT0 noise wave mode",
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
                        "DACx_OUT0 noise wave bit width",
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
                        "DACx_OUT0 DMA enable",
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
                    name: "den1",
                    description: Some(
                        "DACx_OUT1 enable",
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
                        "DACx_OUT1 output buffer turn off",
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
                        "DACx_OUT1 trigger enable",
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
                        "DACx_OUT1 trigger selection",
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
                        "DACx_OUT1 noise wave mode",
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
                        "DACx_OUT1 noise wave bit width",
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
                        "DACx_OUT1 DMA enable",
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
            ],
        },
        FieldSet {
            name: "DaccL12dh",
            extends: None,
            description: Some(
                "DACx concurrent mode 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DACx_OUT0 12-bit left-aligned data",
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
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 12-bit left-aligned data",
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
                "DACx concurrent mode 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DACx_OUT0 12-bit right-aligned data",
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
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 12-bit right-aligned data",
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
                "DACx concurrent mode 8-bit right-aligned data holding register",
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
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 8-bit right-aligned data",
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
                        "DACx_OUT0 12-bit left-aligned data",
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
                "DACx_OUT0 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DACx_OUT0 12-bit right-aligned data",
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
            name: "Out1Do",
            extends: None,
            description: Some(
                "DACx_OUT1 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_do",
                    description: Some(
                        "DACx_OUT1 12-bit output data",
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
            name: "Out1L12dh",
            extends: None,
            description: Some(
                "DACx_OUT1 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 12-bit left-aligned data",
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
            name: "Out1R12dh",
            extends: None,
            description: Some(
                "DACx_OUT1 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 12-bit right-aligned data",
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
            name: "Out1R8dh",
            extends: None,
            description: Some(
                "DACx_OUT1 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DACx_OUT1 8-bit right-aligned data",
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
            name: "Swt",
            extends: None,
            description: Some(
                "DACx software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtr0",
                    description: Some(
                        "DACx_OUT0 software trigger",
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
                        "DACx_OUT1 software trigger",
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
                