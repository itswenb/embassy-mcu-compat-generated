
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc0206d7fed",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0206d7fed::REGISTERS,
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
                kind: "gdadc1152dce23",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1152dce23::REGISTERS,
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
                kind: "gdadc1152dce23",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1152dce23::REGISTERS,
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
                kind: "gdafiocb3bc598",
                version: "v1",
                block: "AFIO",
                ir: &gdafiocb3bc598::REGISTERS,
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
                kind: "gdbkp16a620e0",
                version: "v1",
                block: "BKP",
                ir: &gdbkp16a620e0::REGISTERS,
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
                kind: "gdcan050e9510d",
                version: "v1",
                block: "CAN0",
                ir: &gdcan050e9510d::REGISTERS,
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
                kind: "gdcan050e9510d",
                version: "v1",
                block: "CAN0",
                ir: &gdcan050e9510d::REGISTERS,
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
                kind: "gdcrc66a4f78d",
                version: "v1",
                block: "CRC",
                ir: &gdcrc66a4f78d::REGISTERS,
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
                kind: "gddac1e2c6112",
                version: "v1",
                block: "DAC",
                ir: &gddac1e2c6112::REGISTERS,
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
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg895e48a2",
                version: "v1",
                block: "DBG",
                ir: &gddbg895e48a2::REGISTERS,
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
                kind: "gdexti11a1be47",
                version: "v1",
                block: "EXTI",
                ir: &gdexti11a1be47::REGISTERS,
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
                kind: "gdfmc2c447638",
                version: "v1",
                block: "FMC",
                ir: &gdfmc2c447638::REGISTERS,
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
        name: "GPIOC",
        address: 0x40011000,
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
        name: "GPIOD",
        address: 0x40011400,
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
        name: "GPIOE",
        address: 0x40011800,
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
        name: "GPIOF",
        address: 0x40011c00,
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
        name: "GPIOG",
        address: 0x40012000,
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
        name: "I2C0",
        address: 0x40005400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c037cbcb65",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c037cbcb65::REGISTERS,
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
                kind: "gdi2c037cbcb65",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c037cbcb65::REGISTERS,
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
                kind: "gdpmu3bb0ce43",
                version: "v1",
                block: "PMU",
                ir: &gdpmu3bb0ce43::REGISTERS,
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
                kind: "gdrcu08b36be4",
                version: "v1",
                block: "RCU",
                ir: &gdrcu08b36be4::REGISTERS,
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
                kind: "gdspi0946ced9d",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0946ced9d::REGISTERS,
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
                kind: "gdspi0946ced9d",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0946ced9d::REGISTERS,
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
                kind: "gdspi0946ced9d",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0946ced9d::REGISTERS,
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
                kind: "gdtimer1b47aad48",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1b47aad48::REGISTERS,
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
                kind: "gdtimer911ce6ab6",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer911ce6ab6::REGISTERS,
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
        name: "TIMER12",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer911ce6ab6",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer911ce6ab6::REGISTERS,
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
                kind: "gdtimer911ce6ab6",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer911ce6ab6::REGISTERS,
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
                kind: "gdtimer1b47aad48",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1b47aad48::REGISTERS,
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
                kind: "gdtimer1b47aad48",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1b47aad48::REGISTERS,
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
                kind: "gdtimer1b47aad48",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1b47aad48::REGISTERS,
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
        name: "TIMER7",
        address: 0x40013400,
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
                kind: "gdtimer911ce6ab6",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer911ce6ab6::REGISTERS,
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
                kind: "gduart3c678fe30",
                version: "v1",
                block: "UART3",
                ir: &gduart3c678fe30::REGISTERS,
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
                kind: "gduart3c678fe30",
                version: "v1",
                block: "UART3",
                ir: &gduart3c678fe30::REGISTERS,
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
                kind: "gdusart082eeb6ea",
                version: "v1",
                block: "USART0",
                ir: &gdusart082eeb6ea::REGISTERS,
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
                kind: "gdusart082eeb6ea",
                version: "v1",
                block: "USART0",
                ir: &gdusart082eeb6ea::REGISTERS,
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
                kind: "gdusart082eeb6ea",
                version: "v1",
                block: "USART0",
                ir: &gdusart082eeb6ea::REGISTERS,
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
        name: "USBFS_DEVICE",
        address: 0x50000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfsdevicec5a1209d",
                version: "v1",
                block: "USBFS_DEVICE",
                ir: &gdusbfsdevicec5a1209d::REGISTERS,
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
        name: "USBFS_GLOBAL",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfsglobal3505227a",
                version: "v1",
                block: "USBFS_GLOBAL",
                ir: &gdusbfsglobal3505227a::REGISTERS,
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
        name: "USBFS_HOST",
        address: 0x50000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfshost5f42a79e",
                version: "v1",
                block: "USBFS_HOST",
                ir: &gdusbfshost5f42a79e::REGISTERS,
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
        name: "USBFS_PWRCLK",
        address: 0x50000e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfspwrclk2ac667f0",
                version: "v1",
                block: "USBFS_PWRCLK",
                ir: &gdusbfspwrclk2ac667f0::REGISTERS,
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
        name: "USBFS_WKUP",
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
        name: "DMA1_CHANNEL3",
        number: 59,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 60,
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
        name: "USBFS",
        number: 67,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc0206d7fed_v1.rs"] pub mod gdadc0206d7fed;
