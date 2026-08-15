
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mfcom",
            extends: None,
            description: Some(
                "Multi-function communication Interface",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
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
                                "Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pindata",
                    description: Some(
                        "Pin data register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pindata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sstat",
                    description: Some(
                        "Shifter status register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "serr",
                    description: Some(
                        "Shifter error register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Serr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmstat",
                    description: Some(
                        "Timer status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ssien",
                    description: Some(
                        "Shifter status interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ssien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "seien",
                    description: Some(
                        "Shifter error interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmsien",
                    description: Some(
                        "Timer status interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmsien",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ssdmaen",
                    description: Some(
                        "Shifter status DMA enable register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ssdmaen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctl0",
                    description: Some(
                        "Shifter control x register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctl01",
                    description: Some(
                        "Shifter control x register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctl01",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctl2",
                    description: Some(
                        "Shifter control x register",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sctl3",
                    description: Some(
                        "Shifter control x register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scfg0",
                    description: Some(
                        "Shifter configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scfg1",
                    description: Some(
                        "Shifter configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scfg2",
                    description: Some(
                        "Shifter configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scfg3",
                    description: Some(
                        "Shifter configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Scfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbuf0",
                    description: Some(
                        "Shifter buffer x register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbuf0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbuf1",
                    description: Some(
                        "Shifter buffer x register",
                    ),
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbuf1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbuf2",
                    description: Some(
                        "Shifter buffer x register",
                    ),
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbuf2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbuf3",
                    description: Some(
                        "Shifter buffer x register",
                    ),
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbuf3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbis0",
                    description: Some(
                        "Shifter buffer x bit swapped register",
                    ),
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbis0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbis1",
                    description: Some(
                        "Shifter buffer x bit swapped register",
                    ),
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbis1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbis2",
                    description: Some(
                        "Shifter buffer x bit swapped register",
                    ),
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbis2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbis3",
                    description: Some(
                        "Shifter buffer x bit swapped register",
                    ),
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbis3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbys0",
                    description: Some(
                        "Shifter buffer x byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbys0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbys1",
                    description: Some(
                        "Shifter buffer x byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbys1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbys2",
                    description: Some(
                        "Shifter buffer x byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbys2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbys3",
                    description: Some(
                        "Shifter buffer x byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x30c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbys3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbbs0",
                    description: Some(
                        "Shifter buffer x bit byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbbs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbbs1",
                    description: Some(
                        "Shifter buffer x bit byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbbs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbbs2",
                    description: Some(
                        "Shifter buffer x bit byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbbs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sbufbbs3",
                    description: Some(
                        "Shifter buffer x bit byte swapped register",
                    ),
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sbufbbs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmctl0",
                    description: Some(
                        "Timer control x register",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmctl1",
                    description: Some(
                        "Timer control x register",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmctl2",
                    description: Some(
                        "Timer control x register",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmctl3",
                    description: Some(
                        "Timer control x register",
                    ),
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmctl3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcfg0",
                    description: Some(
                        "Timer configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x480,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcfg1",
                    description: Some(
                        "Timer configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x484,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcfg2",
                    description: Some(
                        "Timer configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x488,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcfg3",
                    description: Some(
                        "Timer configuration x register",
                    ),
                    array: None,
                    byte_offset: 0x48c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcmp0",
                    description: Some(
                        "Timer compare x register",
                    ),
                    array: None,
                    byte_offset: 0x500,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcmp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcmp1",
                    description: Some(
                        "Timer compare x register",
                    ),
                    array: None,
                    byte_offset: 0x504,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcmp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcmp2",
                    description: Some(
                        "Timer compare x register",
                    ),
                    array: None,
                    byte_offset: 0x508,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcmp2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tmcmp3",
                    description: Some(
                        "Timer compare x register",
                    ),
                    array: None,
                    byte_offset: 0x50c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tmcmp3",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfcomen",
                    description: Some(
                        "MFCOM enable",
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
                    name: "swrsten",
                    description: Some(
                        "Software reset enable",
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
            name: "Pindata",
            extends: None,
            description: Some(
                "Pin data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pdata",
                    description: Some(
                        "Input data of pins",
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
            name: "Sbuf0",
            extends: None,
            description: Some(
                "Shifter buffer x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbuf",
                    description: Some(
                        "Shift buffer",
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
            name: "Sbuf1",
            extends: None,
            description: Some(
                "Shifter buffer x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbuf",
                    description: Some(
                        "Shift buffer",
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
            name: "Sbuf2",
            extends: None,
            description: Some(
                "Shifter buffer x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbuf",
                    description: Some(
                        "Shift buffer",
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
            name: "Sbuf3",
            extends: None,
            description: Some(
                "Shifter buffer x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbuf",
                    description: Some(
                        "Shift buffer",
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
            name: "Sbufbbs0",
            extends: None,
            description: Some(
                "Shifter buffer x bit byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbbs",
                    description: Some(
                        "Shift buffer bit byte swapped",
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
            name: "Sbufbbs1",
            extends: None,
            description: Some(
                "Shifter buffer x bit byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbbs",
                    description: Some(
                        "Shift buffer bit byte swapped",
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
            name: "Sbufbbs2",
            extends: None,
            description: Some(
                "Shifter buffer x bit byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbbs",
                    description: Some(
                        "Shift buffer bit byte swapped",
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
            name: "Sbufbbs3",
            extends: None,
            description: Some(
                "Shifter buffer x bit byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbbs",
                    description: Some(
                        "Shift buffer bit byte swapped",
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
            name: "Sbufbis0",
            extends: None,
            description: Some(
                "Shifter buffer x bit swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbis",
                    description: Some(
                        "Shift buffer bit swapped",
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
            name: "Sbufbis1",
            extends: None,
            description: Some(
                "Shifter buffer x bit swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbis",
                    description: Some(
                        "Shift buffer bit swapped",
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
            name: "Sbufbis2",
            extends: None,
            description: Some(
                "Shifter buffer x bit swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbis",
                    description: Some(
                        "Shift buffer bit swapped",
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
            name: "Sbufbis3",
            extends: None,
            description: Some(
                "Shifter buffer x bit swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbis",
                    description: Some(
                        "Shift buffer bit swapped",
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
            name: "Sbufbys0",
            extends: None,
            description: Some(
                "Shifter buffer x byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbys",
                    description: Some(
                        "Shift buffer byte swapped",
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
            name: "Sbufbys1",
            extends: None,
            description: Some(
                "Shifter buffer x byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbys",
                    description: Some(
                        "Shift buffer byte swapped",
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
            name: "Sbufbys2",
            extends: None,
            description: Some(
                "Shifter buffer x byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbys",
                    description: Some(
                        "Shift buffer byte swapped",
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
            name: "Sbufbys3",
            extends: None,
            description: Some(
                "Shifter buffer x byte swapped register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sbufbys",
                    description: Some(
                        "Shift buffer byte swapped",
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
            name: "Scfg0",
            extends: None,
            description: Some(
                "Shifter configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstart",
                    description: Some(
                        "Shifter start bit",
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
                    name: "sstop",
                    description: Some(
                        "Shifter stop bit",
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
                    name: "insrc",
                    description: Some(
                        "Input source",
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
            ],
        },
        FieldSet {
            name: "Scfg1",
            extends: None,
            description: Some(
                "Shifter configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstart",
                    description: Some(
                        "Shifter start bit",
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
                    name: "sstop",
                    description: Some(
                        "Shifter stop bit",
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
                    name: "insrc",
                    description: Some(
                        "Input source",
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
            ],
        },
        FieldSet {
            name: "Scfg2",
            extends: None,
            description: Some(
                "Shifter configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstart",
                    description: Some(
                        "Shifter start bit",
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
                    name: "sstop",
                    description: Some(
                        "Shifter stop bit",
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
                    name: "insrc",
                    description: Some(
                        "Input source",
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
            ],
        },
        FieldSet {
            name: "Scfg3",
            extends: None,
            description: Some(
                "Shifter configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstart",
                    description: Some(
                        "Shifter start bit",
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
                    name: "sstop",
                    description: Some(
                        "Shifter stop bit",
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
                    name: "insrc",
                    description: Some(
                        "Input source",
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
            ],
        },
        FieldSet {
            name: "Sctl0",
            extends: None,
            description: Some(
                "Shifter control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smod",
                    description: Some(
                        "Shifter mode",
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
                    name: "sppl",
                    description: Some(
                        "Shifter pin polarity",
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
                    name: "spsel",
                    description: Some(
                        "Shifter pin select",
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
                    name: "spcfg",
                    description: Some(
                        "Shifter pin configuration",
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
                    name: "tmpl",
                    description: Some(
                        "Timer polarity",
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
                    name: "tmsel",
                    description: Some(
                        "Timer select",
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
            ],
        },
        FieldSet {
            name: "Sctl01",
            extends: None,
            description: Some(
                "Shifter control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smod",
                    description: Some(
                        "Shifter mode",
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
                    name: "sppl",
                    description: Some(
                        "Shifter pin polarity",
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
                    name: "spsel",
                    description: Some(
                        "Shifter pin select",
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
                    name: "spcfg",
                    description: Some(
                        "Shifter pin configuration",
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
                    name: "tmpl",
                    description: Some(
                        "Timer polarity",
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
                    name: "tmsel",
                    description: Some(
                        "Timer select",
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
            ],
        },
        FieldSet {
            name: "Sctl2",
            extends: None,
            description: Some(
                "Shifter control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smod",
                    description: Some(
                        "Shifter mode",
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
                    name: "sppl",
                    description: Some(
                        "Shifter pin polarity",
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
                    name: "spsel",
                    description: Some(
                        "Shifter pin select",
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
                    name: "spcfg",
                    description: Some(
                        "Shifter pin configuration",
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
                    name: "tmpl",
                    description: Some(
                        "Timer polarity",
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
                    name: "tmsel",
                    description: Some(
                        "Timer select",
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
            ],
        },
        FieldSet {
            name: "Sctl3",
            extends: None,
            description: Some(
                "Shifter control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "smod",
                    description: Some(
                        "Shifter mode",
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
                    name: "sppl",
                    description: Some(
                        "Shifter pin polarity",
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
                    name: "spsel",
                    description: Some(
                        "Shifter pin select",
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
                    name: "spcfg",
                    description: Some(
                        "Shifter pin configuration",
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
                    name: "tmpl",
                    description: Some(
                        "Timer polarity",
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
                    name: "tmsel",
                    description: Some(
                        "Timer select",
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
            ],
        },
        FieldSet {
            name: "Seien",
            extends: None,
            description: Some(
                "Shifter error interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seien",
                    description: Some(
                        "Shifter error interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Serr",
            extends: None,
            description: Some(
                "Shifter error register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "serr",
                    description: Some(
                        "Shifter x error flags",
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
            ],
        },
        FieldSet {
            name: "Ssdmaen",
            extends: None,
            description: Some(
                "Shifter status DMA enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ssdmaen",
                    description: Some(
                        "Shifter status DMA enable",
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
            ],
        },
        FieldSet {
            name: "Ssien",
            extends: None,
            description: Some(
                "Shifter status interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ssien",
                    description: Some(
                        "Shifter status interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Sstat",
            extends: None,
            description: Some(
                "Shifter status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sstat",
                    description: Some(
                        "Shifter x status flag",
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
            ],
        },
        FieldSet {
            name: "Tmcfg0",
            extends: None,
            description: Some(
                "Timer configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmstart",
                    description: Some(
                        "Timer start bit",
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
                    name: "tmstop",
                    description: Some(
                        "Timer stop bit",
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
                    name: "tmen",
                    description: Some(
                        "Timer enable",
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
                    name: "tmdis",
                    description: Some(
                        "Timer disable",
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
                    name: "tmrst",
                    description: Some(
                        "Timer reset",
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
                Field {
                    name: "tmdec",
                    description: Some(
                        "Timer decrement",
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
                    name: "tmout",
                    description: Some(
                        "Timer output",
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
            ],
        },
        FieldSet {
            name: "Tmcfg1",
            extends: None,
            description: Some(
                "Timer configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmstart",
                    description: Some(
                        "Timer start bit",
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
                    name: "tmstop",
                    description: Some(
                        "Timer stop bit",
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
                    name: "tmen",
                    description: Some(
                        "Timer enable",
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
                    name: "tmdis",
                    description: Some(
                        "Timer disable",
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
                    name: "tmrst",
                    description: Some(
                        "Timer reset",
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
                Field {
                    name: "tmdec",
                    description: Some(
                        "Timer decrement",
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
                    name: "tmout",
                    description: Some(
                        "Timer output",
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
            ],
        },
        FieldSet {
            name: "Tmcfg2",
            extends: None,
            description: Some(
                "Timer configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmstart",
                    description: Some(
                        "Timer start bit",
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
                    name: "tmstop",
                    description: Some(
                        "Timer stop bit",
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
                    name: "tmen",
                    description: Some(
                        "Timer enable",
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
                    name: "tmdis",
                    description: Some(
                        "Timer disable",
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
                    name: "tmrst",
                    description: Some(
                        "Timer reset",
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
                Field {
                    name: "tmdec",
                    description: Some(
                        "Timer decrement",
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
                    name: "tmout",
                    description: Some(
                        "Timer output",
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
            ],
        },
        FieldSet {
            name: "Tmcfg3",
            extends: None,
            description: Some(
                "Timer configuration x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmstart",
                    description: Some(
                        "Timer start bit",
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
                    name: "tmstop",
                    description: Some(
                        "Timer stop bit",
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
                    name: "tmen",
                    description: Some(
                        "Timer enable",
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
                    name: "tmdis",
                    description: Some(
                        "Timer disable",
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
                    name: "tmrst",
                    description: Some(
                        "Timer reset",
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
                Field {
                    name: "tmdec",
                    description: Some(
                        "Timer decrement",
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
                    name: "tmout",
                    description: Some(
                        "Timer output",
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
            ],
        },
        FieldSet {
            name: "Tmcmp0",
            extends: None,
            description: Some(
                "Timer compare x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmcvalue",
                    description: Some(
                        "Timer compare value",
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
            name: "Tmcmp1",
            extends: None,
            description: Some(
                "Timer compare x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmcvalue",
                    description: Some(
                        "Timer compare value",
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
            name: "Tmcmp2",
            extends: None,
            description: Some(
                "Timer compare x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmcvalue",
                    description: Some(
                        "Timer compare value",
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
            name: "Tmcmp3",
            extends: None,
            description: Some(
                "Timer compare x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmcvalue",
                    description: Some(
                        "Timer compare value",
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
            name: "Tmctl0",
            extends: None,
            description: Some(
                "Timer control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmmod",
                    description: Some(
                        "Timer Mode",
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
                    name: "tmppl",
                    description: Some(
                        "Timer Pin Polarity",
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
                    name: "tmpsel",
                    description: Some(
                        "Timer Pin Select",
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
                    name: "tmpcfg",
                    description: Some(
                        "Timer pin configuration",
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
                    name: "trigsrc",
                    description: Some(
                        "Trigger source",
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
                    name: "trigpl",
                    description: Some(
                        "Trigger polarity",
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
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
            name: "Tmctl1",
            extends: None,
            description: Some(
                "Timer control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmmod",
                    description: Some(
                        "Timer Mode",
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
                    name: "tmppl",
                    description: Some(
                        "Timer Pin Polarity",
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
                    name: "tmpsel",
                    description: Some(
                        "Timer Pin Select",
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
                    name: "tmpcfg",
                    description: Some(
                        "Timer pin configuration",
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
                    name: "trigsrc",
                    description: Some(
                        "Trigger source",
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
                    name: "trigpl",
                    description: Some(
                        "Trigger polarity",
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
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
            name: "Tmctl2",
            extends: None,
            description: Some(
                "Timer control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmmod",
                    description: Some(
                        "Timer Mode",
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
                    name: "tmppl",
                    description: Some(
                        "Timer Pin Polarity",
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
                    name: "tmpsel",
                    description: Some(
                        "Timer Pin Select",
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
                    name: "tmpcfg",
                    description: Some(
                        "Timer pin configuration",
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
                    name: "trigsrc",
                    description: Some(
                        "Trigger source",
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
                    name: "trigpl",
                    description: Some(
                        "Trigger polarity",
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
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
            name: "Tmctl3",
            extends: None,
            description: Some(
                "Timer control x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmmod",
                    description: Some(
                        "Timer Mode",
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
                    name: "tmppl",
                    description: Some(
                        "Timer Pin Polarity",
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
                    name: "tmpsel",
                    description: Some(
                        "Timer Pin Select",
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
                    name: "tmpcfg",
                    description: Some(
                        "Timer pin configuration",
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
                    name: "trigsrc",
                    description: Some(
                        "Trigger source",
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
                    name: "trigpl",
                    description: Some(
                        "Trigger polarity",
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
                    name: "trigsel",
                    description: Some(
                        "Trigger select",
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
            name: "Tmsien",
            extends: None,
            description: Some(
                "Timer status interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmsien",
                    description: Some(
                        "Timer status interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Tmstat",
            extends: None,
            description: Some(
                "Timer status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tmstat",
                    description: Some(
                        "Timer x status flags",
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
            ],
        },
    ],
    enums: &[],
};
                