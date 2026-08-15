
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
                    name: "extout0",
                    description: Some(
                        "Trigger selection for EXTOUT0 register",
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
                    name: "extout1",
                    description: Some(
                        "Trigger selection for EXTOUT1 register",
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
                    name: "adc0",
                    description: Some(
                        "Trigger selection for ADC0 register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    byte_offset: 0xc,
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
                    name: "dac",
                    description: Some(
                        "Trigger selection for DAC register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "timer0in",
                    description: Some(
                        "Trigger selection for TIMER0_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0in",
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
                    byte_offset: 0x18,
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
                    name: "timer7in",
                    description: Some(
                        "Trigger selection for TIMER7_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7in",
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
                    byte_offset: 0x20,
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
                    name: "timer19in",
                    description: Some(
                        "Trigger selection for TIMER19_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer19in",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer19brkin",
                    description: Some(
                        "Trigger selection for TIMER19_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer19brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer20in",
                    description: Some(
                        "Trigger selection for TIMER20_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer20in",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer20brkin",
                    description: Some(
                        "Trigger selection for TIMER20_BRKIN register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer20brkin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1in",
                    description: Some(
                        "Trigger selection for TIMER1_ITI register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1in",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mfcom",
                    description: Some(
                        "Trigger selection for MFCOM register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mfcom",
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
                    byte_offset: 0x3c,
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
                    byte_offset: 0x40,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                "Trigger selection for EXTOUT0 register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                "Trigger selection for EXTOUT1 register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Mfcom",
            extends: None,
            description: Some(
                "Trigger selection for MFCOM register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer0in",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer19brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER19_BRKIN register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer19in",
            extends: None,
            description: Some(
                "Trigger selection for TIMER19_ITI register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer1in",
            extends: None,
            description: Some(
                "Trigger selection for TIMER1_ITI register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer20brkin",
            extends: None,
            description: Some(
                "Trigger selection for TIMER20_BRKIN register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer20in",
            extends: None,
            description: Some(
                "Trigger selection for TIMER20_ITI register",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
            name: "Timer7in",
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
                    bit_size: 7,
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
                    bit_size: 7,
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
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "insel3",
                    description: Some(
                        "Trigger input source selection for output3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lk",
                    description: Some(
                        "TRIGSEL register lock",
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
                