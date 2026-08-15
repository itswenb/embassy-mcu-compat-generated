
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Poc",
            extends: None,
            description: Some(
                "Port Output Controller",
            ),
            items: &[
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "Status register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "in0dcfg",
                    description: Some(
                        "Input n detection configuration regist 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In0dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in1dcfg",
                    description: Some(
                        "Input n detection configuration regist 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In1dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in2dcfg",
                    description: Some(
                        "Input n detection configuration regist 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In2dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in3dcfg",
                    description: Some(
                        "Input n detection configuration regist 3",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In3dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in4dcfg",
                    description: Some(
                        "Input n detection configuration regist 4",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In4dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in5dcfg",
                    description: Some(
                        "Input n detection configuration regist 5",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In5dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "Control register 0",
                    ),
                    array: None,
                    byte_offset: 0x20,
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
                    name: "swdrg",
                    description: Some(
                        "Software disabling request generation register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swdrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdcfg0",
                    description: Some(
                        "Complementary detection configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdcfg1",
                    description: Some(
                        "Complementary detection configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odmode0",
                    description: Some(
                        "Output disable mode register 0",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odmode0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "odmode1",
                    description: Some(
                        "Output disable mode register 1",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Odmode1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reqsel0",
                    description: Some(
                        "Request selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Reqsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reqsel1",
                    description: Some(
                        "Request selection register 1",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Reqsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat1",
                    description: Some(
                        "Status register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extctl0",
                    description: Some(
                        "Extended control register 0",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extctl1",
                    description: Some(
                        "Extended control register 1",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in0dmk",
                    description: Some(
                        "Input n detection mask regist 0",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In0dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in1dmk",
                    description: Some(
                        "Input n detection mask regist 1",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In1dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in2dmk",
                    description: Some(
                        "Input n detection mask regist 2",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In2dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in3dmk",
                    description: Some(
                        "Input n detection mask regist 3",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In3dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in4dmk",
                    description: Some(
                        "Input n detection mask regist 4",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In4dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "in5dmk",
                    description: Some(
                        "Input n detection mask regist 5",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "In5dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp0dmk",
                    description: Some(
                        "Comparator n detection mask regist 0",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp0dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp1dmk",
                    description: Some(
                        "Comparator n detection mask regist 1",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp1dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp2dmk",
                    description: Some(
                        "Comparator n detection mask regist 2",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp2dmk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmp3dmk",
                    description: Some(
                        "Comparator n detection mask regist 3",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmp3dmk",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cdcfg0",
            extends: None,
            description: Some(
                "Complementary detection configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer0_ch0_apsel",
                    description: Some(
                        "TIMER0_CH0 active polarity selection",
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
                    name: "timer0_mch0_apsel",
                    description: Some(
                        "TIMER0_MCH0 active polarity selection",
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
                    name: "timer0_ch1_apsel",
                    description: Some(
                        "TIMER0_CH1 active polarity selection",
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
                    name: "timer0_mch1_apsel",
                    description: Some(
                        "TIMER0_MCH1 active polarity selection",
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
                    name: "timer0_ch2_apsel",
                    description: Some(
                        "TIMER0_CH2 active polarity selection",
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
                    name: "timer0_mch2_apsel",
                    description: Some(
                        "TIMER0_MCH2 active polarity selection",
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
                    name: "timer0_pselen",
                    description: Some(
                        "TIMER0 polarity selection enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ccie",
                    description: Some(
                        "TIMER0 concurrent conduction interrupt enable",
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
                    name: "timer0_ccdren",
                    description: Some(
                        "TIMER0 concurrent conduction disabling request enable",
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
            ],
        },
        FieldSet {
            name: "Cdcfg1",
            extends: None,
            description: Some(
                "Complementary detection configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer7_ch0_apsel",
                    description: Some(
                        "TIMER7_CH0 active polarity selection",
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
                    name: "timer7_mch0_apsel",
                    description: Some(
                        "TIMER7_MCH0 active polarity selection",
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
                    name: "timer7_ch1_apsel",
                    description: Some(
                        "TIMER7_CH1 active polarity selection",
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
                    name: "timer7_mch1_apsel",
                    description: Some(
                        "TIMER7_MCH1 active polarity selection",
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
                    name: "timer7_ch2_apsel",
                    description: Some(
                        "TIMER7_CH2 active polarity selection",
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
                    name: "timer7_mch2_apsel",
                    description: Some(
                        "TIMER7_MCH2 active polarity selection",
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
                    name: "timer7_pselen",
                    description: Some(
                        "TIMER7 polarity selection enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ccie",
                    description: Some(
                        "TIMER7 concurrent conduction interrupt enable",
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
                    name: "timer7_ccdren",
                    description: Some(
                        "TIMER7 concurrent conduction disabling request enable",
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
            ],
        },
        FieldSet {
            name: "Cmp0dmk",
            extends: None,
            description: Some(
                "Comparator n detection mask regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmksel",
                    description: Some(
                        "Comparator n output detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "Cmp1dmk",
            extends: None,
            description: Some(
                "Comparator n detection mask regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmksel",
                    description: Some(
                        "Comparator n output detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "Cmp2dmk",
            extends: None,
            description: Some(
                "Comparator n detection mask regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmksel",
                    description: Some(
                        "Comparator n output detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "Cmp3dmk",
            extends: None,
            description: Some(
                "Comparator n detection mask regist 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpmksel",
                    description: Some(
                        "Comparator n output detection mask source selection",
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
                    name: "hxtals_dren",
                    description: Some(
                        "HXTAL stuck disabling request enable",
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
                    name: "lockup_dren",
                    description: Some(
                        "CPU LOCKUP disabling request enable",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable",
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
                    name: "cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable",
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
                    name: "cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable",
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
                    name: "cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable",
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
            ],
        },
        FieldSet {
            name: "Extctl0",
            extends: None,
            description: Some(
                "Extended control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer0_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for TIMER0",
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
                    name: "timer0_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for TIMER0",
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
                    name: "timer0_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for TIMER0",
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
                    name: "timer0_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for TIMER0",
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
                    name: "timer7_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for TIMER7",
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
                    name: "timer7_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for TIMER7",
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
                    name: "timer7_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for TIMER7",
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
                    name: "timer7_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for TIMER7",
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
            ],
        },
        FieldSet {
            name: "Extctl1",
            extends: None,
            description: Some(
                "Extended control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer1_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for TIMER1",
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
                    name: "timer1_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for TIMER1",
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
                    name: "timer1_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for TIMER1",
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
                    name: "timer1_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for TIMER1",
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
                    name: "timer2_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for TIMER2",
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
                    name: "timer2_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for TIMER2",
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
                    name: "timer2_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for TIMER2",
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
                    name: "timer2_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for TIMER2",
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
                    name: "gptimer0_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for GPTIMER0",
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
                    name: "gptimer0_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for GPTIMER0",
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
                    name: "gptimer0_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for GPTIMER0",
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
                    name: "gptimer0_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for GPTIMER0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_cmp0dren",
                    description: Some(
                        "Comparator 0 disabling request enable for GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_cmp1dren",
                    description: Some(
                        "Comparator 1 disabling request enable for GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_cmp2dren",
                    description: Some(
                        "Comparator 2 disabling request enable for GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_cmp3dren",
                    description: Some(
                        "Comparator 3 disabling request enable for GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "In0dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In0dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "In1dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In1dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "In2dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In2dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "In3dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In3dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "In4dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In4dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "In5dcfg",
            extends: None,
            description: Some(
                "Input n detection configuration regist 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "indmsel",
                    description: Some(
                        "POC_INn input detection mode selection",
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
                Field {
                    name: "indsnum",
                    description: Some(
                        "POC_INn input detection sampling number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "inie",
                    description: Some(
                        "POC_INn input interrupt enable",
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
                    name: "indren",
                    description: Some(
                        "POC_INn disabling request enable",
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
                    name: "inpl",
                    description: Some(
                        "Polarity of POC_INn pin input",
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
            ],
        },
        FieldSet {
            name: "In5dmk",
            extends: None,
            description: Some(
                "Input n detection mask regist 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "inmksel",
                    description: Some(
                        "POC_INn input detection mask source selection",
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
            ],
        },
        FieldSet {
            name: "Odmode0",
            extends: None,
            description: Some(
                "Output disable mode register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer0_osel0",
                    description: Some(
                        "Output selection for TIMER0_CH0/ TIMER0_MCH0 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_osel1",
                    description: Some(
                        "Output selection for TIMER0_CH1/ TIMER0_MCH1 pin",
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
                    name: "timer0_osel2",
                    description: Some(
                        "Output selection for TIMER0_CH2/ TIMER0_MCH2 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_osel3",
                    description: Some(
                        "Output selection for TIMER0_CH3/ TIMER0_MCH3 pin",
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
                    name: "timer7_osel0",
                    description: Some(
                        "Output selection for TIMER7_CH0/ TIMER7_MCH0 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_osel1",
                    description: Some(
                        "Output selection for TIMER7_CH1/ TIMER7_MCH1 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_osel2",
                    description: Some(
                        "Output selection for TIMER7_CH2/ TIMER7_MCH2 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_osel3",
                    description: Some(
                        "Output selection for TIMER7_CH3/ TIMER7_MCH3 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Odmode1",
            extends: None,
            description: Some(
                "Output disable mode register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer1_osel0",
                    description: Some(
                        "Output selection for TIMER1_CH0 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_osel1",
                    description: Some(
                        "Output selection for TIMER1_CH1 pin",
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
                    name: "timer1_osel2",
                    description: Some(
                        "Output selection for TIMER1_CH2 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_osel3",
                    description: Some(
                        "Output selection for TIMER1_CH3 pin",
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
                    name: "timer2_osel0",
                    description: Some(
                        "Output selection for TIMER2_CH0 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_osel1",
                    description: Some(
                        "Output selection for TIMER2_CH1 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_osel2",
                    description: Some(
                        "Output selection for TIMER2_CH2 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_osel3",
                    description: Some(
                        "Output selection for TIMER2_CH3 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer0_osel0",
                    description: Some(
                        "Output selection for GPTIMER0_CH0 pin",
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
                    name: "gptimer0_osel1",
                    description: Some(
                        "Output selection for GPTIMER0_CH1 pin",
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
                    name: "gptimer1_osel0",
                    description: Some(
                        "Output selection for GPTIMER1_CH0 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_osel1",
                    description: Some(
                        "Output selection for GPTIMER1_CH1 pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Reqsel0",
            extends: None,
            description: Some(
                "Request selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer0_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select TIMER0",
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
                    name: "timer0_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select TIMER0",
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
                    name: "timer0_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select TIMER0",
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
                    name: "timer0_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select TIMER0",
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
                    name: "timer0_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select TIMER0",
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
                    name: "timer0_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select TIMER0",
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
                    name: "timer0_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select TIMER0",
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
                    name: "timer7_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select TIMER7",
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
                    name: "timer7_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select TIMER7",
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
                    name: "timer7_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select TIMER7",
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
                    name: "timer7_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select TIMER7",
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
                    name: "timer7_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select TIMER7",
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
                    name: "timer7_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select TIMER7",
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
                    name: "timer7_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select TIMER7",
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
            name: "Reqsel1",
            extends: None,
            description: Some(
                "Request selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer1_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select TIMER1",
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
                    name: "timer1_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select TIMER1",
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
                    name: "timer1_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select TIMER1",
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
                    name: "timer1_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select TIMER1",
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
                    name: "timer1_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select TIMER1",
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
                    name: "timer1_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select TIMER1",
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
                    name: "timer1_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select TIMER1",
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
                    name: "timer2_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select TIMER2",
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
                    name: "timer2_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select TIMER2",
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
                    name: "timer2_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select TIMER2",
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
                    name: "timer2_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select TIMER2",
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
                    name: "timer2_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select TIMER2",
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
                    name: "timer2_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select TIMER2",
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
                    name: "timer2_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select TIMER2",
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
                    name: "gptimer0_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select GPTIMER0",
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
                    name: "gptimer0_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select GPTIMER0",
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
                    name: "gptimer0_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select GPTIMER0",
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
                    name: "gptimer0_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select GPTIMER0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer0_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select GPTIMER0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer0_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select GPTIMER0",
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
                Field {
                    name: "gptimer0_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select GPTIMER0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_in0drsel",
                    description: Some(
                        "POC_IN0 disabling request select GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_in1drsel",
                    description: Some(
                        "POC_IN1 disabling request select GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_in2drsel",
                    description: Some(
                        "POC_IN2 disabling request select GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_in3drsel",
                    description: Some(
                        "POC_IN3 disabling request select GPTIMER1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptimer1_in4drsel",
                    description: Some(
                        "POC_IN4 disabling request select GPTIMER1",
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
                    name: "gptimer1_in5drsel",
                    description: Some(
                        "POC_IN5 disabling request select GPTIMER1",
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
                    name: "gptimer1_cmpdrsel",
                    description: Some(
                        "CMPn (n=0,1,2,3) disabling request select GPTIMER1",
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
            name: "Stat0",
            extends: None,
            description: Some(
                "Status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "in0if",
                    description: Some(
                        "POC_IN0 input interrupt flag",
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
                    name: "in1if",
                    description: Some(
                        "POC_IN1 input interrupt flag",
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
                    name: "in2if",
                    description: Some(
                        "POC_IN2 input interrupt flag",
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
                    name: "in3if",
                    description: Some(
                        "POC_IN3 input interrupt flag",
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
                    name: "in4if",
                    description: Some(
                        "POC_IN4 input interrupt flag",
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
                    name: "in5if",
                    description: Some(
                        "POC_IN5 input interrupt flag",
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
                    name: "hxtalsdf",
                    description: Some(
                        "HXTAL stuck detection flag",
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
                    name: "lockupdf",
                    description: Some(
                        "CPU LOCKUP detection flag",
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
                    name: "timer0_ccif",
                    description: Some(
                        "TIMER0 concurrent conduction interrupt flag",
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
                    name: "timer7_ccif",
                    description: Some(
                        "TIMER7 concurrent conduction interrupt flag",
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
            name: "Stat1",
            extends: None,
            description: Some(
                "Status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmp0df",
                    description: Some(
                        "Comparator 0 detection flag",
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
                    name: "cmp1df",
                    description: Some(
                        "Comparator 1 detection flag",
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
                    name: "cmp2df",
                    description: Some(
                        "Comparator 2 detection flag",
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
                    name: "cmp3df",
                    description: Some(
                        "Comparator 3 detection flag",
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
            ],
        },
        FieldSet {
            name: "Swdrg",
            extends: None,
            description: Some(
                "Software disabling request generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer0_swdrg",
                    description: Some(
                        "Software disabling request generation for TIMER0",
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
                    name: "timer7_swdrg",
                    description: Some(
                        "Software disabling request generation for TIMER7",
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
                    name: "timer1_swdrg",
                    description: Some(
                        "Software disabling request generation for TIMER1",
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
                    name: "timer2_swdrg",
                    description: Some(
                        "Software disabling request generation for TIMER2",
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
                    name: "gptimer0_swdrg",
                    description: Some(
                        "Software disabling request generation for GPTIMER0",
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
                    name: "gptimer1_swdrg",
                    description: Some(
                        "Software disabling request generation for GPTIMER1",
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
    ],
    enums: &[],
};
                