
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
                        "Control register 0",
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
                        "Software trigger register",
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
                        "DAC_OUT0 12-bit left-aligned data holding register",
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
                        "DAC_OUT0 8-bit right-aligned data holding register",
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
                        "DAC_OUT1 12-bit right-aligned data holding register",
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
                        "DAC_OUT1 12-bit left-aligned data holding register",
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
                        "DAC_OUT1 8-bit right-aligned data holding register",
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
                        "DAC concurrent mode 12-bit left-aligned data holding register",
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
                        "DAC concurrent mode 8-bit right-aligned data holding register",
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
                        "DAC_OUT0 data output register",
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
                        "DAC_OUT1 data output register",
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
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "DAC Status register 0",
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
                BlockItem {
                    name: "calr",
                    description: Some(
                        "DAC calibration Register 1",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Calr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mdcr",
                    description: Some(
                        "DAC mode control Register 1",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "skstr0",
                    description: Some(
                        "DAC sample and keep sample time Register 0",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Skstr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "skstr1",
                    description: Some(
                        "DAC sample and keep sample time Register 1",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Skstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "skktr",
                    description: Some(
                        "DAC sample and keep keep time Register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Skktr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "skrtr",
                    description: Some(
                        "DAC sample and keep refresh time Register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Skrtr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Calr",
            extends: None,
            description: Some(
                "DAC calibration Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "otv0",
                    description: Some(
                        "DAC_OUT0 offset calibration value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otv1",
                    description: Some(
                        "DAC_OUT1 offset calibration value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den0",
                    description: Some(
                        "DAC_OUT0 enable",
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
                    name: "dten0",
                    description: Some(
                        "DAC_OUT0 trigger enable",
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
                    name: "dtsel0",
                    description: Some(
                        "DAC_OUT0 trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwm0",
                    description: Some(
                        "DAC_OUT0 noise wave mode",
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
                        "DAC_OUT0 noise wave bit width",
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
                        "DAC_OUT0 DMA enable",
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
                        "DAC_OUT0 DMA underrun interrupt enable",
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
                    name: "calen0",
                    description: Some(
                        "DAC_OUT0 calibration enable",
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
                    name: "den1",
                    description: Some(
                        "DAC_OUT1 enable",
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
                    name: "dten1",
                    description: Some(
                        "DAC_OUT1 trigger enable",
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
                    name: "dtsel1",
                    description: Some(
                        "DAC_OUT1 trigger selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dwm1",
                    description: Some(
                        "DAC_OUT1 noise wave mode",
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
                        "DAC_OUT1 noise wave bit width",
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
                        "DAC_OUT1 DMA enable",
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
                        "DAC_OUT1 DMA underrun interrupt enable",
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
                Field {
                    name: "calen1",
                    description: Some(
                        "DAC_OUT1 calibration enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
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
                "DAC concurrent mode 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DAC_OUT0 12-bit left-aligned data",
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
                        "DAC_OUT1 12-bit left-aligned data",
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
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DAC_OUT1 12-bit right-aligned data",
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
                "DAC concurrent mode 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DAC_OUT0 8-bit right-aligned data",
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
                        "DAC_OUT1 8-bit right-aligned data",
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
            name: "Mdcr",
            extends: None,
            description: Some(
                "DAC mode control Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mode0",
                    description: Some(
                        "DAC OUT0 mode.",
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
                    name: "mode1",
                    description: Some(
                        "DAC OUT1 mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Out0Do",
            extends: None,
            description: Some(
                "DAC_OUT0 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_do",
                    description: Some(
                        "DAC_OUT0 data output",
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
                "DAC_OUT0 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DAC_OUT0 12-bit left-aligned data",
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
                "DAC_OUT0 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out0_dh",
                    description: Some(
                        "DAC_OUT0 8-bit right-aligned data",
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
                "DAC_OUT1 data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_do",
                    description: Some(
                        "DAC_OUT1 data output",
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
                "DAC_OUT1 12-bit left-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "OUT1_DH[11:0]",
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
                "DAC_OUT1 12-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DAC_OUT1 12-bit right-aligned data",
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
                "DAC_OUT1 8-bit right-aligned data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "out1_dh",
                    description: Some(
                        "DAC_OUT1 8-bit right-aligned data",
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
            name: "Skktr",
            extends: None,
            description: Some(
                "DAC sample and keep keep time Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tkeep0",
                    description: Some(
                        "DAC_OUT0 keep time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tkeep1",
                    description: Some(
                        "DAC_OUT1 keep time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Skrtr",
            extends: None,
            description: Some(
                "DAC sample and keep refresh time Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tref0",
                    description: Some(
                        "DAC_OUT0 refresh time",
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
                    name: "tref1",
                    description: Some(
                        "DAC_OUT1 refresh time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Skstr0",
            extends: None,
            description: Some(
                "DAC sample and keep sample time Register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsamp0",
                    description: Some(
                        "DAC_OUT0 sample time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Skstr1",
            extends: None,
            description: Some(
                "DAC sample and keep sample time Register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tsamp1",
                    description: Some(
                        "DAC_OUT refresh time (only valid in Sample and hold mode)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
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
                    name: "ddudr0",
                    description: Some(
                        "DAC_OUT0 DMA underrun flag, set by hardware, cleared by software write 1",
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
                    name: "calf0",
                    description: Some(
                        "DAC_OUT0 calibration offset flag",
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
                    name: "bwt0",
                    description: Some(
                        "This bit will be set after sample and keep mode enable",
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
                    name: "ddudr1",
                    description: Some(
                        "DAC_OUT1 DMA underrun flag, set by hardware, cleared by software write 1",
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
                Field {
                    name: "calf1",
                    description: Some(
                        "DAC_OUT1 calibration offset flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bwt1",
                    description: Some(
                        "This bit will be set after sample and keep mode enable",
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
            name: "Swt",
            extends: None,
            description: Some(
                "Software trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtr0",
                    description: Some(
                        "DAC_OUT0 software trigger, cleared by hardware",
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
                        "DAC_OUT1 software trigger, cleared by hardware",
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
