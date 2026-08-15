
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Trigsel",
            extends: None,
            description: Some(
                "Flash memory controller",
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
                    name: "timer0eti",
                    description: Some(
                        "Trigger selection for TIMER0_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "timer1iti",
                    description: Some(
                        "Trigger selection for TIMER1_ETI register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2iti",
                    description: Some(
                        "Trigger selection for TIMER2_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3iti",
                    description: Some(
                        "Trigger selection for TIMER3_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4iti",
                    description: Some(
                        "Trigger selection for TIMER4_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7iti",
                    description: Some(
                        "Trigger selection for TIMER7_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer15iti",
                    description: Some(
                        "Trigger selection for TIMER15_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer15iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer16iti",
                    description: Some(
                        "Trigger selection for TIMER16_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer16iti",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac",
                    description: Some(
                        "Trigger selection for DAC register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc0_routrg",
                    description: Some(
                        "Trigger selection for ADC0_ROUTRG register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc0Routrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc0_instrg",
                    description: Some(
                        "Trigger selection for ADC0_INSTRG register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc0Instrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc1_routrg",
                    description: Some(
                        "Trigger selection for ADC1_ROUTRG register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc1Routrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc1_instrg",
                    description: Some(
                        "Trigger selection for ADC1_INSTRG register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc1Instrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc2_routrg",
                    description: Some(
                        "Trigger selection for ADC2_ROUTRG register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc2Routrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc2_instrg",
                    description: Some(
                        "Trigger selection for ADC2_INSTRG register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc2Instrg",
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
                    byte_offset: 0x50,
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
                    name: "timer0chbrkin",
                    description: Some(
                        "Trigger selection for TIMER0_CHBRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0chbrkin",
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
                    byte_offset: 0x58,
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
                    name: "timer7chbrkin",
                    description: Some(
                        "Trigger selection for TIMER7_CHBRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7chbrkin",
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
                    byte_offset: 0x60,
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
                    byte_offset: 0x64,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adc0Instrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC0_INSTRG register",
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
            name: "Adc0Routrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC0_ROUTRG register",
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
            name: "Adc1Instrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC1_INSTRG register",
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
            name: "Adc1Routrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC1_ROUTRG register",
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
            name: "Adc2Instrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC2_INSTRG register",
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
            name: "Adc2Routrg",
            extends: None,
            description: Some(
                "Trigger selection for ADC2_ROUTRG register",
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
            name: "Dac",
            extends: None,
            description: Some(
                "Trigger selection for DAC register",
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
                        "Trigger input source selection for TIMER0_BRKIN",
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
            name: "Timer0chbrkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER0_CHBRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for TIMER0_CH0BRKIN",
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
                        "Trigger input source selection for TIMER0_CH1BRKIN",
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
                        "Trigger input source selection for TIMER0_CH2BRKIN.",
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
                "Trigger selection for TIMER0_ITI register",
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
                        "Trigger input source selection for TIMER15_BRKIN.",
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
            name: "Timer15iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER15_ITI register",
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
                        "Trigger input source selection for TIMER16_BRKIN",
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
            name: "Timer16iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER16_ITI register",
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
            name: "Timer1iti",
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
            name: "Timer2iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER2_ITI register",
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
            name: "Timer3iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER3_ITI register",
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
            name: "Timer4iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER4_ITI register",
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
                        "Trigger input source selection for TIMER7_BRKIN",
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
            name: "Timer7chbrkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER7_CHBRKIN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "insel0",
                    description: Some(
                        "Trigger input source selection for TIMER7_CH0BRKIN",
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
                        "Trigger input source selection for TIMER7_CH1BRKIN",
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
                        "Trigger input source selection for TIMER7_CH2BRKIN.",
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
            name: "Timer7iti",
            extends: None,
            description: Some(
                "Trigger selection for TIMER7_ITI register",
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
                