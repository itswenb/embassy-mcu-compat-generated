
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Efuse",
            extends: None,
            description: Some(
                "EFUSE",
            ),
            items: &[
                BlockItem {
                    name: "cs",
                    description: Some(
                        "Control and status register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cs",
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
                    byte_offset: 0x4,
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
                    name: "ctl",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                    name: "tzctl",
                    description: Some(
                        "Trustzone control register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fp_ctl",
                    description: Some(
                        "Flash protection control register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "FpCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_ctl",
                    description: Some(
                        "User byte control register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "UserCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcu_init_data0",
                    description: Some(
                        "Mcu initialization data register0",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "McuInitData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcu_init_data1",
                    description: Some(
                        "Mcu initialization data register1",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "McuInitData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcu_init_data2",
                    description: Some(
                        "Mcu initialization data register2",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "McuInitData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aes_key0",
                    description: Some(
                        "Firmware AES key register 0",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AesKey0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aes_key1",
                    description: Some(
                        "Firmware AES key register 1",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AesKey1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aes_key2",
                    description: Some(
                        "Firmware AES key register 2",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AesKey2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aes_key3",
                    description: Some(
                        "Firmware AES key register 3",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AesKey3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key0",
                    description: Some(
                        "RoTPK key register 0",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key1",
                    description: Some(
                        "RoTPK key register 1",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key2",
                    description: Some(
                        "RoTPK key register 2",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key3",
                    description: Some(
                        "RoTPK key register 3",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key4",
                    description: Some(
                        "RoTPK key register 4",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key5",
                    description: Some(
                        "RoTPK key register 5",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key6",
                    description: Some(
                        "RoTPK key register 6",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rotpk_key7",
                    description: Some(
                        "RoTPK key register 7",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RotpkKey7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dp0",
                    description: Some(
                        "Debug password register 0",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dp0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dp1",
                    description: Some(
                        "Debug password register 1",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dp1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss0",
                    description: Some(
                        "IAK key or RSS register 0",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss1",
                    description: Some(
                        "IAK key or RSS register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss2",
                    description: Some(
                        "IAK key or RSS register 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss3",
                    description: Some(
                        "IAK key or RSS register 3",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss4",
                    description: Some(
                        "IAK key or RSS register 4",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss5",
                    description: Some(
                        "IAK key or RSS register 5",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss6",
                    description: Some(
                        "IAK key or RSS register 6",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss7",
                    description: Some(
                        "IAK key or RSS register 7",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss8",
                    description: Some(
                        "IAK key or RSS register 8",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss9",
                    description: Some(
                        "IAK key or RSS register 9",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss10",
                    description: Some(
                        "IAK key or RSS register 10",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss11",
                    description: Some(
                        "IAK key or RSS register 11",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss12",
                    description: Some(
                        "IAK key or RSS register 12",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss13",
                    description: Some(
                        "IAK key or RSS register 13",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss14",
                    description: Some(
                        "IAK key or RSS register 14",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iak_rss15",
                    description: Some(
                        "IAK key or RSS register 15",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IakRss15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "puid0",
                    description: Some(
                        "Product UID register 0",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Puid0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "puid1",
                    description: Some(
                        "Product UID register 1",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Puid1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "puid2",
                    description: Some(
                        "Product UID register 2",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Puid2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "puid3",
                    description: Some(
                        "Product UID register 3",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Puid3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huk_key0",
                    description: Some(
                        "HUK key register 0",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HukKey0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huk_key1",
                    description: Some(
                        "HUK key register 1",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HukKey1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huk_key2",
                    description: Some(
                        "HUK key register 2",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HukKey2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "huk_key3",
                    description: Some(
                        "HUK key register 3",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "HukKey3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data0",
                    description: Some(
                        "RF data register",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data1",
                    description: Some(
                        "RF data register 1",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data2",
                    description: Some(
                        "RF data register 2",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data3",
                    description: Some(
                        "RF data register 3",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data4",
                    description: Some(
                        "RF data register 4",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data5",
                    description: Some(
                        "RF data register 5",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data6",
                    description: Some(
                        "RF data register 6",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data7",
                    description: Some(
                        "RF data register 7",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data8",
                    description: Some(
                        "RF data register 8",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data9",
                    description: Some(
                        "RF data register 9",
                    ),
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data10",
                    description: Some(
                        "RF data register 10",
                    ),
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rf_data11",
                    description: Some(
                        "RF data register 11",
                    ),
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RfData11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data0",
                    description: Some(
                        "User data register 0",
                    ),
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data1",
                    description: Some(
                        "User data register 1",
                    ),
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data2",
                    description: Some(
                        "User data register 2",
                    ),
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data3",
                    description: Some(
                        "User data register 3",
                    ),
                    array: None,
                    byte_offset: 0xf8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data4",
                    description: Some(
                        "User data register 4",
                    ),
                    array: None,
                    byte_offset: 0xfc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data5",
                    description: Some(
                        "User data register 5",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data6",
                    description: Some(
                        "User data register 6",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "user_data7",
                    description: Some(
                        "User data register 7",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "UserData7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pre_tzen",
                    description: Some(
                        "EFUSE Pre-TZEN register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PreTzen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tz_boot_addr",
                    description: Some(
                        "TrustZone boot address register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "TzBootAddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ntz_boot_addr",
                    description: Some(
                        "No-TrustZone boot address register",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "NtzBootAddr",
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
                    name: "efaddr",
                    description: Some(
                        "Read or write efuse data start address",
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
                    name: "efsize",
                    description: Some(
                        "Read or write efuse data size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AesKey0",
            extends: None,
            description: Some(
                "Firmware AES key register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aeskey",
                    description: Some(
                        "Efuse AES key value.",
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
            name: "AesKey1",
            extends: None,
            description: Some(
                "Firmware AES key register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aeskey",
                    description: Some(
                        "Efuse AES key value.",
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
            name: "AesKey2",
            extends: None,
            description: Some(
                "Firmware AES key register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aeskey",
                    description: Some(
                        "Efuse AES key value.",
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
            name: "AesKey3",
            extends: None,
            description: Some(
                "Firmware AES key register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aeskey",
                    description: Some(
                        "Efuse AES key value.",
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
            name: "Cs",
            extends: None,
            description: Some(
                "Control and status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efstr",
                    description: Some(
                        "Start efuse operation",
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
                    name: "efrw",
                    description: Some(
                        "The selection of efuse operation",
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
                    name: "cfgrss",
                    description: Some(
                        "EFUSE_IAK_RSS register contribute configuer",
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
                    name: "pgif",
                    description: Some(
                        "Program operation completed flag",
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
                    name: "rdif",
                    description: Some(
                        "Read operation complete flag",
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
                    name: "oberif",
                    description: Some(
                        "Overstep boundary error flag",
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
                    name: "pgie",
                    description: Some(
                        "Enable bit for program operation completed interrupt",
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
                    name: "rdie",
                    description: Some(
                        "Enable bit for read operation completed interrupt",
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
                    name: "oberie",
                    description: Some(
                        "Enable bit for overstep boundary error interrupt",
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
                    name: "pgic",
                    description: Some(
                        "Clear bit for program operation completed interrupt flag",
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
                    name: "rdic",
                    description: Some(
                        "Clear bit for read operation completed interrupt flag",
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
                    name: "oberic",
                    description: Some(
                        "Clear bit for overstep boundary error interrupt flag",
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
                    name: "efsb",
                    description: Some(
                        "Boot from Secure boot",
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
                    name: "efbootlk",
                    description: Some(
                        "EFUSE_CTL register bit[5:2] lock bit",
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
                    name: "efboot1",
                    description: Some(
                        "Efuse boot1",
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
                    name: "swboot1",
                    description: Some(
                        "Efuse boot 1 bit enable",
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
                    name: "efboot0",
                    description: Some(
                        "Efuse boot0",
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
                    name: "swboot0",
                    description: Some(
                        "Efuse boot 0 bit enable",
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
            name: "Dp0",
            extends: None,
            description: Some(
                "Debug password register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dp",
                    description: Some(
                        "Efuse Debug password value",
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
            name: "Dp1",
            extends: None,
            description: Some(
                "Debug password register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dp",
                    description: Some(
                        "Efuse Debug password value",
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
            name: "FpCtl",
            extends: None,
            description: Some(
                "Flash protection control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fp",
                    description: Some(
                        "Efuse flash protection value",
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
            name: "HukKey0",
            extends: None,
            description: Some(
                "HUK key register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hkey",
                    description: Some(
                        "Efuse HUK value",
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
            name: "HukKey1",
            extends: None,
            description: Some(
                "HUK key register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hkey",
                    description: Some(
                        "Efuse HUK value",
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
            name: "HukKey2",
            extends: None,
            description: Some(
                "HUK key register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hkey",
                    description: Some(
                        "Efuse HUK value",
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
            name: "HukKey3",
            extends: None,
            description: Some(
                "HUK key register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hkey",
                    description: Some(
                        "Efuse HUK value",
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
            name: "IakRss0",
            extends: None,
            description: Some(
                "IAK key or RSS register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss1",
            extends: None,
            description: Some(
                "IAK key or RSS register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss10",
            extends: None,
            description: Some(
                "IAK key or RSS register 10",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss11",
            extends: None,
            description: Some(
                "IAK key or RSS register 11",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss12",
            extends: None,
            description: Some(
                "IAK key or RSS register 12",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss13",
            extends: None,
            description: Some(
                "IAK key or RSS register 13",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss14",
            extends: None,
            description: Some(
                "IAK key or RSS register 14",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss15",
            extends: None,
            description: Some(
                "IAK key or RSS register 15",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss2",
            extends: None,
            description: Some(
                "IAK key or RSS register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss3",
            extends: None,
            description: Some(
                "IAK key or RSS register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss4",
            extends: None,
            description: Some(
                "IAK key or RSS register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss5",
            extends: None,
            description: Some(
                "IAK key or RSS register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss6",
            extends: None,
            description: Some(
                "IAK key or RSS register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss7",
            extends: None,
            description: Some(
                "IAK key or RSS register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss8",
            extends: None,
            description: Some(
                "IAK key or RSS register 8",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "IakRss9",
            extends: None,
            description: Some(
                "IAK key or RSS register 9",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iakrss",
                    description: Some(
                        "Efuse IAK/RSS value",
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
            name: "McuInitData0",
            extends: None,
            description: Some(
                "Mcu initialization data register0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "initdata",
                    description: Some(
                        "Efuse mcu_init value",
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
            name: "McuInitData1",
            extends: None,
            description: Some(
                "Mcu initialization data register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "initdata",
                    description: Some(
                        "Efuse mcu_init value",
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
            name: "McuInitData2",
            extends: None,
            description: Some(
                "Mcu initialization data register2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "initdata",
                    description: Some(
                        "Efuse mcu_init value",
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
            name: "NtzBootAddr",
            extends: None,
            description: Some(
                "No-TrustZone boot address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ntzbootaddr",
                    description: Some(
                        "Boot from the address when TrustZone is disabled",
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
            name: "PreTzen",
            extends: None,
            description: Some(
                "EFUSE Pre-TZEN register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "stzen",
                    description: Some(
                        "Enable Trustzone function by software",
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
            name: "Puid0",
            extends: None,
            description: Some(
                "Product UID register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uid",
                    description: Some(
                        "Efuse MCU UID value.",
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
            name: "Puid1",
            extends: None,
            description: Some(
                "Product UID register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uid",
                    description: Some(
                        "Efuse MCU UID value.",
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
            name: "Puid2",
            extends: None,
            description: Some(
                "Product UID register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uid",
                    description: Some(
                        "Efuse MCU UID value.",
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
            name: "Puid3",
            extends: None,
            description: Some(
                "Product UID register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uid",
                    description: Some(
                        "Efuse MCU UID value.",
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
            name: "RfData0",
            extends: None,
            description: Some(
                "RF data register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hkey",
                    description: Some(
                        "Efuse HUK value",
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
            name: "RfData1",
            extends: None,
            description: Some(
                "RF data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData10",
            extends: None,
            description: Some(
                "RF data register 10",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData11",
            extends: None,
            description: Some(
                "RF data register 11",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData2",
            extends: None,
            description: Some(
                "RF data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData3",
            extends: None,
            description: Some(
                "RF data register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData4",
            extends: None,
            description: Some(
                "RF data register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData5",
            extends: None,
            description: Some(
                "RF data register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData6",
            extends: None,
            description: Some(
                "RF data register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData7",
            extends: None,
            description: Some(
                "RF data register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData8",
            extends: None,
            description: Some(
                "RF data register 8",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RfData9",
            extends: None,
            description: Some(
                "RF data register 9",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rfdata",
                    description: Some(
                        "Efuse RF data value",
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
            name: "RotpkKey0",
            extends: None,
            description: Some(
                "RoTPK key register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey1",
            extends: None,
            description: Some(
                "RoTPK key register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey2",
            extends: None,
            description: Some(
                "RoTPK key register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey3",
            extends: None,
            description: Some(
                "RoTPK key register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey4",
            extends: None,
            description: Some(
                "RoTPK key register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey5",
            extends: None,
            description: Some(
                "RoTPK key register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey6",
            extends: None,
            description: Some(
                "RoTPK key register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "RotpkKey7",
            extends: None,
            description: Some(
                "RoTPK key register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rkey",
                    description: Some(
                        "Efuse RoTPK or its hash value.",
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
            name: "TzBootAddr",
            extends: None,
            description: Some(
                "TrustZone boot address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tzbootaddr",
                    description: Some(
                        "Boot from the address when TrustZone is enabled",
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
            name: "Tzctl",
            extends: None,
            description: Some(
                "Trustzone control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tzen",
                    description: Some(
                        "Trust zone enable bit",
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
                    name: "ndbg",
                    description: Some(
                        "Debugging permission setting",
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
                    name: "rotlk",
                    description: Some(
                        "EFUSE_ROTPK_KEY register lock bit",
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
                    name: "rflk",
                    description: Some(
                        "EFUSE_RF_DATA register lock bit",
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
                    name: "irlk",
                    description: Some(
                        "EFUSE_IAK_RSS register lock bit",
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
                    name: "dplk",
                    description: Some(
                        "EFUSE_DP register lock bit",
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
                    name: "vfimg",
                    description: Some(
                        "Verify firmware image",
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
                    name: "vfcert",
                    description: Some(
                        "Verify firmware certificate",
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
            ],
        },
        FieldSet {
            name: "UserCtl",
            extends: None,
            description: Some(
                "User byte control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hwdg",
                    description: Some(
                        "Free watchdog timer selection",
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
                    name: "nrststdby",
                    description: Some(
                        "Reset option of entry standby mode",
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
                    name: "nrstdpslp",
                    description: Some(
                        "Reset option of entry deep sleep mode",
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
                    name: "efoplk",
                    description: Some(
                        "EFUSE_FP_CTL and EFUSE_USER_CTL register lock bit",
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
                    name: "mcuinitlk",
                    description: Some(
                        "EFUSE_MCU_INIT_DATA register lock bit",
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
                    name: "aesen",
                    description: Some(
                        "Lock EFUSE_AES_KEY register and enable AES decrypt function",
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
                    name: "udlk",
                    description: Some(
                        "EFUSE_USER_DATA register lock bit",
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
            name: "UserData0",
            extends: None,
            description: Some(
                "User data register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData1",
            extends: None,
            description: Some(
                "User data register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData2",
            extends: None,
            description: Some(
                "User data register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData3",
            extends: None,
            description: Some(
                "User data register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData4",
            extends: None,
            description: Some(
                "User data register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData5",
            extends: None,
            description: Some(
                "User data register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData6",
            extends: None,
            description: Some(
                "User data register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
            name: "UserData7",
            extends: None,
            description: Some(
                "User data register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "userdata",
                    description: Some(
                        "Efuse USER_DATA value.",
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
    ],
    enums: &[],
};
                