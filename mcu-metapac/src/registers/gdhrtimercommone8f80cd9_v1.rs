
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "HrtimerCommon",
            extends: None,
            description: Some(
                "HRTIMER Common",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "HRTIMER control register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
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
                    name: "ctl1",
                    description: Some(
                        "HRTIMER control register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
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
                    name: "intf",
                    description: Some(
                        "HRTIMER interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
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
                        "HRTIMER interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "HRTIMER interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x10,
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
                    name: "chouten",
                    description: Some(
                        "HRTIMER channel output enable register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Chouten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "choutdis",
                    description: Some(
                        "HRTIMER channel output disable register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Choutdis",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "choutdisf",
                    description: Some(
                        "HRTIMER channel output disable flag register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Choutdisf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmctl",
                    description: Some(
                        "HRTIMER bunch mode control register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmstrg",
                    description: Some(
                        "HRTIMER bunch mode start trigger register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmstrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmcmpv",
                    description: Some(
                        "HRTIMER bunch mode compare value register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmcmpv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmcar",
                    description: Some(
                        "HRTIMER bunch mode counter auto reload register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmcar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exevcfg0",
                    description: Some(
                        "HRTIMER external event configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exevcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exevcfg1",
                    description: Some(
                        "HRTIMER external event configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exevcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exevdfctl",
                    description: Some(
                        "HRTIMER external event digital filter control register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Exevdfctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs0",
                    description: Some(
                        "HRTIMER trigger source 0 to ADC register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs1",
                    description: Some(
                        "HRTIMER trigger source 1 to ADC register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs2",
                    description: Some(
                        "HRTIMER trigger source 2 to ADC register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs3",
                    description: Some(
                        "HRTIMER trigger source 3 to ADC register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dllcctl",
                    description: Some(
                        "HRTIMER DLL calibration control register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dllcctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltincfg0",
                    description: Some(
                        "HRTIMER fault input configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltincfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltincfg1",
                    description: Some(
                        "HRTIMER fault input configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltincfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupmtr",
                    description: Some(
                        "HRTIMER DMA update Master_TIMER register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupmtr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst0r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 0",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst0r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst1r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst1r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst2r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 2",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst2r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst3r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 3",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst3r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst4r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 4",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst4r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmatb",
                    description: Some(
                        "HRTIMER DMA transfer buffer register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst5r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 5",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst5r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcexttrg",
                    description: Some(
                        "HRTIMER ADC extended trigger register",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adcexttrg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrgupd",
                    description: Some(
                        "HRTIMER ADC trigger update register",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrgupd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcpscr0",
                    description: Some(
                        "HRTIMER ADC post scaler registe register 0",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adcpscr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcpscr1",
                    description: Some(
                        "HRTIMER ADC post scaler registe register 1",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adcpscr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltincfg2",
                    description: Some(
                        "HRTIMER fault input configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltincfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltincfg3",
                    description: Some(
                        "HRTIMER fault input configuration register 3",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltincfg3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst6r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 6",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst6r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaupst7r",
                    description: Some(
                        "HRTIMER DMA update Slave_TIMERx regist 7",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaupst7r",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bmstrga",
                    description: Some(
                        "HRTIMER bunch mode start trigger add register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bmstrga",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltincfg4",
                    description: Some(
                        "HRTIMER fault input configuration register 4",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltincfg4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adcexttrga",
                    description: Some(
                        "HRTIMER ADC extended trigger add register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adcexttrga",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs0a",
                    description: Some(
                        "HRTIMER trigger source 0 to ADC add register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs0a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs1a",
                    description: Some(
                        "HRTIMER trigger source 1 to ADC add register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs1a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs2a",
                    description: Some(
                        "HRTIMER trigger source 2 to ADC add register",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs2a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adctrigs3a",
                    description: Some(
                        "HRTIMER trigger source 3 to ADC add register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Adctrigs3a",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fltrecctl",
                    description: Some(
                        "HRTIMER fault recovery control register",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fltrecctl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adcexttrg",
            extends: None,
            description: Some(
                "HRTIMER ADC extended trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc4trg",
                    description: Some(
                        "ADC trigger 4 selection This bit selects the ADC trigger 4 source",
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
                    name: "adc5trg",
                    description: Some(
                        "ADC trigger 5 selection This bit selects the ADC trigger 5 source",
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
                    name: "adc6trg",
                    description: Some(
                        "ADC trigger 6 selection This bit selects the ADC trigger 6 source",
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
                    name: "adc7trg",
                    description: Some(
                        "ADC trigger 7 selection This bit selects the ADC trigger 7 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc8trg",
                    description: Some(
                        "ADC trigger 8 selection This bit selects the ADC trigger 8 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc9trg",
                    description: Some(
                        "ADC trigger 9 selection This bit selects the ADC trigger 9 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Adcexttrga",
            extends: None,
            description: Some(
                "HRTIMER ADC extended trigger add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc4trg_5",
                    description: Some(
                        "ADC trigger 4 selection This bit selects the ADC trigger 4 source",
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
                    name: "adc5trg_5",
                    description: Some(
                        "ADC trigger 5 selection This bit selects the ADC trigger 5 source",
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
                    name: "adc6trg_5",
                    description: Some(
                        "ADC trigger 6 selection This bit selects the ADC trigger 6 source",
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
                    name: "adc7trg_5",
                    description: Some(
                        "ADC trigger 7 selection This bit selects the ADC trigger 7 source",
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
                    name: "adc8trg_5",
                    description: Some(
                        "ADC trigger 8 selection This bit selects the ADC trigger 8 source",
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
                    name: "adc9trg_5",
                    description: Some(
                        "ADC trigger 9 selection This bit selects the ADC trigger 9 source",
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
            name: "Adcpscr0",
            extends: None,
            description: Some(
                "HRTIMER ADC post scaler registe register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc0psc",
                    description: Some(
                        "ADC trigger 0 prescaler",
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
                    name: "adc1psc",
                    description: Some(
                        "ADC trigger 1 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2psc",
                    description: Some(
                        "ADC trigger 2 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc3psc",
                    description: Some(
                        "ADC trigger 3 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc4psc",
                    description: Some(
                        "ADC trigger 4 prescaler",
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
            name: "Adcpscr1",
            extends: None,
            description: Some(
                "HRTIMER ADC post scaler registe register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc5psc",
                    description: Some(
                        "ADC trigger 5 prescaler",
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
                    name: "adc6psc",
                    description: Some(
                        "ADC trigger 6 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc7psc",
                    description: Some(
                        "ADC trigger 7 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc8psc",
                    description: Some(
                        "ADC trigger 8 prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc9psc",
                    description: Some(
                        "ADC trigger 9 prescaler",
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
            name: "Adctrgupd",
            extends: None,
            description: Some(
                "HRTIMER ADC trigger update register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adtg4usrc",
                    description: Some(
                        "ADC4TRG update source",
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
                    name: "adtg5usrc",
                    description: Some(
                        "ADC trigger5 update source",
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
                    name: "adtg6usrc",
                    description: Some(
                        "ADC trigger 6update source",
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
                    name: "adtg7usrc",
                    description: Some(
                        "ADC trigger 7 update source",
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
                Field {
                    name: "adtg8usrc",
                    description: Some(
                        "ADC trigger 8 update source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adtg9usrc",
                    description: Some(
                        "ADC trigger 9 update source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Adctrigs0",
            extends: None,
            description: Some(
                "HRTIMER trigger source 0 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg0mtc0",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Master_TIMER compare 0 event",
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
                    name: "trg0mtc1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Master_TIMER compare 1 event",
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
                    name: "trg0mtc2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Master_TIMER compare 2 event",
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
                    name: "trg0mtc3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Master_TIMER compare 3 event",
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
                    name: "trg0mtper",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Master_TIMER period event",
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
                    name: "trg0exev0",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on external event 0",
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
                    name: "trg0exev1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on external event 1",
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
                    name: "trg0exev2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on external event 2",
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
                    name: "trg0exev3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on external event 3",
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
                    name: "trg0exev4",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on external event 4",
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
                    name: "trg0st0c1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER0 compare 1 event",
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
                    name: "trg0st0c2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER0 compare 2 event",
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
                    name: "trg0st0c3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER0 compare 3 event",
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
                    name: "trg0st0per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER0 period event",
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
                    name: "trg0st0rst",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER0 reset",
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
                    name: "trg0st1c1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER1 compare 1 event",
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
                    name: "trg0st1c2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER1 compare 2 event",
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
                    name: "trg0st1c3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER1 compare 3 event",
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
                    name: "trg0st1per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER1 period event",
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
                    name: "trg0st1rst",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER1 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg0st2c1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER2 compare 1 event",
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
                    name: "trg0st2c2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER2 compare 2 event",
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
                    name: "trg0st2c3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER2 compare 3 event",
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
                    name: "trg0st2per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER2 period event",
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
                    name: "trg0st3c1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER3 compare 1 event",
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
                    name: "trg0st3c2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER3 compare 2 event",
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
                    name: "trg0st3c3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER3 compare 3 event",
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
                    name: "trg0st3per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER3 period event",
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
                    name: "trg0st4c1",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER4 compare 1 event",
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
                    name: "trg0st4c2",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER4 compare 2 event",
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
                    name: "trg0st4c3",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER4 compare 3 event",
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
                    name: "trg0st4per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER4 period event",
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
            name: "Adctrigs0a",
            extends: None,
            description: Some(
                "HRTIMER trigger source 0 to ADC add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg0st5c1",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER5 compare 1 event",
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
                    name: "trg0st5c2",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER5 compare 2 event",
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
                    name: "trg0st5c3",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER5 compare 3 event",
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
                    name: "trg0st5per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER5 period event",
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
                    name: "trg0st5rst",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER5 reset",
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
                    name: "trg0st6c1",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER6 compare 1 event",
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
                    name: "trg0st6c2",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER6 compare 2 event",
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
                    name: "trg0st6c3",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER6 compare 3 event",
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
                    name: "trg0st6per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER6 period event",
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
                    name: "trg0st6rst",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER6 reset",
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
                    name: "trg0st7c1",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER7 compare 1 event",
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
                    name: "trg0st7c2",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER7 compare 2 event",
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
                    name: "trg0st7c3",
                    description: Some(
                        "HRTIMER_ADCTRG0 on Slave_TIMER7 compare 3 event",
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
                    name: "trg0st7per",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER7 period event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg0st7rst",
                    description: Some(
                        "HRTIMER_ADCTRIG0 on Slave_TIMER7 reset",
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
            ],
        },
        FieldSet {
            name: "Adctrigs1",
            extends: None,
            description: Some(
                "HRTIMER trigger source 1 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg1mtc0",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Master_TIMER compare 0 event",
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
                    name: "trg1mtc1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Master_TIMER compare 1 event",
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
                    name: "trg1mtc2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Master_TIMER compare 2 event",
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
                    name: "trg1mtc3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Master_TIMER compare 3 event",
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
                    name: "trg1mtper",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Master_TIMER period event",
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
                    name: "trg1exev5",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on external event 5",
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
                    name: "trg1exev6",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on external event 6",
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
                    name: "trg1exev7",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on external event 7",
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
                    name: "trg1exev8",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on external event 8",
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
                    name: "trg1exev9",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on external event 9",
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
                    name: "trg1st0c1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER0 compare 1 event",
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
                    name: "trg1st0c2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER0 compare 2 event",
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
                    name: "trg1st0c3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER0 compare 3 event",
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
                    name: "trg1st0per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER0 period event",
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
                    name: "trg1st1c1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER1 compare 1 event",
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
                    name: "trg1st1c2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER1 compare 2 event",
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
                    name: "trg1st1c3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER1 compare 3 event",
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
                    name: "trg1st1per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER1 period event",
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
                    name: "trg1st2c1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER2 compare 1 event",
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
                    name: "trg1st2c2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER2 compare 2 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg1st2c3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER2 compare 3 event",
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
                    name: "trg1st2per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER2 period event",
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
                    name: "trg1st2rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER2 reset",
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
                    name: "trg1st3c1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER3 compare 1 event",
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
                    name: "trg1st3c2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER3 compare 2 event",
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
                    name: "trg1st3c3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER3 compare 3 event",
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
                    name: "trg1st3per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER3 period event",
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
                    name: "trg1st3rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER3 reset",
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
                    name: "trg1st4c1",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER4 compare 1 event",
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
                    name: "trg1st4c2",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER4 compare 2 event",
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
                    name: "trg1st4c3",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER4 compare 3 event",
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
                    name: "trg1st4rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER4 reset",
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
            name: "Adctrigs1a",
            extends: None,
            description: Some(
                "HRTIMER trigger source 1 to ADC add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg1st5c1",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER5 compare 1 event",
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
                    name: "trg1st5c2",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER5 compare 2 event",
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
                    name: "trg1st5c3",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER5 compare 3 event",
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
                    name: "trg1st5per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER5 period event",
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
                    name: "trg1st5rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER5 reset",
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
                    name: "trg1st6c1",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER6 compare 1 event",
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
                    name: "trg1st6c2",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER6 compare 2 event",
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
                    name: "trg1st6c3",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER6 compare 3 event",
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
                    name: "trg1st6per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER6 period event",
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
                    name: "trg1st6rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER6 reset",
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
                    name: "trg1st7c1",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER7 compare 1 event",
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
                    name: "trg1st7c2",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER7 compare 2 event",
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
                    name: "trg1st7c3",
                    description: Some(
                        "HRTIMER_ADCTRG1 on Slave_TIMER7 compare 3 event",
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
                    name: "trg1st7per",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER7 period event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg1st7rst",
                    description: Some(
                        "HRTIMER_ADCTRIG1 on Slave_TIMER7 reset",
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
            ],
        },
        FieldSet {
            name: "Adctrigs2",
            extends: None,
            description: Some(
                "HRTIMER trigger source 2 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg2mtc0",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Master_TIMER compare 0 event",
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
                    name: "trg2mtc1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Master_TIMER compare 1 event",
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
                    name: "trg2mtc2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Master_TIMER compare 2 event",
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
                    name: "trg2mtc3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Master_TIMER compare 3 event",
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
                    name: "trg2mtper",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Master_TIMER period event",
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
                    name: "trg2exev0",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on external event 0",
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
                    name: "trg2exev1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on external event 1",
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
                    name: "trg2exev2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on external event 2",
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
                    name: "trg2exev3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on external event 3",
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
                    name: "trg2exev4",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on external event 4",
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
                    name: "trg2st0c1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER0 compare 1 event",
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
                    name: "trg2st0c2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER0 compare 2 event",
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
                    name: "trg2st0c3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER0 compare 3 event",
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
                    name: "trg2st0per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER0 period event",
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
                    name: "trg2st0rst",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER0 reset",
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
                    name: "trg2st1c1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER1 compare 1 event",
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
                    name: "trg2st1c2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER1 compare 2 event",
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
                    name: "trg2st1c3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER1 compare 3 event",
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
                    name: "trg2st1per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER1 period event",
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
                    name: "trg2st1rst",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER1 reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg2st2c1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER2 compare 1 event",
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
                    name: "trg2st2c2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER2 compare 2 event",
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
                    name: "trg2st2c3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER2 compare 3 event",
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
                    name: "trg2st2per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER2 period event",
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
                    name: "trg2st3c1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER3 compare 1 event",
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
                    name: "trg2st3c2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER3 compare 2 event",
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
                    name: "trg2st3c3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER3 compare 3 event",
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
                    name: "trg2st3per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER3 period event",
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
                    name: "trg2st4c1",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER4 compare 1 event",
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
                    name: "trg2st4c2",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER4 compare 2 event",
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
                    name: "trg2st4c3",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER4 compare 3 event",
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
                    name: "trg2st4per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER4 period event",
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
            name: "Adctrigs2a",
            extends: None,
            description: Some(
                "HRTIMER trigger source 2 to ADC add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg2st5c1",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER5 compare 1 event",
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
                    name: "trg2st5c2",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER5 compare 2 event",
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
                    name: "trg2st5c3",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER5 compare 3 event",
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
                    name: "trg2st5per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER5 period event",
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
                    name: "trg2st5rst",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER5 reset",
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
                    name: "trg2st6c1",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER6 compare 1 event",
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
                    name: "trg2st6c2",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER6 compare 2 event",
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
                    name: "trg2st6c3",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER6 compare 3 event",
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
                    name: "trg2st6per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER6 period event",
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
                    name: "trg2st6rst",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER6 reset",
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
                    name: "trg2st7c1",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER7 compare 1 event",
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
                    name: "trg2st7c2",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER7 compare 2 event",
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
                    name: "trg2st7c3",
                    description: Some(
                        "HRTIMER_ADCTRG2 on Slave_TIMER7 compare 3 event",
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
                    name: "trg2st7per",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER7 period event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg2st7rst",
                    description: Some(
                        "HRTIMER_ADCTRIG2 on Slave_TIMER7 reset",
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
            ],
        },
        FieldSet {
            name: "Adctrigs3",
            extends: None,
            description: Some(
                "HRTIMER trigger source 3 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg3mtc0",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Master_TIMER compare 0 event",
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
                    name: "trg3mtc1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Master_TIMER compare 1 event",
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
                    name: "trg3mtc2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Master_TIMER compare 2 event",
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
                    name: "trg3mtc3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Master_TIMER compare 3 event",
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
                    name: "trg3mtper",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Master_TIMER period event",
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
                    name: "trg3exev5",
                    description: Some(
                        "HRTIMER_ADCTRG3 on external event 5",
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
                    name: "trg3exev6",
                    description: Some(
                        "HRTIMER_ADCTRG3 on external event 6",
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
                    name: "trg3exev7",
                    description: Some(
                        "HRTIMER_ADCTRG3 on external event 7",
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
                    name: "trg3exev8",
                    description: Some(
                        "HRTIMER_ADCTRG3 on external event 8",
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
                    name: "trg3exev9",
                    description: Some(
                        "HRTIMER_ADCTRG3 on external event 9",
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
                    name: "trg3st0c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER0 compare 1 event",
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
                    name: "trg3st0c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER0 compare 2 event",
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
                    name: "trg3st0c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER0 compare 3 event",
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
                    name: "trg3st0per",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER0 period event",
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
                    name: "trg3st1c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER1 compare 1 event",
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
                    name: "trg3st1c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER1 compare 2 event",
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
                    name: "trg3st1c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER1 compare 3 event",
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
                    name: "trg3st1per",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER1 period event",
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
                    name: "trg3st2c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER2 compare 1 event",
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
                    name: "trg3st2c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER2 compare 2 event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg3st2c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER2 compare 3 event",
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
                    name: "trg3st2per",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER2 period event",
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
                    name: "trg3st2rst",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER2 reset",
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
                    name: "trg3st3c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER3 compare 1 event",
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
                    name: "trg3st3c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER3 compare 2 event",
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
                    name: "trg3st3c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER3 compare 3 event",
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
                    name: "trg3st3per",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER3 period event",
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
                    name: "trg3st3rst",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER3 reset",
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
                    name: "trg3st4c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER4 compare 1 event",
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
                    name: "trg3st4c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER4 compare 2 event",
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
                    name: "trg3st4c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER4 compare 3 event",
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
                    name: "trg3st4rst",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER4 reset",
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
            name: "Adctrigs3a",
            extends: None,
            description: Some(
                "HRTIMER trigger source 3 to ADC add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg3st5c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER5 compare 1 event",
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
                    name: "trg3st5c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER5 compare 2 event",
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
                    name: "trg3st5c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER5 compare 3 event",
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
                    name: "trg3st5per",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER5 period event",
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
                    name: "trg3st5rst",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER5 reset",
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
                    name: "trg3st6c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER6 compare 1 event",
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
                    name: "trg3st6c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER6 compare 2 event",
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
                    name: "trg3st6c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER6 compare 3 event",
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
                    name: "trg3st6per",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER6 period event",
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
                    name: "trg3st6rst",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER6 reset",
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
                    name: "trg3st7c1",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER7 compare 1 event",
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
                    name: "trg3st7c2",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER7 compare 2 event",
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
                    name: "trg3st7c3",
                    description: Some(
                        "HRTIMER_ADCTRG3 on Slave_TIMER7 compare 3 event",
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
                    name: "trg3st7per",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER7 period event",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "trg3st7rst",
                    description: Some(
                        "HRTIMER_ADCTRIG3 on Slave_TIMER7 reset",
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
            ],
        },
        FieldSet {
            name: "Bmcar",
            extends: None,
            description: Some(
                "HRTIMER bunch mode counter auto reload register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmcarl",
                    description: Some(
                        "Bunch mode counter auto reload value",
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
            name: "Bmcmpv",
            extends: None,
            description: Some(
                "HRTIMER bunch mode compare value register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmcmpval",
                    description: Some(
                        "Bunch mode compare value",
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
            name: "Bmctl",
            extends: None,
            description: Some(
                "HRTIMER bunch mode control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bmen",
                    description: Some(
                        "Bunch mode enable",
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
                    name: "bmctn",
                    description: Some(
                        "Continuous mode in bunch mode",
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
                    name: "bmclks",
                    description: Some(
                        "Bunch mode clock source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmpsc",
                    description: Some(
                        "Bunch mode clock division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmse",
                    description: Some(
                        "Bunch mode shadow enable",
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
                    name: "bmmt",
                    description: Some(
                        "Master_TIMER bunch mode",
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
                    name: "bmst0",
                    description: Some(
                        "Slave_TIMER0 bunch mode",
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
                    name: "bmst1",
                    description: Some(
                        "Slave_TIMER1 bunch mode",
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
                    name: "bmst2",
                    description: Some(
                        "Slave_TIMER2 bunch mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bmst3",
                    description: Some(
                        "Slave_TIMER3 bunch mode",
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
                    name: "bmst4",
                    description: Some(
                        "Slave_TIMER4 bunch mode",
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
                    name: "bmst5",
                    description: Some(
                        "Slave_TIMER5 bunch mode",
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
                    name: "bmst6",
                    description: Some(
                        "Slave_TIMER6 bunch mode",
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
                    name: "bmst7",
                    description: Some(
                        "Slave_TIMER7 bunch mode",
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
                    name: "bmoptf",
                    description: Some(
                        "Bunch mode operating flag",
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
            name: "Bmstrg",
            extends: None,
            description: Some(
                "HRTIMER bunch mode start trigger register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swtrg",
                    description: Some(
                        "Software triggers bunch mode operation",
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
                    name: "mtrst",
                    description: Some(
                        "Master_TIMER reset event triggers bunch mode operation",
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
                    name: "mtrep",
                    description: Some(
                        "Master_TIMER repetition event triggers bunch mode operation",
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
                    name: "mtcmp0",
                    description: Some(
                        "Master_TIMER compare 0 event triggers bunch mode operation",
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
                    name: "mtcmp1",
                    description: Some(
                        "Master_TIMER compare 1 event triggers bunch mode operation",
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
                    name: "mtcmp2",
                    description: Some(
                        "Master_TIMER compare 2 event triggers bunch mode operation",
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
                    name: "mtcmp3",
                    description: Some(
                        "Master_TIMER compare 3 event triggers bunch mode operation",
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
                    name: "st0rst",
                    description: Some(
                        "Slave_TIMER0 reset event triggers bunch mode operation",
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
                    name: "st0rep",
                    description: Some(
                        "Slave_TIMER0 repetition event triggers bunch mode operation",
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
                    name: "st0cmp0",
                    description: Some(
                        "Slave_TIMER0 compare 0 event triggers bunch mode operation",
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
                    name: "st0cmp1",
                    description: Some(
                        "Slave_TIMER0 compare 1 event triggers bunch mode operation",
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
                    name: "st1rst",
                    description: Some(
                        "Slave_TIMER1 reset event triggers bunch mode operation",
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
                    name: "st1rep",
                    description: Some(
                        "Slave_TIMER1 repetition event triggers bunch mode operation",
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
                    name: "st1cmp0",
                    description: Some(
                        "Slave_TIMER1 compare 0 event triggers bunch mode operation",
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
                    name: "st1cmp1",
                    description: Some(
                        "Slave_TIMER1 compare 1 event triggers bunch mode operation",
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
                    name: "st2rst",
                    description: Some(
                        "Slave_TIMER2 reset event triggers bunch mode operation",
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
                    name: "st2rep",
                    description: Some(
                        "Slave_TIMER2 repetition event triggers bunch mode operation",
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
                    name: "st2cmp0",
                    description: Some(
                        "Slave_TIMER2 compare 0 event triggers bunch mode operation",
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
                    name: "st2cmp1",
                    description: Some(
                        "Slave_TIMER2 compare 1 event triggers bunch mode operation",
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
                    name: "st3rst",
                    description: Some(
                        "Slave_TIMER3 reset event triggers bunch mode operation",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st3rep",
                    description: Some(
                        "Slave_TIMER3 repetition event triggers bunch mode operation",
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
                    name: "st3cmp0",
                    description: Some(
                        "Slave_TIMER3 compare 0 event triggers bunch mode operation",
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
                    name: "st3cmp1",
                    description: Some(
                        "Slave_TIMER3 compare 1 event triggers bunch mode operation",
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
                    name: "st4rst",
                    description: Some(
                        "Slave_TIMER4 reset event triggers bunch mode operation",
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
                    name: "st4rep",
                    description: Some(
                        "Slave_TIMER4 repetition event triggers bunch mode operation",
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
                    name: "st4cmp0",
                    description: Some(
                        "Slave_TIMER4 compare 0 event triggers bunch mode operation",
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
                    name: "st4cmp1",
                    description: Some(
                        "Slave_TIMER4 compare 1 event triggers bunch mode operation",
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
                    name: "st0exev6",
                    description: Some(
                        "Slave_TIMER0 period event following external event 6 triggers bunch mode operation",
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
                    name: "st3exev7",
                    description: Some(
                        "Slave_TIMER3 period event following external event 7 triggers bunch mode operation",
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
                    name: "exev6",
                    description: Some(
                        "External event 6 triggers bunch mode operation",
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
                    name: "exev7",
                    description: Some(
                        "External event 7 triggers bunch mode operation",
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
                    name: "cisgn",
                    description: Some(
                        "Chip internal signal triggers bunch mode operation",
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
            name: "Bmstrga",
            extends: None,
            description: Some(
                "HRTIMER bunch mode start trigger add register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st5rst",
                    description: Some(
                        "Slave_TIMER5 reset event triggers bunch mode operation",
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
                    name: "st5rep",
                    description: Some(
                        "Slave_TIMER5 repetition event triggers bunch mode operation",
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
                    name: "st5cmp0",
                    description: Some(
                        "Slave_TIMER5 compare 0 event triggers bunch mode operation",
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
                    name: "st5cmp1",
                    description: Some(
                        "Slave_TIMER5 compare 1 event triggers bunch mode operation",
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
                    name: "st6rst",
                    description: Some(
                        "Slave_TIMER6 reset event triggers bunch mode operation",
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
                    name: "st6rep",
                    description: Some(
                        "Slave_TIMER6 repetition event triggers bunch mode operation",
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
                    name: "st6cmp0",
                    description: Some(
                        "Slave_TIMER6 compare 0 event triggers bunch mode operation",
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
                    name: "st6cmp1",
                    description: Some(
                        "Slave_TIMER6 compare 1 event triggers bunch mode operation",
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
                    name: "st7rst",
                    description: Some(
                        "Slave_TIMER7 reset event triggers bunch mode operation",
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
                    name: "st7rep",
                    description: Some(
                        "Slave_TIMER7 repetition event triggers bunch mode operation",
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
                    name: "st7cmp0",
                    description: Some(
                        "Slave_TIMER7 compare 0 event triggers bunch mode operation",
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
                    name: "st7cmp1",
                    description: Some(
                        "Slave_TIMER7 compare 1 event triggers bunch mode operation",
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
            ],
        },
        FieldSet {
            name: "Choutdis",
            extends: None,
            description: Some(
                "HRTIMER channel output disable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st0ch0dis",
                    description: Some(
                        "Slave_TIMER0 channel 0 output (ST0CH0_O) disable",
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
                    name: "st0ch1dis",
                    description: Some(
                        "Slave_TIMER0 channel 1 output (ST0CH1_O) disable",
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
                    name: "st1ch0dis",
                    description: Some(
                        "Slave_TIMER1 channel 0 output (ST1CH0_O) disable",
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
                    name: "st1ch1dis",
                    description: Some(
                        "Slave_TIMER1 channel 1 output (ST1CH1_O) disable",
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
                    name: "st2ch0dis",
                    description: Some(
                        "Slave_TIMER2 channel 0 output (ST2CH0_O) disable",
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
                    name: "st2ch1dis",
                    description: Some(
                        "Slave_TIMER2 channel 1 output (ST2CH1_O) disable",
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
                    name: "st3ch0dis",
                    description: Some(
                        "Slave_TIMER3 channel 0 output (ST3CH0_O) disable",
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
                    name: "st3ch1dis",
                    description: Some(
                        "Slave_TIMER3 channel 1 output (ST3CH1_O) disable",
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
                    name: "st4ch0dis",
                    description: Some(
                        "Slave_TIMER4 channel 0 output (ST4CH0_O) disable",
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
                    name: "st4ch1dis",
                    description: Some(
                        "Slave_TIMER4 channel 1 output (ST4CH1_O) disable",
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
                    name: "st5ch0dis",
                    description: Some(
                        "Slave_TIMER5 channel 0 output (ST5CH0_O) disable",
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
                    name: "st5ch1dis",
                    description: Some(
                        "Slave_TIMER5 channel 1 output (ST5CH1_O) disable",
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
                    name: "st6ch0dis",
                    description: Some(
                        "Slave_TIMER6 channel 0 output (ST6CH0_O) disable",
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
                    name: "st6ch1dis",
                    description: Some(
                        "Slave_TIMER6 channel 1 output (ST6CH1_O) disable",
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
                    name: "st7ch0dis",
                    description: Some(
                        "Slave_TIMER7 channel 0 output (ST7CH0_O) disable",
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
                    name: "st7ch1dis",
                    description: Some(
                        "Slave_TIMER7 channel 1 output (ST7CH1_O) disable",
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
            name: "Choutdisf",
            extends: None,
            description: Some(
                "HRTIMER channel output disable flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st0ch0disf",
                    description: Some(
                        "Slave_TIMER0 channel 0 output (ST0CH0_O) disable flag",
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
                    name: "st0ch1disf",
                    description: Some(
                        "Slave_TIMER0 channel 1 output (ST0CH1_O) disable flag",
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
                    name: "st1ch0disf",
                    description: Some(
                        "Slave_TIMER1 channel 0 output (ST1CH0_O) disable flag",
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
                    name: "st1ch1disf",
                    description: Some(
                        "Slave_TIMER1 channel 1 output (ST1CH1_O) disable flag",
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
                    name: "st2ch0disf",
                    description: Some(
                        "Slave_TIMER2 channel 0 output (ST2CH0_O) disable flag",
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
                    name: "st2ch1disf",
                    description: Some(
                        "Slave_TIMER2 channel 1 output (ST2CH1_O) disable flag",
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
                    name: "st3ch0disf",
                    description: Some(
                        "Slave_TIMER3 channel 0 output (ST3CH0_O) disable flag",
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
                    name: "st3ch1disf",
                    description: Some(
                        "Slave_TIMER3 channel 1 output (ST3CH1_O) disable flag",
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
                    name: "st4ch0disf",
                    description: Some(
                        "Slave_TIMER4 channel 0 output (ST4CH0_O) disable flag",
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
                    name: "st4ch1disf",
                    description: Some(
                        "Slave_TIMER4 channel 1 output (ST4CH1_O) disable flag",
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
                    name: "st5ch0disf",
                    description: Some(
                        "Slave_TIMER5 channel 0 output (ST5CH0_O) disable flag",
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
                    name: "st5ch1disf",
                    description: Some(
                        "Slave_TIMER5 channel 1 output (ST5CH1_O) disable flag",
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
                    name: "st6ch0disf",
                    description: Some(
                        "Slave_TIMER6 channel 0 output (ST6CH0_O) disable flag",
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
                    name: "st6ch1disf",
                    description: Some(
                        "Slave_TIMER6 channel 1 output (ST6CH1_O) disable flag",
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
                    name: "st7ch0disf",
                    description: Some(
                        "Slave_TIMER7 channel 0 output (ST7CH0_O) disable flag",
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
                    name: "st7ch1disf",
                    description: Some(
                        "Slave_TIMER7 channel 1 output (ST7CH1_O) disable flag",
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
            name: "Chouten",
            extends: None,
            description: Some(
                "HRTIMER channel output enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st0ch0en",
                    description: Some(
                        "Slave_TIMER0 channel 0 output (ST0CH0_O) enable",
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
                    name: "st0ch1en",
                    description: Some(
                        "Slave_TIMER0 channel 1 output (ST0CH1_O) enable",
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
                    name: "st1ch0en",
                    description: Some(
                        "Slave_TIMER1 channel 0 output (ST1CH0_O) enable",
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
                    name: "st1ch1en",
                    description: Some(
                        "Slave_TIMER1 channel 1 output (ST1CH1_O) enable",
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
                    name: "st2ch0en",
                    description: Some(
                        "Slave_TIMER2 channel 0 output (ST2CH0_O) enable",
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
                    name: "st2ch1en",
                    description: Some(
                        "Slave_TIMER2 channel 1 output (ST2CH1_O) enable",
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
                    name: "st3ch0en",
                    description: Some(
                        "Slave_TIMER3 channel 0 output (ST3CH0_O) enable",
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
                    name: "st3ch1en",
                    description: Some(
                        "Slave_TIMER3 channel 1 output (ST3CH1_O) enable",
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
                    name: "st4ch0en",
                    description: Some(
                        "Slave_TIMER4 channel 0 output (ST4CH0_O) enable",
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
                    name: "st4ch1en",
                    description: Some(
                        "Slave_TIMER4 channel 1 output (ST4CH1_O) enable",
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
                    name: "st5ch0en",
                    description: Some(
                        "Slave_TIMER5 channel 0 output (ST5CH0_O) enable",
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
                    name: "st5ch1en",
                    description: Some(
                        "Slave_TIMER5 channel 1 output (ST5CH1_O) enable",
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
                    name: "st6ch0en",
                    description: Some(
                        "Slave_TIMER6 channel 0 output (ST6CH0_O) enable",
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
                    name: "st6ch1en",
                    description: Some(
                        "Slave_TIMER6 channel 1 output (ST6CH1_O) enable",
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
                    name: "st7ch0en",
                    description: Some(
                        "Slave_TIMER7 channel 0 output (ST7CH0_O) enable",
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
                    name: "st7ch1en",
                    description: Some(
                        "Slave_TIMER7 channel 1 output (ST7CH1_O) enable",
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
            name: "Ctl0",
            extends: None,
            description: Some(
                "HRTIMER control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mtupdis",
                    description: Some(
                        "Master_TIMER update disable",
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
                    name: "st0updis",
                    description: Some(
                        "Slave_TIMER0 update disable",
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
                    name: "st1updis",
                    description: Some(
                        "Slave_TIMER1 update disable",
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
                    name: "st2updis",
                    description: Some(
                        "Slave_TIMER2 update disable",
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
                    name: "st3updis",
                    description: Some(
                        "Slave_TIMER3 update disable",
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
                    name: "st4updis",
                    description: Some(
                        "Slave_TIMER4 update disable",
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
                    name: "st5updis",
                    description: Some(
                        "Slave_TIMER5 update disable",
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
                    name: "st6updis",
                    description: Some(
                        "Slave_TIMER6 update disable",
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
                    name: "st7updis",
                    description: Some(
                        "Slave_TIMER7 update disable",
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
                    name: "adtg0usrc0_2",
                    description: Some(
                        "HRTIMER_ADTRIG0 update source",
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
                    name: "adtg1usrc0_2",
                    description: Some(
                        "HRTIMER_ADTRIG1 update source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adtg2usrc0_2",
                    description: Some(
                        "HRTIMER_ADTRIG2 update source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adtg3usrc0_2",
                    description: Some(
                        "HRTIMER_ADTRIG3 update source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adtg0usrc_3",
                    description: Some(
                        "HRTIMER_ADTRIG0 update source",
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
                    name: "adtg1usrc_3",
                    description: Some(
                        "HRTIMER_ADTRIG1 update source",
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
                    name: "adtg2usrc_3",
                    description: Some(
                        "HRTIMER_ADTRIG2 update source",
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
                    name: "adtg3usrc_3",
                    description: Some(
                        "HRTIMER_ADTRIG3 update source",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "HRTIMER control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mtsup",
                    description: Some(
                        "Master_TIMER software update",
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
                    name: "st0sup",
                    description: Some(
                        "Slave_TIMER0 software update",
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
                    name: "st1sup",
                    description: Some(
                        "Slave_TIMER1 software update",
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
                    name: "st2sup",
                    description: Some(
                        "Slave_TIMER2 software update",
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
                    name: "st3sup",
                    description: Some(
                        "Slave_TIMER3 software update",
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
                    name: "st4sup",
                    description: Some(
                        "Slave_TIMER4 software update",
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
                    name: "st5sup",
                    description: Some(
                        "Slave_TIMER5 software update",
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
                    name: "st6sup",
                    description: Some(
                        "Slave_TIMER6 software update",
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
                    name: "mtsrst",
                    description: Some(
                        "Master_TIMER software reset",
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
                    name: "st0srst",
                    description: Some(
                        "Slave_TIMER0 software reset",
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
                    name: "st1srst",
                    description: Some(
                        "Slave_TIMER1 software reset",
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
                    name: "st2srst",
                    description: Some(
                        "Slave_TIMER2 software reset",
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
                    name: "st3srst",
                    description: Some(
                        "Slave_TIMER3 software reset",
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
                    name: "st4srst",
                    description: Some(
                        "Slave_TIMER4 software reset",
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
                    name: "st5srst",
                    description: Some(
                        "Slave_TIMER5 software reset",
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
                    name: "st6srst",
                    description: Some(
                        "Slave_TIMER6 software reset",
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
                    name: "exc0",
                    description: Some(
                        "Exchange Slave_TIMER0 outputs",
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
                    name: "exc1",
                    description: Some(
                        "Exchange Slave_TIMER1 outputs",
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
                    name: "exc2",
                    description: Some(
                        "Exchange Slave_TIMER2 outputs",
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
                    name: "exc3",
                    description: Some(
                        "Exchange Slave_TIMER3 outputs",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exc4",
                    description: Some(
                        "Exchange Slave_TIMER4 outputs",
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
                    name: "exc5",
                    description: Some(
                        "Exchange Slave_TIMER5 outputs",
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
                    name: "exc6",
                    description: Some(
                        "Exchange Slave_TIMER6 outputs",
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
                    name: "exc7",
                    description: Some(
                        "Exchange Slave_TIMER7 outputs",
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
                    name: "st7sup",
                    description: Some(
                        "Slave_TIMER7 software update",
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
                    name: "st7srst",
                    description: Some(
                        "Slave_TIMER7 software reset",
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
            name: "Dllcctl",
            extends: None,
            description: Some(
                "HRTIMER DLL calibration control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clbstrt",
                    description: Some(
                        "DLL calibration start once",
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
                    name: "clbperen",
                    description: Some(
                        "DLL periodic calibration enable",
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
                    name: "clbper",
                    description: Some(
                        "DLL calibration period",
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
            ],
        },
        FieldSet {
            name: "Dmatb",
            extends: None,
            description: Some(
                "HRTIMER DMA transfer buffer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmatb",
                    description: Some(
                        "DMA transfer buffer",
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
            name: "Dmaupmtr",
            extends: None,
            description: Some(
                "HRTIMER DMA update Master_TIMER register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mtctl0",
                    description: Some(
                        "HRTIMER_MTCTL0 update by DMA mode",
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
                    name: "mtintc",
                    description: Some(
                        "HRTIMER_MTINTC update by DMA mode",
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
                    name: "mtdmainten",
                    description: Some(
                        "HRTIMER_MTDMAINTEN update by DMA mode",
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
                    name: "mtcnt",
                    description: Some(
                        "HRTIMER_MTCNT update by DMA mode",
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
                    name: "mtcar",
                    description: Some(
                        "HRTIMER_MTCAR update by DMA mode",
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
                    name: "mtcrep",
                    description: Some(
                        "HRTIMER_MTCREP update by DMA mode",
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
                    name: "mtcmp0v",
                    description: Some(
                        "HRTIMER_MTCMP0V update by DMA mode",
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
                    name: "mtcmp1v",
                    description: Some(
                        "HRTIMER_MTCMP1V update by DMA mode",
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
                    name: "mtcmp2v",
                    description: Some(
                        "HRTIMER_MTCMP2V update by DMA mode",
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
                    name: "mtcmp3v",
                    description: Some(
                        "HRTIMER_MTCMP3V update by DMA mode",
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
            ],
        },
        FieldSet {
            name: "Dmaupst0r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st0ctl0",
                    description: Some(
                        "HRTIMER_ST0CTL0 update by DMA mode",
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
                    name: "st0intc",
                    description: Some(
                        "HRTIMER_ST0INTC update by DMA mode",
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
                    name: "st0dmainten",
                    description: Some(
                        "HRTIMER_ST0DMAINTEN update by DMA mode",
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
                    name: "st0cnt",
                    description: Some(
                        "HRTIMER_ST0CNT update by DMA mode",
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
                    name: "st0car",
                    description: Some(
                        "HRTIMER_ST0CAR update by DMA mode",
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
                    name: "st0crep",
                    description: Some(
                        "HRTIMER_ST0CREP update by DMA mode",
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
                    name: "st0cmp0v",
                    description: Some(
                        "HRTIMER_ST0CMP0V update by DMA mode",
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
                    name: "st0cmp1v",
                    description: Some(
                        "HRTIMER_ST0CMP1V update by DMA mode",
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
                    name: "st0cmp2v",
                    description: Some(
                        "HRTIMER_ST0CMP2V update by DMA mode",
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
                    name: "st0cmp3v",
                    description: Some(
                        "HRTIMER_ST0CMP3V update by DMA mode",
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
                    name: "st0dtctl",
                    description: Some(
                        "HRTIMER_ST0DTCTL update by DMA mode",
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
                    name: "st0ch0set",
                    description: Some(
                        "HRTIMER_ST0CH0SET update by DMA mode",
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
                    name: "st0ch0rst",
                    description: Some(
                        "HRTIMER_ST0CH0RST update by DMA mode",
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
                    name: "st0ch1set",
                    description: Some(
                        "HRTIMER_ST0CH1SET update by DMA mode",
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
                    name: "st0ch1rst",
                    description: Some(
                        "HRTIMER_ST0CH1RST update by DMA mode",
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
                    name: "st0exevfcfg0",
                    description: Some(
                        "HRTIMER_ST0EXEVFCFG0update by DMA mode",
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
                    name: "st0exevfcfg1",
                    description: Some(
                        "HRTIMER_ST0EXEVFCFG1update by DMA mode",
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
                    name: "st0cntrst",
                    description: Some(
                        "HRTIMER_ST0CNTRST update by DMA mode",
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
                    name: "st0csctl",
                    description: Some(
                        "HRTIMER_ST0CSCTL update by DMA mode",
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
                    name: "st0choctl",
                    description: Some(
                        "HRTIMER_ST0CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st0fltctl",
                    description: Some(
                        "HRTIMER_ST0FLTCTL update by DMA mode",
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
                    name: "st0ctl1",
                    description: Some(
                        "HRTIMER_ST0CTL1 update by DMA mode",
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
                    name: "st0exevfcfg2",
                    description: Some(
                        "HRTIMER_ST0EXEVFCFG2 update by DMA mode",
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
                    name: "st0actl",
                    description: Some(
                        "HRTIMER_ST0ACTL update by DMA mode",
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
            name: "Dmaupst1r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st1ctl0",
                    description: Some(
                        "HRTIMER_ST1CTL0 update by DMA mode",
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
                    name: "st1intc",
                    description: Some(
                        "HRTIMER_ST1INTC update by DMA mode",
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
                    name: "st1dmainten",
                    description: Some(
                        "HRTIMER_ST1DMAINTEN update by DMA mode",
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
                    name: "st1cnt",
                    description: Some(
                        "HRTIMER_ST1CNT update by DMA mode",
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
                    name: "st1car",
                    description: Some(
                        "HRTIMER_ST1CAR update by DMA mode",
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
                    name: "st1crep",
                    description: Some(
                        "HRTIMER_ST1CREP update by DMA mode",
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
                    name: "st1cmp0v",
                    description: Some(
                        "HRTIMER_ST1CMP0V update by DMA mode",
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
                    name: "st1cmp1v",
                    description: Some(
                        "HRTIMER_ST1CMP1V update by DMA mode",
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
                    name: "st1cmp2v",
                    description: Some(
                        "HRTIMER_ST1CMP2V update by DMA mode",
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
                    name: "st1cmp3v",
                    description: Some(
                        "HRTIMER_ST1CMP3V update by DMA mode",
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
                    name: "st1dtctl",
                    description: Some(
                        "HRTIMER_ST1DTCTL update by DMA mode",
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
                    name: "st1ch0set",
                    description: Some(
                        "HRTIMER_ST1CH0SET update by DMA mode",
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
                    name: "st1ch0rst",
                    description: Some(
                        "HRTIMER_ST1CH0RST update by DMA mode",
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
                    name: "st1ch1set",
                    description: Some(
                        "HRTIMER_ST1CH1SET update by DMA mode",
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
                    name: "st1ch1rst",
                    description: Some(
                        "HRTIMER_ST1CH1RST update by DMA mode",
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
                    name: "st1exevfcfg0",
                    description: Some(
                        "HRTIMER_ST1EXEVFCFG0update by DMA mode",
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
                    name: "st1exevfcfg1",
                    description: Some(
                        "HRTIMER_ST1EXEVFCFG1update by DMA mode",
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
                    name: "st1cntrst",
                    description: Some(
                        "HRTIMER_ST1CNTRST update by DMA mode",
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
                    name: "st1csctl",
                    description: Some(
                        "HRTIMER_ST1CSCTL update by DMA mode",
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
                    name: "st1choctl",
                    description: Some(
                        "HRTIMER_ST1CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st1fltctl",
                    description: Some(
                        "HRTIMER_ST1FLTCTL update by DMA mode",
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
                    name: "st1ctl1",
                    description: Some(
                        "HRTIMER_ST1CTL1 update by DMA mode",
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
                    name: "st1exevfcfg2",
                    description: Some(
                        "HRTIMER_ST1EXEVFCFG2 update by DMA mode",
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
                    name: "st1actl",
                    description: Some(
                        "HRTIMER_ST1ACTL update by DMA mode",
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
            name: "Dmaupst2r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st2ctl0",
                    description: Some(
                        "HRTIMER_ST2CTL0 update by DMA mode",
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
                    name: "st2intc",
                    description: Some(
                        "HRTIMER_ST2INTC update by DMA mode",
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
                    name: "st2dmainten",
                    description: Some(
                        "HRTIMER_ST2DMAINTEN update by DMA mode",
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
                    name: "st2cnt",
                    description: Some(
                        "HRTIMER_ST2CNT update by DMA mode",
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
                    name: "st2car",
                    description: Some(
                        "HRTIMER_ST2CAR update by DMA mode",
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
                    name: "st2crep",
                    description: Some(
                        "HRTIMER_ST2CREP update by DMA mode",
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
                    name: "st2cmp0v",
                    description: Some(
                        "HRTIMER_ST2CMP0V update by DMA mode",
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
                    name: "st2cmp1v",
                    description: Some(
                        "HRTIMER_ST2CMP1V update by DMA mode",
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
                    name: "st2cmp2v",
                    description: Some(
                        "HRTIMER_ST2CMP2V update by DMA mode",
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
                    name: "st2cmp3v",
                    description: Some(
                        "HRTIMER_ST2CMP3V update by DMA mode",
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
                    name: "st2dtctl",
                    description: Some(
                        "HRTIMER_ST2DTCTL update by DMA mode",
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
                    name: "st2ch0set",
                    description: Some(
                        "HRTIMER_ST2CH0SET update by DMA mode",
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
                    name: "st2ch0rst",
                    description: Some(
                        "HRTIMER_ST2CH0RST update by DMA mode",
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
                    name: "st2ch1set",
                    description: Some(
                        "HRTIMER_ST2CH1SET update by DMA mode",
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
                    name: "st2ch1rst",
                    description: Some(
                        "HRTIMER_ST2CH1RST update by DMA mode",
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
                    name: "st2exevfcfg0",
                    description: Some(
                        "HRTIMER_ST2EXEVFCFG0update by DMA mode",
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
                    name: "st2exevfcfg1",
                    description: Some(
                        "HRTIMER_ST2EXEVFCFG1update by DMA mode",
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
                    name: "st2cntrst",
                    description: Some(
                        "HRTIMER_ST2CNTRST update by DMA mode",
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
                    name: "st2csctl",
                    description: Some(
                        "HRTIMER_ST2CSCTL update by DMA mode",
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
                    name: "st2choctl",
                    description: Some(
                        "HRTIMER_ST2CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st2fltctl",
                    description: Some(
                        "HRTIMER_ST2FLTCTL update by DMA mode",
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
                    name: "st2ctl1",
                    description: Some(
                        "HRTIMER_ST2CTL1 update by DMA mode",
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
                    name: "st2exevfcfg2",
                    description: Some(
                        "HRTIMER_ST2EXEVFCFG2 update by DMA mode",
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
                    name: "st2actl",
                    description: Some(
                        "HRTIMER_ST2ACTL update by DMA mode",
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
            name: "Dmaupst3r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st3ctl0",
                    description: Some(
                        "HRTIMER_ST3CTL0 update by DMA mode",
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
                    name: "st3intc",
                    description: Some(
                        "HRTIMER_ST3INTC update by DMA mode",
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
                    name: "st3dmainten",
                    description: Some(
                        "HRTIMER_ST3DMAINTEN update by DMA mode",
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
                    name: "st3cnt",
                    description: Some(
                        "HRTIMER_ST3CNT update by DMA mode",
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
                    name: "st3car",
                    description: Some(
                        "HRTIMER_ST3CAR update by DMA mode",
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
                    name: "st3crep",
                    description: Some(
                        "HRTIMER_ST3CREP update by DMA mode",
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
                    name: "st3cmp0v",
                    description: Some(
                        "HRTIMER_ST3CMP0V update by DMA mode",
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
                    name: "st3cmp1v",
                    description: Some(
                        "HRTIMER_ST3CMP1V update by DMA mode",
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
                    name: "st3cmp2v",
                    description: Some(
                        "HRTIMER_ST3CMP2V update by DMA mode",
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
                    name: "st3cmp3v",
                    description: Some(
                        "HRTIMER_ST3CMP3V update by DMA mode",
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
                    name: "st3dtctl",
                    description: Some(
                        "HRTIMER_ST3DTCTL update by DMA mode",
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
                    name: "st3ch0set",
                    description: Some(
                        "HRTIMER_ST3CH0SET update by DMA mode",
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
                    name: "st3ch0rst",
                    description: Some(
                        "HRTIMER_ST3CH0RST update by DMA mode",
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
                    name: "st3ch1set",
                    description: Some(
                        "HRTIMER_ST3CH1SET update by DMA mode",
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
                    name: "st3ch1rst",
                    description: Some(
                        "HRTIMER_ST3CH1RST update by DMA mode",
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
                    name: "st3exevfcfg0",
                    description: Some(
                        "HRTIMER_ST3EXEVFCFG0update by DMA mode",
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
                    name: "st3exevfcfg1",
                    description: Some(
                        "HRTIMER_ST3EXEVFCFG1update by DMA mode",
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
                    name: "st3cntrst",
                    description: Some(
                        "HRTIMER_ST3CNTRST update by DMA mode",
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
                    name: "st3csctl",
                    description: Some(
                        "HRTIMER_ST3CSCTL update by DMA mode",
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
                    name: "st3choctl",
                    description: Some(
                        "HRTIMER_ST3CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st3fltctl",
                    description: Some(
                        "HRTIMER_ST3FLTCTL update by DMA mode",
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
                    name: "st3ctl1",
                    description: Some(
                        "HRTIMER_ST3CTL1 update by DMA mode",
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
                    name: "st3exevfcfg2",
                    description: Some(
                        "HRTIMER_ST3EXEVFCFG2 update by DMA mode",
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
                    name: "st3actl",
                    description: Some(
                        "HRTIMER_ST3ACTL update by DMA mode",
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
            name: "Dmaupst4r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st4ctl0",
                    description: Some(
                        "HRTIMER_ST4CTL0 update by DMA mode",
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
                    name: "st4intc",
                    description: Some(
                        "HRTIMER_ST4INTC update by DMA mode",
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
                    name: "st4dmainten",
                    description: Some(
                        "HRTIMER_ST4DMAINTEN update by DMA mode",
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
                    name: "st4cnt",
                    description: Some(
                        "HRTIMER_ST4CNT update by DMA mode",
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
                    name: "st4car",
                    description: Some(
                        "HRTIMER_ST4CAR update by DMA mode",
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
                    name: "st4crep",
                    description: Some(
                        "HRTIMER_ST4CREP update by DMA mode",
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
                    name: "st4cmp0v",
                    description: Some(
                        "HRTIMER_ST4CMP0V update by DMA mode",
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
                    name: "st4cmp1v",
                    description: Some(
                        "HRTIMER_ST4CMP1V update by DMA mode",
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
                    name: "st4cmp2v",
                    description: Some(
                        "HRTIMER_ST4CMP2V update by DMA mode",
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
                    name: "st4cmp3v",
                    description: Some(
                        "HRTIMER_ST4CMP3V update by DMA mode",
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
                    name: "st4dtctl",
                    description: Some(
                        "HRTIMER_ST4DTCTL update by DMA mode",
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
                    name: "st4ch0set",
                    description: Some(
                        "HRTIMER_ST4CH0SET update by DMA mode",
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
                    name: "st4ch0rst",
                    description: Some(
                        "HRTIMER_ST4CH0RST update by DMA mode",
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
                    name: "st4ch1set",
                    description: Some(
                        "HRTIMER_ST4CH1SET update by DMA mode",
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
                    name: "st4ch1rst",
                    description: Some(
                        "HRTIMER_ST4CH1RST update by DMA mode",
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
                    name: "st4exevfcfg0",
                    description: Some(
                        "HRTIMER_ST4EXEVFCFG0update by DMA mode",
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
                    name: "st4exevfcfg1",
                    description: Some(
                        "HRTIMER_ST4EXEVFCFG1update by DMA mode",
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
                    name: "st4cntrst",
                    description: Some(
                        "HRTIMER_ST4CNTRST update by DMA mode",
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
                    name: "st4csctl",
                    description: Some(
                        "HRTIMER_ST4CSCTL update by DMA mode",
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
                    name: "st4choctl",
                    description: Some(
                        "HRTIMER_ST4CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st4fltctl",
                    description: Some(
                        "HRTIMER_ST4FLTCTL update by DMA mode",
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
                    name: "st4ctl1",
                    description: Some(
                        "HRTIMER_ST4CTL1 update by DMA mode",
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
                    name: "st4exevfcfg2",
                    description: Some(
                        "HRTIMER_ST4EXEVFCFG2 update by DMA mode",
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
                    name: "st4actl",
                    description: Some(
                        "HRTIMER_ST4ACTL update by DMA mode",
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
            name: "Dmaupst5r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st5ctl0",
                    description: Some(
                        "HRTIMER_ST5CTL0 update by DMA mode",
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
                    name: "st5intc",
                    description: Some(
                        "HRTIMER_ST5INTC update by DMA mode",
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
                    name: "st5dmainten",
                    description: Some(
                        "HRTIMER_ST5DMAINTEN update by DMA mode",
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
                    name: "st5cnt",
                    description: Some(
                        "HRTIMER_ST5CNT update by DMA mode",
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
                    name: "st5car",
                    description: Some(
                        "HRTIMER_ST5CAR update by DMA mode",
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
                    name: "st5crep",
                    description: Some(
                        "HRTIMER_ST5CREP update by DMA mode",
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
                    name: "st5cmp0v",
                    description: Some(
                        "HRTIMER_ST5CMP0V update by DMA mode",
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
                    name: "st5cmp1v",
                    description: Some(
                        "HRTIMER_ST5CMP1V update by DMA mode",
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
                    name: "st5cmp2v",
                    description: Some(
                        "HRTIMER_ST5CMP2V update by DMA mode",
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
                    name: "st5cmp3v",
                    description: Some(
                        "HRTIMER_ST5CMP3V update by DMA mode",
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
                    name: "st5dtctl",
                    description: Some(
                        "HRTIMER_ST5DTCTL update by DMA mode",
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
                    name: "st5ch0set",
                    description: Some(
                        "HRTIMER_ST5CH0SET update by DMA mode",
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
                    name: "st5ch0rst",
                    description: Some(
                        "HRTIMER_ST5CH0RST update by DMA mode",
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
                    name: "st5ch1set",
                    description: Some(
                        "HRTIMER_ST5CH1SET update by DMA mode",
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
                    name: "st5ch1rst",
                    description: Some(
                        "HRTIMER_ST5CH1RST update by DMA mode",
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
                    name: "st5exevfcfg0",
                    description: Some(
                        "HRTIMER_ST5EXEVFCFG0update by DMA mode",
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
                    name: "st5exevfcfg1",
                    description: Some(
                        "HRTIMER_ST5EXEVFCFG1update by DMA mode",
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
                    name: "st5cntrst",
                    description: Some(
                        "HRTIMER_ST5CNTRST update by DMA mode",
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
                    name: "st5csctl",
                    description: Some(
                        "HRTIMER_ST5CSCTL update by DMA mode",
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
                    name: "st5choctl",
                    description: Some(
                        "HRTIMER_ST5CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st5fltctl",
                    description: Some(
                        "HRTIMER_ST5FLTCTL update by DMA mode",
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
                    name: "st5ctl1",
                    description: Some(
                        "HRTIMER_ST5CTL1 update by DMA mode",
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
                    name: "st5exevfcfg2",
                    description: Some(
                        "HRTIMER_ST5EXEVFCFG2 update by DMA mode",
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
                    name: "st5actl",
                    description: Some(
                        "HRTIMER_ST5ACTL update by DMA mode",
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
            name: "Dmaupst6r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st6ctl0",
                    description: Some(
                        "HRTIMER_ST6CTL0 update by DMA mode",
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
                    name: "st6intc",
                    description: Some(
                        "HRTIMER_ST6INTC update by DMA mode",
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
                    name: "st6dmainten",
                    description: Some(
                        "HRTIMER_ST6DMAINTEN update by DMA mode",
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
                    name: "st6cnt",
                    description: Some(
                        "HRTIMER_ST6CNT update by DMA mode",
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
                    name: "st6car",
                    description: Some(
                        "HRTIMER_ST6CAR update by DMA mode",
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
                    name: "st6crep",
                    description: Some(
                        "HRTIMER_ST6CREP update by DMA mode",
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
                    name: "st6cmp0v",
                    description: Some(
                        "HRTIMER_ST6CMP0V update by DMA mode",
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
                    name: "st6cmp1v",
                    description: Some(
                        "HRTIMER_ST6CMP1V update by DMA mode",
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
                    name: "st6cmp2v",
                    description: Some(
                        "HRTIMER_ST6CMP2V update by DMA mode",
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
                    name: "st6cmp3v",
                    description: Some(
                        "HRTIMER_ST6CMP3V update by DMA mode",
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
                    name: "st6dtctl",
                    description: Some(
                        "HRTIMER_ST6DTCTL update by DMA mode",
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
                    name: "st6ch0set",
                    description: Some(
                        "HRTIMER_ST6CH0SET update by DMA mode",
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
                    name: "st6ch0rst",
                    description: Some(
                        "HRTIMER_ST6CH0RST update by DMA mode",
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
                    name: "st6ch1set",
                    description: Some(
                        "HRTIMER_ST6CH1SET update by DMA mode",
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
                    name: "st6ch1rst",
                    description: Some(
                        "HRTIMER_ST6CH1RST update by DMA mode",
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
                    name: "st6exevfcfg0",
                    description: Some(
                        "HRTIMER_ST6EXEVFCFG0update by DMA mode",
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
                    name: "st6exevfcfg1",
                    description: Some(
                        "HRTIMER_ST6EXEVFCFG1update by DMA mode",
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
                    name: "st6cntrst",
                    description: Some(
                        "HRTIMER_ST6CNTRST update by DMA mode",
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
                    name: "st6csctl",
                    description: Some(
                        "HRTIMER_ST6CSCTL update by DMA mode",
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
                    name: "st6choctl",
                    description: Some(
                        "HRTIMER_ST6CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st6fltctl",
                    description: Some(
                        "HRTIMER_ST6FLTCTL update by DMA mode",
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
                    name: "st6ctl1",
                    description: Some(
                        "HRTIMER_ST6CTL1 update by DMA mode",
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
                    name: "st6exevfcfg2",
                    description: Some(
                        "HRTIMER_ST6EXEVFCFG2 update by DMA mode",
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
                    name: "st6actl",
                    description: Some(
                        "HRTIMER_ST6ACTL update by DMA mode",
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
            name: "Dmaupst7r",
            extends: None,
            description: Some(
                "HRTIMER DMA update Slave_TIMERx regist 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st7ctl0",
                    description: Some(
                        "HRTIMER_ST7CTL0 update by DMA mode",
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
                    name: "st7intc",
                    description: Some(
                        "HRTIMER_ST7INTC update by DMA mode",
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
                    name: "st7dmainten",
                    description: Some(
                        "HRTIMER_ST7DMAINTEN update by DMA mode",
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
                    name: "st7cnt",
                    description: Some(
                        "HRTIMER_ST7CNT update by DMA mode",
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
                    name: "st7car",
                    description: Some(
                        "HRTIMER_ST7CAR update by DMA mode",
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
                    name: "st7crep",
                    description: Some(
                        "HRTIMER_ST7CREP update by DMA mode",
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
                    name: "st7cmp0v",
                    description: Some(
                        "HRTIMER_ST7CMP0V update by DMA mode",
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
                    name: "st7cmp1v",
                    description: Some(
                        "HRTIMER_ST7CMP1V update by DMA mode",
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
                    name: "st7cmp2v",
                    description: Some(
                        "HRTIMER_ST7CMP2V update by DMA mode",
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
                    name: "st7cmp3v",
                    description: Some(
                        "HRTIMER_ST7CMP3V update by DMA mode",
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
                    name: "st7dtctl",
                    description: Some(
                        "HRTIMER_ST7DTCTL update by DMA mode",
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
                    name: "st7ch0set",
                    description: Some(
                        "HRTIMER_ST7CH0SET update by DMA mode",
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
                    name: "st7ch0rst",
                    description: Some(
                        "HRTIMER_ST7CH0RST update by DMA mode",
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
                    name: "st7ch1set",
                    description: Some(
                        "HRTIMER_ST7CH1SET update by DMA mode",
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
                    name: "st7ch1rst",
                    description: Some(
                        "HRTIMER_ST7CH1RST update by DMA mode",
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
                    name: "st7exevfcfg0",
                    description: Some(
                        "HRTIMER_ST7EXEVFCFG0update by DMA mode",
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
                    name: "st7exevfcfg1",
                    description: Some(
                        "HRTIMER_ST7EXEVFCFG1update by DMA mode",
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
                    name: "st7cntrst",
                    description: Some(
                        "HRTIMER_ST7CNTRST update by DMA mode",
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
                    name: "st7csctl",
                    description: Some(
                        "HRTIMER_ST7CSCTL update by DMA mode",
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
                    name: "st7choctl",
                    description: Some(
                        "HRTIMER_ST7CHOCTL update by DMA mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "st7fltctl",
                    description: Some(
                        "HRTIMER_ST7FLTCTL update by DMA mode",
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
                    name: "st7ctl1",
                    description: Some(
                        "HRTIMER_ST7CTL1 update by DMA mode",
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
                    name: "st7exevfcfg2",
                    description: Some(
                        "HRTIMER_ST7EXEVFCFG2 update by DMA mode",
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
                    name: "st7actl",
                    description: Some(
                        "HRTIMER_ST7ACTL update by DMA mode",
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
            name: "Exevcfg0",
            extends: None,
            description: Some(
                "HRTIMER external event configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exev0src",
                    description: Some(
                        "External event 0 source",
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
                    name: "exev0p",
                    description: Some(
                        "External event 0 polarity",
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
                    name: "exev0eg",
                    description: Some(
                        "External event 0 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev0fast",
                    description: Some(
                        "External Event 0 fast mode",
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
                    name: "exev1src",
                    description: Some(
                        "External event 1 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev1p",
                    description: Some(
                        "External event 1 polarity",
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
                    name: "exev1eg",
                    description: Some(
                        "External event 1 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev1fast",
                    description: Some(
                        "External Event 1 fast mode",
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
                    name: "exev2src",
                    description: Some(
                        "External event 2 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev2p",
                    description: Some(
                        "External event 2 polarity",
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
                    name: "exev2eg",
                    description: Some(
                        "External event 2 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev2fast",
                    description: Some(
                        "External Event 2 fast mode",
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
                    name: "exev3src",
                    description: Some(
                        "External event 3 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev3p",
                    description: Some(
                        "External event 3 polarity",
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
                    name: "exev3eg",
                    description: Some(
                        "External event 3 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev3fast",
                    description: Some(
                        "External Event 3 fast mode",
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
                    name: "exev4src",
                    description: Some(
                        "External event 4 source",
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
                Field {
                    name: "exev4p",
                    description: Some(
                        "External event 4 polarity",
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
                    name: "exev4eg",
                    description: Some(
                        "External event 4 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev4fast",
                    description: Some(
                        "External Event 4 fast mode",
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
            name: "Exevcfg1",
            extends: None,
            description: Some(
                "HRTIMER external event configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exev5src",
                    description: Some(
                        "External event 5 source",
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
                    name: "exev5p",
                    description: Some(
                        "External event 5 polarity",
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
                    name: "exev5eg",
                    description: Some(
                        "External event 5 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev6src",
                    description: Some(
                        "External event 6 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev6p",
                    description: Some(
                        "External event 6 polarity",
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
                    name: "exev6eg",
                    description: Some(
                        "External event 6 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev7src",
                    description: Some(
                        "External event 7 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev7p",
                    description: Some(
                        "External event 7 polarity",
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
                    name: "exev7eg",
                    description: Some(
                        "External event 7 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev8src",
                    description: Some(
                        "External event 8 source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev8p",
                    description: Some(
                        "External event 8 polarity",
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
                    name: "exev8eg",
                    description: Some(
                        "External event 8 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev9src",
                    description: Some(
                        "External event 9 source",
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
                Field {
                    name: "exev9p",
                    description: Some(
                        "External event 9 polarity",
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
                    name: "exev9eg",
                    description: Some(
                        "External event 9 edge sensitivity",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Exevdfctl",
            extends: None,
            description: Some(
                "HRTIMER external event digital filter control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exev5fc",
                    description: Some(
                        "External event 5 filter control",
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
                    name: "exev6fc",
                    description: Some(
                        "External event 6 filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev7fc",
                    description: Some(
                        "External event 7 filter control",
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
                Field {
                    name: "exev8fc",
                    description: Some(
                        "External event 8 filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exev9fc",
                    description: Some(
                        "External event 9 filter control",
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
                Field {
                    name: "exevfdiv",
                    description: Some(
                        "External event clock division",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fltincfg0",
            extends: None,
            description: Some(
                "HRTIMER fault input configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0inen",
                    description: Some(
                        "Fault 0 input enable",
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
                    name: "flt0inp",
                    description: Some(
                        "Fault 0 input polarity",
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
                    name: "flt0insrc_0",
                    description: Some(
                        "Fault 0 input source, combine with FLT0INSRC[1]",
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
                    name: "flt0infc",
                    description: Some(
                        "Fault 0 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt0inprot",
                    description: Some(
                        "Protect fault 0 input configuration",
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
                    name: "flt1inen",
                    description: Some(
                        "Fault 1 input enable",
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
                    name: "flt1inp",
                    description: Some(
                        "Fault 1 input polarity",
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
                    name: "flt1insrc_0",
                    description: Some(
                        "Fault 1 input source",
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
                    name: "flt1infc",
                    description: Some(
                        "Fault 1 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt1inprot",
                    description: Some(
                        "Protect fault 1 input configuration",
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
                    name: "flt2inen",
                    description: Some(
                        "Fault 2 input enable",
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
                    name: "flt2inp",
                    description: Some(
                        "Fault 2 input polarity",
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
                    name: "flt2insrc_0",
                    description: Some(
                        "Fault 2 input source",
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
                    name: "flt2infc",
                    description: Some(
                        "Fault 2 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt2inprot",
                    description: Some(
                        "Protect fault 2 input configuration",
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
                    name: "flt3inen",
                    description: Some(
                        "Fault 3 input enable",
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
                    name: "flt3inp",
                    description: Some(
                        "Fault 3 input polarity",
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
                    name: "flt3insrc_0",
                    description: Some(
                        "Fault 3 input source",
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
                    name: "flt3infc",
                    description: Some(
                        "Fault 3 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt3inprot",
                    description: Some(
                        "Protect fault 3 input configuration",
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
            name: "Fltincfg1",
            extends: None,
            description: Some(
                "HRTIMER fault input configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt4inen",
                    description: Some(
                        "Fault 4 input enable",
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
                    name: "flt4inp",
                    description: Some(
                        "Fault 4 input polarity",
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
                    name: "flt4insrc_0",
                    description: Some(
                        "Fault 4 input source",
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
                    name: "flt4infc",
                    description: Some(
                        "Fault 4 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt4inprot",
                    description: Some(
                        "Protect fault 4 input configuration",
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
                    name: "flt5inen",
                    description: Some(
                        "Fault 5 input enable",
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
                    name: "flt5inp",
                    description: Some(
                        "Fault 5 input polarity",
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
                    name: "flt5insrc_0",
                    description: Some(
                        "Fault 5 input source",
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
                    name: "flt5infc",
                    description: Some(
                        "Fault 5 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt5inprot",
                    description: Some(
                        "Protect fault 5 input configuration",
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
                    name: "flt0insrc_1",
                    description: Some(
                        "Fault 0 input source",
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
                    name: "flt1insrc_1",
                    description: Some(
                        "Fault 1 input source",
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
                    name: "flt2insrc_1",
                    description: Some(
                        "Fault 2 input source",
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
                    name: "flt3insrc_1",
                    description: Some(
                        "Fault 3 input source",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt4insrc_1",
                    description: Some(
                        "Fault 4 input source",
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
                    name: "flt5insrc_1",
                    description: Some(
                        "Fault 5 input source",
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
                    name: "flt6insrc_1",
                    description: Some(
                        "Fault 6 input source",
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
                    name: "flt7insrc_1",
                    description: Some(
                        "Fault 7 input source",
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
                    name: "fltfdiv",
                    description: Some(
                        "Fault input clock division",
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
            name: "Fltincfg2",
            extends: None,
            description: Some(
                "HRTIMER fault input configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0blken",
                    description: Some(
                        "Fault 0 blanking enable",
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
                    name: "flt0blks",
                    description: Some(
                        "Fault 0 blanking source",
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
                    name: "flt0cnt",
                    description: Some(
                        "Fault 0 Counter:",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt0cntrst",
                    description: Some(
                        "Fault 0 counter reset",
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
                    name: "flt0rst",
                    description: Some(
                        "Fault 0 reset mode:",
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
                    name: "flt1blken",
                    description: Some(
                        "Fault 1 blanking enable",
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
                    name: "flt1blks",
                    description: Some(
                        "Fault 1 blanking source",
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
                    name: "flt1cnt",
                    description: Some(
                        "Fault 1 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt1cntrst",
                    description: Some(
                        "Fault 1 counter reset",
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
                    name: "flt1rst",
                    description: Some(
                        "Fault 1 reset mode",
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
                    name: "flt2blken",
                    description: Some(
                        "Fault 3 blanking enable",
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
                    name: "flt2blks",
                    description: Some(
                        "Fault 2 blanking source",
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
                    name: "flt2cnt",
                    description: Some(
                        "Fault 2 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt2cntrst",
                    description: Some(
                        "Fault 2 counter reset",
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
                    name: "flt2rst",
                    description: Some(
                        "Fault 2 reset mode",
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
                    name: "flt3blken",
                    description: Some(
                        "Fault 3 blanking enable",
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
                    name: "flt3blks",
                    description: Some(
                        "Fault 3 blanking source",
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
                    name: "flt3cnt",
                    description: Some(
                        "Fault 3 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt3cntrst",
                    description: Some(
                        "Fault 3 counter reset",
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
                    name: "flt3rst",
                    description: Some(
                        "Fault 3 reset mode",
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
            name: "Fltincfg3",
            extends: None,
            description: Some(
                "HRTIMER fault input configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt4blken",
                    description: Some(
                        "Fault 4 blanking enable",
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
                    name: "flt4blks",
                    description: Some(
                        "Fault 4 blanking source",
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
                    name: "flt4cnt",
                    description: Some(
                        "Fault 4 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt4cntrst",
                    description: Some(
                        "Fault 4 counter reset",
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
                    name: "flt4rst",
                    description: Some(
                        "Fault 4 reset mode",
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
                    name: "flt5blken",
                    description: Some(
                        "Fault 5 blanking enable",
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
                    name: "flt5blks",
                    description: Some(
                        "Fault 5 blanking source",
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
                    name: "flt5cnt",
                    description: Some(
                        "Fault 5 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt5cntrst",
                    description: Some(
                        "Fault 5 counter reset",
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
                    name: "flt5rst",
                    description: Some(
                        "Fault 5 reset mode",
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
                    name: "flt6blken",
                    description: Some(
                        "Fault 6 blanking enable",
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
                    name: "flt6blks",
                    description: Some(
                        "Fault 6 blanking source",
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
                    name: "flt6cnt",
                    description: Some(
                        "Fault 6 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt6cntrst",
                    description: Some(
                        "Fault 6 counter reset",
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
                    name: "flt6rst",
                    description: Some(
                        "Fault 6 reset mode",
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
                    name: "flt7blken",
                    description: Some(
                        "Fault 7 blanking enable",
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
                    name: "flt7blks",
                    description: Some(
                        "Fault 7 blanking source",
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
                    name: "flt7cnt",
                    description: Some(
                        "Fault 7 counter",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt7cntrst",
                    description: Some(
                        "Fault 7 counter reset",
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
                    name: "flt7rst",
                    description: Some(
                        "Fault 7 reset mode",
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
            name: "Fltincfg4",
            extends: None,
            description: Some(
                "HRTIMER fault input configuration register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt6inen",
                    description: Some(
                        "Fault 6 input enable",
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
                    name: "flt6inp",
                    description: Some(
                        "Fault 6 input polarity",
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
                    name: "flt6insrc_0",
                    description: Some(
                        "Fault 6 input source",
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
                    name: "flt6infc",
                    description: Some(
                        "Fault 6 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt6inprot",
                    description: Some(
                        "Protect fault 6 input configuration",
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
                    name: "flt7inen",
                    description: Some(
                        "Fault 7 input enable",
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
                    name: "flt7inp",
                    description: Some(
                        "Fault 7 input polarity",
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
                    name: "flt7insrc_0",
                    description: Some(
                        "Fault 7 input source",
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
                    name: "flt7infc",
                    description: Some(
                        "Fault 7 input filter control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flt7inprot",
                    description: Some(
                        "Protect fault 7 input configuration",
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
            name: "Fltrecctl",
            extends: None,
            description: Some(
                "HRTIMER fault recovery control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fltrecctl",
                    description: Some(
                        "fault recovery control",
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
            name: "Intc",
            extends: None,
            description: Some(
                "HRTIMER interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0ifc",
                    description: Some(
                        "Clear fault 0 interrupt flag",
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
                    name: "flt1ifc",
                    description: Some(
                        "Clear fault 1 interrupt flag",
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
                    name: "flt2ifc",
                    description: Some(
                        "Clear fault 2 interrupt flag",
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
                    name: "flt3ifc",
                    description: Some(
                        "Clear fault 3 interrupt flag",
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
                    name: "flt4ifc",
                    description: Some(
                        "Clear fault 4 interrupt flag",
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
                    name: "sysfltifc",
                    description: Some(
                        "Clear system fault interrupt flag",
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
                    name: "flt5ifc",
                    description: Some(
                        "Clear fault 5 interrupt flag",
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
                    name: "flt6ifc",
                    description: Some(
                        "Clear fault 6 interrupt flag",
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
                    name: "flt7ifc",
                    description: Some(
                        "Clear fault 7 interrupt flag",
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
                    name: "dllcalifc",
                    description: Some(
                        "Clear DLL calibration completed interrupt flag",
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
                    name: "bmperifc",
                    description: Some(
                        "Clear bunch mode period interrupt flag",
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
            ],
        },
        FieldSet {
            name: "Inten",
            extends: None,
            description: Some(
                "HRTIMER interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0ie",
                    description: Some(
                        "fault 0 interrupt enable",
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
                    name: "flt1ie",
                    description: Some(
                        "fault 1 interrupt enable",
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
                    name: "flt2ie",
                    description: Some(
                        "fault 2 interrupt enable",
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
                    name: "flt3ie",
                    description: Some(
                        "fault 3 interrupt enable",
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
                    name: "flt4ie",
                    description: Some(
                        "fault 4 interrupt enable",
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
                    name: "sysfltie",
                    description: Some(
                        "System fault interrupt enable",
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
                    name: "flt5ie",
                    description: Some(
                        "fault 5 interrupt enable",
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
                    name: "flt6ie",
                    description: Some(
                        "fault 6 interrupt enable",
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
                    name: "flt7ie",
                    description: Some(
                        "fault 7 interrupt enable",
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
                    name: "dllcalie",
                    description: Some(
                        "DLL calibration completed interrupt enable",
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
                    name: "bmperie",
                    description: Some(
                        "Bunch mode period interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Intf",
            extends: None,
            description: Some(
                "HRTIMER interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0if",
                    description: Some(
                        "Fault 0 interrupt flag",
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
                    name: "flt1if",
                    description: Some(
                        "Fault 1 interrupt flag",
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
                    name: "flt2if",
                    description: Some(
                        "Fault 2 interrupt flag",
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
                    name: "flt3if",
                    description: Some(
                        "Fault 3 interrupt flag",
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
                    name: "flt4if",
                    description: Some(
                        "Fault 4 interrupt flag",
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
                    name: "sysfltif",
                    description: Some(
                        "System fault interrupt flag",
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
                    name: "flt5if",
                    description: Some(
                        "Fault 5 interrupt flag",
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
                    name: "flt6if",
                    description: Some(
                        "Fault 6 interrupt flag",
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
                    name: "flt7if",
                    description: Some(
                        "Fault 7 interrupt flag",
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
                    name: "dllcalif",
                    description: Some(
                        "DLL calibration completed interrupt flag",
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
                    name: "bmperif",
                    description: Some(
                        "Bunch mode period interrupt flag",
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
            ],
        },
    ],
    enums: &[],
};
                