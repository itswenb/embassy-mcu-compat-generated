
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Evic",
            extends: None,
            description: Some(
                "Event interconnection unit",
            ),
            items: &[
                BlockItem {
                    name: "swev",
                    description: Some(
                        "Software event register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Swev",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgio0",
                    description: Some(
                        "Event interconnect for single I/O register 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgio0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgio1",
                    description: Some(
                        "Event interconnect for single I/O register 1",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgio1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgio2",
                    description: Some(
                        "Event interconnect for single I/O register 2",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgio2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgio3",
                    description: Some(
                        "Event interconnect for single I/O register 3",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgio3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ingrpe",
                    description: Some(
                        "Event interconnect for input group E register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ingrpe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ingrpf",
                    description: Some(
                        "Event interconnect for input group F register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ingrpf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "outgrpe",
                    description: Some(
                        "Event interconnect for output group E register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Outgrpe",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "outgrpf",
                    description: Some(
                        "Event interconnect for output group F register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Outgrpf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0cov",
                    description: Some(
                        "Event interconnect for DAC0 conversion register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0cov",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc0cov",
                    description: Some(
                        "Event interconnect for ADC0 conversion register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc0cov",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc2cov",
                    description: Some(
                        "Event interconnect for ADC2 conversion register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adc2cov",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rcu",
                    description: Some(
                        "Event interconnect for RCU register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rcu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0",
                    description: Some(
                        "Event interconnect for TIMER0 register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7",
                    description: Some(
                        "Event interconnect for TIMER7 register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cptimer0",
                    description: Some(
                        "Event interconnect for CPTIMER0 register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cptimer0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cptimerw",
                    description: Some(
                        "Event interconnect for CPTIMERW register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cptimerw",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1",
                    description: Some(
                        "Event interconnect for TIMER1 register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2",
                    description: Some(
                        "Event interconnect for TIMER2 register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gptimer_0",
                    description: Some(
                        "Event interconnect for GPTIMER register 0",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gptimer0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gptimer_1",
                    description: Some(
                        "Event interconnect for GPTIMER register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gptimer1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gptimer_2",
                    description: Some(
                        "Event interconnect for GPTIMER register 2",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gptimer2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gptimer_3",
                    description: Some(
                        "Event interconnect for GPTIMER register 3",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gptimer3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dac0en",
                    description: Some(
                        "Event interconnect for DAC0 enable register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dac0en",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_0",
                    description: Some(
                        "Event interconnect for DMAMUX register 0",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_1",
                    description: Some(
                        "Event interconnect for DMAMUX register 1",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_2",
                    description: Some(
                        "Event interconnect for DMAMUX register 2",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_3",
                    description: Some(
                        "Event interconnect for DMAMUX register 3",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_4",
                    description: Some(
                        "Event interconnect for DMAMUX register 4",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmamux_5",
                    description: Some(
                        "Event interconnect for DMAMUX register 5",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmamux5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcfg0",
                    description: Some(
                        "Slave mode configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "smcfg1",
                    description: Some(
                        "Slave mode configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Smcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgiocfg0",
                    description: Some(
                        "Single I/O configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgiocfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgiocfg1",
                    description: Some(
                        "Single I/O configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgiocfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgiocfg2",
                    description: Some(
                        "Single I/O configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgiocfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sgiocfg3",
                    description: Some(
                        "Single I/O configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sgiocfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grpecfg",
                    description: Some(
                        "Group E configuration register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grpecfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grpedh",
                    description: Some(
                        "Group E data holding register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grpedh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gprfcfg",
                    description: Some(
                        "Group F configuration register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gprfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grpfdh",
                    description: Some(
                        "Group F data holding register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grpfdh",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adc0cov",
            extends: None,
            description: Some(
                "Event interconnect for ADC0 conversion register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection 0 for ADC0 group conversion",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection 1 for ADC0 group conversion",
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
                        "EVIC_ADC0COV register lock.",
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
            name: "Adc2cov",
            extends: None,
            description: Some(
                "Event interconnect for ADC2 conversion register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection 0 for ADC2 group conversion",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection 1 for ADC2 group conversion",
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
                        "EVIC_ADC2COV register lock.",
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
            name: "Cptimer0",
            extends: None,
            description: Some(
                "Event interconnect for CPTIMER0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for CPTIMER0 slave mode",
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
                        "EVIC_CPTIMER0 register lock.",
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
            name: "Cptimerw",
            extends: None,
            description: Some(
                "Event interconnect for CPTIMERW register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for CPTIMERW slave mode",
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
                        "EVIC_CPTIMERW register lock.",
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
            name: "Dac0cov",
            extends: None,
            description: Some(
                "Event interconnect for DAC0 conversion register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for DAC0_OUT0 conversion",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection for DAC0_OUT1 conversion",
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
                        "EVIC_DAC0COV register lock.",
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
            name: "Dac0en",
            extends: None,
            description: Some(
                "Event interconnect for DAC0 enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for enabling DAC0_OUT0",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection for enabling DAC0_OUT1",
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
                        "EVIC_DAC0EN register lock.",
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
            name: "Dmamux0",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection 0 for DMAMUX",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection 1 for DMAMUX",
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
                        "EVIC_DMAMUX_0 register lock.",
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
            name: "Dmamux1",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel2",
                    description: Some(
                        "Event source selection 2 for DMAMUX",
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
                    name: "evsel3",
                    description: Some(
                        "Event source selection 3 for DMAMUX",
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
                        "EVIC_DMAMUX_1 register lock.",
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
            name: "Dmamux2",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel4",
                    description: Some(
                        "Event source selection 4 for DMAMUX",
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
                    name: "evsel5",
                    description: Some(
                        "Event source selection 5 for DMAMUX",
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
                        "EVIC_DMAMUX_2 register lock.",
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
            name: "Dmamux3",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel6",
                    description: Some(
                        "Event source selection 6 for DMAMUX",
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
                    name: "evsel7",
                    description: Some(
                        "Event source selection 7 for DMAMUX",
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
                        "EVIC_DMAMUX_3 register lock.",
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
            name: "Dmamux4",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel8",
                    description: Some(
                        "Event source selection 8 for DMAMUX",
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
                    name: "evsel9",
                    description: Some(
                        "Event source selection 9 for DMAMUX",
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
                        "EVIC_DMAMUX_4 register lock.",
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
            name: "Dmamux5",
            extends: None,
            description: Some(
                "Event interconnect for DMAMUX register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel10",
                    description: Some(
                        "Event source selection 10 for DMAMUX",
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
                    name: "evsel11",
                    description: Some(
                        "Event source selection 11 for DMAMUX",
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
                        "EVIC_DMAMUX_5 register lock.",
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
            name: "Gprfcfg",
            extends: None,
            description: Some(
                "Group F configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "grpfind",
                    description: Some(
                        "Group F input detection",
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
                    name: "grpfosel",
                    description: Some(
                        "Group F output selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovwen",
                    description: Some(
                        "EVIC_GRPFDH register overwrite enable",
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
                    name: "pf8sel",
                    description: Some(
                        "PF8 pin selection",
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
                    name: "pf9sel",
                    description: Some(
                        "PF9 pin selection",
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
                    name: "pf10sel",
                    description: Some(
                        "PF10 pin selection",
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
                    name: "pf11sel",
                    description: Some(
                        "PE11 pin selection",
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
                    name: "pf12sel",
                    description: Some(
                        "PF12 pin selection",
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
                    name: "pf13sel",
                    description: Some(
                        "PF13 pin selection",
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
                    name: "pf14sel",
                    description: Some(
                        "PF14 pin selection",
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
            name: "Gptimer0",
            extends: None,
            description: Some(
                "Event interconnect for GPTIMER register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection 0 for GPTIMER slave mode",
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
                    name: "evsel1",
                    description: Some(
                        "Event source selection 1 for GPTIMER slave mode",
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
                        "EVIC_GPTIMER_0 register lock.",
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
            name: "Gptimer1",
            extends: None,
            description: Some(
                "Event interconnect for GPTIMER register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel2",
                    description: Some(
                        "Event source selection 2 for GPTIMER slave mode",
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
                    name: "evsel3",
                    description: Some(
                        "Event source selection 3 for GPTIMER slave mode",
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
                        "EVIC_GPTIMER_1 register lock.",
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
            name: "Gptimer2",
            extends: None,
            description: Some(
                "Event interconnect for GPTIMER register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel4",
                    description: Some(
                        "Event source selection 4 for GPTIMER slave mode",
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
                    name: "evsel5",
                    description: Some(
                        "Event source selection 5 for GPTIMER slave mode",
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
                        "EVIC_GPTIMER_2 register lock.",
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
            name: "Gptimer3",
            extends: None,
            description: Some(
                "Event interconnect for GPTIMER register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel6",
                    description: Some(
                        "Event source selection 6 for GPTIMER slave mode",
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
                    name: "evsel7",
                    description: Some(
                        "Event source selection 7 for GPTIMER slave mode",
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
                        "EVIC_GPTIMER_3 register lock.",
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
            name: "Grpecfg",
            extends: None,
            description: Some(
                "Group E configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "grpeind",
                    description: Some(
                        "Group E input detection",
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
                    name: "grpeosel",
                    description: Some(
                        "Group E output selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovwen",
                    description: Some(
                        "EVIC_GRPEDH register overwrite enable",
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
                    name: "pe8sel",
                    description: Some(
                        "PE8 pin selection",
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
                    name: "pe9sel",
                    description: Some(
                        "PE9 pin selection",
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
                    name: "pe10sel",
                    description: Some(
                        "PE10 pin selection",
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
                    name: "pe11sel",
                    description: Some(
                        "PE11 pin selection",
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
                    name: "pe12sel",
                    description: Some(
                        "PE12 pin selection",
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
                    name: "pe13sel",
                    description: Some(
                        "PE13 pin selection",
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
                    name: "pe14sel",
                    description: Some(
                        "PE14 pin selection",
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
            name: "Grpedh",
            extends: None,
            description: Some(
                "Group E data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "grpe_dh",
                    description: Some(
                        "Group E holding data",
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
            ],
        },
        FieldSet {
            name: "Grpfdh",
            extends: None,
            description: Some(
                "Group F data holding register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "grpf_dh",
                    description: Some(
                        "Group F holding data",
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
            ],
        },
        FieldSet {
            name: "Ingrpe",
            extends: None,
            description: Some(
                "Event interconnect for input group E register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for group E I/O input",
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
                        "EVIC_INGRPE register lock.",
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
            name: "Ingrpf",
            extends: None,
            description: Some(
                "Event interconnect for input group F register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for group F I/O input",
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
                        "EVIC_INGRPF register lock.",
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
            name: "Outgrpe",
            extends: None,
            description: Some(
                "Event interconnect for output group E register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for group E I/O output",
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
                        "EVIC_OUTGRPE register lock.",
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
            name: "Outgrpf",
            extends: None,
            description: Some(
                "Event interconnect for output group F register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for group F I/O output",
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
                        "EVIC_OUTGRPF register lock.",
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
            name: "Rcu",
            extends: None,
            description: Some(
                "Event interconnect for RCU register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for IRC32M switching",
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
                        "EVIC_RCU register lock.",
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
            name: "Sgio0",
            extends: None,
            description: Some(
                "Event interconnect for single I/O register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for single I/O",
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
                        "EVIC_SGIOx register lock.",
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
            name: "Sgio1",
            extends: None,
            description: Some(
                "Event interconnect for single I/O register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for single I/O",
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
                        "EVIC_SGIOx register lock.",
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
            name: "Sgio2",
            extends: None,
            description: Some(
                "Event interconnect for single I/O register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for single I/O",
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
                        "EVIC_SGIOx register lock.",
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
            name: "Sgio3",
            extends: None,
            description: Some(
                "Event interconnect for single I/O register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for single I/O",
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
                        "EVIC_SGIOx register lock.",
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
            name: "Sgiocfg0",
            extends: None,
            description: Some(
                "Single I/O configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sgioport",
                    description: Some(
                        "Single I/O port",
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
                    name: "sgiopin",
                    description: Some(
                        "Single I/O pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgiom",
                    description: Some(
                        "Single I/O mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sgiocfg1",
            extends: None,
            description: Some(
                "Single I/O configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sgioport",
                    description: Some(
                        "Single I/O port",
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
                    name: "sgiopin",
                    description: Some(
                        "Single I/O pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgiom",
                    description: Some(
                        "Single I/O mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sgiocfg2",
            extends: None,
            description: Some(
                "Single I/O configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sgioport",
                    description: Some(
                        "Single I/O port",
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
                    name: "sgiopin",
                    description: Some(
                        "Single I/O pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgiom",
                    description: Some(
                        "Single I/O mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sgiocfg3",
            extends: None,
            description: Some(
                "Single I/O configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sgioport",
                    description: Some(
                        "Single I/O port",
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
                    name: "sgiopin",
                    description: Some(
                        "Single I/O pin",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgiom",
                    description: Some(
                        "Single I/O mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smcfg0",
            extends: None,
            description: Some(
                "Slave mode configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cptimer0_smsel",
                    description: Some(
                        "CPTIMER0 slave mode selection",
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
            ],
        },
        FieldSet {
            name: "Smcfg1",
            extends: None,
            description: Some(
                "Slave mode configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cptimerw_smsel",
                    description: Some(
                        "CPTIMERW slave mode selection",
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
            ],
        },
        FieldSet {
            name: "Swev",
            extends: None,
            description: Some(
                "Software event register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bwen",
                    description: Some(
                        "SWEVG bit write enable",
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
                    name: "swevg",
                    description: Some(
                        "Software event generation",
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
                    name: "rwen",
                    description: Some(
                        "EVIC_SWEV register write enable",
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
            name: "Timer0",
            extends: None,
            description: Some(
                "Event interconnect for TIMER0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for TIMER0 slave mode.",
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
                        "EVIC_TIMER0 register lock.",
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
            name: "Timer1",
            extends: None,
            description: Some(
                "Event interconnect for TIMER1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event input source selection for TIMER1 slave mode trigger",
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
                        "EVIC_TIMER1 register lock.",
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
            name: "Timer2",
            extends: None,
            description: Some(
                "Event interconnect for TIMER2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for TIMER2 slave mode",
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
                        "EVIC_TIMER2 register lock.",
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
            name: "Timer7",
            extends: None,
            description: Some(
                "Event interconnect for TIMER7 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evsel0",
                    description: Some(
                        "Event source selection for TIMER7 slave mode.",
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
                        "EVIC_TIMER7 register lock.",
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
                