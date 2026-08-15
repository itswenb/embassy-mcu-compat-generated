
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
                kind: "gdafio79140b31",
                version: "v1",
                block: "AFIO",
                ir: &gdafio79140b31::REGISTERS,
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
        name: "CAN0",
        address: 0x40006400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan0b8705c1f",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0b8705c1f::REGISTERS,
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
        name: "CAN1",
        address: 0x40006800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan1ab0668da",
                version: "v1",
                block: "CAN1",
                ir: &gdcan1ab0668da::REGISTERS,
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
        name: "CAN2",
        address: 0x4000cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan2368703f4",
                version: "v1",
                block: "CAN2",
                ir: &gdcan2368703f4::REGISTERS,
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
                kind: "gdcmp49f0325b",
                version: "v1",
                block: "CMP",
                ir: &gdcmp49f0325b::REGISTERS,
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
                kind: "gddbg0aebad37",
                version: "v1",
                block: "DBG",
                ir: &gddbg0aebad37::REGISTERS,
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
        name: "MASTER_TIMER",
        address: 0x40017400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdmastertimerc379fdf1",
                version: "v1",
                block: "MASTER_TIMER",
                ir: &gdmastertimerc379fdf1::REGISTERS,
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
                kind: "gdrcu325d5693",
                version: "v1",
                block: "RCU",
                ir: &gdrcu325d5693::REGISTERS,
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
        name: "SHRTIMER_COMMON",
        address: 0x40017780,
        registers: Some(
            PeripheralRegisters {
                kind: "gdshrtimercommon3523955c",
                version: "v1",
                block: "SHRTIMER_COMMON",
                ir: &gdshrtimercommon3523955c::REGISTERS,
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
        name: "SLAVE_TIMER0",
        address: 0x40017480,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslavetimer01ad417fe",
                version: "v1",
                block: "SLAVE_TIMER0",
                ir: &gdslavetimer01ad417fe::REGISTERS,
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
        name: "SLAVE_TIMER1",
        address: 0x40017500,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslavetimer105c1946a",
                version: "v1",
                block: "SLAVE_TIMER1",
                ir: &gdslavetimer105c1946a::REGISTERS,
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
        name: "SLAVE_TIMER2",
        address: 0x40017580,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslavetimer249c017c7",
                version: "v1",
                block: "SLAVE_TIMER2",
                ir: &gdslavetimer249c017c7::REGISTERS,
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
        name: "SLAVE_TIMER3",
        address: 0x40017600,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslavetimer3b64bedd3",
                version: "v1",
                block: "SLAVE_TIMER3",
                ir: &gdslavetimer3b64bedd3::REGISTERS,
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
        name: "SLAVE_TIMER4",
        address: 0x40017680,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslavetimer45a69fbbd",
                version: "v1",
                block: "SLAVE_TIMER4",
                ir: &gdslavetimer45a69fbbd::REGISTERS,
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
        name: "TIMER10",
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer10ba08925f",
                version: "v1",
                block: "TIMER10",
                ir: &gdtimer10ba08925f::REGISTERS,
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
        name: "TIMER11",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer118878f54b",
                version: "v1",
                block: "TIMER11",
                ir: &gdtimer118878f54b::REGISTERS,
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
        name: "TIMER12",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer12119fcec0",
                version: "v1",
                block: "TIMER12",
                ir: &gdtimer12119fcec0::REGISTERS,
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
                kind: "gdtimer13e079e34e",
                version: "v1",
                block: "TIMER13",
                ir: &gdtimer13e079e34e::REGISTERS,
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
        name: "TIMER8",
        address: 0x40014c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer895e47fd0",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer895e47fd0::REGISTERS,
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
        name: "TIMER9",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94ea426f7",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer94ea426f7::REGISTERS,
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
        name: "TMU",
        address: 0x40080000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtmu528d66a8",
                version: "v1",
                block: "TMU",
                ir: &gdtmu528d66a8::REGISTERS,
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
        name: "USBGS_HOST",
        address: 0x50000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbgshost2794baaa",
                version: "v1",
                block: "USBGS_HOST",
                ir: &gdusbgshost2794baaa::REGISTERS,
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
        name: "USBHS_DEVICE",
        address: 0x50000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhsdevicea32ae2bb",
                version: "v1",
                block: "USBHS_DEVICE",
                ir: &gdusbhsdevicea32ae2bb::REGISTERS,
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
        name: "USBHS_GLOBAL",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhsglobalef49f048",
                version: "v1",
                block: "USBHS_GLOBAL",
                ir: &gdusbhsglobalef49f048::REGISTERS,
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
        name: "USBHS_PWRCLK",
        address: 0x50000e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhspwrclk77209260",
                version: "v1",
                block: "USBHS_PWRCLK",
                ir: &gdusbhspwrclk77209260::REGISTERS,
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
        name: "CAN0_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN0_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN0_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN0_EWMC",
        number: 22,
    },
    Interrupt {
        name: "EXTI_LINE9_5",
        number: 23,
    },
    Interrupt {
        name: "TIMER0_BRK_TIMER8",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP_TIMER9",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT_TIMER10",
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
        name: "USBHS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIMER7_BRK_TIMER11",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP_TIMER12",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TRG_CMT_TIMER13",
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
        name: "CAN1_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN1_EWMC",
        number: 66,
    },
    Interrupt {
        name: "USBHS",
        number: 67,
    },
    Interrupt {
        name: "SHRTIMER_IRQ2",
        number: 69,
    },
    Interrupt {
        name: "SHRTIMER_IRQ3",
        number: 70,
    },
    Interrupt {
        name: "SHRTIMER_IRQ4",
        number: 71,
    },
    Interrupt {
        name: "SHRTIMER_IRQ5",
        number: 72,
    },
    Interrupt {
        name: "SHRTIMER_IRQ6",
        number: 73,
    },
    Interrupt {
        name: "SHRTIMER_IRQ0",
        number: 76,
    },
    Interrupt {
        name: "SHRTIMER_IRQ1",
        number: 77,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 78,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 79,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 80,
    },
    Interrupt {
        name: "CAN2_EWMC",
        number: 81,
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
    Interrupt {
        name: "TMU",
        number: 87,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc0eac10ba1_v1.rs"] pub mod gdadc0eac10ba1;
#[path="../registers/gdadc1a7ac49f7_v1.rs"] pub mod gdadc1a7ac49f7;
#[path="../registers/gdadc2fc8b862c_v1.rs"] pub mod gdadc2fc8b862c;
#[path="../registers/gdafio79140b31_v1.rs"] pub mod gdafio79140b31;
#[path="../registers/gdbkp9347e01b_v1.rs"] pub mod gdbkp9347e01b;
#[path="../registers/gdcan0b8705c1f_v1.rs"] pub mod gdcan0b8705c1f;
#[path="../registers/gdcan1ab0668da_v1.rs"] pub mod gdcan1ab0668da;
#[path="../registers/gdcan2368703f4_v1.rs"] pub mod gdcan2368703f4;
#[path="../registers/gdcmp49f0325b_v1.rs"] pub mod gdcmp49f0325b;
#[path="../registers/gdcrc2255b0ef_v1.rs"] pub mod gdcrc2255b0ef;
#[path="../registers/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../registers/gddace9519f30_v1.rs"] pub mod gddace9519f30;
#[path="../registers/gddbg0aebad37_v1.rs"] pub mod gddbg0aebad37;
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
#[path="../registers/gdmastertimerc379fdf1_v1.rs"] pub mod gdmastertimerc379fdf1;
#[path="../registers/gdpmuce6d9d5f_v1.rs"] pub mod gdpmuce6d9d5f;
#[path="../registers/gdrcu325d5693_v1.rs"] pub mod gdrcu325d5693;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdshrtimercommon3523955c_v1.rs"] pub mod gdshrtimercommon3523955c;
#[path="../registers/gdslavetimer01ad417fe_v1.rs"] pub mod gdslavetimer01ad417fe;
#[path="../registers/gdslavetimer105c1946a_v1.rs"] pub mod gdslavetimer105c1946a;
#[path="../registers/gdslavetimer249c017c7_v1.rs"] pub mod gdslavetimer249c017c7;
#[path="../registers/gdslavetimer3b64bedd3_v1.rs"] pub mod gdslavetimer3b64bedd3;
#[path="../registers/gdslavetimer45a69fbbd_v1.rs"] pub mod gdslavetimer45a69fbbd;
#[path="../registers/gdspi03c62ee5d_v1.rs"] pub mod gdspi03c62ee5d;
#[path="../registers/gdspi18ad3a9b8_v1.rs"] pub mod gdspi18ad3a9b8;
#[path="../registers/gdspi2541b1531_v1.rs"] pub mod gdspi2541b1531;
#[path="../registers/gdsqpi2c944cc9_v1.rs"] pub mod gdsqpi2c944cc9;
#[path="../registers/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../registers/gdtimer10ba08925f_v1.rs"] pub mod gdtimer10ba08925f;
#[path="../registers/gdtimer118878f54b_v1.rs"] pub mod gdtimer118878f54b;
#[path="../registers/gdtimer11e77ba65_v1.rs"] pub mod gdtimer11e77ba65;
#[path="../registers/gdtimer12119fcec0_v1.rs"] pub mod gdtimer12119fcec0;
#[path="../registers/gdtimer13e079e34e_v1.rs"] pub mod gdtimer13e079e34e;
#[path="../registers/gdtimer2868720fc_v1.rs"] pub mod gdtimer2868720fc;
#[path="../registers/gdtimer3b6308935_v1.rs"] pub mod gdtimer3b6308935;
#[path="../registers/gdtimer456047876_v1.rs"] pub mod gdtimer456047876;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtimer642c906a4_v1.rs"] pub mod gdtimer642c906a4;
#[path="../registers/gdtimer7b78318c5_v1.rs"] pub mod gdtimer7b78318c5;
#[path="../registers/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../registers/gdtimer94ea426f7_v1.rs"] pub mod gdtimer94ea426f7;
#[path="../registers/gdtmu528d66a8_v1.rs"] pub mod gdtmu528d66a8;
#[path="../registers/gduart3a8b9d61c_v1.rs"] pub mod gduart3a8b9d61c;
#[path="../registers/gduart421c89746_v1.rs"] pub mod gduart421c89746;
#[path="../registers/gdusart016d80f16_v1.rs"] pub mod gdusart016d80f16;
#[path="../registers/gdusart1c03e4b14_v1.rs"] pub mod gdusart1c03e4b14;
#[path="../registers/gdusart2719a6075_v1.rs"] pub mod gdusart2719a6075;
#[path="../registers/gdusart55c097497_v1.rs"] pub mod gdusart55c097497;
#[path="../registers/gdusbgshost2794baaa_v1.rs"] pub mod gdusbgshost2794baaa;
#[path="../registers/gdusbhsdevicea32ae2bb_v1.rs"] pub mod gdusbhsdevicea32ae2bb;
#[path="../registers/gdusbhsglobalef49f048_v1.rs"] pub mod gdusbhsglobalef49f048;
#[path="../registers/gdusbhspwrclk77209260_v1.rs"] pub mod gdusbhspwrclk77209260;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
