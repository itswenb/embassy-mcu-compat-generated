
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dmamux",
            extends: None,
            description: Some(
                "DMAMUX controller",
            ),
            items: &[
                BlockItem {
                    name: "rm_ch0cfg",
                    description: Some(
                        "Request multiplexer channel x configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RmCh0cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rm_ch1cfg",
                    description: Some(
                        "Request multiplexer channel x configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RmCh1cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rm_ch2cfg",
                    description: Some(
                        "Request multiplexer channel x configuration register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RmCh2cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rm_intf",
                    description: Some(
                        "Request multiplexer channel interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RmIntf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rm_intc",
                    description: Some(
                        "Request multiplexer channel interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "RmIntc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_ch0cfg",
                    description: Some(
                        "Request generator channel x configuration register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RgCh0cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_ch1cfg",
                    description: Some(
                        "Request generator channel 1 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RgCh1cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_ch2cfg",
                    description: Some(
                        "Request generator channel 2 configuration register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RgCh2cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_ch3cfg",
                    description: Some(
                        "Request generator channel x configuration register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RgCh3cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_intf",
                    description: Some(
                        "Request generator interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RgIntf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rg_intc",
                    description: Some(
                        "Rquest generator interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "RgIntc",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "RgCh0cfg",
            extends: None,
            description: Some(
                "Request generator channel x configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tid",
                    description: Some(
                        "Trigger input identification",
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
                    name: "toie",
                    description: Some(
                        "Trigger overrun interrupt enable",
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
                    name: "rgen",
                    description: Some(
                        "DMAMUX request generator channel x enable",
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
                    name: "rgtp",
                    description: Some(
                        "DMA request generator trigger polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbrg",
                    description: Some(
                        "Number of DMA requests to be generated",
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
            ],
        },
        FieldSet {
            name: "RgCh1cfg",
            extends: None,
            description: Some(
                "Request generator channel 1 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tid",
                    description: Some(
                        "Trigger input identification",
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
                    name: "toie",
                    description: Some(
                        "Trigger overrun interrupt enable",
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
                    name: "rgtp",
                    description: Some(
                        "DMA request generator trigger polarity",
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
                    name: "nbrg",
                    description: Some(
                        "Number of DMA requests to be generated",
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
            ],
        },
        FieldSet {
            name: "RgCh2cfg",
            extends: None,
            description: Some(
                "Request generator channel 2 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tid",
                    description: Some(
                        "Trigger input identification",
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
                    name: "toie",
                    description: Some(
                        "Trigger overrun interrupt enable",
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
                    name: "rgtp",
                    description: Some(
                        "DMA request generator trigger polarity",
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
                    name: "nbrg",
                    description: Some(
                        "Number of DMA requests to be generated",
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
            ],
        },
        FieldSet {
            name: "RgCh3cfg",
            extends: None,
            description: Some(
                "Request generator channel x configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tid",
                    description: Some(
                        "Trigger input identification",
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
                    name: "toie",
                    description: Some(
                        "Trigger overrun interrupt enable",
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
                    name: "rgen",
                    description: Some(
                        "DMAMUX request generator channel x enable",
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
                    name: "rgtp",
                    description: Some(
                        "DMA request generator trigger polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbrg",
                    description: Some(
                        "Number of DMA requests to be generated",
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
            ],
        },
        FieldSet {
            name: "RgIntc",
            extends: None,
            description: Some(
                "Rquest generator interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toifc0",
                    description: Some(
                        "Clear bit for trigger overrun event flag of request generator channel 0",
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
                    name: "toifc1",
                    description: Some(
                        "Clear bit for trigger overrun event flag of request generator channel 1",
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
                    name: "toifc2",
                    description: Some(
                        "Clear bit for trigger overrun event flag of request generator channel 2",
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
                    name: "toifc3",
                    description: Some(
                        "Clear bit for trigger overrun event flag of request generator channel 3",
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
            name: "RgIntf",
            extends: None,
            description: Some(
                "Request generator interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toif0",
                    description: Some(
                        "Trigger overrun event flag of request generator channel 0",
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
                    name: "toif1",
                    description: Some(
                        "Trigger overrun event flag of request generator channel 1",
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
                    name: "toif2",
                    description: Some(
                        "Trigger overrun event flag of request generator channel 2",
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
                    name: "toif3",
                    description: Some(
                        "Trigger overrun event flag of request generator channel 3",
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
            name: "RmCh0cfg",
            extends: None,
            description: Some(
                "Request multiplexer channel x configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "muxinid",
                    description: Some(
                        "Multiplexer input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "soie",
                    description: Some(
                        "Synchronization overrun interrupt enable",
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
                    name: "evgen",
                    description: Some(
                        "Event generation enable",
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
                    name: "syncen",
                    description: Some(
                        "Synchronization enable",
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
                    name: "syncp",
                    description: Some(
                        "Synchronization input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbr",
                    description: Some(
                        "Number of DMA requests to forward",
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
                    name: "syncid",
                    description: Some(
                        "Synchronization input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RmCh1cfg",
            extends: None,
            description: Some(
                "Request multiplexer channel x configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "muxinid",
                    description: Some(
                        "Multiplexer input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "soie",
                    description: Some(
                        "Synchronization overrun interrupt enable",
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
                    name: "evgen",
                    description: Some(
                        "Event generation enable",
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
                    name: "syncen",
                    description: Some(
                        "Synchronization enable",
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
                    name: "syncp",
                    description: Some(
                        "Synchronization input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbr",
                    description: Some(
                        "Number of DMA requests to forward",
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
                    name: "syncid",
                    description: Some(
                        "Synchronization input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RmCh2cfg",
            extends: None,
            description: Some(
                "Request multiplexer channel x configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "muxinid",
                    description: Some(
                        "Multiplexer input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "soie",
                    description: Some(
                        "Synchronization overrun interrupt enable",
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
                    name: "evgen",
                    description: Some(
                        "Event generation enable",
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
                    name: "syncen",
                    description: Some(
                        "Synchronization enable",
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
                    name: "syncp",
                    description: Some(
                        "Synchronization input polarity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nbr",
                    description: Some(
                        "Number of DMA requests to forward",
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
                    name: "syncid",
                    description: Some(
                        "Synchronization input identification",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RmIntc",
            extends: None,
            description: Some(
                "Request multiplexer channel interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soifc0",
                    description: Some(
                        "Clear bit for synchronization overrun event flag of request multiplexer channel 0",
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
                    name: "soifc1",
                    description: Some(
                        "Clear bit for synchronization overrun event flag of request multiplexer channel 1",
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
                    name: "soifc2",
                    description: Some(
                        "Clear bit for synchronization overrun event flag of request multiplexer channel 2",
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
            ],
        },
        FieldSet {
            name: "RmIntf",
            extends: None,
            description: Some(
                "Request multiplexer channel interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "soif0",
                    description: Some(
                        "Synchronization overrun event flag of request multiplexer channel 0",
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
                    name: "soif1",
                    description: Some(
                        "Synchronization overrun event flag of request multiplexer channel 1",
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
                    name: "soif2",
                    description: Some(
                        "Synchronization overrun event flag of request multiplexer channel 2",
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
            ],
        },
    ],
    enums: &[],
};
                