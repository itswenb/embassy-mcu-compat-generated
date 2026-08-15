
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
                    name: "ws",
                    description: Some(
                        "Wait state register",
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
                    name: "runkey",
                    description: Some(
                        "Unlock flash mode during run mode key register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Runkey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key",
                    description: Some(
                        "Unlock key register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    byte_offset: 0xc,
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
                    name: "ctl",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x14,
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
                    name: "ecccs",
                    description: Some(
                        "ECC control and status register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ecccs",
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
                    byte_offset: 0x20,
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
                    name: "dcrp_saddr0",
                    description: Some(
                        "DCRP start address register 0",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpSaddr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcrp_eaddr0",
                    description: Some(
                        "DCRP end address register 0",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpEaddr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0wp0",
                    description: Some(
                        "Bank0 erase/program protection area 0 register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0wp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0wp1",
                    description: Some(
                        "Bank0 erase/program protection area 1 register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0wp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcrp_saddr1",
                    description: Some(
                        "DCRP start address register 1",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpSaddr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dcrp_eaddr1",
                    description: Some(
                        "DCRP end address register 1",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DcrpEaddr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1wp0",
                    description: Some(
                        "Bank1 erase/program protection area 0 register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1wp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1wp1",
                    description: Some(
                        "Bank1 erase/program protection area 1 register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1wp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk0scr",
                    description: Some(
                        "Bank0 secure user area register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk0scr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bk1scr",
                    description: Some(
                        "Bank1 secure user area register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bk1scr",
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
                            access: Access::ReadWrite,
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
            name: "Bk0scr",
            extends: None,
            description: Some(
                "Bank0 secure user area register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scr_page_cnt0",
                    description: Some(
                        "Configure the number of pages in the bank0 secure user area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bootlk",
                    description: Some(
                        "This bit is set to force boot from user flash area",
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
            ],
        },
        FieldSet {
            name: "Bk0wp0",
            extends: None,
            description: Some(
                "Bank0 erase/program protection area 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bk0wp0_saddr",
                    description: Some(
                        "WP first area start offset",
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
                    name: "bk0wp0_eaddr",
                    description: Some(
                        "WP first area end offset",
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
            name: "Bk0wp1",
            extends: None,
            description: Some(
                "Bank0 erase/program protection area 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bk0wp1_saddr",
                    description: Some(
                        "WP second area start offset",
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
                    name: "bk0wp1_eaddr",
                    description: Some(
                        "WP second area end offset",
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
            name: "Bk1scr",
            extends: None,
            description: Some(
                "Bank1 secure user area register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "scr_page_cnt1",
                    description: Some(
                        "Configure the number of pages in the bank1 secure user area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bk1wp0",
            extends: None,
            description: Some(
                "Bank1 erase/program protection area 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bk1wp0_saddr",
                    description: Some(
                        "WP third area start offset",
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
                    name: "bk1wp0_eaddr",
                    description: Some(
                        "WP third area end offset",
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
            name: "Bk1wp1",
            extends: None,
            description: Some(
                "Bank1 erase/program protection area 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bk1wp1_saddr",
                    description: Some(
                        "WP fourth area start offset",
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
                    name: "bk1wp1_eaddr",
                    description: Some(
                        "WP fourth area end offset",
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
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pg",
                    description: Some(
                        "Main flash page program command bit",
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
                        "Main flash page erase command bit",
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
                    name: "mer0",
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
                    name: "pnsel",
                    description: Some(
                        "Page number selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bksel",
                    description: Some(
                        "Bank number selection for page erase",
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
                    name: "mer1",
                    description: Some(
                        "Main flash mass erase for bank1 command bit",
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
                    name: "start",
                    description: Some(
                        "Send erase command to FMC bit",
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
                    name: "obstart",
                    description: Some(
                        "Send option byte change command to FMC bit",
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
                    name: "endie",
                    description: Some(
                        "End of operation interrupt enable bit",
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
                    name: "errie",
                    description: Some(
                        "OPRERR error interrupt enable bit",
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
                    name: "rperrie",
                    description: Some(
                        "Read protection error interrupt enable bit",
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
                    name: "obrld",
                    description: Some(
                        "Option byte reload bit",
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
                    name: "scr0",
                    description: Some(
                        "Bank0 secure user area enable bit",
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
                    name: "scr1",
                    description: Some(
                        "Bank1 secure user area enable bit",
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
                    name: "oblk",
                    description: Some(
                        "FMC_OBCTL lock bit",
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
                    name: "lk",
                    description: Some(
                        "FMC_CTL lock bit",
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
            name: "DcrpEaddr0",
            extends: None,
            description: Some(
                "DCRP end address register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp0_eaddr",
                    description: Some(
                        "DCRP area end address offset configuration bits for bank0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcrp_eren",
                    description: Some(
                        "DCRP area erase enable configuration bit",
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
            name: "DcrpEaddr1",
            extends: None,
            description: Some(
                "DCRP end address register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp1_earea",
                    description: Some(
                        "DCRP area end address offset configuration bits for bank1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DcrpSaddr0",
            extends: None,
            description: Some(
                "DCRP start address register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp0_saddr",
                    description: Some(
                        "DCRP area start address offset configuration bits for bank0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DcrpSaddr1",
            extends: None,
            description: Some(
                "DCRP start address register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcrp1_saddr",
                    description: Some(
                        "DCRP area start address offset configuration bits for bank1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ecccs",
            extends: None,
            description: Some(
                "ECC control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eccaddr",
                    description: Some(
                        "ECC fail address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bk_ecc",
                    description: Some(
                        "ECC fail bank",
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
                    name: "sys_ecc",
                    description: Some(
                        "If an ECC error correction or double ECC error is detected in bootloader, this bit will be set",
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
                    name: "ecccorie",
                    description: Some(
                        "One-bit error correction interrupt enable",
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
                    name: "ecccor1",
                    description: Some(
                        "One-bit error detected and corrected flag",
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
                    name: "eccdet1",
                    description: Some(
                        "Two bit errors detected flag",
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
                    name: "ecccor0",
                    description: Some(
                        "One-bit error detected and corrected flag",
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
                    name: "eccdet0",
                    description: Some(
                        "Two bit errors detected flag",
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
            name: "Obctl",
            extends: None,
            description: Some(
                "Option byte control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spc",
                    description: Some(
                        "Security protection level option byte status bits",
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
                    name: "bor_th",
                    description: Some(
                        "BOR threshold status bits",
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
                    name: "n_rst_dpslp",
                    description: Some(
                        "Option byte deepsleep reset value",
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
                    name: "n_rst_stdby",
                    description: Some(
                        "Option byte standby reset value",
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
                    name: "fmc_swp",
                    description: Some(
                        "FMC memory mapping swap",
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
                    name: "n_fwdg_hw",
                    description: Some(
                        "Free watchdog configuration bit",
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
                    name: "bb",
                    description: Some(
                        "Option byte boot bank value",
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
                    name: "dbs",
                    description: Some(
                        "Double banks or single bank selection",
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
                    name: "n_boot1",
                    description: Some(
                        "Boot1 configuration bit",
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
                    name: "sram_eccen",
                    description: Some(
                        "SRAM and TCM SRAM ECC disable",
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
                    name: "tcmsram_ers",
                    description: Some(
                        "TCM SRAM erase if system reset",
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
                    name: "n_swbt0",
                    description: Some(
                        "Software BOOT0 disable",
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
                    name: "n_boot0",
                    description: Some(
                        "BOOT0 option bit",
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
                    name: "nrst_mdsel",
                    description: Some(
                        "NRST pin mode selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
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
                        "FMC_CTL option bytes operation unlock register",
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
            name: "Runkey",
            extends: None,
            description: Some(
                "Unlock flash mode during run mode key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "run_key",
                    description: Some(
                        "RUN_MDSEL unlock register",
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
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "endf",
                    description: Some(
                        "End of operation flag bit",
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
                    name: "oprerr",
                    description: Some(
                        "Operation error flag bit",
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
                    name: "pgerr",
                    description: Some(
                        "Program error flag bit",
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
                    name: "wperr",
                    description: Some(
                        "Erase / program protection error flag bit",
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
                    name: "pgaerr",
                    description: Some(
                        "Program alignment error flag bit",
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
                    name: "pgmerr",
                    description: Some(
                        "Program size not match error flag bit",
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
                    name: "pgserr",
                    description: Some(
                        "Program sequence error flag bit",
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
                    name: "rperr",
                    description: Some(
                        "Read protection error flag bit",
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
                    name: "oberr",
                    description: Some(
                        "Option byte read error bit",
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
                    name: "busy",
                    description: Some(
                        "The flash is busy bit",
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
            ],
        },
        FieldSet {
            name: "Ws",
            extends: None,
            description: Some(
                "Wait state register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wscnt",
                    description: Some(
                        "Wait state counter register",
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
                    name: "pfen",
                    description: Some(
                        "Pre-fetch enable",
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
                    name: "icen",
                    description: Some(
                        "Instruction cache enable",
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
                    name: "dcen",
                    description: Some(
                        "Data cache enable",
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
                    name: "icrst",
                    description: Some(
                        "Instruction cache reset",
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
                    name: "dcrst",
                    description: Some(
                        "Data cache reset",
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
                    name: "run_mdsel",
                    description: Some(
                        "Flash power-down mode during run mode",
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
                    name: "slp_mdsel",
                    description: Some(
                        "Flash power-down mode during sleep mode",
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
                    name: "dbgen",
                    description: Some(
                        "This bit is used to enable/disable the debugger by software",
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
            ],
        },
    ],
    enums: &[],
};
                