#[path="../registers/gdadc1152dce23_v1.rs"] pub mod gdadc1152dce23;
#[path="../registers/gdafiocb3bc598_v1.rs"] pub mod gdafiocb3bc598;
#[path="../registers/gdbkp16a620e0_v1.rs"] pub mod gdbkp16a620e0;
#[path="../registers/gdcan050e9510d_v1.rs"] pub mod gdcan050e9510d;
#[path="../registers/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../registers/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../registers/gddac1e2c6112_v1.rs"] pub mod gddac1e2c6112;
#[path="../registers/gddbg895e48a2_v1.rs"] pub mod gddbg895e48a2;
#[path="../registers/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../registers/gdenetdma717f16f5_v1.rs"] pub mod gdenetdma717f16f5;
#[path="../registers/gdenetmac93552dd1_v1.rs"] pub mod gdenetmac93552dd1;
#[path="../registers/gdenetmacfcth8ada9e21_v1.rs"] pub mod gdenetmacfcth8ada9e21;
#[path="../registers/gdenetmsc10390666_v1.rs"] pub mod gdenetmsc10390666;
#[path="../registers/gdenetptp5c8a2d48_v1.rs"] pub mod gdenetptp5c8a2d48;
#[path="../registers/gdexmc9f6a36f3_v1.rs"] pub mod gdexmc9f6a36f3;
#[path="../registers/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../registers/gdfmc2c447638_v1.rs"] pub mod gdfmc2c447638;
#[path="../registers/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../registers/gdgpioac3e5c224_v1.rs"] pub mod gdgpioac3e5c224;
#[path="../registers/gdi2c037cbcb65_v1.rs"] pub mod gdi2c037cbcb65;
#[path="../registers/gdpmu3bb0ce43_v1.rs"] pub mod gdpmu3bb0ce43;
#[path="../registers/gdrcu08b36be4_v1.rs"] pub mod gdrcu08b36be4;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdspi0946ced9d_v1.rs"] pub mod gdspi0946ced9d;
#[path="../registers/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../registers/gdtimer1b47aad48_v1.rs"] pub mod gdtimer1b47aad48;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../registers/gdtimer911ce6ab6_v1.rs"] pub mod gdtimer911ce6ab6;
#[path="../registers/gduart3c678fe30_v1.rs"] pub mod gduart3c678fe30;
#[path="../registers/gdusart082eeb6ea_v1.rs"] pub mod gdusart082eeb6ea;
#[path="../registers/gdusbfsdevicec5a1209d_v1.rs"] pub mod gdusbfsdevicec5a1209d;
#[path="../registers/gdusbfsglobal3505227a_v1.rs"] pub mod gdusbfsglobal3505227a;
#[path="../registers/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../registers/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
