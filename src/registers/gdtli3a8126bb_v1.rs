
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tli",
            extends: None,
            description: Some(
                "TFT-LCD interface",
            ),
            items: &[
                BlockItem {
                    name: "spsz",
                    description: Some(
                        "Synchronous pulse size register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Spsz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bpsz",
                    description: Some(
                        "Back-porch size register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bpsz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "asz",
                    description: Some(
                        "Active size register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Asz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tsz",
                    description: Some(
                        "Total size register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tsz",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
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
                    name: "rl",
                    description: Some(
                        "Reload layer register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bgc",
                    description: Some(
                        "Background color register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bgc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intf",
                    description: Some(
                        "Interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intc",
                    description: Some(
                        "Interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lm",
                    description: Some(
                        "Line mark register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cppos",
                    description: Some(
                        "Current pixel position register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cppos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0ctl",
                    description: Some(
                        "Layer 0 control register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0hpos",
                    description: Some(
                        "Layer 0 horizontal position parameters register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0hpos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0vpos",
                    description: Some(
                        "Layer 0 vertical position parameters register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0vpos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0ckey",
                    description: Some(
                        "Layer 0 color key register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0ckey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0ppf",
                    description: Some(
                        "Layer 0 packeted pixel format register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0ppf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0sa",
                    description: Some(
                        "Layer 0 specified alpha register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0sa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0dc",
                    description: Some(
                        "Layer 0 default color register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0dc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0blend",
                    description: Some(
                        "Layer 0 blending register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0blend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0fbaddr",
                    description: Some(
                        "Layer 0 frame base address register",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0fbaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0fllen",
                    description: Some(
                        "Layer 0 frame line length register",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0fllen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0ftln",
                    description: Some(
                        "Layer 0 frame total line number register",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0ftln",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l0lut",
                    description: Some(
                        "Layer 0 look up table register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L0lut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1ctl",
                    description: Some(
                        "Layer 1 control register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1hpos",
                    description: Some(
                        "Layer 1 horizontal position parameters register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1hpos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1vpos",
                    description: Some(
                        "Layer 1 vertical position parameters register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1vpos",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1ckey",
                    description: Some(
                        "Layer 1 color key register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1ckey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1ppf",
                    description: Some(
                        "Layer 1 packeted pixel format register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1ppf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1sa",
                    description: Some(
                        "Layer 1 specified alpha register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1sa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1dc",
                    description: Some(
                        "Layer 1 default color register",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1dc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1blend",
                    description: Some(
                        "Layer 1 blending register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1blend",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1fbaddr",
                    description: Some(
                        "Layer 1 frame base address register",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1fbaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1fllen",
                    description: Some(
                        "Layer 1 frame line length register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1fllen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1ftln",
                    description: Some(
                        "Layer 1 frame total line number register",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1ftln",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "l1lut",
                    description: Some(
                        "Layer 1 look up table register",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "L1lut",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Asz",
            extends: None,
            description: Some(
                "Active size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vasz",
                    description: Some(
                        "Size of the vertical active area width plus back porch and synchronous pulse",
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
                    name: "hasz",
                    description: Some(
                        "Size of the horizontal active area width plus back porch and synchronous pulse",
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
            name: "Bgc",
            extends: None,
            description: Some(
                "Background color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bvb",
                    description: Some(
                        "Background value blue",
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
                    name: "bvg",
                    description: Some(
                        "Background value green",
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
                    name: "bvr",
                    description: Some(
                        "Background value red",
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
            name: "Bpsz",
            extends: None,
            description: Some(
                "Back-porch size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbpsz",
                    description: Some(
                        "Size of the vertical back porch plus synchronous pulse",
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
                    name: "hbpsz",
                    description: Some(
                        "Size of the horizontal back porch plus synchronous pulse",
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
            name: "Cppos",
            extends: None,
            description: Some(
                "Current pixel position register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpos",
                    description: Some(
                        "Vertical position",
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
                    name: "hpos",
                    description: Some(
                        "Horizontal position",
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
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlien",
                    description: Some(
                        "TLI enable bit",
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
                    name: "bdb",
                    description: Some(
                        "Blue channel Dither Bits Number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gdb",
                    description: Some(
                        "Green channel Dither Bits Number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb",
                    description: Some(
                        "Red channel Dither Bits Number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfen",
                    description: Some(
                        "Dither Function Enable",
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
                    name: "clkps",
                    description: Some(
                        "Pixel Clock Polarity Selection",
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
                    name: "deps",
                    description: Some(
                        "Data Enable Polarity Selection",
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
                    name: "vpps",
                    description: Some(
                        "Vertical Pulse Polarity Selection",
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
                    name: "hpps",
                    description: Some(
                        "Horizontal Pulse Polarity Selection",
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
            name: "Intc",
            extends: None,
            description: Some(
                "Interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lmc",
                    description: Some(
                        "Line Mark Flag Clear",
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
                    name: "fec",
                    description: Some(
                        "FIFO Error Flag Clear",
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
                    name: "tec",
                    description: Some(
                        "Transaction Error Flag Clear",
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
                    name: "lcrc",
                    description: Some(
                        "Layer Configuration Reloaded Flag Clear",
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
            name: "Inten",
            extends: None,
            description: Some(
                "Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lmie",
                    description: Some(
                        "Line Mark Interrupt Enable",
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
                    name: "feie",
                    description: Some(
                        "FIFO Error Interrupt Enable",
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
                    name: "teie",
                    description: Some(
                        "Transaction Error Interrupt Enable",
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
                    name: "lcrie",
                    description: Some(
                        "Layer Configuration Reloaded Interrupt Enable",
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
            name: "Intf",
            extends: None,
            description: Some(
                "Interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lmf",
                    description: Some(
                        "Line Mark Flag",
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
                    name: "fef",
                    description: Some(
                        "FIFO Error Flag",
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
                    name: "tef",
                    description: Some(
                        "Transaction Error Flag",
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
                    name: "lcrf",
                    description: Some(
                        "Layer Configuration Reloaded Flag",
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
            name: "L0blend",
            extends: None,
            description: Some(
                "Layer 0 blending register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acf2",
                    description: Some(
                        "Alpha Calculation Factor 2 of Blending Method",
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
                    name: "acf1",
                    description: Some(
                        "Alpha Calculation Factor 1 of Blending Method",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L0ckey",
            extends: None,
            description: Some(
                "Layer 0 color key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckeyb",
                    description: Some(
                        "Color Key Blue",
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
                    name: "ckeyg",
                    description: Some(
                        "Color Key Green",
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
                    name: "ckeyr",
                    description: Some(
                        "Color Key Red",
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
            name: "L0ctl",
            extends: None,
            description: Some(
                "Layer 0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Layer enable",
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
                    name: "ckeyen",
                    description: Some(
                        "Color keying enable",
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
                    name: "luten",
                    description: Some(
                        "LUT enable",
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
            ],
        },
        FieldSet {
            name: "L0dc",
            extends: None,
            description: Some(
                "Layer 0 default color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcb",
                    description: Some(
                        "The default color blue",
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
                    name: "dcg",
                    description: Some(
                        "The default color green",
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
                    name: "dcr",
                    description: Some(
                        "The default color red",
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
                    name: "dca",
                    description: Some(
                        "The default color ALPHA",
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
            name: "L0fbaddr",
            extends: None,
            description: Some(
                "Layer 0 frame base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fbadd",
                    description: Some(
                        "Frame Buffer base Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L0fllen",
            extends: None,
            description: Some(
                "Layer 0 frame line length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fll",
                    description: Some(
                        "Frame Line Length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stdoff",
                    description: Some(
                        "Frame Buffer Stride Offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L0ftln",
            extends: None,
            description: Some(
                "Layer 0 frame total line number register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftln",
                    description: Some(
                        "Frame Total Line Number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L0hpos",
            extends: None,
            description: Some(
                "Layer 0 horizontal position parameters register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wlp",
                    description: Some(
                        "Window left position",
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
                    name: "wrp",
                    description: Some(
                        "Window right position",
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
            name: "L0lut",
            extends: None,
            description: Some(
                "Layer 0 look up table register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tb",
                    description: Some(
                        "Blue channel of a LUT entry",
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
                    name: "tg",
                    description: Some(
                        "Green channel of a LUT entry",
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
                    name: "tr",
                    description: Some(
                        "Red Channel of a LUT entry",
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
                    name: "tadd",
                    description: Some(
                        "Look Up Table Write Address",
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
            name: "L0ppf",
            extends: None,
            description: Some(
                "Layer 0 packeted pixel format register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppf",
                    description: Some(
                        "Packeted Pixel Format",
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
            ],
        },
        FieldSet {
            name: "L0sa",
            extends: None,
            description: Some(
                "Layer 0 specified alpha register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa",
                    description: Some(
                        "Specified alpha",
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
            name: "L0vpos",
            extends: None,
            description: Some(
                "Layer 0 vertical position parameters register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wtp",
                    description: Some(
                        "Window top position",
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
                    name: "wbp",
                    description: Some(
                        "Window bottom position",
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
            name: "L1blend",
            extends: None,
            description: Some(
                "Layer 1 blending register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "acf2",
                    description: Some(
                        "Alpha Calculation Factor 2 of Blending Method",
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
                    name: "acf1",
                    description: Some(
                        "Alpha Calculation Factor 1 of Blending Method",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L1ckey",
            extends: None,
            description: Some(
                "Layer 1 color key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckeyb",
                    description: Some(
                        "Color Key Blue",
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
                    name: "ckeyg",
                    description: Some(
                        "Color Key Green",
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
                    name: "ckeyr",
                    description: Some(
                        "Color Key Red",
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
            name: "L1ctl",
            extends: None,
            description: Some(
                "Layer 1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Layer enable",
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
                    name: "ckeyen",
                    description: Some(
                        "Color keying enable",
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
                    name: "luten",
                    description: Some(
                        "LUT enable",
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
            ],
        },
        FieldSet {
            name: "L1dc",
            extends: None,
            description: Some(
                "Layer 1 default color register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcb",
                    description: Some(
                        "The default color blue",
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
                    name: "dcg",
                    description: Some(
                        "The default color green",
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
                    name: "dcr",
                    description: Some(
                        "The default color red",
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
                    name: "dca",
                    description: Some(
                        "The default color ALPHA",
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
            name: "L1fbaddr",
            extends: None,
            description: Some(
                "Layer 1 frame base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fbadd",
                    description: Some(
                        "Frame Buffer base Address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L1fllen",
            extends: None,
            description: Some(
                "Layer 1 frame line length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fll",
                    description: Some(
                        "Frame Line Length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stdoff",
                    description: Some(
                        "Frame Buffer Stride Offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L1ftln",
            extends: None,
            description: Some(
                "Layer 1 frame total line number register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftln",
                    description: Some(
                        "Frame Total Line Number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "L1hpos",
            extends: None,
            description: Some(
                "Layer 1 horizontal position parameters register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wlp",
                    description: Some(
                        "Window left position",
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
                    name: "wrp",
                    description: Some(
                        "Window right position",
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
            name: "L1lut",
            extends: None,
            description: Some(
                "Layer 1 look up table register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tb",
                    description: Some(
                        "Blue channel of a LUT entry",
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
                    name: "tg",
                    description: Some(
                        "Green channel of a LUT entry",
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
                    name: "tr",
                    description: Some(
                        "Red channel of a LUT entry",
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
                    name: "tadd",
                    description: Some(
                        "Look Up Table Write Address",
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
            name: "L1ppf",
            extends: None,
            description: Some(
                "Layer 1 packeted pixel format register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppf",
                    description: Some(
                        "Packeted Pixel Format",
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
            ],
        },
        FieldSet {
            name: "L1sa",
            extends: None,
            description: Some(
                "Layer 1 specified alpha register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sa",
                    description: Some(
                        "Specified alpha",
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
            name: "L1vpos",
            extends: None,
            description: Some(
                "Layer 1 vertical position parameters register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wtp",
                    description: Some(
                        "Window top position",
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
                    name: "wbp",
                    description: Some(
                        "Window bottom position",
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
            name: "Lm",
            extends: None,
            description: Some(
                "Line mark register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lm",
                    description: Some(
                        "Line Mark value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rl",
            extends: None,
            description: Some(
                "Reload layer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rqr",
                    description: Some(
                        "Request Reload",
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
                    name: "fbr",
                    description: Some(
                        "Frame Blank Reload",
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
            name: "Spsz",
            extends: None,
            description: Some(
                "Synchronous pulse size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpsz",
                    description: Some(
                        "size of vertical synchronous pluse",
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
                    name: "hpsz",
                    description: Some(
                        "size of horizontal synchronous pluse",
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
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vde",
                    description: Some(
                        "Current VDE status",
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
                    name: "hde",
                    description: Some(
                        "Current HDE status",
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
                    name: "vs",
                    description: Some(
                        "Current VS staus of the TLI",
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
                    name: "hs",
                    description: Some(
                        "Current HS staus of the TLI",
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
            name: "Tsz",
            extends: None,
            description: Some(
                "Total size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vtsz",
                    description: Some(
                        "Vertical total size of the display",
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
                    name: "htsz",
                    description: Some(
                        "Horizontal total size of the display",
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
    ],
    enums: &[],
};
