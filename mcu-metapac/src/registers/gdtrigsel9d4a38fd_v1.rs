
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Trigsel",
            extends: None,
            description: Some(
                "Trigger selection controller",
            ),
            items: &[
                BlockItem {
                    name: "extout_0",
                    description: Some(
                        "Trigger selection for EXTOUT register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extout0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extout_1",
                    description: Some(
                        "Trigger selection for EXTOUT register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extout1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extout_2",
                    description: Some(
                        "Trigger selection for EXTOUT register 2",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extout2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extout_3",
                    description: Some(
                        "Trigger selection for EXTOUT register 3",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extout3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc0",
                    description: Some(
                        "Trigger selection for ADC0 register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc1",
                    description: Some(
                        "Trigger selection for ADC1 register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc2",
                    description: Some(
                        "Trigger selection for ADC2 register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0out0",
                    description: Some(
                        "Trigger selection for DAC0_OUT0 register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0out0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0out1",
                    description: Some(
                        "Trigger selection for DAC0_OUT1 register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0out1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0brkin",
                    description: Some(
                        "Trigger selection for TIMER0_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7brkin",
                    description: Some(
                        "Trigger selection for TIMER7_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14brkin",
                    description: Some(
                        "Trigger selection for TIMER14_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer15brkin",
                    description: Some(
                        "Trigger selection for TIMER15_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer15brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer16brkin",
                    description: Some(
                        "Trigger selection for TIMER16_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer16brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer40brkin",
                    description: Some(
                        "Trigger selection for TIMER40_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer40brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer41brkin",
                    description: Some(
                        "Trigger selection for TIMER41_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer41brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer42brkin",
                    description: Some(
                        "Trigger selection for TIMER42_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer42brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer43brkin",
                    description: Some(
                        "Trigger selection for TIMER43_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer43brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer44brkin",
                    description: Some(
                        "Trigger selection for TIMER44_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer44brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "can0",
                    description: Some(
                        "Trigger selection for CAN0 register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Can0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "can1",
                    description: Some(
                        "Trigger selection for CAN1 register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Can1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "can2",
                    description: Some(
                        "Trigger selection for CAN2 register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Can2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpdts",
                    description: Some(
                        "Trigger selection for LPDTS register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpdts",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0eti",
                    description: Some(
                        "Trigger selection for TIMER0_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1eti",
                    description: Some(
                        "Trigger selection for TIMER1_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2eti",
                    description: Some(
                        "Trigger selection for TIMER2_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3eti",
                    description: Some(
                        "Trigger selection for TIMER3_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4eti",
                    description: Some(
                        "Trigger selection for TIMER4_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7eti",
                    description: Some(
                        "Trigger selection for TIMER7_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer22eti",
                    description: Some(
                        "Trigger selection for TIMER22_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer22eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer23eti",
                    description: Some(
                        "Trigger selection for TIMER23_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer23eti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edout",
                    description: Some(
                        "Trigger selection for EDOUT register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Edout",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf",
                    description: Some(
                        "Trigger selection for HPDF register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hpdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0iti14",
                    description: Some(
                        "Trigger selection for TIMER0_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1iti14",
                    description: Some(
                        "Trigger selection for TIMER1_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2iti14",
                    description: Some(
                        "Trigger selection for TIMER2_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3iti14",
                    description: Some(
                        "Trigger selection for TIMER3_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4iti14",
                    description: Some(
                        "Trigger selection for TIMER4_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7iti14",
                    description: Some(
                        "Trigger selection for TIMER7_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14iti14",
                    description: Some(
                        "Trigger selection for TIMER14_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer22iti14",
                    description: Some(
                        "Trigger selection for TIMER22_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer22iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer23iti14",
                    description: Some(
                        "Trigger selection for TIMER23_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer23iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer40iti14",
                    description: Some(
                        "Trigger selection for TIMER40_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer40iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer41iti14",
                    description: Some(
                        "Trigger selection for TIMER41_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer41iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer42iti14",
                    description: Some(
                        "Trigger selection for TIMER42_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer42iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer43iti14",
                    description: Some(
                        "Trigger selection for TIMER43_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer43iti14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer44iti14",
                    description: Some(
                        "Trigger selection for TIMER44_ITI14 register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer44iti14",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adc0",
            extends: None,
            description: Some(
                "Trigger selection for ADC0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Adc1",
            extends: None,
            description: Some(
                "Trigger selection for ADC1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Adc2",
            extends: None,
            description: Some(
                "Trigger selection for ADC2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Can0",
            extends: None,
            description: Some(
                "Trigger selection for CAN0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Can1",
            extends: None,
            description: Some(
                "Trigger selection for CAN1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Can2",
            extends: None,
            description: Some(
                "Trigger selection for CAN2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Dac0out0",
            extends: None,
            description: Some(
                "Trigger selection for DAC0_OUT0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Dac0out1",
            extends: None,
            description: Some(
                "Trigger selection for DAC0_OUT1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Edout",
            extends: None,
            description: Some(
                "Trigger selection for EDOUT register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Extout0",
            extends: None,
            description: Some(
                "Trigger selection for EXTOUT register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Extout1",
            extends: None,
            description: Some(
                "Trigger selection for EXTOUT register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Extout2",
            extends: None,
            description: Some(
                "Trigger selection for EXTOUT register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Extout3",
            extends: None,
            description: Some(
                "Trigger selection for EXTOUT register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Hpdf",
            extends: None,
            description: Some(
                "Trigger selection for HPDF register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Lpdts",
            extends: None,
            description: Some(
                "Trigger selection for LPDTS register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer0brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER0_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "insel2",
                    description: Some(
                        "Trigger input source selection for output2",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer0eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER0_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer0iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER0_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer14brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER14_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer14iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER14_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer15brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER15_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer16brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER16_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer1eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER1_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer1iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER1_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer22eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER22_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer22iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER22_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer23eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER23_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer23iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER23_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer2eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER2_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer2iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER2_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer3eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER3_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer3iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER3_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer40brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER40_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer40iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER40_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer41brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER41_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer41iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER41_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer42brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER42_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer42iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER42_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer43brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER43_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer43iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER43_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer44brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER44_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer44iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER44_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer4eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER4_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer4iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER4_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer7brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER7_BRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "insel1",
                    description: Some(
                        "Trigger input source selection for output1",
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
                Field {
                    name: "insel2",
                    description: Some(
                        "Trigger input source selection for output2",
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
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer7eti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER7_ETI register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
            name: "Timer7iti14",
            extends: None,
            description: Some(
                "Trigger selection for TIMER7_ITI14 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for output0",
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
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock.",
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
                