
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ipa",
            extends: None,
            description: Some(
                "Image processing accelerator",
            ),
            items: &[
                BlockItem {
                    name: "ipa_ctl",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_intf",
                    description: Some(
                        "Interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaIntf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_intc",
                    description: Some(
                        "Interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaIntc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_fmaddr",
                    description: Some(
                        "Foreground memory base address register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaFmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_floff",
                    description: Some(
                        "Foreground line offset register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaFloff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_bmaddr",
                    description: Some(
                        "Background memory base address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaBmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_bloff",
                    description: Some(
                        "Background line offset register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaBloff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_fpctl",
                    description: Some(
                        "Foreground pixel control register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaFpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_fpv",
                    description: Some(
                        "Foreground pixel value register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaFpv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_bpctl",
                    description: Some(
                        "Background pixel control register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaBpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_bpv",
                    description: Some(
                        "Background pixel value register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaBpv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_flmaddr",
                    description: Some(
                        "Foreground LUT memory base address register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaFlmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_blmaddr",
                    description: Some(
                        "Background LUT memory base address register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaBlmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpctl",
                    description: Some(
                        "Destination pixel control register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpv_argb1555",
                    description: Some(
                        "Destination pixel value register(When the destination pixel format is ARGB1555)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpvArgb1555",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpv_argb4444",
                    description: Some(
                        "Destination pixel value register(When the destination pixel format is ARGB4444,)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpvArgb4444",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpv_argb8888",
                    description: Some(
                        "Destination pixel value register(When the destination pixel format is ARGB8888)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpvArgb8888",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpv_rgb565",
                    description: Some(
                        "Destination pixel value register(When the destination pixel format is RGB565)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpvRgb565",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dpv_rgb888",
                    description: Some(
                        "Destination pixel value register(When the destination pixel format is RGB888)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDpvRgb888",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dmaddr",
                    description: Some(
                        "Destination memory base address register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDmaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_dloff",
                    description: Some(
                        "Destination line offset register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaDloff",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_ims",
                    description: Some(
                        "Image size register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaIms",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_lm",
                    description: Some(
                        "Line mark register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaLm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ipa_itctl",
                    description: Some(
                        "Inter-timer control register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IpaItctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "IpaBlmaddr",
            extends: None,
            description: Some(
                "Background LUT memory base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blmaddr",
                    description: Some(
                        "Background LUT memory base address",
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
            name: "IpaBloff",
            extends: None,
            description: Some(
                "Background line offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bloff",
                    description: Some(
                        "Background line offset",
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
            ],
        },
        FieldSet {
            name: "IpaBmaddr",
            extends: None,
            description: Some(
                "Background memory base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmaddr",
                    description: Some(
                        "Background memory base address",
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
            name: "IpaBpctl",
            extends: None,
            description: Some(
                "Background pixel control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpf",
                    description: Some(
                        "Background pixel format",
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
                    name: "blpf",
                    description: Some(
                        "Background LUT pixel format",
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
                    name: "bllen",
                    description: Some(
                        "Background LUT loading enable",
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
                    name: "bcnp",
                    description: Some(
                        "Background LUT number of pixel",
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
                    name: "bavca",
                    description: Some(
                        "Background alpha value calculation algorithm",
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
                    name: "bpdav",
                    description: Some(
                        "Background pre- defined alpha value",
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
            name: "IpaBpv",
            extends: None,
            description: Some(
                "Background pixel value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpdbv",
                    description: Some(
                        "Background pre-defined blue value",
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
                    name: "bpdgv",
                    description: Some(
                        "Background pre-defined green value",
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
                    name: "bpdrv",
                    description: Some(
                        "Background pre-defined red value",
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
            name: "IpaCtl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ten",
                    description: Some(
                        "Transfer enable",
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
                    name: "thu",
                    description: Some(
                        "Transfer hang up",
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
                    name: "tst",
                    description: Some(
                        "Transfer stop",
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
                    name: "taeie",
                    description: Some(
                        "Enable bit for transfer access error interrupt",
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
                    name: "ftfie",
                    description: Some(
                        "Enable bit for full transfer finish interrupt",
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
                    name: "tlmie",
                    description: Some(
                        "Enable bit for transfer line mark interrupt",
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
                    name: "lacie",
                    description: Some(
                        "Enable bit for LUT access conflict interrupt",
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
                    name: "llfie",
                    description: Some(
                        "Enable bit for LUT loading finish interrupt",
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
                    name: "wcfie",
                    description: Some(
                        "Enable bit for wrong configuration interrupt",
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
                    name: "pfcm",
                    description: Some(
                        "Pixel format convert mode",
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
            ],
        },
        FieldSet {
            name: "IpaDloff",
            extends: None,
            description: Some(
                "Destination line offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dloff",
                    description: Some(
                        "Destination line offset",
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
            ],
        },
        FieldSet {
            name: "IpaDmaddr",
            extends: None,
            description: Some(
                "Destination memory base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaddr",
                    description: Some(
                        "Destination memory base address",
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
            name: "IpaDpctl",
            extends: None,
            description: Some(
                "Destination pixel control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpf",
                    description: Some(
                        "Destination pixel format",
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
            name: "IpaDpvArgb1555",
            extends: None,
            description: Some(
                "Destination pixel value register(When the destination pixel format is ARGB1555)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpdbv",
                    description: Some(
                        "Destination pre-defined blue value",
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
                    name: "dpdgv",
                    description: Some(
                        "Destination pre-defined green value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpdrv",
                    description: Some(
                        "Destination pre-defined red value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpdav",
                    description: Some(
                        "Destination pre-defined alpha value",
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
            name: "IpaDpvArgb4444",
            extends: None,
            description: Some(
                "Destination pixel value register(When the destination pixel format is ARGB4444,)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpdbv",
                    description: Some(
                        "Destination pre-defined blue value",
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
                    name: "dpdgv",
                    description: Some(
                        "Destination pre-defined green value",
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
                    name: "dpdrv",
                    description: Some(
                        "Destination pre-defined red value",
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
                    name: "dpdav",
                    description: Some(
                        "Destination pre-defined alpha value",
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
            ],
        },
        FieldSet {
            name: "IpaDpvArgb8888",
            extends: None,
            description: Some(
                "Destination pixel value register(When the destination pixel format is ARGB8888)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpdbv",
                    description: Some(
                        "Destination pre-defined blue value",
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
                    name: "dpdgv",
                    description: Some(
                        "Destination pre-defined green value",
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
                    name: "dpdrv",
                    description: Some(
                        "Destination pre-defined red value",
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
                    name: "dpdav",
                    description: Some(
                        "Destination pre-defined alpha value",
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
            name: "IpaDpvRgb565",
            extends: None,
            description: Some(
                "Destination pixel value register(When the destination pixel format is RGB565)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpdbv",
                    description: Some(
                        "Destination pre-defined blue value",
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
                    name: "dpdgv",
                    description: Some(
                        "Destination pre-defined green value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpdrv",
                    description: Some(
                        "Destination pre-defined red value",
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
            ],
        },
        FieldSet {
            name: "IpaDpvRgb888",
            extends: None,
            description: Some(
                "Destination pixel value register(When the destination pixel format is RGB888)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpdbv",
                    description: Some(
                        "Destination pre-defined blue value",
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
                    name: "dpdgv",
                    description: Some(
                        "Destination pre-defined green value",
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
                    name: "dpdrv",
                    description: Some(
                        "Destination pre-defined red value",
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
            name: "IpaFlmaddr",
            extends: None,
            description: Some(
                "Foreground LUT memory base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flmaddr",
                    description: Some(
                        "Foreground LUT memory base address",
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
            name: "IpaFloff",
            extends: None,
            description: Some(
                "Foreground line offset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "floff",
                    description: Some(
                        "Foreground line offset",
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
            ],
        },
        FieldSet {
            name: "IpaFmaddr",
            extends: None,
            description: Some(
                "Foreground memory base address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmaddr",
                    description: Some(
                        "Foreground memory base address",
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
            name: "IpaFpctl",
            extends: None,
            description: Some(
                "Foreground pixel control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpf",
                    description: Some(
                        "Foreground pixel format",
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
                    name: "flpf",
                    description: Some(
                        "Foreground LUT pixel format",
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
                    name: "fllen",
                    description: Some(
                        "Foreground LUT loading enable",
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
                    name: "fcnp",
                    description: Some(
                        "Foreground LUT number of pixel",
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
                    name: "favca",
                    description: Some(
                        "Foreground alpha value calculation algorithm",
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
                    name: "fpdav",
                    description: Some(
                        "Foreground pre- defined alpha value",
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
            name: "IpaFpv",
            extends: None,
            description: Some(
                "Foreground pixel value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fpdbv",
                    description: Some(
                        "Foreground pre-defined blue value",
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
                    name: "fpdgv",
                    description: Some(
                        "Foreground pre-defined green value",
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
                    name: "fpdrv",
                    description: Some(
                        "Foreground pre-defined red value",
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
            name: "IpaIms",
            extends: None,
            description: Some(
                "Image size register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "height",
                    description: Some(
                        "Height of the image to be processed",
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
                    name: "width",
                    description: Some(
                        "Width of the image to be processed",
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
            name: "IpaIntc",
            extends: None,
            description: Some(
                "Interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "taeifc",
                    description: Some(
                        "Clear bit for transfer access error interrupt flag",
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
                    name: "ftfifc",
                    description: Some(
                        "Clear bit for full transfer finish interrupt flag",
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
                    name: "tlmifc",
                    description: Some(
                        "Clear bit for transfer line mark interrupt flag",
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
                    name: "lacifc",
                    description: Some(
                        "Clear bit for LUT access conflict interrupt flag",
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
                    name: "llfifc",
                    description: Some(
                        "Clear bit for LUT loading finish interrupt flag",
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
                    name: "wcfifc",
                    description: Some(
                        "Clear bit for wrong configuration interrupt flag",
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
        FieldSet {
            name: "IpaIntf",
            extends: None,
            description: Some(
                "Interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "taeif",
                    description: Some(
                        "Transfer access error interrupt flag",
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
                    name: "ftfif",
                    description: Some(
                        "Full transfer finish interrupt flag",
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
                    name: "tlmif",
                    description: Some(
                        "Transfer line mark interrupt flag",
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
                    name: "lacif",
                    description: Some(
                        "LUT access conflict interrupt flag",
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
                    name: "llfif",
                    description: Some(
                        "LUT loading finish interrupt flag",
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
                    name: "wcfif",
                    description: Some(
                        "Wrong configuration interrupt flag",
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
        FieldSet {
            name: "IpaItctl",
            extends: None,
            description: Some(
                "Inter-timer control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iten",
                    description: Some(
                        "Inter-timer enable",
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
                    name: "ncci",
                    description: Some(
                        "Number of clock cycles interval",
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
            name: "IpaLm",
            extends: None,
            description: Some(
                "Line mark register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lm",
                    description: Some(
                        "line mark",
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
    ],
    enums: &[],
};
                