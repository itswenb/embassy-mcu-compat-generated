
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc059fb2391",
                version: "v1",
                block: "ADC0",
                ir: &gdadc059fb2391::REGISTERS,
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
                kind: "gdadc059fb2391",
                version: "v1",
                block: "ADC0",
                ir: &gdadc059fb2391::REGISTERS,
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
                kind: "gdadc059fb2391",
                version: "v1",
                block: "ADC0",
                ir: &gdadc059fb2391::REGISTERS,
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
                kind: "gdafio0d83dbd7",
                version: "v1",
                block: "AFIO",
                ir: &gdafio0d83dbd7::REGISTERS,
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
                kind: "gdbkpb0abe2e5",
                version: "v1",
                block: "BKP",
                ir: &gdbkpb0abe2e5::REGISTERS,
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
                kind: "gdcan01d9ed235",
                version: "v1",
                block: "CAN0",
                ir: &gdcan01d9ed235::REGISTERS,
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
                kind: "gdcan01d9ed235",
                version: "v1",
                block: "CAN0",
                ir: &gdcan01d9ed235::REGISTERS,
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
        name: "CAU",
        address: 0x50060000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcau1d48f570",
                version: "v1",
                block: "CAU",
                ir: &gdcau1d48f570::REGISTERS,
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
                kind: "gdcrc5c562b59",
                version: "v1",
                block: "CRC",
                ir: &gdcrc5c562b59::REGISTERS,
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
                kind: "gddac9551a4ef",
                version: "v1",
                block: "DAC",
                ir: &gddac9551a4ef::REGISTERS,
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
                kind: "gddbgb21f1063",
                version: "v1",
                block: "DBG",
                ir: &gddbgb21f1063::REGISTERS,
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
        name: "DCI",
        address: 0x50050000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddcia70582ff",
                version: "v1",
                block: "DCI",
                ir: &gddcia70582ff::REGISTERS,
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
                kind: "gddma1517575c6",
                version: "v1",
                block: "DMA1",
                ir: &gddma1517575c6::REGISTERS,
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
                kind: "gdenetdmacba9250b",
                version: "v1",
                block: "ENET_DMA",
                ir: &gdenetdmacba9250b::REGISTERS,
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
                kind: "gdenetmac09c072f4",
                version: "v1",
                block: "ENET_MAC",
                ir: &gdenetmac09c072f4::REGISTERS,
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
                kind: "gdenetmsc9217fdbd",
                version: "v1",
                block: "ENET_MSC",
                ir: &gdenetmsc9217fdbd::REGISTERS,
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
                kind: "gdenetptpf491bb9d",
                version: "v1",
                block: "ENET_PTP",
                ir: &gdenetptpf491bb9d::REGISTERS,
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
                kind: "gdexmcb6dcdf27",
                version: "v1",
                block: "EXMC",
                ir: &gdexmcb6dcdf27::REGISTERS,
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
                kind: "gdfmcd52b6db1",
                version: "v1",
                block: "FMC",
                ir: &gdfmcd52b6db1::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
        name: "GPIOH",
        address: 0x40017400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
        name: "GPIOI",
        address: 0x40017800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioa979b0f67",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa979b0f67::REGISTERS,
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
        name: "HAU",
        address: 0x50060400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhau67f15641",
                version: "v1",
                block: "HAU",
                ir: &gdhau67f15641::REGISTERS,
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
                kind: "gdi2c0700b93ad",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0700b93ad::REGISTERS,
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
                kind: "gdi2c0700b93ad",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0700b93ad::REGISTERS,
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
                kind: "gdi2c0700b93ad",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0700b93ad::REGISTERS,
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
                kind: "gdpmu0a98243f",
                version: "v1",
                block: "PMU",
                ir: &gdpmu0a98243f::REGISTERS,
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
                kind: "gdrcu29040b39",
                version: "v1",
                block: "RCU",
                ir: &gdrcu29040b39::REGISTERS,
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
        name: "SDIO",
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsdio299e3279",
                version: "v1",
                block: "SDIO",
                ir: &gdsdio299e3279::REGISTERS,
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
                kind: "gdspi09ee016b2",
                version: "v1",
                block: "SPI0",
                ir: &gdspi09ee016b2::REGISTERS,
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
                kind: "gdspi09ee016b2",
                version: "v1",
                block: "SPI0",
                ir: &gdspi09ee016b2::REGISTERS,
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
                kind: "gdspi09ee016b2",
                version: "v1",
                block: "SPI0",
                ir: &gdspi09ee016b2::REGISTERS,
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
                kind: "gdtimer0f10fcbf6",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0f10fcbf6::REGISTERS,
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
                kind: "gdtimer1974d22f3",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1974d22f3::REGISTERS,
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
                kind: "gdtimer9c6ee1d55",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9c6ee1d55::REGISTERS,
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
                kind: "gdtimer865e4b21d",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer865e4b21d::REGISTERS,
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
                kind: "gdtimer9c6ee1d55",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9c6ee1d55::REGISTERS,
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
                kind: "gdtimer9c6ee1d55",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9c6ee1d55::REGISTERS,
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
                kind: "gdtimer1974d22f3",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1974d22f3::REGISTERS,
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
                kind: "gdtimer1974d22f3",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1974d22f3::REGISTERS,
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
                kind: "gdtimer1974d22f3",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1974d22f3::REGISTERS,
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
                kind: "gdtimer55eec4d84",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer55eec4d84::REGISTERS,
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
                kind: "gdtimer55eec4d84",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer55eec4d84::REGISTERS,
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
                kind: "gdtimer0f10fcbf6",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0f10fcbf6::REGISTERS,
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
                kind: "gdtimer865e4b21d",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer865e4b21d::REGISTERS,
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
                kind: "gdtimer9c6ee1d55",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9c6ee1d55::REGISTERS,
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
        name: "TLI",
        address: 0x40016800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli89ae65d5",
                version: "v1",
                block: "TLI",
                ir: &gdtli89ae65d5::REGISTERS,
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
        name: "TRNG",
        address: 0x50060800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrngbf61c352",
                version: "v1",
                block: "TRNG",
                ir: &gdtrngbf61c352::REGISTERS,
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
                kind: "gduart36dbe0a8a",
                version: "v1",
                block: "UART3",
                ir: &gduart36dbe0a8a::REGISTERS,
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
                kind: "gduart36dbe0a8a",
                version: "v1",
                block: "UART3",
                ir: &gduart36dbe0a8a::REGISTERS,
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
        name: "UART6",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "gduart36dbe0a8a",
                version: "v1",
                block: "UART3",
                ir: &gduart36dbe0a8a::REGISTERS,
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
        name: "UART7",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gduart36dbe0a8a",
                version: "v1",
                block: "UART3",
                ir: &gduart36dbe0a8a::REGISTERS,
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
                kind: "gdusart0677bab67",
                version: "v1",
                block: "USART0",
                ir: &gdusart0677bab67::REGISTERS,
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
                kind: "gdusart0677bab67",
                version: "v1",
                block: "USART0",
                ir: &gdusart0677bab67::REGISTERS,
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
                kind: "gdusart0677bab67",
                version: "v1",
                block: "USART0",
                ir: &gdusart0677bab67::REGISTERS,
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
                kind: "gdusart0677bab67",
                version: "v1",
                block: "USART0",
                ir: &gdusart0677bab67::REGISTERS,
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
                kind: "gdusbfsdevicea4903788",
                version: "v1",
                block: "USBFS_DEVICE",
                ir: &gdusbfsdevicea4903788::REGISTERS,
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
                kind: "gdusbfsglobal019fa48e",
                version: "v1",
                block: "USBFS_GLOBAL",
                ir: &gdusbfsglobal019fa48e::REGISTERS,
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
        name: "RCU",
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
        name: "EXTI_LINE5_9",
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
        name: "TIMER0_CHANNEL",
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
        name: "EXTI_LINE10_15",
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
        name: "TIMER7_CHANNEL",
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
        name: "SDIO",
        number: 49,
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
    Interrupt {
        name: "USART5",
        number: 71,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 73,
    },
    Interrupt {
        name: "DCI",
        number: 78,
    },
    Interrupt {
        name: "CAU",
        number: 79,
    },
    Interrupt {
        name: "HAU_TRNG",
        number: 80,
    },
    Interrupt {
        name: "UART6",
        number: 82,
    },
    Interrupt {
        name: "UART7",
        number: 83,
    },
    Interrupt {
        name: "TLI",
        number: 88,
    },
    Interrupt {
        name: "TLI_ER",
        number: 89,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc059fb2391_v1.rs"] pub mod gdadc059fb2391;
#[path="../registers/gdafio0d83dbd7_v1.rs"] pub mod gdafio0d83dbd7;
#[path="../registers/gdbkpb0abe2e5_v1.rs"] pub mod gdbkpb0abe2e5;
#[path="../registers/gdcan01d9ed235_v1.rs"] pub mod gdcan01d9ed235;
#[path="../registers/gdcau1d48f570_v1.rs"] pub mod gdcau1d48f570;
#[path="../registers/gdcrc5c562b59_v1.rs"] pub mod gdcrc5c562b59;
#[path="../registers/gddac9551a4ef_v1.rs"] pub mod gddac9551a4ef;
#[path="../registers/gddbgb21f1063_v1.rs"] pub mod gddbgb21f1063;
#[path="../registers/gddcia70582ff_v1.rs"] pub mod gddcia70582ff;
#[path="../registers/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../registers/gddma1517575c6_v1.rs"] pub mod gddma1517575c6;
#[path="../registers/gdenetdmacba9250b_v1.rs"] pub mod gdenetdmacba9250b;
#[path="../registers/gdenetmac09c072f4_v1.rs"] pub mod gdenetmac09c072f4;
#[path="../registers/gdenetmsc9217fdbd_v1.rs"] pub mod gdenetmsc9217fdbd;
#[path="../registers/gdenetptpf491bb9d_v1.rs"] pub mod gdenetptpf491bb9d;
#[path="../registers/gdexmcb6dcdf27_v1.rs"] pub mod gdexmcb6dcdf27;
#[path="../registers/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../registers/gdfmcd52b6db1_v1.rs"] pub mod gdfmcd52b6db1;
#[path="../registers/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../registers/gdgpioa979b0f67_v1.rs"] pub mod gdgpioa979b0f67;
#[path="../registers/gdhau67f15641_v1.rs"] pub mod gdhau67f15641;
#[path="../registers/gdi2c0700b93ad_v1.rs"] pub mod gdi2c0700b93ad;
#[path="../registers/gdpmu0a98243f_v1.rs"] pub mod gdpmu0a98243f;
#[path="../registers/gdrcu29040b39_v1.rs"] pub mod gdrcu29040b39;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdsdio299e3279_v1.rs"] pub mod gdsdio299e3279;
#[path="../registers/gdspi09ee016b2_v1.rs"] pub mod gdspi09ee016b2;
#[path="../registers/gdtimer0f10fcbf6_v1.rs"] pub mod gdtimer0f10fcbf6;
#[path="../registers/gdtimer1974d22f3_v1.rs"] pub mod gdtimer1974d22f3;
#[path="../registers/gdtimer55eec4d84_v1.rs"] pub mod gdtimer55eec4d84;
#[path="../registers/gdtimer865e4b21d_v1.rs"] pub mod gdtimer865e4b21d;
#[path="../registers/gdtimer9c6ee1d55_v1.rs"] pub mod gdtimer9c6ee1d55;
#[path="../registers/gdtli89ae65d5_v1.rs"] pub mod gdtli89ae65d5;
#[path="../registers/gdtrngbf61c352_v1.rs"] pub mod gdtrngbf61c352;
#[path="../registers/gduart36dbe0a8a_v1.rs"] pub mod gduart36dbe0a8a;
#[path="../registers/gdusart0677bab67_v1.rs"] pub mod gdusart0677bab67;
#[path="../registers/gdusbfsdevicea4903788_v1.rs"] pub mod gdusbfsdevicea4903788;
#[path="../registers/gdusbfsglobal019fa48e_v1.rs"] pub mod gdusbfsglobal019fa48e;
#[path="../registers/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../registers/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
