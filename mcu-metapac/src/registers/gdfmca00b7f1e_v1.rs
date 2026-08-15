
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
                    name: "key",
                    description: Some(
                        "Unlock key register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                            access: Access::Write,
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
                    byte_offset: 0xc,
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
                    byte_offset: 0x10,
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
                    name: "addr",
                    description: Some(
                        "Address register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Addr",
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
                    name: "seckey",
                    description: Some(
                        "Secure Unlock key register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Seckey",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secstat",
                    description: Some(
                        "Secure status register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secctl",
                    description: Some(
                        "Secure Control register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secaddr",
                    description: Some(
                        "Secure Address register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Secaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obr",
                    description: Some(
                        "Option byte register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obuser",
                    description: Some(
                        "Option byte user register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obuser",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secmcfg0",
                    description: Some(
                        "Secure mark configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secmcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmp0",
                    description: Some(
                        "Secure dedicated mark protection register 0",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obwrp0",
                    description: Some(
                        "Option byte write protection area register 0",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obwrp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secm_cfg1",
                    description: Some(
                        "Secure mark configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SecmCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmp1",
                    description: Some(
                        "Secure dedicated mark protection register 1",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obwrp1",
                    description: Some(
                        "Option byte write protection area register 1",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obwrp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secmcfg2",
                    description: Some(
                        "Secure mark configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secmcfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "secmcfg3",
                    description: Some(
                        "Secure mark configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Secmcfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "obr1",
                    description: Some(
                        "Option byte register 1",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Obr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nodec0",
                    description: Some(
                        "NO OTFDEC region0 register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nodec0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nodec1",
                    description: Some(
                        "NO OTFDEC region1 register",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nodec1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nodec2",
                    description: Some(
                        "NO OTFDEC region2 register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nodec2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "nodec3",
                    description: Some(
                        "NO OTFDEC region3 register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Nodec3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ofrg",
                    description: Some(
                        "Offset region register",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ofvr",
                    description: Some(
                        "Offset value register",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ofvr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmpctl",
                    description: Some(
                        "DMP control register",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmpctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfg",
                    description: Some(
                        "Privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfg",
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
                        "Main flash program command bit",
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
                    name: "mer",
                    description: Some(
                        "Main flash mass erase command bit",
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
                        "Send erase command to FMC",
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
                        "FMC_CTL lock bit",
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
                        "FMC_OFVR write enable bit",
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
                Field {
                    name: "obstart",
                    description: Some(
                        "Option bytes modification start bit",
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
                    name: "obrld",
                    description: Some(
                        "Option byte reload bit",
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
            name: "Dmp0",
            extends: None,
            description: Some(
                "Secure dedicated mark protection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmp0_epage",
                    description: Some(
                        "End page of DMP mark secure area 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmp0_en",
                    description: Some(
                        "DMP area 0 enable",
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
            name: "Dmp1",
            extends: None,
            description: Some(
                "Secure dedicated mark protection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmp1_epage",
                    description: Some(
                        "End page of DMP mark secure area 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmp1_en",
                    description: Some(
                        "DMP area 1 enable",
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
            name: "Dmpctl",
            extends: None,
            description: Some(
                "DMP control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmp0_acc_cfg",
                    description: Some(
                        "DMP area 0 access configuration bit",
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
                    name: "dmp1_acc_cfg",
                    description: Some(
                        "DMP area 1 access configuration bit",
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
            name: "Nodec0",
            extends: None,
            description: Some(
                "NO OTFDEC region0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nodec0_spage",
                    description: Some(
                        "Start page of NODEC region 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nodec0_epage",
                    description: Some(
                        "End page of NODEC region 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nodec1",
            extends: None,
            description: Some(
                "NO OTFDEC region1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nodec1_spage",
                    description: Some(
                        "Start page of NODEC region 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nodec1_epage",
                    description: Some(
                        "End page of NODEC region 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nodec2",
            extends: None,
            description: Some(
                "NO OTFDEC region2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nodec2_spage",
                    description: Some(
                        "Start page of NODEC region 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nodec2_epage",
                    description: Some(
                        "End page of NODEC region 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Nodec3",
            extends: None,
            description: Some(
                "NO OTFDEC region3 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nodec0_spage",
                    description: Some(
                        "Start page of NODEC region 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nodec3_epage",
                    description: Some(
                        "End page of NODEC region 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
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
                        "FMC_ OBCTL0 option byte operation unlock register",
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
            name: "Obr",
            extends: None,
            description: Some(
                "Option byte register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spc",
                    description: Some(
                        "Option byte security protection value",
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
                    name: "sram1_rst",
                    description: Some(
                        "SRAM1 reset enable bit",
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
                    name: "tzen",
                    description: Some(
                        "Trust zone enable bit",
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
            name: "Obr1",
            extends: None,
            description: Some(
                "Option byte register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "n_wdg_hw",
                    description: Some(
                        "Watchdog configuration bit",
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
                    name: "fwdgspd_dpslp",
                    description: Some(
                        "FWDGT suspend option in deepsleep mode configuration bit",
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
                    name: "fwdgspd_stdby",
                    description: Some(
                        "FWDGT suspend option in standby mode configuration bit",
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
                    name: "swboot0",
                    description: Some(
                        "Select BOOT0",
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
                    name: "n_boot0",
                    description: Some(
                        "BOOT0 option bit",
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
                    name: "swboot1",
                    description: Some(
                        "Select BOOT1",
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
                    name: "n_boot1",
                    description: Some(
                        "BOOT1 option bit",
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
                    name: "spc_p5",
                    description: Some(
                        "Security protection level 0.5 state",
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
                        "Security protection level 1 state",
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
                    name: "wp",
                    description: Some(
                        "Write/erase protection state",
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
                    name: "tzen_stat",
                    description: Some(
                        "Trust zone state",
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
                    name: "fmcob",
                    description: Some(
                        "Whether the option byte exist or not",
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
            name: "Obuser",
            extends: None,
            description: Some(
                "Option byte user register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "user",
                    description: Some(
                        "Option byte USER value",
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
            name: "Obwrp0",
            extends: None,
            description: Some(
                "Option byte write protection area register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp0_spage",
                    description: Some(
                        "Start page of write protection area 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp0_epage",
                    description: Some(
                        "End page of write protection area 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Obwrp1",
            extends: None,
            description: Some(
                "Option byte write protection area register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrp1_spage",
                    description: Some(
                        "Start page of write protection area 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrp1_epage",
                    description: Some(
                        "End page of write protection area 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofrg",
            extends: None,
            description: Some(
                "Offset region register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "of_spage",
                    description: Some(
                        "Start page of offset region",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "of_epage",
                    description: Some(
                        "End page of offset region",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ofvr",
            extends: None,
            description: Some(
                "Offset value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "of_value",
                    description: Some(
                        "Offset value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
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
            name: "Privcfg",
            extends: None,
            description: Some(
                "Privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmc_priv",
                    description: Some(
                        "FMC privilege configuration",
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
        FieldSet {
            name: "Secaddr",
            extends: None,
            description: Some(
                "Secure Address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secaddr",
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
            name: "Secctl",
            extends: None,
            description: Some(
                "Secure Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secpg",
                    description: Some(
                        "Main flash program command bit",
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
                    name: "secper",
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
                    name: "secmer",
                    description: Some(
                        "Main flash mass erase command bit",
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
                    name: "secstart",
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
                    name: "seclk",
                    description: Some(
                        "FMC_SECCTL lock bit",
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
                    name: "secerrie",
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
                    name: "secendie",
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
            name: "Seckey",
            extends: None,
            description: Some(
                "Secure Unlock key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "seckey",
                    description: Some(
                        "FMC_SECCTL unlock register",
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
            name: "SecmCfg1",
            extends: None,
            description: Some(
                "Secure mark configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secm1_spage",
                    description: Some(
                        "Start page of mark secure area 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "secm1_epage",
                    description: Some(
                        "End page of mark secure area 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secmcfg0",
            extends: None,
            description: Some(
                "Secure mark configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secm0_spage",
                    description: Some(
                        "Start page of mark secure area 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "secm0_epage",
                    description: Some(
                        "End page of mark secure area 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secmcfg2",
            extends: None,
            description: Some(
                "Secure mark configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secm2_spage",
                    description: Some(
                        "Start page of mark secure area 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "secm2_epage",
                    description: Some(
                        "End page of mark secure area 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secmcfg3",
            extends: None,
            description: Some(
                "Secure mark configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secm3_spage",
                    description: Some(
                        "Start page of mark secure area 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "secm3_epage",
                    description: Some(
                        "End page of mark secure area 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Secstat",
            extends: None,
            description: Some(
                "Secure status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secbusy",
                    description: Some(
                        "The flash is busy",
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
                    name: "secerr",
                    description: Some(
                        "Secure error flag",
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
                    name: "secwperr",
                    description: Some(
                        "Erase/Program protection error flag",
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
                    name: "secendf",
                    description: Some(
                        "End of operation flag",
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
                    name: "oberr",
                    description: Some(
                        "Option bytes error flag bit",
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
    ],
    enums: &[],
};
                