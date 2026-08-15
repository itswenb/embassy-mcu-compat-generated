
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fmc",
            extends: None,
            description: Some(
                "FMC",
            ),
            items: &[
                BlockItem {
                    name: "ws",
                    description: Some(
                        "wait state counter register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ws",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key0",
                    description: Some(
                        "Unlock key register 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obkey",
                    description: Some(
                        "Option byte unlock key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Obkey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "Status register 0",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "ctl0",
                    description: Some(
                        "Control register 0",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "addr0",
                    description: Some(
                        "Address register 0",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obstat",
                    description: Some(
                        "Option byte status register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Obstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wp",
                    description: Some(
                        "Erase/Program Protection register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wp",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key1",
                    description: Some(
                        "Unlock key register 1",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key1",
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
                    byte_offset: 0x4c,
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
                    byte_offset: 0x50,
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
                    name: "addr1",
                    description: Some(
                        "Address register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wsen",
                    description: Some(
                        "Wait state enable register",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wsen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pid",
                    description: Some(
                        "Product ID register",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Pid",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr0",
            extends: None,
            description: Some(
                "Address register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Flash erase/program command address bits",
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
            name: "Addr1",
            extends: None,
            description: Some(
                "Address register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Flash erase/program command address bits",
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
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Main flash program for bank0 command bit",
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
                    name: "per",
                    description: Some(
                        "Main flash page erase for bank0 command bit",
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
                    name: "mer",
                    description: Some(
                        "Main flash mass erase for bank0 command bit",
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
                    name: "obpg",
                    description: Some(
                        "Option bytes program command bit",
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
                    name: "ober",
                    description: Some(
                        "Option bytes erase command bit",
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
                    name: "start",
                    description: Some(
                        "Send erase command to FMC bit",
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
                    name: "lk",
                    description: Some(
                        "FMC_CTL0 lock bit",
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
                    name: "obwen",
                    description: Some(
                        "Option byte erase/program enable bit",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable bit",
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
                    name: "endie",
                    description: Some(
                        "End of operation interrupt enable bit",
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
                    name: "pg",
                    description: Some(
                        "Main flash program for bank1 command bit",
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
                    name: "per",
                    description: Some(
                        "Main flash page erase for bank1 command bit",
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
                    name: "mer",
                    description: Some(
                        "Main flash mass erase for bank1 command bit",
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
                    name: "start",
                    description: Some(
                        "Send erase command to FMC bit",
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
                    name: "lk",
                    description: Some(
                        "FMC_CTL1 lock bit",
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
                    name: "errie",
                    description: Some(
                        "Error interrupt enable bit",
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
                    name: "endie",
                    description: Some(
                        "End of operation interrupt enable bit",
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
            ],
        },
        FieldSet {
            name: "Key0",
            extends: None,
            description: Some(
                "Unlock key register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "FMC_CTL0 unlock key",
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
            name: "Key1",
            extends: None,
            description: Some(
                "Unlock key register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "FMC_CTL1 unlock register",
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
            name: "Obkey",
            extends: None,
            description: Some(
                "Option byte unlock key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "obkey",
                    description: Some(
                        "FMC_ CTL0 option byte operation unlock register",
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
            name: "Obstat",
            extends: None,
            description: Some(
                "Option byte status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oberr",
                    description: Some(
                        "Option bytes read error bit",
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
                    name: "spc",
                    description: Some(
                        "Option bytes security protection code",
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
                    name: "user",
                    description: Some(
                        "Store USER of option bytes block after system reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data",
                    description: Some(
                        "Store DATA[15:0] of option bytes block after system reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pid",
            extends: None,
            description: Some(
                "Product ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid",
                    description: Some(
                        "Product reserved ID code register",
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
            name: "Stat0",
            extends: None,
            description: Some(
                "Status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "The flash is busy bit",
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
                    name: "pgerr",
                    description: Some(
                        "Program error flag bit",
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
                    name: "wperr",
                    description: Some(
                        "Erase/Program protection error flag bit",
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
                    name: "endf",
                    description: Some(
                        "End of operation flag bit",
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
            name: "Stat1",
            extends: None,
            description: Some(
                "Status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "The flash is busy bit",
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
                    name: "pgerr",
                    description: Some(
                        "Program error flag bit",
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
                    name: "wperr",
                    description: Some(
                        "Erase/Program protection error flag bit",
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
                    name: "endf",
                    description: Some(
                        "End of operation flag bit",
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
            name: "Wp",
            extends: None,
            description: Some(
                "Erase/Program Protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Store WP[31:0] of option bytes block after system reset",
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
            name: "Ws",
            extends: None,
            description: Some(
                "wait state counter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wscnt",
                    description: Some(
                        "wait state counter register",
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
            name: "Wsen",
            extends: None,
            description: Some(
                "Wait state enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wsen",
                    description: Some(
                        "FMC wait state enable register",
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
            ],
        },
    ],
    enums: &[],
};
                