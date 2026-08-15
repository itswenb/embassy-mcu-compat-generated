
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc0eac10ba1",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0eac10ba1::REGISTERS,
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
        name: "ADC1",
        address: 0x40012800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc1a7ac49f7",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1a7ac49f7::REGISTERS,
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
        name: "ADC2",
        address: 0x40013c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc2fc8b862c",
                version: "v1",
                block: "ADC2",
                ir: &gdadc2fc8b862c::REGISTERS,
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
        name: "AFIO",
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdafiof7a0f5d1",
                version: "v1",
                block: "AFIO",
                ir: &gdafiof7a0f5d1::REGISTERS,
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
        name: "BKP",
        address: 0x40006c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdbkp9347e01b",
                version: "v1",
                block: "BKP",
                ir: &gdbkp9347e01b::REGISTERS,
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
                kind: "gdcrc2255b0ef",
                version: "v1",
                block: "CRC",
                ir: &gdcrc2255b0ef::REGISTERS,
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
        name: "CTC",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdctc0cad8643",
                version: "v1",
                block: "CTC",
                ir: &gdctc0cad8643::REGISTERS,
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
        name: "DAC",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddace9519f30",
                version: "v1",
                block: "DAC",
                ir: &gddace9519f30::REGISTERS,
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
        address: 0xe0044000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg92e59a99",
                version: "v1",
                block: "DBG",
                ir: &gddbg92e59a99::REGISTERS,
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
        name: "DMA0",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma011392832",
                version: "v1",
                block: "DMA0",
                ir: &gddma011392832::REGISTERS,
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
        name: "DMA1",
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma13e943824",
                version: "v1",
                block: "DMA1",
                ir: &gddma13e943824::REGISTERS,
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
        name: "ENET_DMA",
        address: 0x40029000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetdma717f16f5",
                version: "v1",
                block: "ENET_DMA",
                ir: &gdenetdma717f16f5::REGISTERS,
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
        name: "ENET_MAC",
        address: 0x40028000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmac93552dd1",
                version: "v1",
                block: "ENET_MAC",
                ir: &gdenetmac93552dd1::REGISTERS,
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
        name: "ENET_MAC_FCTH",
        address: 0x40029080,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmacfcth8ada9e21",
                version: "v1",
                block: "ENET_MAC_FCTH",
                ir: &gdenetmacfcth8ada9e21::REGISTERS,
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
        name: "ENET_MSC",
        address: 0x40028100,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmsc10390666",
                version: "v1",
                block: "ENET_MSC",
                ir: &gdenetmsc10390666::REGISTERS,
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
        name: "ENET_PTP",
        address: 0x40028700,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetptp5c8a2d48",
                version: "v1",
                block: "ENET_PTP",
                ir: &gdenetptp5c8a2d48::REGISTERS,
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
        name: "EXMC",
        address: 0xa0000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc9f6a36f3",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc9f6a36f3::REGISTERS,
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
                kind: "gdexti285c938f",
                version: "v1",
                block: "EXTI",
                ir: &gdexti285c938f::REGISTERS,
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
                kind: "gdfmce1c61199",
                version: "v1",
                block: "FMC",
                ir: &gdfmce1c61199::REGISTERS,
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
                kind: "gdfwdgtdc3d0d7a",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtdc3d0d7a::REGISTERS,
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
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
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
        address: 0x40010c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiob48ef64a7",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob48ef64a7::REGISTERS,
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
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiocbac6a6b9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocbac6a6b9::REGISTERS,
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
        address: 0x40011400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiod0082ea0a",
                version: "v1",
                block: "GPIOD",
                ir: &gdgpiod0082ea0a::REGISTERS,
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
        name: "GPIOE",
        address: 0x40011800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioebd4c6204",
                version: "v1",
                block: "GPIOE",
                ir: &gdgpioebd4c6204::REGISTERS,
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
        address: 0x40011c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiofc46221fa",
                version: "v1",
                block: "GPIOF",
                ir: &gdgpiofc46221fa::REGISTERS,
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
        name: "GPIOG",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiogc07baa36",
                version: "v1",
                block: "GPIOG",
                ir: &gdgpiogc07baa36::REGISTERS,
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
                kind: "gdi2c0fc829b2b",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0fc829b2b::REGISTERS,
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
                kind: "gdi2c1c9f3d155",
                version: "v1",
                block: "I2C1",
                ir: &gdi2c1c9f3d155::REGISTERS,
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
        name: "I2C2",
        address: 0x4000c000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c2c47a3cd3",
                version: "v1",
                block: "I2C2",
                ir: &gdi2c2c47a3cd3::REGISTERS,
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
                kind: "gdpmuce6d9d5f",
                version: "v1",
                block: "PMU",
                ir: &gdpmuce6d9d5f::REGISTERS,
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
                kind: "gdrcu5b6d7d51",
                version: "v1",
                block: "RCU",
                ir: &gdrcu5b6d7d51::REGISTERS,
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
                kind: "gdrtc6b0c077c",
                version: "v1",
                block: "RTC",
                ir: &gdrtc6b0c077c::REGISTERS,
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
                kind: "gdspi03c62ee5d",
                version: "v1",
                block: "SPI0",
                ir: &gdspi03c62ee5d::REGISTERS,
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
                kind: "gdspi18ad3a9b8",
                version: "v1",
                block: "SPI1",
                ir: &gdspi18ad3a9b8::REGISTERS,
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
        name: "SPI2",
        address: 0x40003c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi2541b1531",
                version: "v1",
                block: "SPI2",
                ir: &gdspi2541b1531::REGISTERS,
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
        name: "SQPI",
        address: 0xa0001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsqpi2c944cc9",
                version: "v1",
                block: "SQPI",
                ir: &gdsqpi2c944cc9::REGISTERS,
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
                kind: "gdtimer0a0aa2af0",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a0aa2af0::REGISTERS,
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
        name: "TIMER1",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer11e77ba65",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer11e77ba65::REGISTERS,
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
                kind: "gdtimer2868720fc",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer2868720fc::REGISTERS,
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
        name: "TIMER3",
        address: 0x40000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer3b6308935",
                version: "v1",
                block: "TIMER3",
                ir: &gdtimer3b6308935::REGISTERS,
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
        name: "TIMER4",
        address: 0x40000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer456047876",
                version: "v1",
                block: "TIMER4",
                ir: &gdtimer456047876::REGISTERS,
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
                kind: "gdtimer54b5e73ec",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer54b5e73ec::REGISTERS,
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
        name: "TIMER6",
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer642c906a4",
                version: "v1",
                block: "TIMER6",
                ir: &gdtimer642c906a4::REGISTERS,
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
        name: "TIMER7",
        address: 0x40013400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer7b78318c5",
                version: "v1",
                block: "TIMER7",
                ir: &gdtimer7b78318c5::REGISTERS,
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
        name: "UART3",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gduart3a8b9d61c",
                version: "v1",
                block: "UART3",
                ir: &gduart3a8b9d61c::REGISTERS,
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
        name: "UART4",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "gduart421c89746",
                version: "v1",
                block: "UART4",
                ir: &gduart421c89746::REGISTERS,
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
                kind: "gdusart016d80f16",
                version: "v1",
                block: "USART0",
                ir: &gdusart016d80f16::REGISTERS,
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
                kind: "gdusart1c03e4b14",
                version: "v1",
                block: "USART1",
                ir: &gdusart1c03e4b14::REGISTERS,
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
                kind: "gdusart2719a6075",
                version: "v1",
                block: "USART2",
                ir: &gdusart2719a6075::REGISTERS,
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
        name: "USART5",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart55c097497",
                version: "v1",
                block: "USART5",
                ir: &gdusart55c097497::REGISTERS,
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
        name: "USBD",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbd2168ff9b",
                version: "v1",
                block: "USBD",
                ir: &gdusbd2168ff9b::REGISTERS,
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
        name: "TAMPER",
        number: 2,
    },
    Interrupt {
        name: "RTC",
        number: 3,
    },
    Interrupt {
        name: "FMC",
        number: 4,
    },
    Interrupt {
        name: "RCU_CTC",
        number: 5,
    },
    Interrupt {
        name: "EXTI_LINE0",
        number: 6,
    },
    Interrupt {
        name: "EXTI_LINE1",
        number: 7,
    },
    Interrupt {
        name: "EXTI_LINE2",
        number: 8,
    },
    Interrupt {
        name: "EXTI_LINE3",
        number: 9,
    },
    Interrupt {
        name: "EXTI_LINE4",
        number: 10,
    },
    Interrupt {
        name: "DMA0_CHANNEL0",
        number: 11,
    },
    Interrupt {
        name: "DMA0_CHANNEL1",
        number: 12,
    },
    Interrupt {
        name: "DMA0_CHANNEL2",
        number: 13,
    },
    Interrupt {
        name: "DMA0_CHANNEL3",
        number: 14,
    },
    Interrupt {
        name: "DMA0_CHANNEL4",
        number: 15,
    },
    Interrupt {
        name: "DMA0_CHANNEL5",
        number: 16,
    },
    Interrupt {
        name: "DMA0_CHANNEL6",
        number: 17,
    },
    Interrupt {
        name: "ADC0_1",
        number: 18,
    },
    Interrupt {
        name: "USBD_HP",
        number: 19,
    },
    Interrupt {
        name: "USBD_LP",
        number: 20,
    },
    Interrupt {
        name: "EXTI_LINE9_5",
        number: 23,
    },
    Interrupt {
        name: "TIMER0_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT",
        number: 26,
    },
    Interrupt {
        name: "TIMER0_CC",
        number: 27,
    },
    Interrupt {
        name: "TIMER1",
        number: 28,
    },
    Interrupt {
        name: "TIMER2",
        number: 29,
    },
    Interrupt {
        name: "TIMER3",
        number: 30,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI0",
        number: 35,
    },
    Interrupt {
        name: "SPI1",
        number: 36,
    },
    Interrupt {
        name: "USART0",
        number: 37,
    },
    Interrupt {
        name: "USART1",
        number: 38,
    },
    Interrupt {
        name: "USART2",
        number: 39,
    },
    Interrupt {
        name: "EXTI_LINE15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "USBD_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIMER7_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TRG_CMT",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CC",
        number: 46,
    },
    Interrupt {
        name: "ADC2",
        number: 47,
    },
    Interrupt {
        name: "EXMC",
        number: 48,
    },
    Interrupt {
        name: "TIMER4",
        number: 50,
    },
    Interrupt {
        name: "SPI2",
        number: 51,
    },
    Interrupt {
        name: "UART3",
        number: 52,
    },
    Interrupt {
        name: "UART4",
        number: 53,
    },
    Interrupt {
        name: "TIMER5",
        number: 54,
    },
    Interrupt {
        name: "TIMER6",
        number: 55,
    },
    Interrupt {
        name: "DMA1_CHANNEL0",
        number: 56,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 57,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 58,
    },
    Interrupt {
        name: "DMA1_CHANNEL3_DMA1_CHANNEL4",
        number: 59,
    },
    Interrupt {
        name: "ENET",
        number: 61,
    },
    Interrupt {
        name: "ENET_WKUP",
        number: 62,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 82,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 83,
    },
    Interrupt {
        name: "USART5",
        number: 84,
    },
    Interrupt {
        name: "I2C2_WKUP",
        number: 85,
    },
    Interrupt {
        name: "USART5_WKUP",
        number: 86,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc0eac10ba1_v1.rs"] pub mod gdadc0eac10ba1;
#[path="../registers/gdadc1a7ac49f7_v1.rs"] pub mod gdadc1a7ac49f7;
#[path="../registers/gdadc2fc8b862c_v1.rs"] pub mod gdadc2fc8b862c;
#[path="../registers/gdafiof7a0f5d1_v1.rs"] pub mod gdafiof7a0f5d1;
#[path="../registers/gdbkp9347e01b_v1.rs"] pub mod gdbkp9347e01b;
#[path="../registers/gdcrc2255b0ef_v1.rs"] pub mod gdcrc2255b0ef;
#[path="../registers/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../registers/gddace9519f30_v1.rs"] pub mod gddace9519f30;
#[path="../registers/gddbg92e59a99_v1.rs"] pub mod gddbg92e59a99;
#[path="../registers/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../registers/gddma13e943824_v1.rs"] pub mod gddma13e943824;
#[path="../registers/gdenetdma717f16f5_v1.rs"] pub mod gdenetdma717f16f5;
#[path="../registers/gdenetmac93552dd1_v1.rs"] pub mod gdenetmac93552dd1;
#[path="../registers/gdenetmacfcth8ada9e21_v1.rs"] pub mod gdenetmacfcth8ada9e21;
#[path="../registers/gdenetmsc10390666_v1.rs"] pub mod gdenetmsc10390666;
#[path="../registers/gdenetptp5c8a2d48_v1.rs"] pub mod gdenetptp5c8a2d48;
#[path="../registers/gdexmc9f6a36f3_v1.rs"] pub mod gdexmc9f6a36f3;
#[path="../registers/gdexti285c938f_v1.rs"] pub mod gdexti285c938f;
#[path="../registers/gdfmce1c61199_v1.rs"] pub mod gdfmce1c61199;
#[path="../registers/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../registers/gdgpioac3e5c224_v1.rs"] pub mod gdgpioac3e5c224;
#[path="../registers/gdgpiob48ef64a7_v1.rs"] pub mod gdgpiob48ef64a7;
#[path="../registers/gdgpiocbac6a6b9_v1.rs"] pub mod gdgpiocbac6a6b9;
#[path="../registers/gdgpiod0082ea0a_v1.rs"] pub mod gdgpiod0082ea0a;
#[path="../registers/gdgpioebd4c6204_v1.rs"] pub mod gdgpioebd4c6204;
#[path="../registers/gdgpiofc46221fa_v1.rs"] pub mod gdgpiofc46221fa;
#[path="../registers/gdgpiogc07baa36_v1.rs"] pub mod gdgpiogc07baa36;
#[path="../registers/gdi2c0fc829b2b_v1.rs"] pub mod gdi2c0fc829b2b;
#[path="../registers/gdi2c1c9f3d155_v1.rs"] pub mod gdi2c1c9f3d155;
#[path="../registers/gdi2c2c47a3cd3_v1.rs"] pub mod gdi2c2c47a3cd3;
#[path="../registers/gdpmuce6d9d5f_v1.rs"] pub mod gdpmuce6d9d5f;
#[path="../registers/gdrcu5b6d7d51_v1.rs"] pub mod gdrcu5b6d7d51;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdspi03c62ee5d_v1.rs"] pub mod gdspi03c62ee5d;
#[path="../registers/gdspi18ad3a9b8_v1.rs"] pub mod gdspi18ad3a9b8;
#[path="../registers/gdspi2541b1531_v1.rs"] pub mod gdspi2541b1531;
#[path="../registers/gdsqpi2c944cc9_v1.rs"] pub mod gdsqpi2c944cc9;
#[path="../registers/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../registers/gdtimer11e77ba65_v1.rs"] pub mod gdtimer11e77ba65;
#[path="../registers/gdtimer2868720fc_v1.rs"] pub mod gdtimer2868720fc;
#[path="../registers/gdtimer3b6308935_v1.rs"] pub mod gdtimer3b6308935;
#[path="../registers/gdtimer456047876_v1.rs"] pub mod gdtimer456047876;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtimer642c906a4_v1.rs"] pub mod gdtimer642c906a4;
#[path="../registers/gdtimer7b78318c5_v1.rs"] pub mod gdtimer7b78318c5;
#[path="../registers/gduart3a8b9d61c_v1.rs"] pub mod gduart3a8b9d61c;
#[path="../registers/gduart421c89746_v1.rs"] pub mod gduart421c89746;
#[path="../registers/gdusart016d80f16_v1.rs"] pub mod gdusart016d80f16;
#[path="../registers/gdusart1c03e4b14_v1.rs"] pub mod gdusart1c03e4b14;
#[path="../registers/gdusart2719a6075_v1.rs"] pub mod gdusart2719a6075;
#[path="../registers/gdusart55c097497_v1.rs"] pub mod gdusart55c097497;
#[path="../registers/gdusbd2168ff9b_v1.rs"] pub mod gdusbd2168ff9b;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
