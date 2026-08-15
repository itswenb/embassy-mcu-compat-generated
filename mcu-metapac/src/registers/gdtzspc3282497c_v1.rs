
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Tzspc",
            extends: None,
            description: Some(
                "TrustZone security controller",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "TZSPC control register",
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
                    name: "sam_cfg0",
                    description: Some(
                        "TZSPC secure access mode configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SamCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sam_cfg1",
                    description: Some(
                        "TZSPC secure access mode configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SamCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sam_cfg2",
                    description: Some(
                        "TZSPC secure access mode configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SamCfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pam_cfg0",
                    description: Some(
                        "TZSPC privilege access mode configuration register1",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PamCfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pam_cfg1",
                    description: Some(
                        "TZSPC privilege access mode configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PamCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pam_cfg2",
                    description: Some(
                        "TZSPC privilege access mode configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PamCfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmmpc0_nsm0",
                    description: Some(
                        "TZSPC external memory 0 non-secure mark register 0",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmmpc0Nsm0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmmpc0_nsm1",
                    description: Some(
                        "TZSPC external memory 0 non-secure mark register 1",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmmpc0Nsm1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmpc0_nsm2",
                    description: Some(
                        "TZSPC external memory 0 non-secure mark register 2",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmpc0Nsm2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmpc0_nsm3",
                    description: Some(
                        "TZSPC external memory 0 non-secure mark register 3",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmpc0Nsm3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmmpc1_nsm0",
                    description: Some(
                        "TZSPC external memory 1 non-secure mark register 0",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmmpc1Nsm0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tzmmpc1_nsm1",
                    description: Some(
                        "TZSPC external memory 1 non-secure mark register 1",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tzmmpc1Nsm1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dbg_cfg",
                    description: Some(
                        "TZSPC debug configuration register",
                    ),
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DbgCfg",
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
                "TZSPC control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lk",
                    description: Some(
                        "TZSPC items lock configuration bit",
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
            name: "DbgCfg",
            extends: None,
            description: Some(
                "TZSPC debug configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iden",
                    description: Some(
                        "Invasive debug enable bit",
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
                    name: "niden",
                    description: Some(
                        "Non-invasive debug enable bit",
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
                    name: "spiden",
                    description: Some(
                        "Secure invasive debug enable bit",
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
                    name: "spniden",
                    description: Some(
                        "Secure non-invasive debug enable bit",
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
            name: "PamCfg0",
            extends: None,
            description: Some(
                "TZSPC privilege access mode configuration register1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer1pam",
                    description: Some(
                        "TIMER1 privilege access mode configuration bit",
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
                    name: "timer2pam",
                    description: Some(
                        "TIMER2 privilege access mode configuration bit",
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
                    name: "timer3pam",
                    description: Some(
                        "TIMER3 privilege access mode configuration bit",
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
                    name: "timer4pam",
                    description: Some(
                        "TIMER4 privilege access mode configuration bit",
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
                    name: "timer5pam",
                    description: Some(
                        "TIMER5 privilege access mode configuration bit",
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
                    name: "wwdgpam",
                    description: Some(
                        "WWDG privilege access mode configuration bit",
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
                    name: "fwdgpam",
                    description: Some(
                        "FWDG privilege access mode configuration bit",
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
                    name: "spi1pam",
                    description: Some(
                        "SPI1 privilege access mode configuration bit",
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
                    name: "usart1pam",
                    description: Some(
                        "USART1 privilege access mode configuration bit",
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
                    name: "usart2pam",
                    description: Some(
                        "USART2 privilege access mode configuration bit",
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
                    name: "i2c0pam",
                    description: Some(
                        "I2C0 privilege access mode configuration bit",
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
                    name: "i2c1pam",
                    description: Some(
                        "I2C1 privilege access mode configuration bit",
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
                    name: "usbfspam",
                    description: Some(
                        "USBFS privilege access mode configuration bit",
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
                    name: "timer0pam",
                    description: Some(
                        "TIMER0 privilege access mode configuration bit",
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
                    name: "spi0pam",
                    description: Some(
                        "SPI0 privilege access mode configuration bit",
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
            name: "PamCfg1",
            extends: None,
            description: Some(
                "TZSPC privilege access mode configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart0pam",
                    description: Some(
                        "USART0 privilege access mode configuration bit",
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
                    name: "timer15pam",
                    description: Some(
                        "TIMER15 privilege access mode configuration bit",
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
                    name: "timer16pam",
                    description: Some(
                        "TIMER16 privilege access mode configuration bit",
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
                    name: "hpdfpam",
                    description: Some(
                        "HPDF privilege access mode configuration bit",
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
                    name: "crcpam",
                    description: Some(
                        "CRC privilege access mode configuration bit",
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
                    name: "tsipam",
                    description: Some(
                        "TSI privilege access mode configuration bit",
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
                    name: "icachepam",
                    description: Some(
                        "ICACHE register privilege access mode configuration bit",
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
                    name: "adcpam",
                    description: Some(
                        "ADC privilege access mode configuration bit",
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
                    name: "caupam",
                    description: Some(
                        "CAU privilege access mode configuration bit",
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
                    name: "haupam",
                    description: Some(
                        "HAU privilege access mode configuration bit",
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
                    name: "trngpam",
                    description: Some(
                        "TRNG privilege access mode configuration bit",
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
                    name: "pkcaupam",
                    description: Some(
                        "PKCAU privilege access mode configuration bit",
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
                    name: "sdiopam",
                    description: Some(
                        "SDIO privilege access mode configuration bit",
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
            name: "PamCfg2",
            extends: None,
            description: Some(
                "TZSPC privilege access mode configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efusepam",
                    description: Some(
                        "EFUSE register privilege access mode configuration bit",
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
                    name: "dbgpam",
                    description: Some(
                        "DBG register privilege access mode configuration bit",
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
                    name: "sqpi_psramregpam",
                    description: Some(
                        "SQPI PSRAM register privilege access mode configuration bit",
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
                    name: "qspi_flashregpam",
                    description: Some(
                        "QSPI flash register privilege access mode configuration bit",
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
                    name: "wifi_rfpam",
                    description: Some(
                        "WIFI RF privilege access mode configuration bit",
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
                    name: "i2s1addpam",
                    description: Some(
                        "I2S1ADD privilege access mode configuration bit",
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
                    name: "dcipam",
                    description: Some(
                        "DCI privilege access mode configuration bit",
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
                    name: "wifipam",
                    description: Some(
                        "WIFI privilege access mode configuration bit",
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
            name: "SamCfg0",
            extends: None,
            description: Some(
                "TZSPC secure access mode configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer1sam",
                    description: Some(
                        "TIMER1 secure access mode configuration bit",
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
                    name: "timer2sam",
                    description: Some(
                        "TIMER2 secure access mode configuration bit",
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
                    name: "timer3sam",
                    description: Some(
                        "TIMER3 secure access mode configuration bit",
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
                    name: "timer4sam",
                    description: Some(
                        "TIMER4 secure access mode configuration bit",
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
                    name: "timer5sam",
                    description: Some(
                        "TIMER5 secure access mode configuration bit",
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
                    name: "wwdgsam",
                    description: Some(
                        "WWDG secure access mode configuration bit",
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
                    name: "fwdgsam",
                    description: Some(
                        "FWDG secure access mode configuration bit",
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
                    name: "spi1sam",
                    description: Some(
                        "SPI1 secure access mode configuration bit",
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
                    name: "usart1sam",
                    description: Some(
                        "USART1 secure access mode configuration bit",
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
                    name: "usart2sam",
                    description: Some(
                        "USART2 secure access mode configuration bit",
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
                    name: "i2c0sam",
                    description: Some(
                        "I2C0 secure access mode configuration bit",
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
                    name: "i2c1sam",
                    description: Some(
                        "I2C1 secure access mode configuration bit",
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
                    name: "usbfssam",
                    description: Some(
                        "USBFS secure access mode configuration bit",
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
                    name: "timer0sam",
                    description: Some(
                        "TIMER0 secure access mode configuration bit",
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
                    name: "spi0sam",
                    description: Some(
                        "SPI0 secure access mode configuration bit",
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
            name: "SamCfg1",
            extends: None,
            description: Some(
                "TZSPC secure access mode configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart0sam",
                    description: Some(
                        "USART0 secure access mode configuration bit",
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
                    name: "timer15sam",
                    description: Some(
                        "TIMER15 secure access mode configuration bit",
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
                    name: "timer16sam",
                    description: Some(
                        "TIMER16 secure access mode configuration bit",
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
                    name: "hpdfsam",
                    description: Some(
                        "HPDF secure access mode configuration bit",
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
                    name: "crcsam",
                    description: Some(
                        "CRC secure access mode configuration bit",
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
                    name: "tsisam",
                    description: Some(
                        "TSI secure access mode configuration bit",
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
                    name: "icachesam",
                    description: Some(
                        "ICACHE secure access mode configuration bit",
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
                    name: "adcsam",
                    description: Some(
                        "ADC secure access mode configuration bit",
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
                    name: "causam",
                    description: Some(
                        "CAU secure access mode configuration bit",
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
                    name: "hausam",
                    description: Some(
                        "HAU secure access mode configuration bit",
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
                    name: "trngsam",
                    description: Some(
                        "TRNG secure access mode configuration bit",
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
                    name: "pkcausam",
                    description: Some(
                        "PKCAU secure access mode configuration bit",
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
                    name: "sdiosam",
                    description: Some(
                        "SDIO secure access mode configuration bit",
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
            name: "SamCfg2",
            extends: None,
            description: Some(
                "TZSPC secure access mode configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "efusesam",
                    description: Some(
                        "EFUSE register secure access mode configuration bit",
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
                    name: "sqpi_psramregsam",
                    description: Some(
                        "SQPI PSRAM register secure access mode configuration bit",
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
                    name: "qspi_flashregsam",
                    description: Some(
                        "QSPI flash register secure access mode configuration bit",
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
                    name: "wifi_rfsam",
                    description: Some(
                        "WIFI RF secure access mode configuration bit",
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
                    name: "i2s1addsam",
                    description: Some(
                        "I2S1ADD secure access mode configuration bit",
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
                    name: "dcmisam",
                    description: Some(
                        "DCMI secure access mode configuration bit",
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
                    name: "wifisam",
                    description: Some(
                        "WIFI secure access mode configuration bit",
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
            name: "Tzmmpc0Nsm0",
            extends: None,
            description: Some(
                "TZSPC external memory 0 non-secure mark register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm0_sadd",
                    description: Some(
                        "The non-secure area (multiple of 8 Kbytes) start address of TZBPC0",
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
                    name: "nsm0_len",
                    description: Some(
                        "Length of the first non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tzmmpc0Nsm1",
            extends: None,
            description: Some(
                "TZSPC external memory 0 non-secure mark register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm1_sadd",
                    description: Some(
                        "The first non-secure area (multiple of 8 Kbytes) start address",
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
                    name: "nsm1_len",
                    description: Some(
                        "Length of the first non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tzmmpc1Nsm0",
            extends: None,
            description: Some(
                "TZSPC external memory 1 non-secure mark register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm0_sadd",
                    description: Some(
                        "The non-secure area (multiple of 8 Kbytes) start address",
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
                    name: "nsm0_len",
                    description: Some(
                        "Length of the non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tzmmpc1Nsm1",
            extends: None,
            description: Some(
                "TZSPC external memory 1 non-secure mark register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm1_sadd",
                    description: Some(
                        "The non-secure area (multiple of 8 Kbytes) start address",
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
                    name: "nsm1_len",
                    description: Some(
                        "Length of the non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tzmpc0Nsm2",
            extends: None,
            description: Some(
                "TZSPC external memory 0 non-secure mark register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm2_sadd",
                    description: Some(
                        "The non-secure area (multiple of 8 Kbytes) start address",
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
                    name: "nsm2_len",
                    description: Some(
                        "Length of the non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tzmpc0Nsm3",
            extends: None,
            description: Some(
                "TZSPC external memory 0 non-secure mark register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nsm3_sadd",
                    description: Some(
                        "The first non-secure area (multiple of 8 Kbytes) start address",
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
                    name: "nsm3_len",
                    description: Some(
                        "Length of the first non-secure area",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                