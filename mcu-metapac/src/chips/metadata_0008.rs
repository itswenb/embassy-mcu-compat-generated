
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcf283e75f",
                version: "v1",
                block: "ADC",
                ir: &gdadcf283e75f::REGISTERS,
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
        address: 0x40017c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmp90f31e19",
                version: "v1",
                block: "CMP",
                ir: &gdcmp90f31e19::REGISTERS,
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
                kind: "gdcrcc5aea4f6",
                version: "v1",
                block: "CRC",
                ir: &gdcrcc5aea4f6::REGISTERS,
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
        name: "DBG",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg63bf1c6a",
                version: "v1",
                block: "DBG",
                ir: &gddbg63bf1c6a::REGISTERS,
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
                kind: "gddma4ef405a0",
                version: "v1",
                block: "DMA",
                ir: &gddma4ef405a0::REGISTERS,
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
        name: "DMAMUX",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamux6e5e79f5",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux6e5e79f5::REGISTERS,
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
                kind: "gdexti0b771307",
                version: "v1",
                block: "EXTI",
                ir: &gdexti0b771307::REGISTERS,
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
                kind: "gdfmcfda27991",
                version: "v1",
                block: "FMC",
                ir: &gdfmcfda27991::REGISTERS,
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
                kind: "gdgpioaf444c8f9",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf444c8f9::REGISTERS,
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
                kind: "gdgpiob6efbc75f",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob6efbc75f::REGISTERS,
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
                kind: "gdgpiob6efbc75f",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob6efbc75f::REGISTERS,
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
        name: "GPIOD",
        address: 0x48000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiob6efbc75f",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob6efbc75f::REGISTERS,
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
                kind: "gdgpiob6efbc75f",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob6efbc75f::REGISTERS,
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
                kind: "gdi2c0cd973dc4",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0cd973dc4::REGISTERS,
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
                kind: "gdi2c0cd973dc4",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0cd973dc4::REGISTERS,
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
                kind: "gdpmu5c455f73",
                version: "v1",
                block: "PMU",
                ir: &gdpmu5c455f73::REGISTERS,
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
                kind: "gdrcu53e64d61",
                version: "v1",
                block: "RCU",
                ir: &gdrcu53e64d61::REGISTERS,
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
                kind: "gdrtcadbe90b4",
                version: "v1",
                block: "RTC",
                ir: &gdrtcadbe90b4::REGISTERS,
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
                kind: "gdspi058ca27a4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi058ca27a4::REGISTERS,
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
                kind: "gdspi187e06d26",
                version: "v1",
                block: "SPI1",
                ir: &gdspi187e06d26::REGISTERS,
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
                kind: "gdsyscfg4a40a7d3",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg4a40a7d3::REGISTERS,
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
                kind: "gdtimer0533ef489",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0533ef489::REGISTERS,
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
                kind: "gdtimer13b6ee86d8",
                version: "v1",
                block: "TIMER13",
                ir: &gdtimer13b6ee86d8::REGISTERS,
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
                kind: "gdtimer15a386f15f",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15a386f15f::REGISTERS,
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
                kind: "gdtimer15a386f15f",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15a386f15f::REGISTERS,
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
                kind: "gdtimer270dbabdc",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer270dbabdc::REGISTERS,
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
                kind: "gdusart0a0301eea",
                version: "v1",
                block: "USART0",
                ir: &gdusart0a0301eea::REGISTERS,
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
                kind: "gdusart13d9e0c6f",
                version: "v1",
                block: "USART1",
                ir: &gdusart13d9e0c6f::REGISTERS,
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
        name: "USART2",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart13d9e0c6f",
                version: "v1",
                block: "USART1",
                ir: &gdusart13d9e0c6f::REGISTERS,
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
                kind: "gdwwdgtdd622579",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgtdd622579::REGISTERS,
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
        name: "RTC_TAMPER_TIMESTAMP",
        number: 1,
    },
    Interrupt {
        name: "RTC_WAKE",
        number: 2,
    },
    Interrupt {
        name: "FMC_GLOBAL",
        number: 3,
    },
    Interrupt {
        name: "EXTI0",
        number: 5,
    },
    Interrupt {
        name: "EXTI01",
        number: 6,
    },
    Interrupt {
        name: "EXTI2",
        number: 7,
    },
    Interrupt {
        name: "EXTI3",
        number: 8,
    },
    Interrupt {
        name: "EXTI4",
        number: 9,
    },
    Interrupt {
        name: "DMA_CHANNEL0",
        number: 10,
    },
    Interrupt {
        name: "DMA_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "ADC",
        number: 13,
    },
    Interrupt {
        name: "USART0",
        number: 14,
    },
    Interrupt {
        name: "USART1",
        number: 15,
    },
    Interrupt {
        name: "USART2",
        number: 16,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 17,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 18,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 19,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 20,
    },
    Interrupt {
        name: "SPI0",
        number: 21,
    },
    Interrupt {
        name: "SPI1",
        number: 22,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 23,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_TRIG_UP_BREAK",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_CAP",
        number: 26,
    },
    Interrupt {
        name: "TIMER2",
        number: 27,
    },
    Interrupt {
        name: "TIMER13",
        number: 28,
    },
    Interrupt {
        name: "TIMER15",
        number: 29,
    },
    Interrupt {
        name: "TIMER16",
        number: 30,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 31,
    },
    Interrupt {
        name: "DMA_MUX",
        number: 33,
    },
    Interrupt {
        name: "CMP0",
        number: 34,
    },
    Interrupt {
        name: "CMP1",
        number: 35,
    },
    Interrupt {
        name: "I2C0_WAKE",
        number: 36,
    },
    Interrupt {
        name: "I2C1_WAKE",
        number: 37,
    },
    Interrupt {
        name: "USART0_WAKE",
        number: 38,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadcf283e75f_v1.rs"] pub mod gdadcf283e75f;
#[path="../registers/gdcmp90f31e19_v1.rs"] pub mod gdcmp90f31e19;
#[path="../registers/gdcrcc5aea4f6_v1.rs"] pub mod gdcrcc5aea4f6;
#[path="../registers/gddbg63bf1c6a_v1.rs"] pub mod gddbg63bf1c6a;
#[path="../registers/gddma4ef405a0_v1.rs"] pub mod gddma4ef405a0;
#[path="../registers/gddmamux6e5e79f5_v1.rs"] pub mod gddmamux6e5e79f5;
#[path="../registers/gdexti0b771307_v1.rs"] pub mod gdexti0b771307;
#[path="../registers/gdfmcfda27991_v1.rs"] pub mod gdfmcfda27991;
#[path="../registers/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../registers/gdgpioaf444c8f9_v1.rs"] pub mod gdgpioaf444c8f9;
#[path="../registers/gdgpiob6efbc75f_v1.rs"] pub mod gdgpiob6efbc75f;
#[path="../registers/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../registers/gdpmu5c455f73_v1.rs"] pub mod gdpmu5c455f73;
#[path="../registers/gdrcu53e64d61_v1.rs"] pub mod gdrcu53e64d61;
#[path="../registers/gdrtcadbe90b4_v1.rs"] pub mod gdrtcadbe90b4;
#[path="../registers/gdspi058ca27a4_v1.rs"] pub mod gdspi058ca27a4;
#[path="../registers/gdspi187e06d26_v1.rs"] pub mod gdspi187e06d26;
#[path="../registers/gdsyscfg4a40a7d3_v1.rs"] pub mod gdsyscfg4a40a7d3;
#[path="../registers/gdtimer0533ef489_v1.rs"] pub mod gdtimer0533ef489;
#[path="../registers/gdtimer13b6ee86d8_v1.rs"] pub mod gdtimer13b6ee86d8;
#[path="../registers/gdtimer15a386f15f_v1.rs"] pub mod gdtimer15a386f15f;
#[path="../registers/gdtimer270dbabdc_v1.rs"] pub mod gdtimer270dbabdc;
#[path="../registers/gdusart0a0301eea_v1.rs"] pub mod gdusart0a0301eea;
#[path="../registers/gdusart13d9e0c6f_v1.rs"] pub mod gdusart13d9e0c6f;
#[path="../registers/gdwwdgtdd622579_v1.rs"] pub mod gdwwdgtdd622579;
