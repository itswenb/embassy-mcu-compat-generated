
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc0",
            extends: None,
            description: Some(
                "Analog-to-digital converter",
            ),
            items: &[
                BlockItem {
                    name: "eocctl",
                    description: Some(
                        "Software EOC control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Eocctl",
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
                    byte_offset: 0x4,
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
                    name: "gpcgf0",
                    description: Some(
                        "Group config register 0",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpcgf0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpcgf1",
                    description: Some(
                        "Group config register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpcgf1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chsel0",
                    description: Some(
                        "Channel selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chsel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chsel1",
                    description: Some(
                        "Channel selection register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chsel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sampr0",
                    description: Some(
                        "Sample time register 0",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sampr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sampr1",
                    description: Some(
                        "Sample time register 1",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sampr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chpri0",
                    description: Some(
                        "Channel priority register 0",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chpri0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addt0",
                    description: Some(
                        "ADC channel addition times register 0",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addt0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sddata",
                    description: Some(
                        "Self-diagnosis data register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sddata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp1bidata",
                    description: Some(
                        "Group_prix bifurcate data register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp1bidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp1bidata1",
                    description: Some(
                        "Group_prix bifurcate data register 1",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp1bidata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp1bidata2",
                    description: Some(
                        "Group_prix bifurcate data register 2",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp1bidata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp2bidata",
                    description: Some(
                        "Group_prix bifurcate data register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp2bidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp2bidata1",
                    description: Some(
                        "Group_prix bifurcate data register 1",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp2bidata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp2bidata2",
                    description: Some(
                        "Group_prix bifurcate data register 2",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp2bidata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp3bidata",
                    description: Some(
                        "Group_prix bifurcate data register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp3bidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp3bidata1",
                    description: Some(
                        "Group_prix bifurcate data register 1",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp3bidata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp3bidata2",
                    description: Some(
                        "Group_prix bifurcate data register 2",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp3bidata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp4bidata",
                    description: Some(
                        "Group_prix bifurcate data register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp4bidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp4bidata1",
                    description: Some(
                        "Group_prix bifurcate data register 1",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp4bidata1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp4bidata2",
                    description: Some(
                        "Group_prix bifurcate data register 2",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp4bidata2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp1dmar",
                    description: Some(
                        "Group_prix data DMA register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp1dmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp2dmar",
                    description: Some(
                        "Group_prix data DMA register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp2dmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp3dmar",
                    description: Some(
                        "Group_prix data DMA register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp3dmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gp4dmar",
                    description: Some(
                        "Group_prix data DMA register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gp4dmar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch0data",
                    description: Some(
                        "Channel 0 data register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch0data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch1data",
                    description: Some(
                        "Channel 1 data register",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch1data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch2data",
                    description: Some(
                        "Channel 2 data register",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch2data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch3data",
                    description: Some(
                        "Channel 3 data register",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch3data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch4data",
                    description: Some(
                        "Channel 4 data register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch4data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch5data",
                    description: Some(
                        "Channel 5 data register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch5data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ch6data",
                    description: Some(
                        "Channel 6 data register",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ch6data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdctl",
                    description: Some(
                        "Watchdog control register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdathold",
                    description: Some(
                        "Watchdog A threshold register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdathold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdbthold",
                    description: Some(
                        "Watchdog B threshold register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdbthold",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdach",
                    description: Some(
                        "Watchdog A channel config register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wdach",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdga_chan_str",
                    description: Some(
                        "Window A compare channel status register, ADC_WDGA_CHAN_STR",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WdgaChanStr",
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
                    byte_offset: 0x150,
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
                    name: "shctl",
                    description: Some(
                        "Sample-and-hold control register",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Shctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmactl",
                    description: Some(
                        "DMA control register",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmactl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bitrgctl",
                    description: Some(
                        "Bifurcate trigger control register",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bitrgctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addt0",
            extends: None,
            description: Some(
                "ADC channel addition times register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch0addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN00",
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
                    name: "ch1addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN01",
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
                    name: "ch2addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN02",
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
                    name: "ch3addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN03",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch4addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN04",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch5addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN05",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ch6addt",
                    description: Some(
                        "The total addition conversion times of ADCx_IN06",
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
            ],
        },
        FieldSet {
            name: "Bitrgctl",
            extends: None,
            description: Some(
                "Bifurcate trigger control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dblrscn_pri1",
                    description: Some(
                        "For any group use double trigger function, the next trigger during the current A/D conversion round restore enable:",
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
                    name: "dble_pri1",
                    description: Some(
                        "Bifurcate Trigger Mode enable of Group_pri1",
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
                    name: "dblans_pri1",
                    description: Some(
                        "Bifurcate Trigger Channel Select of Group_pri1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp2blrscn",
                    description: Some(
                        "For any group use double trigger function, the next trigger during the current A/D conversion round restore enable:",
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
                    name: "gp2bimen",
                    description: Some(
                        "Bifurcate Trigger Mode enable of Group_pri4",
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
                    name: "gp2bichsel",
                    description: Some(
                        "Bifurcate Trigger Channel Select of Group_pri4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp3bitgrs",
                    description: Some(
                        "Enable the next trigger restore of Group_pri3",
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
                    name: "gp3bimen",
                    description: Some(
                        "Bifurcate Trigger Mode enable of Group_pri4",
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
                    name: "gp3bichsel",
                    description: Some(
                        "Bifurcate Trigger Channel Select of Group_pri4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp4blrscn",
                    description: Some(
                        "For any group use double trigger function, the next trigger during the current A/D conversion round restore enable:",
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
                    name: "gp4bimen",
                    description: Some(
                        "Bifurcate Trigger Mode enable of Group_pri4",
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
                    name: "gp4bichsel",
                    description: Some(
                        "Bifurcate Trigger Channel Select of Group_pri4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch0data",
            extends: None,
            description: Some(
                "Channel 0 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch1data",
            extends: None,
            description: Some(
                "Channel 1 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch2data",
            extends: None,
            description: Some(
                "Channel 2 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch3data",
            extends: None,
            description: Some(
                "Channel 3 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch4data",
            extends: None,
            description: Some(
                "Channel 4 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch5data",
            extends: None,
            description: Some(
                "Channel 5 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ch6data",
            extends: None,
            description: Some(
                "Channel 6 data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "chx_data",
                    description: Some(
                        "Channel 15 data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Chpri0",
            extends: None,
            description: Some(
                "Channel priority register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pri0",
                    description: Some(
                        "The zeroth priority of the selected channel",
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
                    name: "pri1",
                    description: Some(
                        "The 1st priority of the selected channel",
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
                    name: "pri2",
                    description: Some(
                        "The 2nd priority of the selected channel",
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
                    name: "pri3",
                    description: Some(
                        "The 3th priority of the selected channel",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pri4",
                    description: Some(
                        "The 4th priority of the selected channel",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pri5",
                    description: Some(
                        "The 5th priority of the selected channel",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pri6",
                    description: Some(
                        "The 6th priority of the selected channel",
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
            ],
        },
        FieldSet {
            name: "Chsel0",
            extends: None,
            description: Some(
                "Channel selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gp1ch0",
                    description: Some(
                        "Select channel ADCx_IN00 in Group_pri1",
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
                    name: "gp1ch1",
                    description: Some(
                        "Select channel ADCx_IN01 in Group_pri1",
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
                    name: "gp1ch2",
                    description: Some(
                        "Select channel ADCx_IN02 in Group_pri1",
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
                    name: "gp1ch3",
                    description: Some(
                        "Select channel ADCx_IN03 in Group_pri1",
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
                    name: "gp1ch4",
                    description: Some(
                        "Select channel ADCx_IN04 in Group_pri1",
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
                    name: "gp1ch5",
                    description: Some(
                        "Select channel ADCx_IN05 in Group_pri1",
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
                    name: "gp1ch6",
                    description: Some(
                        "Select channel ADCx_IN06 in Group_pri1",
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
                    name: "gp2ch0",
                    description: Some(
                        "Select channel ADCx_IN00 in Group_pri2",
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
                    name: "gp2ch1",
                    description: Some(
                        "Select channel ADCx_IN01 in Group_pri2",
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
                    name: "gp2ch2",
                    description: Some(
                        "Select channel ADCx_IN02 in Group_pri2",
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
                    name: "gp2ch3",
                    description: Some(
                        "Select channel ADCx_IN03 in Group_pri2",
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
                    name: "gp2ch4",
                    description: Some(
                        "Select channel ADCx_IN04 in Group_pri2",
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
                    name: "gp2ch5",
                    description: Some(
                        "Select channel ADCx_IN05 in Group_pri2",
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
                    name: "gp2ch6",
                    description: Some(
                        "Select channel ADCx_IN06 in Group_pri2",
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
            ],
        },
        FieldSet {
            name: "Chsel1",
            extends: None,
            description: Some(
                "Channel selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gp3ch0",
                    description: Some(
                        "Select channel ADCx_IN00 in Group_pri3",
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
                    name: "gp3ch1",
                    description: Some(
                        "Select channel ADCx_IN01 in Group_pri3",
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
                    name: "gp3ch2",
                    description: Some(
                        "Select channel ADCx_IN02 in Group_pri3",
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
                    name: "gp3ch3",
                    description: Some(
                        "Select channel ADCx_IN03 in Group_pri3",
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
                    name: "gp3ch4",
                    description: Some(
                        "Select channel ADCx_IN04 in Group_pri3",
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
                    name: "gp3ch5",
                    description: Some(
                        "Select channel ADCx_IN05 in Group_pri3",
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
                    name: "gp3ch6",
                    description: Some(
                        "Select channel ADCx_IN06 in Group_pri3",
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
                    name: "gp4ch0",
                    description: Some(
                        "Select channel ADCx_IN00 in Group_pri4",
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
                    name: "gp4ch1",
                    description: Some(
                        "Select channel ADCx_IN01 in Group_pri4",
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
                    name: "gp4ch2",
                    description: Some(
                        "Select channel ADCx_IN02 in Group_pri4",
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
                    name: "gp4ch3",
                    description: Some(
                        "Select channel ADCx_IN03 in Group_pri4",
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
                    name: "gp4ch4",
                    description: Some(
                        "Select channel ADCx_IN04 in Group_pri4",
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
                    name: "gp4ch5",
                    description: Some(
                        "Select channel ADCx_IN05 in Group_pri4",
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
                    name: "gp4ch6",
                    description: Some(
                        "Select channel ADCx_IN06 in Group_pri4",
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
                    name: "lpgcnt",
                    description: Some(
                        "Enable lowest-priority group scan continuous",
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
                    name: "gprien",
                    description: Some(
                        "Enable group priority",
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
                    name: "chrsel",
                    description: Some(
                        "Select precharge and discharge",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dal",
                    description: Some(
                        "Data storage alignment mode",
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
                    name: "sdsen",
                    description: Some(
                        "Enable self-diagnosis",
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
                    name: "sdsfix",
                    description: Some(
                        "Enable fix mode for self-diagnosis voltage",
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
                    name: "sdvog",
                    description: Some(
                        "Configure fix voltage for self-diagnosis conversion",
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
                    name: "acdata",
                    description: Some(
                        "Enable automatic clearing data registers",
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
                    name: "asdata",
                    description: Some(
                        "Enable automatic setting data registers",
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
                    name: "adden",
                    description: Some(
                        "Addition function enable",
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
                    name: "procf",
                    description: Some(
                        "A/D conversion process flag",
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
                    name: "gscan",
                    description: Some(
                        "Group scan mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "addsel",
                    description: Some(
                        "A/D conversion data addition mode selection",
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
                    name: "gp3en",
                    description: Some(
                        "Enables A/D conversion operation for Group_pri4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp4en",
                    description: Some(
                        "Enables A/D conversion operation for Group_pri4",
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
                    name: "evicctl",
                    description: Some(
                        "EVIC Link signal control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swend",
                    description: Some(
                        "Software start end of Group_pri1/pri2/pri3",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adcon",
                    description: Some(
                        "ADC ON",
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
                    name: "dres",
                    description: Some(
                        "ADC resolution",
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
            name: "Dmactl",
            extends: None,
            description: Some(
                "DMA control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gp1ovrie",
                    description: Some(
                        "DMA overrun detect interrupt enable of Group_pri1",
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
                    name: "gp1ovren",
                    description: Some(
                        "DMA overrun detect enable of Group_pri1",
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
                    name: "gp1ovrf",
                    description: Some(
                        "DMA overrun flag of Group_pri1",
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
                    name: "gp1dds",
                    description: Some(
                        "DMA disable mode of Group_pri1",
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
                    name: "gp1dmaen",
                    description: Some(
                        "DMA request enable of Group_pri1",
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
                    name: "gp2ovrie",
                    description: Some(
                        "DMA overrun detect interrupt enable of Group_pri2",
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
                    name: "gp2ovren",
                    description: Some(
                        "DMA overrun detect enable of Group_pri2",
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
                    name: "gp2ovrf",
                    description: Some(
                        "DMA overrun flag of Group_pri2",
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
                    name: "gp2dds",
                    description: Some(
                        "DMA disable mode of Group_pri2",
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
                    name: "gp2dmaen",
                    description: Some(
                        "DMA request enable of Group_pri2",
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
                    name: "gp3ovrie",
                    description: Some(
                        "DMA overrun detect interrupt enable of Group_pri3",
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
                    name: "gp3ovren",
                    description: Some(
                        "DMA overrun detect enable of Group_pri3",
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
                    name: "gp3ovrf",
                    description: Some(
                        "DMA overrun flag of Group_pri3",
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
                    name: "gp3dds",
                    description: Some(
                        "DMA disable mode of Group_pri3",
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
                    name: "gp3dmaen",
                    description: Some(
                        "DMA request enable of Group_pri3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp4ovrie",
                    description: Some(
                        "DMA overrun detect interrupt enable of Group_pri4",
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
                    name: "gp4ovren",
                    description: Some(
                        "DMA overrun detect enable of Group_pri4",
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
                    name: "gp4ovrf",
                    description: Some(
                        "DMA overrun flag of Group_pri4",
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
                    name: "gp4dds",
                    description: Some(
                        "DMA disable mode of Group_pri4",
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
                    name: "gp4dmaen",
                    description: Some(
                        "DMA request enable of Group_pri4",
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
            name: "Eocctl",
            extends: None,
            description: Some(
                "Software EOC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "evicg1sel",
                    description: Some(
                        "EVIC trigger signal select from Group_pri1",
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
                    name: "gp1swst",
                    description: Some(
                        "Software start flag of Group_pri1",
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
                    name: "eoc1rie",
                    description: Some(
                        "Interrupt enable for EOC1RF",
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
                    name: "eoc1f",
                    description: Some(
                        "End of Group_pri1 conversion flag",
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
                    name: "eoc1rf",
                    description: Some(
                        "End of Group_pri1 conversion round flag",
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
                    name: "eoc1rcnt",
                    description: Some(
                        "End of Group_pri1 conversion round counts",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evicg2sel",
                    description: Some(
                        "EVIC trigger signal select from Group_pri2",
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
                    name: "gp2swst",
                    description: Some(
                        "Software start flag of Group_pri2",
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
                    name: "eoc2rie",
                    description: Some(
                        "Interrupt enable for EOC2RF",
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
                    name: "eoc2f",
                    description: Some(
                        "End of Group_pri2 conversion flag",
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
                    name: "eoc2rf",
                    description: Some(
                        "End of Group_pri2 conversion round flag",
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
                    name: "eoc2rcnt",
                    description: Some(
                        "End of Group_pri2 conversion round counts",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evicg3sel",
                    description: Some(
                        "EVIC trigger signal select from Group_pri3",
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
                    name: "gp3swst",
                    description: Some(
                        "Software start flag of Group_pri3",
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
                    name: "eoc3rie",
                    description: Some(
                        "Interrupt enable for EOC3RF",
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
                    name: "eoc3f",
                    description: Some(
                        "End of Group_pri3 conversion flag",
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
                    name: "eoc3rf",
                    description: Some(
                        "End of Group_pri3 conversion round flag",
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
                    name: "eoc3rcnt",
                    description: Some(
                        "End of Group_pri3 conversion round counts",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "evicg4sel",
                    description: Some(
                        "EVIC trigger signal select from Group_pri4",
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
                    name: "gp4swst",
                    description: Some(
                        "Software start flag of Group_pri4",
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
                    name: "eoc4ie",
                    description: Some(
                        "Software interrupt generated by end of Group_pri4 conversion round control enable",
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
                    name: "eoc4f",
                    description: Some(
                        "End of Group_pri4 conversion flag",
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
                    name: "eoc4rf",
                    description: Some(
                        "End of Group_pri4 conversion round flag",
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
                    name: "eoc4rcnt",
                    description: Some(
                        "End of Group_pri4 conversion round counts",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp1bidata",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp1bidata1",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp1bidata2",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata1",
                    description: Some(
                        "The data by the first trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp1dmar",
            extends: None,
            description: Some(
                "Group_prix data DMA register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata2",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp2bidata",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp2bidata1",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata1",
                    description: Some(
                        "The data by the first trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp2bidata2",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata1",
                    description: Some(
                        "The data by the first trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp2dmar",
            extends: None,
            description: Some(
                "Group_prix data DMA register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata2",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp3bidata",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp3bidata1",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata1",
                    description: Some(
                        "The data by the first trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp3bidata2",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata2",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp3dmar",
            extends: None,
            description: Some(
                "Group_prix data DMA register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata2",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp4bidata",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp4bidata1",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata1",
                    description: Some(
                        "The data by the first trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp4bidata2",
            extends: None,
            description: Some(
                "Group_prix bifurcate data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bidata2",
                    description: Some(
                        "The data by the second trigger in bifurcate trigger mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gp4dmar",
            extends: None,
            description: Some(
                "Group_prix data DMA register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "The data of Group_prix which can be accessed by DMA",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gpcgf0",
            extends: None,
            description: Some(
                "Group config register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "g1trgen",
                    description: Some(
                        "Enable Group_pri1 synchronous or asynchronous trigger",
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
                    name: "gp1trgty",
                    description: Some(
                        "Group_pri1 trigger type select",
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
                    name: "gp1trgsrc",
                    description: Some(
                        "Synchronous trigger source for Group_pri1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp1trged",
                    description: Some(
                        "Trigger edge select of Group_pri1",
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
                    name: "g2trgen",
                    description: Some(
                        "Enable Group_pri2 synchronous or asynchronous trigger",
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
                    name: "gp2trgty",
                    description: Some(
                        "Group_pri2 trigger type select",
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
                    name: "gp2trgsrc",
                    description: Some(
                        "Synchronous trigger source for Group_pri2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp2trged",
                    description: Some(
                        "Trigger edge select of Group_pri2",
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
                    name: "gp2rtch",
                    description: Some(
                        "Group_pri2 Restart Channel Select",
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
                    name: "gp2rten",
                    description: Some(
                        "Group_pri2 Restart Setting",
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
            name: "Gpcgf1",
            extends: None,
            description: Some(
                "Group config register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "g3trgen",
                    description: Some(
                        "Enable Group_pri3 synchronous or asynchronous trigger",
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
                    name: "gp3trgsel",
                    description: Some(
                        "Group_pri3 trigger type select",
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
                    name: "gp3trgsrc",
                    description: Some(
                        "Synchronous trigger source for Group_pri3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp3trged",
                    description: Some(
                        "Trigger edge select of Group_pri3",
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
                    name: "gp3rtch",
                    description: Some(
                        "Group_pri3 Restart Channel Select",
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
                    name: "gp3rten",
                    description: Some(
                        "Group_pri3 Restart Setting",
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
                    name: "g4trgen",
                    description: Some(
                        "Enable Group_pri4 synchronous or asynchronous trigger",
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
                    name: "gp4trgty",
                    description: Some(
                        "Group_pri4 trigger type select",
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
                    name: "gp4trgsrc",
                    description: Some(
                        "Synchronous trigger source for Group_pri4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gp4trged",
                    description: Some(
                        "Trigger edge select of Group_pri4",
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
                    name: "gp4rtch",
                    description: Some(
                        "Group_pri4 Restart Channel Select",
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
                    name: "gp4rten",
                    description: Some(
                        "Group_pri4 Restart Setting",
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
            name: "Sampr0",
            extends: None,
            description: Some(
                "Sample time register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spt00",
                    description: Some(
                        "Channel ADCx_IN00 sampling time",
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
                    name: "spt01",
                    description: Some(
                        "Channel ADCx_IN01 sampling time",
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
                    name: "spt02",
                    description: Some(
                        "Channel ADCx_IN02 sampling time",
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
                    name: "spt03",
                    description: Some(
                        "Channel ADCx_IN03 sampling time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sampr1",
            extends: None,
            description: Some(
                "Sample time register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spt04",
                    description: Some(
                        "Channel ADCx_IN04 sampling time",
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
                    name: "spt05",
                    description: Some(
                        "Channel ADCx_IN05 sampling time",
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
                    name: "spt06",
                    description: Some(
                        "Channel ADCx_IN06 sampling time",
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
            name: "Sddata",
            extends: None,
            description: Some(
                "Self-diagnosis data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sddata",
                    description: Some(
                        "The self-diagnosis data",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Shctl",
            extends: None,
            description: Some(
                "Sample-and-hold control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sh00en",
                    description: Some(
                        "Enable channel ADCx_IN00 sample-and-hold circuit",
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
                    name: "sh01en",
                    description: Some(
                        "Enable channel ADCx_IN01 sample-and-hold circuit",
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
                    name: "sh02en",
                    description: Some(
                        "Enable channel ADCx_IN02 sample-and-hold circuit",
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
                    name: "shmd",
                    description: Some(
                        "Channel-Dedicated Sample-and-Hold Circuit Operating Mode Setting",
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
                    name: "shend",
                    description: Some(
                        "Software end flag of sample-and-hold circuit in Constant sampling mod",
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
                    name: "shstar",
                    description: Some(
                        "Software start flag of sample-and-hold circuit in Constant sampling mod",
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
                    name: "shhtime",
                    description: Some(
                        "The hold time of sample-and-hold circuit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shstime",
                    description: Some(
                        "The sample time of sample-and-hold circuit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdach",
            extends: None,
            description: Some(
                "Watchdog A channel config register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdach0m",
                    description: Some(
                        "Channel ADCx_IN00 watchdog A compare mode",
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
                    name: "wdach1m",
                    description: Some(
                        "Channel ADCx_IN01 watchdog A compare mode",
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
                    name: "wdach2m",
                    description: Some(
                        "Channel ADCx_IN02 watchdog A compare mode",
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
                    name: "wdach3m",
                    description: Some(
                        "Channel ADCx_IN03 watchdog A compare mode",
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
                    name: "wdach4m",
                    description: Some(
                        "Channel ADCx_IN04 watchdog A compare mode",
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
                    name: "wdach5m",
                    description: Some(
                        "Channel ADCx_IN05 watchdog A compare mode",
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
                    name: "wdach6m",
                    description: Some(
                        "Channel ADCx_IN06 watchdog A compare mode",
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
                    name: "wdach0",
                    description: Some(
                        "Select ADCx_IN0 for watchdog A",
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
                    name: "wdach1",
                    description: Some(
                        "Select ADCx_IN1 for watchdog A",
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
                    name: "wdach2",
                    description: Some(
                        "Select ADCx_IN2 for watchdog A",
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
                    name: "wdach3",
                    description: Some(
                        "Select ADCx_IN3 for watchdog A",
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
                    name: "wdach4",
                    description: Some(
                        "Select ADCx_IN4 for watchdog A",
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
                    name: "wdach5",
                    description: Some(
                        "Select ADCx_IN5 for watchdog A",
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
                    name: "wdach6",
                    description: Some(
                        "Select ADCx_IN6 for watchdog A",
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
            ],
        },
        FieldSet {
            name: "Wdathold",
            extends: None,
            description: Some(
                "Watchdog A threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdalt",
                    description: Some(
                        "Low threshold for analog watchdog A",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdaht",
                    description: Some(
                        "High threshold for analog watchdog A",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdbthold",
            extends: None,
            description: Some(
                "Watchdog B threshold register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdblt",
                    description: Some(
                        "Low threshold for analog watchdog B",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdbht",
                    description: Some(
                        "High threshold for analog watchdog B",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wdctl",
            extends: None,
            description: Some(
                "Watchdog control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wdaen",
                    description: Some(
                        "Watchdog A enable",
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
                    name: "wdaie",
                    description: Some(
                        "Interrupt enable for watchdog A",
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
                    name: "wdben",
                    description: Some(
                        "Watchdog B enable",
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
                    name: "wdbie",
                    description: Some(
                        "Interrupt enable for watchdog B",
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
                    name: "wdbcm",
                    description: Some(
                        "Watchdog B compare mode",
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
                    name: "winen",
                    description: Some(
                        "Window mode enable",
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
                    name: "wdabcc",
                    description: Some(
                        "Watchdog A/B complex conditions configuration",
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
                    name: "wdae",
                    description: Some(
                        "Analog watchdog A event flag",
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
                    name: "wdbe",
                    description: Some(
                        "Analog watchdog B event flag",
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
                    name: "wdamf",
                    description: Some(
                        "Analog watchdog A compare monitor flag",
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
                    name: "wdbmf",
                    description: Some(
                        "Analog watchdog B compare monitor flag",
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
                    name: "wdabmf",
                    description: Some(
                        "Analog watchdog A/B complex compare monitor flag",
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
                    name: "wdbchsel",
                    description: Some(
                        "Analog watchdog B channel select",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WdgaChanStr",
            extends: None,
            description: Some(
                "Window A compare channel status register, ADC_WDGA_CHAN_STR",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch0cmpf",
                    description: Some(
                        "Channel ADCx_IN00 Window A compare status",
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
                    name: "ch1cmpf",
                    description: Some(
                        "Channel ADCx_IN01 Window A compare status",
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
                    name: "ch2cmpf",
                    description: Some(
                        "Channel ADCx_IN02 Window A compare status",
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
                    name: "ch3cmpf",
                    description: Some(
                        "Channel ADCx_IN03 Window A compare status",
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
                    name: "ch4cmpf",
                    description: Some(
                        "Channel ADCx_IN04 Window A compare status",
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
                    name: "ch5cmpf",
                    description: Some(
                        "Channel ADCx_IN05 Window A compare status",
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
                    name: "ch6cmpf",
                    description: Some(
                        "Channel ADCx_IN06 Window A compare status",
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
                    name: "ch7cmpf",
                    description: Some(
                        "Channel ADCx_IN07 Window A compare status",
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
                    name: "ch8cmpf",
                    description: Some(
                        "Channel ADCX_IN08 Window A compare status",
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
                    name: "ch9cmpf",
                    description: Some(
                        "Channel ADCx_IN09 Window A compare status",
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
                    name: "tempcmpf",
                    description: Some(
                        "Temperature sensor channel Window A compare status",
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
                    name: "vintcmpf",
                    description: Some(
                        "Internal reference voltage channel Window A compare status",
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
    ],
    enums: &[],
};
                