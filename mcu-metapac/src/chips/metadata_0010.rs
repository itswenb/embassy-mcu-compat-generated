
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc47519d73",
                version: "v1",
                block: "ADC",
                ir: &gdadc47519d73::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CMP",
        address: 0x4001001c,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmpfe28bbad",
                version: "v1",
                block: "CMP",
                ir: &gdcmpfe28bbad::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcrc8a4036fe",
                version: "v1",
                block: "CRC",
                ir: &gdcrc8a4036fe::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbgmcu751e9bc8",
                version: "v1",
                block: "DBGMCU",
                ir: &gddbgmcu751e9bc8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma3c234e7c",
                version: "v1",
                block: "DMA",
                ir: &gddma3c234e7c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXTI",
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexti83469f4a",
                version: "v1",
                block: "EXTI",
                ir: &gdexti83469f4a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FMC",
        address: 0x40022000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfmc66eac118",
                version: "v1",
                block: "FMC",
                ir: &gdfmc66eac118::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FWDGT",
        address: 0x40003000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfwdgt5932fb56",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgt5932fb56::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x48000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOB",
        address: 0x48000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiob3a01cf30",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob3a01cf30::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOC",
        address: 0x48000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc88586c6c",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc88586c6c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOF",
        address: 0x48001400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiof41ef0f55",
                version: "v1",
                block: "GPIOF",
                ir: &gdgpiof41ef0f55::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C0",
        address: 0x40005400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c08b9ac71f",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c08b9ac71f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c08b9ac71f",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c08b9ac71f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "PMU",
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpmu1342a8f5",
                version: "v1",
                block: "PMU",
                ir: &gdpmu1342a8f5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RCU",
        address: 0x40021000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrcu2eec98a2",
                version: "v1",
                block: "RCU",
                ir: &gdrcu2eec98a2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RTC",
        address: 0x40002800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrtc335eb78b",
                version: "v1",
                block: "RTC",
                ir: &gdrtc335eb78b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI0",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi0e52b5b99",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0e52b5b99::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x40003800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi1de3ba8e1",
                version: "v1",
                block: "SPI1",
                ir: &gdspi1de3ba8e1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfg99035fab",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg99035fab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER0",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer039d8e338",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer039d8e338::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER13",
        address: 0x40002000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1309b6b8cc",
                version: "v1",
                block: "TIMER13",
                ir: &gdtimer1309b6b8cc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER14",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer144dec44bb",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer144dec44bb::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER15",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer15dc6fd783",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15dc6fd783::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER16",
        address: 0x40014800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer15dc6fd783",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15dc6fd783::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER2",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer20cf13e9f",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer20cf13e9f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER5",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5183dba8f",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer5183dba8f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART0",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart0bd1afef3",
                version: "v1",
                block: "USART0",
                ir: &gdusart0bd1afef3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart0bd1afef3",
                version: "v1",
                block: "USART0",
                ir: &gdusart0bd1afef3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "WWDGT",
        address: 0x40002c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdwwdgt50884229",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt50884229::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDGT",
        number: 0,
    },
    Interrupt {
        name: "LVD",
        number: 1,
    },
    Interrupt {
        name: "RTC",
        number: 2,
    },
    Interrupt {
        name: "FMC",
        number: 3,
    },
    Interrupt {
        name: "RCU",
        number: 4,
    },
    Interrupt {
        name: "EXTI0_1",
        number: 5,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 6,
    },
    Interrupt {
        name: "EXTI4_15",
        number: 7,
    },
    Interrupt {
        name: "DMA_CHANNEL0",
        number: 9,
    },
    Interrupt {
        name: "DMA_CHANNEL1_2",
        number: 10,
    },
    Interrupt {
        name: "DMA_CHANNEL3_4",
        number: 11,
    },
    Interrupt {
        name: "ADC_CMP",
        number: 12,
    },
    Interrupt {
        name: "TIMER0_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIMER0_CC",
        number: 14,
    },
    Interrupt {
        name: "TIMER2",
        number: 16,
    },
    Interrupt {
        name: "TIMER5",
        number: 17,
    },
    Interrupt {
        name: "TIMER13",
        number: 19,
    },
    Interrupt {
        name: "TIMER14",
        number: 20,
    },
    Interrupt {
        name: "TIMER15",
        number: 21,
    },
    Interrupt {
        name: "TIMER16",
        number: 22,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 23,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 24,
    },
    Interrupt {
        name: "SPI0",
        number: 25,
    },
    Interrupt {
        name: "SPI1",
        number: 26,
    },
    Interrupt {
        name: "USART0",
        number: 27,
    },
    Interrupt {
        name: "USART1",
        number: 28,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 34,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc47519d73_v1.rs"] pub mod gdadc47519d73;
#[path="../registers/gdcmpfe28bbad_v1.rs"] pub mod gdcmpfe28bbad;
#[path="../registers/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../registers/gddbgmcu751e9bc8_v1.rs"] pub mod gddbgmcu751e9bc8;
#[path="../registers/gddma3c234e7c_v1.rs"] pub mod gddma3c234e7c;
#[path="../registers/gdexti83469f4a_v1.rs"] pub mod gdexti83469f4a;
#[path="../registers/gdfmc66eac118_v1.rs"] pub mod gdfmc66eac118;
#[path="../registers/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../registers/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../registers/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../registers/gdgpioc88586c6c_v1.rs"] pub mod gdgpioc88586c6c;
#[path="../registers/gdgpiof41ef0f55_v1.rs"] pub mod gdgpiof41ef0f55;
#[path="../registers/gdi2c08b9ac71f_v1.rs"] pub mod gdi2c08b9ac71f;
#[path="../registers/gdpmu1342a8f5_v1.rs"] pub mod gdpmu1342a8f5;
#[path="../registers/gdrcu2eec98a2_v1.rs"] pub mod gdrcu2eec98a2;
#[path="../registers/gdrtc335eb78b_v1.rs"] pub mod gdrtc335eb78b;
#[path="../registers/gdspi0e52b5b99_v1.rs"] pub mod gdspi0e52b5b99;
#[path="../registers/gdspi1de3ba8e1_v1.rs"] pub mod gdspi1de3ba8e1;
#[path="../registers/gdsyscfg99035fab_v1.rs"] pub mod gdsyscfg99035fab;
#[path="../registers/gdtimer039d8e338_v1.rs"] pub mod gdtimer039d8e338;
#[path="../registers/gdtimer1309b6b8cc_v1.rs"] pub mod gdtimer1309b6b8cc;
#[path="../registers/gdtimer144dec44bb_v1.rs"] pub mod gdtimer144dec44bb;
#[path="../registers/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../registers/gdtimer20cf13e9f_v1.rs"] pub mod gdtimer20cf13e9f;
#[path="../registers/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../registers/gdusart0bd1afef3_v1.rs"] pub mod gdusart0bd1afef3;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
