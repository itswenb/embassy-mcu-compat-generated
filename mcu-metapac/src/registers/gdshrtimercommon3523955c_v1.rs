
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "ShrtimerCommon",
            extends: None,
            description: Some(
                "SHRTIMER Common registers",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "SHRTIMER control register 0",
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
                        "SHRTIMER control register 1",
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
                        "SHRTIMER interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x8,
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
                        "SHRTIMER interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                    name: "inten",
                    description: Some(
                        "SHRTIMER interrupt enable register",
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
                        "SHRTIMER channel output enable register",
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
                        "SHRTIMER channel output disable register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
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
                        "SHRTIMER channel output disable flag register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "SHRTIMER bunch mode control register",
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
                        "SHRTIMER bunch mode start trigger register",
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
                        "SHRTIMER bunch mode compare value register",
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
                        "SHRTIMER bunch mode counter auto reload register",
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
                        "SHRTIMER external event configuration register 0",
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
                        "SHRTIMER external event configuration register 1",
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
                        "SHRTIMER external event digital filter control register",
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
                        "SHRTIMER trigger source 0 to ADC register",
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
                        "SHRTIMER trigger source 1 to ADC register",
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
                        "SHRTIMER trigger source 2 to ADC register",
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
                        "SHRTIMER trigger source 3 to ADC register",
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
                        "SHRTIMER DLL calibration control register",
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
                        "SHRTIMER fault input configuration register 0",
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
                        "SHRTIMER fault input configuration register 1",
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
                        "SHRTIMER DMA update Master_TIMER register",
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
                        "SHRTIMER DMA update Slave_TIMER0 register",
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
                        "SHRTIMER DMA update Slave_TIMER1 register",
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
                        "SHRTIMER DMA update Slave_TIMER2 register",
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
                        "SHRTIMER DMA update Slave_TIMER3 register",
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
                        "SHRTIMER DMA update Slave_TIMER4 register",
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
                        "SHRTIMER DMA transfer buffer register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmatb",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Adctrigs0",
            extends: None,
            description: Some(
                "SHRTIMER trigger source 0 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg0mtc0",
                    description: Some(
                        "SHRTIMER_ADCTRIG0 on Master_TIMER compare 0 event",
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
                        "SHRTIMER_ADCTRIG0 on Master_TIMER compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Master_TIMER compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Master_TIMER compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Master_TIMER period event",
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
                        "SHRTIMER_ADCTRIG0 on external event 0",
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
                        "SHRTIMER_ADCTRIG0 on external event 1",
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
                        "SHRTIMER_ADCTRIG0 on external event 2",
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
                        "SHRTIMER_ADCTRIG0 on external event 3",
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
                        "SHRTIMER_ADCTRIG0 on external event 4",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER0 compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER0 compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER0 compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER0 period event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER0 reset",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER1 compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER1 compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER1 compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER1 period event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER1 reset",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER2 compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER2 compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER2 compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER2 period event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER3 compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER3 compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER3 compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER3 period event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER4 compare 1 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER4 compare 2 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER4 compare 3 event",
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
                        "SHRTIMER_ADCTRIG0 on Slave_TIMER4 period event",
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
            name: "Adctrigs1",
            extends: None,
            description: Some(
                "SHRTIMER trigger source 1 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg1mtc0",
                    description: Some(
                        "SHRTIMER_ADCTRIG1 on Master_TIMER compare 0 event",
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
                        "SHRTIMER_ADCTRIG1 on Master_TIMER compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Master_TIMER compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Master_TIMER compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Master_TIMER period event",
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
                        "SHRTIMER_ADCTRIG1 on external event 5",
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
                        "SHRTIMER_ADCTRIG1 on external event 6",
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
                        "SHRTIMER_ADCTRIG1 on external event 7",
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
                        "SHRTIMER_ADCTRIG1 on external event 8",
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
                        "SHRTIMER_ADCTRIG1 on external event 9",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER0 compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER0 compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER0 compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER0 period event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER1 compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER1 compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER1 compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER1 period event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER2 compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER2 compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER2 compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER2 period event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER2 reset",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER3 compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER3 compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER3 compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER3 period event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER3 reset",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER4 compare 1 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER4 compare 2 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER4 compare 3 event",
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
                        "SHRTIMER_ADCTRIG1 on Slave_TIMER4 reset",
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
            name: "Adctrigs2",
            extends: None,
            description: Some(
                "SHRTIMER trigger source 2 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg2mtc0",
                    description: Some(
                        "SHRTIMER_ADCTRIG2 on Master_TIMER compare 0 event",
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
                        "SHRTIMER_ADCTRIG2 on Master_TIMER compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Master_TIMER compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Master_TIMER compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Master_TIMER period event",
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
                        "SHRTIMER_ADCTRIG2 on external event 0",
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
                        "SHRTIMER_ADCTRIG2 on external event 1",
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
                        "SHRTIMER_ADCTRIG2 on external event 2",
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
                        "SHRTIMER_ADCTRIG2 on external event 3",
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
                        "SHRTIMER_ADCTRIG2 on external event 4",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER0 compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER0 compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER0 compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER0 period event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER0 reset",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER1 compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER1 compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER1 compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER1 period event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER1 reset",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER2 compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER2 compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER2 compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER2 period event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER3 compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER3 compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER3 compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER3 period event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER4 compare 1 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER4 compare 2 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER4 compare 3 event",
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
                        "SHRTIMER_ADCTRIG2 on Slave_TIMER4 period event",
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
            name: "Adctrigs3",
            extends: None,
            description: Some(
                "SHRTIMER trigger source 3 to ADC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trg3mtc0",
                    description: Some(
                        "SHRTIMER_ADCTRIG3 on Master_TIMER compare 0 event",
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
                    name: "trg1mtc3",
                    description: Some(
                        "SHRTIMER_ADCTRIG3 on Master_TIMER compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Master_TIMER compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Master_TIMER compare 3 event",
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
                        "SHRTIMER_ADCTRIG3 on Master_TIMER period event",
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
                        "SHRTIMER_ADCTRIG3 on external event 5",
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
                        "SHRTIMER_ADCTRIG3 on external event 6",
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
                        "SHRTIMER_ADCTRIG3 on external event 7",
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
                        "SHRTIMER_ADCTRIG3 on external event 8",
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
                        "SHRTIMER_ADCTRIG3 on external event 9",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER0 compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER0 compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER0 compare 3 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER0 period event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER1 compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER1 compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER1 compare 3 event",
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
                    name: "trg1st3per",
                    description: Some(
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER1 period event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER2 compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER2 compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER2 compare 3 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER2 period event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER2 reset",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER3 compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER3 compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER3 compare 3 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER3 period event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER3 reset",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER4 compare 1 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER4 compare 2 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER4 compare 3 event",
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
                        "SHRTIMER_ADCTRIG3 on Slave_TIMER4 reset",
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
            name: "Bmcar",
            extends: None,
            description: Some(
                "SHRTIMER bunch mode counter auto reload register",
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
                "SHRTIMER bunch mode compare value register",
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
                "SHRTIMER bunch mode control register",
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
                "SHRTIMER bunch mode start trigger register",
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
                        "Slave_TIMER1 repetition event triggers bunch mode operation",
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
            name: "Choutdis",
            extends: None,
            description: Some(
                "SHRTIMER channel output disable register",
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
                        "Slave_TIMER0 channel 1 output (ST4CH0_O) disable",
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
            ],
        },
        FieldSet {
            name: "Choutdisf",
            extends: None,
            description: Some(
                "SHRTIMER channel output disable flag register",
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
            ],
        },
        FieldSet {
            name: "Chouten",
            extends: None,
            description: Some(
                "SHRTIMER channel output enable register",
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
            ],
        },
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "SHRTIMER control register 0",
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
                    name: "adtg0usrc",
                    description: Some(
                        "SHRTIMER_ADCTRIG0 update source",
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
                    name: "adtg1usrc",
                    description: Some(
                        "SHRTIMER_ADCTRIG1 update source",
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
                    name: "adtg2usrc",
                    description: Some(
                        "SHRTIMER_ADCTRIG2 update source",
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
                    name: "adtg3usrc",
                    description: Some(
                        "SHRTIMER_ADCTRIG3 update source",
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
            ],
        },
        FieldSet {
            name: "Ctl1",
            extends: None,
            description: Some(
                "SHRTIMER control register 1",
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
            ],
        },
        FieldSet {
            name: "Dllcctl",
            extends: None,
            description: Some(
                "SHRTIMER DLL calibration control register",
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
                "SHRTIMER DMA transfer buffer register",
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
                "SHRTIMER DMA update Master_TIMER register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mtctl0",
                    description: Some(
                        "SHRTIMER_MTCTL0 update by DMA mode",
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
                        "SHRTIMER_MTINTC update by DMA mode",
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
                        "SHRTIMER_MTDMAINTEN update by DMA mode",
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
                        "SHRTIMER_MTCNT update by DMA mode",
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
                        "SHRTIMER_MTCAR update by DMA mode",
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
                        "SHRTIMER_MTCAR update by DMA mode",
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
                        "SHRTIMER_MTCMP0V update by DMA mode",
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
                        "SHRTIMER_MTCMP1V update by DMA mode",
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
                        "SHRTIMER_MTCMP2V update by DMA mode",
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
                        "SHRTIMER_MTCMP3V update by DMA mode",
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
                    name: "mtactl",
                    description: Some(
                        "SHRTIMER_MTACTL update by DMA mode",
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
            name: "Dmaupst0r",
            extends: None,
            description: Some(
                "SHRTIMER DMA update Slave_TIMER0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st0ctl0",
                    description: Some(
                        "SHRTIMER_ST0CTL0 update by DMA mode",
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
                        "SHRTIMER_ST0INTC update by DMA mode",
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
                        "SHRTIMER_ST0DMAINTEN update by DMA mode",
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
                        "SHRTIMER_ST0CNT update by DMA mode",
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
                        "SHRTIMER_ST0CAR update by DMA mode",
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
                        "SHRTIMER_ST0CREP update by DMA mode",
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
                        "SHRTIMER_ST0CMP0V update by DMA mode",
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
                        "SHRTIMER_ST0CMP1V update by DMA mode",
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
                        "SHRTIMER_ST0CMP2V update by DMA mode",
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
                        "SHRTIMER_ST0CMP3V update by DMA mode",
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
                        "SHRTIMER_ST0DTCTL update by DMA mode",
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
                        "SHRTIMER_ST0CH0SET update by DMA mode",
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
                        "SHRTIMER_ST0CH0RST update by DMA mode",
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
                        "SHRTIMER_ST0CH1SET update by DMA mode",
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
                        "SHRTIMER_ST0CH1RST update by DMA mode",
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
                        "SHRTIMER_ST0EXEVFCFG0update by DMA mode",
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
                        "SHRTIMER_ST0EXEVFCFG1update by DMA mode",
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
                        "SHRTIMER_ST0CNTRST update by DMA mode",
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
                        "SHRTIMER_ST0CSCTL update by DMA mode",
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
                        "SHRTIMER_ST0CHOCTL update by DMA mode",
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
                        "SHRTIMER_ST0FLTCTL update by DMA mode",
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
                    name: "st0actl",
                    description: Some(
                        "SHRTIMER_ST0ACTL update by DMA mode",
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
                "SHRTIMER DMA update Slave_TIMER1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st1ctl0",
                    description: Some(
                        "SHRTIMER_ST1CTL0 update by DMA mode",
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
                        "SHRTIMER_ST1INTC update by DMA mode",
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
                        "SHRTIMER_ST1DMAINTEN update by DMA mode",
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
                        "SHRTIMER_ST1CNT update by DMA mode",
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
                        "SHRTIMER_ST1CAR update by DMA mode",
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
                        "SHRTIMER_ST1CREP update by DMA mode",
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
                        "SHRTIMER_ST1CMP0V update by DMA mode",
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
                        "SHRTIMER_ST1CMP1V update by DMA mode",
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
                        "SHRTIMER_ST1CMP2V update by DMA mode",
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
                        "SHRTIMER_ST1CMP3V update by DMA mode",
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
                        "SHRTIMER_ST1DTCTL update by DMA mode",
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
                        "SHRTIMER_ST1CH0SET update by DMA mode",
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
                        "SHRTIMER_ST1CH0RST update by DMA mode",
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
                        "SHRTIMER_ST1CH1SET update by DMA mode",
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
                        "SHRTIMER_ST1CH1RST update by DMA mode",
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
                        "SHRTIMER_ST1EXEVFCFG0update by DMA mode",
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
                        "SHRTIMER_ST1EXEVFCFG1update by DMA mode",
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
                        "SHRTIMER_ST1CNTRST update by DMA mode",
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
                        "SHRTIMER_ST1CSCTL update by DMA mode",
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
                        "SHRTIMER_ST1CHOCTL update by DMA mode",
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
                        "SHRTIMER_ST1FLTCTL update by DMA mode",
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
                    name: "st1actl",
                    description: Some(
                        "SHRTIMER_ST1ACTL update by DMA mode",
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
                "SHRTIMER DMA update Slave_TIMER2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st2ctl0",
                    description: Some(
                        "SHRTIMER_ST2CTL0 update by DMA mode",
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
                        "SHRTIMER_ST2INTC update by DMA mode",
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
                        "SHRTIMER_ST2DMAINTEN update by DMA mode",
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
                        "SHRTIMER_ST2CNT update by DMA mode",
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
                        "SHRTIMER_ST2CAR update by DMA mode",
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
                        "SHRTIMER_ST2CREP update by DMA mode",
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
                        "SHRTIMER_ST2CMP0V update by DMA mode",
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
                        "SHRTIMER_ST2CMP1V update by DMA mode",
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
                        "SHRTIMER_ST2CMP2V update by DMA mode",
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
                        "SHRTIMER_ST2CMP3V update by DMA mode",
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
                        "SHRTIMER_ST2DTCTL update by DMA mode",
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
                        "SHRTIMER_ST2CH0SET update by DMA mode",
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
                        "SHRTIMER_ST2CH0RST update by DMA mode",
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
                        "SHRTIMER_ST2CH1SET update by DMA mode",
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
                        "SHRTIMER_ST2CH1RST update by DMA mode",
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
                        "SHRTIMER_ST2EXEVFCFG0update by DMA mode",
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
                        "SHRTIMER_ST2EXEVFCFG1update by DMA mode",
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
                        "SHRTIMER_ST2CNTRST update by DMA mode",
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
                        "SHRTIMER_ST2CSCTL update by DMA mode",
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
                        "SHRTIMER_ST2CHOCTL update by DMA mode",
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
                        "SHRTIMER_ST2FLTCTL update by DMA mode",
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
                    name: "st2actl",
                    description: Some(
                        "SHRTIMER_ST2ACTL update by DMA mode",
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
                "SHRTIMER DMA update Slave_TIMER3 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st3ctl0",
                    description: Some(
                        "SHRTIMER_ST3CTL0 update by DMA mode",
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
                        "SHRTIMER_ST3INTC update by DMA mode",
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
                        "SHRTIMER_ST3DMAINTEN update by DMA mode",
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
                        "SHRTIMER_ST3CNT update by DMA mode",
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
                        "SHRTIMER_ST3CAR update by DMA mode",
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
                        "SHRTIMER_ST3CREP update by DMA mode",
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
                        "SHRTIMER_ST3CMP0V update by DMA mode",
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
                        "SHRTIMER_ST3CMP1V update by DMA mode",
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
                        "SHRTIMER_ST3CMP2V update by DMA mode",
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
                        "SHRTIMER_ST3CMP3V update by DMA mode",
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
                        "SHRTIMER_ST3DTCTL update by DMA mode",
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
                        "SHRTIMER_ST3CH0SET update by DMA mode",
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
                        "SHRTIMER_ST3CH0RST update by DMA mode",
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
                        "SHRTIMER_ST3CH1SET update by DMA mode",
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
                        "SHRTIMER_ST3CH1RST update by DMA mode",
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
                        "SHRTIMER_ST3EXEVFCFG0update by DMA mode",
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
                        "SHRTIMER_ST3EXEVFCFG1update by DMA mode",
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
                        "SHRTIMER_ST3CNTRST update by DMA mode",
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
                        "SHRTIMER_ST3CSCTL update by DMA mode",
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
                        "SHRTIMER_ST3CHOCTL update by DMA mode",
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
                        "SHRTIMER_ST3FLTCTL update by DMA mode",
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
                    name: "st3actl",
                    description: Some(
                        "SHRTIMER_ST3ACTL update by DMA mode",
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
                "SHRTIMER DMA update Slave_TIMER4 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "st4ctl0",
                    description: Some(
                        "SHRTIMER_ST4CTL0 update by DMA mode",
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
                        "SHRTIMER_ST4INTC update by DMA mode",
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
                        "SHRTIMER_ST4DMAINTEN update by DMA mode",
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
                        "SHRTIMER_ST4CNT update by DMA mode",
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
                        "SHRTIMER_ST4CAR update by DMA mode",
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
                        "SHRTIMER_ST4CREP update by DMA mode",
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
                        "SHRTIMER_ST4CMP0V update by DMA mode",
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
                        "SHRTIMER_ST4CMP1V update by DMA mode",
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
                        "SHRTIMER_ST4CMP2V update by DMA mode",
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
                        "SHRTIMER_ST4CMP3V update by DMA mode",
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
                        "SHRTIMER_ST4DTCTL update by DMA mode",
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
                        "SHRTIMER_ST4CH0SET update by DMA mode",
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
                        "SHRTIMER_ST4CH0RST update by DMA mode",
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
                        "SHRTIMER_ST4CH1SET update by DMA mode",
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
                        "SHRTIMER_ST4CH1RST update by DMA mode",
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
                        "SHRTIMER_ST4EXEVFCFG0update by DMA mode",
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
                        "SHRTIMER_ST4EXEVFCFG1update by DMA mode",
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
                        "SHRTIMER_ST4CNTRST update by DMA mode",
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
                        "SHRTIMER_ST4CSCTL update by DMA mode",
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
                        "SHRTIMER_ST4CHOCTL update by DMA mode",
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
                        "SHRTIMER_ST4FLTCTL update by DMA mode",
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
                    name: "st4actl",
                    description: Some(
                        "SHRTIMER_ST4ACTL update by DMA mode",
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
                "SHRTIMER external event configuration register 0",
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
            ],
        },
        FieldSet {
            name: "Exevcfg1",
            extends: None,
            description: Some(
                "SHRTIMER external event configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exev5src",
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
                    name: "exev5p",
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
                    name: "exev5eg",
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
                    name: "exev6src",
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
                    name: "exev6p",
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
                        "External event 7polarity",
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
                "SHRTIMER external event digital filter control register",
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
                        "External event digital filter clock division",
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
                "SHRTIMER fault input configuration register 0",
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
                    name: "flt0insrc",
                    description: Some(
                        "Fault 0 input source",
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
                    name: "flt1insrc",
                    description: Some(
                        "Fault 2 input source",
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
                    name: "flt2insrc",
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
                    name: "flt3insrc",
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
                "SHRTIMER fault input configuration register 1",
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
                    name: "flt4insrc",
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
                    name: "fltfdiv",
                    description: Some(
                        "Fault input digital filter clock division",
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
            name: "Intc",
            extends: None,
            description: Some(
                "SHRTIMER interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flt0ifc",
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
                    name: "dllcalif",
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
                "SHRTIMER interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
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
                "SHRTIMER interrupt flag register",
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
                