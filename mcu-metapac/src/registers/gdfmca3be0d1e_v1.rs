
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Fmc",
            extends: None,
            description: Some(
                "Flash memory controller",
            ),
            items: &[
                BlockItem {
                    name: "key",
                    description: Some(
                        "Unlock key register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Key",
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
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obkey",
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
                    byte_offset: 0xc,
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
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "addr",
                    description: Some(
                        "Address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obctl",
                    description: Some(
                        "Option byte control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obstat0_eft",
                    description: Some(
                        "Option byte status register 0",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obstat0Eft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obstat0_mdf",
                    description: Some(
                        "Option byte status register 0",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obstat0Mdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcrpaddr_eft",
                    description: Some(
                        "DCRP address register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpaddrEft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcrpaddr_mdf",
                    description: Some(
                        "DCRP address register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpaddrMdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scraddr_eft",
                    description: Some(
                        "Secure address register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ScraddrEft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scraddr_mdf",
                    description: Some(
                        "Secure address register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ScraddrMdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wp_eft",
                    description: Some(
                        "Erase/program protection register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WpEft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wp_mdf",
                    description: Some(
                        "Erase/program protection register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "WpMdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btaddr_eft",
                    description: Some(
                        "Boot address register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BtaddrEft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "btaddr_mdf",
                    description: Some(
                        "Boot address register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "BtaddrMdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obstat1_eft",
                    description: Some(
                        "Option byte status register 1",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obstat1Eft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obstat1_mdf",
                    description: Some(
                        "Option byte status register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obstat1Mdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nodec",
                    description: Some(
                        "NO-RTDEC area register",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nodec",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv0_eft",
                    description: Some(
                        "AES IV regist 0",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv0Eft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv1_eft",
                    description: Some(
                        "AES IV regist 1",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv1Eft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv2_eft",
                    description: Some(
                        "AES IV regist 2",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv2Eft",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv0_mdf",
                    description: Some(
                        "AES IV regist 0",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv0Mdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv1_mdf",
                    description: Some(
                        "AES IV regist 1",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv1Mdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aesiv2_mdf",
                    description: Some(
                        "AES IV regist 2",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Aesiv2Mdf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pid0",
                    description: Some(
                        "Product ID register 0",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pid0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pid1",
                    description: Some(
                        "Product ID register 1",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pid1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Addr",
            extends: None,
            description: Some(
                "Address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr",
                    description: Some(
                        "Flash erase command address bits",
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
            name: "Aesiv0Eft",
            extends: None,
            description: Some(
                "AES IV regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector status value",
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
            name: "Aesiv0Mdf",
            extends: None,
            description: Some(
                "AES IV regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector configuration value",
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
            name: "Aesiv1Eft",
            extends: None,
            description: Some(
                "AES IV regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector status value",
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
            name: "Aesiv1Mdf",
            extends: None,
            description: Some(
                "AES IV regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector configuration value",
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
            name: "Aesiv2Eft",
            extends: None,
            description: Some(
                "AES IV regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector status value",
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
            name: "Aesiv2Mdf",
            extends: None,
            description: Some(
                "AES IV regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aesiv",
                    description: Some(
                        "AES initialization vector configuration value",
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
            name: "BtaddrEft",
            extends: None,
            description: Some(
                "Boot address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_addr0",
                    description: Some(
                        "Boot address 0 status bits",
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
                    name: "boot_addr1",
                    description: Some(
                        "Boot address 1 status bits",
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
            name: "BtaddrMdf",
            extends: None,
            description: Some(
                "Boot address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "boot_addr0",
                    description: Some(
                        "Boot address 0 configuration bits.",
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
                    name: "boot_addr1",
                    description: Some(
                        "Boot address 1 configuration bits.",
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
                    name: "lk",
                    description: Some(
                        "FMC_CTL lock bit",
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
                    name: "pg",
                    description: Some(
                        "Main Flash program command bit",
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
                    name: "ser",
                    description: Some(
                        "Sector erase command bit",
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
                    name: "mer",
                    description: Some(
                        "Mass erase command bit",
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
                    name: "pgchen",
                    description: Some(
                        "Check programming area enable",
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
                    name: "start",
                    description: Some(
                        "Send erase command to FMC",
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
                    name: "endie",
                    description: Some(
                        "End of operation interrupt enable",
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
                    name: "wperrie",
                    description: Some(
                        "Erase/program protection error interrupt enable",
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
                    name: "pgserrie",
                    description: Some(
                        "Program sequence error interrupt enable",
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
                    name: "rperrie",
                    description: Some(
                        "Read protection error interrupt enable",
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
                    name: "rserrie",
                    description: Some(
                        "Read secure error interrupt enable.",
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
            ],
        },
        FieldSet {
            name: "DcrpaddrEft",
            extends: None,
            description: Some(
                "DCRP address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp_area_start",
                    description: Some(
                        "DCRP area start address status bits",
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
                Field {
                    name: "dcrp_area_end",
                    description: Some(
                        "DCRP area end address status bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrp_eren",
                    description: Some(
                        "DCRP area erase enable status bit.",
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
            name: "DcrpaddrMdf",
            extends: None,
            description: Some(
                "DCRP address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp_area_start",
                    description: Some(
                        "DCRP area start address configuration bits",
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
                Field {
                    name: "dcrp_area_end",
                    description: Some(
                        "DCRP area end address configuration bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrp_eren",
                    description: Some(
                        "DCRP area erase enable configuration bit.",
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
            name: "Key",
            extends: None,
            description: Some(
                "Unlock key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key",
                    description: Some(
                        "FMC_CTL unlock register",
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
            name: "Nodec",
            extends: None,
            description: Some(
                "NO-RTDEC area register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nodec_area_start",
                    description: Some(
                        "No-RTDEC area start address",
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
                Field {
                    name: "nodec_area_end",
                    description: Some(
                        "NO-RTDEC area end address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Obctl",
            extends: None,
            description: Some(
                "Option byte control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "oblk",
                    description: Some(
                        "FMC_OBCTL lock bit",
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
                    name: "obstart",
                    description: Some(
                        "Send option byte change command to FMC bit.",
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
                    name: "obmerrie",
                    description: Some(
                        "Option byte modify error interrupt enable.",
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
                        "These bits are only be written by software.",
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
            name: "Obstat0Eft",
            extends: None,
            description: Some(
                "Option byte status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bor_th",
                    description: Some(
                        "BOR threshold status bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_wdg_hw",
                    description: Some(
                        "Watchdog status bit",
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
                    name: "n_rst_dpslp",
                    description: Some(
                        "Option byte deepsleep reset status bit",
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
                    name: "n_rst_stdby",
                    description: Some(
                        "Option byte standby reset status bit",
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
                    name: "spc",
                    description: Some(
                        "Security protection level option byte status bits",
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
                    name: "fwdgspd_dpslp",
                    description: Some(
                        "FWDGT suspend option in deepsleep mode status bit",
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
                    name: "fwdgspd_stdby",
                    description: Some(
                        "FWDGT suspend option in standby mode status bit",
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
                    name: "scr",
                    description: Some(
                        "Secure mode status bit",
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
                    name: "itcmeccen",
                    description: Some(
                        "ITCM ECC function enable status bit",
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
                    name: "dtcm0eccen",
                    description: Some(
                        "DTCM0 ECC function enable status bit",
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
                    name: "dtcm1eccen",
                    description: Some(
                        "DTCM1 ECC function enable status bit",
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
                    name: "iospdopen",
                    description: Some(
                        "Allowed enable status bit for I/O speed optimization at low-voltage.",
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
            name: "Obstat0Mdf",
            extends: None,
            description: Some(
                "Option byte status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bor_th",
                    description: Some(
                        "BOR threshold configuration bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "n_wdg_hw",
                    description: Some(
                        "Watchdog configuration bit",
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
                    name: "n_rst_dpslp",
                    description: Some(
                        "Option byte deepsleep reset configuration bit",
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
                    name: "n_rst_stdby",
                    description: Some(
                        "Option byte standby reset configuration bit",
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
                    name: "spc",
                    description: Some(
                        "Security protection level option configuration bits",
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
                    name: "fwdgspd_dpslp",
                    description: Some(
                        "FWDGT suspend option in deepsleep mode configuration bit",
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
                    name: "fwdgspd_stdby",
                    description: Some(
                        "FWDGT suspend option in standby mode configuration bit",
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
                    name: "scr",
                    description: Some(
                        "Secure mode configuration bit",
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
                    name: "itcmeccen",
                    description: Some(
                        "ITCM ECC function enable configuration bit",
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
                    name: "dtcm0eccen",
                    description: Some(
                        "DTCM0 ECC function enable configuration bit",
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
                    name: "dtcm1eccen",
                    description: Some(
                        "DTCM1 ECC function enable configuration bit",
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
                    name: "iospdopen",
                    description: Some(
                        "Allowed enable configuration bit for I/O speed optimization at low-voltage.",
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
            name: "Obstat1Eft",
            extends: None,
            description: Some(
                "Option byte status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itcm_sz_shrram",
                    description: Some(
                        "ITCM size of shared RAM status bits.",
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
                    name: "dtcm_sz_shrram",
                    description: Some(
                        "DTCM size of shared RAM status bits.",
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
                    name: "data",
                    description: Some(
                        "User defined option byte data status value",
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
            name: "Obstat1Mdf",
            extends: None,
            description: Some(
                "Option byte status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "itcm_sz_shrram",
                    description: Some(
                        "ITCM size of shared RAM configuration bits.",
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
                    name: "dtcm_sz_shrram",
                    description: Some(
                        "DTCM size of shared RAM configuration bits.",
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
                    name: "data",
                    description: Some(
                        "User defined option byte data configuration value",
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
            name: "Pid0",
            extends: None,
            description: Some(
                "Product ID register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid",
                    description: Some(
                        "Product reserved ID code register x",
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
            name: "Pid1",
            extends: None,
            description: Some(
                "Product ID register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pid",
                    description: Some(
                        "Product reserved ID code register x",
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
            name: "ScraddrEft",
            extends: None,
            description: Some(
                "Secure address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scr_area_start",
                    description: Some(
                        "Secure user area start address status bits",
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
                Field {
                    name: "scr_area_end",
                    description: Some(
                        "Secure user area end address status bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scr_eren",
                    description: Some(
                        "Secure user area erase enable option status bit.",
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
            name: "ScraddrMdf",
            extends: None,
            description: Some(
                "Secure address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scr_area_start",
                    description: Some(
                        "Secure user area start address configuration bits",
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
                Field {
                    name: "scr_area_end",
                    description: Some(
                        "Secure user area end address configuration bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scr_eren",
                    description: Some(
                        "Secure user area erase enable option configuration bit.",
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
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "busy",
                    description: Some(
                        "The Flash is busy bit",
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
                    name: "endf",
                    description: Some(
                        "End of operation flag bit.",
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
                    name: "wperr",
                    description: Some(
                        "Erase/program protection error flag bit.",
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
                    name: "pgserr",
                    description: Some(
                        "Program sequence error flag bit.",
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
                    name: "rperr",
                    description: Some(
                        "Read protection error flag bit.",
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
                    name: "rserr",
                    description: Some(
                        "Read secure error flag bit.",
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
                    name: "obmerr",
                    description: Some(
                        "Option byte modify error flag .",
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
            ],
        },
        FieldSet {
            name: "WpEft",
            extends: None,
            description: Some(
                "Erase/program protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Sector erase/program protection option status bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WpMdf",
            extends: None,
            description: Some(
                "Erase/program protection register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wp",
                    description: Some(
                        "Sector erase/program protection option configuration bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                