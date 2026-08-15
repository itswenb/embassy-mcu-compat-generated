
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc06d279556",
                version: "v1",
                block: "ADC0",
                ir: &gdadc06d279556::REGISTERS,
            },
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
                kind: "gdadc1425a4aff",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1425a4aff::REGISTERS,
            },
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
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc2efea3dc8",
                version: "v1",
                block: "ADC2",
                ir: &gdadc2efea3dc8::REGISTERS,
            },
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
        address: 0x4001a000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan0ab6ea0b5",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0ab6ea0b5::REGISTERS,
            },
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
        address: 0x4001b000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan0ab6ea0b5",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0ab6ea0b5::REGISTERS,
            },
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
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan0ab6ea0b5",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0ab6ea0b5::REGISTERS,
            },
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
        address: 0x58003800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmpb718ae34",
                version: "v1",
                block: "CMP",
                ir: &gdcmpb718ae34::REGISTERS,
            },
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
        address: 0x58024c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcrc1ff07d05",
                version: "v1",
                block: "CRC",
                ir: &gdcrc1ff07d05::REGISTERS,
            },
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
        address: 0x40008400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdctcdb80f1ce",
                version: "v1",
                block: "CTC",
                ir: &gdctcdb80f1ce::REGISTERS,
            },
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
                kind: "gddac555b6194",
                version: "v1",
                block: "DAC",
                ir: &gddac555b6194::REGISTERS,
            },
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
        address: 0xe00e1000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg21d11b7d",
                version: "v1",
                block: "DBG",
                ir: &gddbg21d11b7d::REGISTERS,
            },
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
                kind: "gddma09f21797a",
                version: "v1",
                block: "DMA0",
                ir: &gddma09f21797a::REGISTERS,
            },
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
                kind: "gddma09f21797a",
                version: "v1",
                block: "DMA0",
                ir: &gddma09f21797a::REGISTERS,
            },
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
                kind: "gddmamuxeaace10d",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamuxeaace10d::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDOUT",
        address: 0x40018800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedoutfebca4f4",
                version: "v1",
                block: "EDOUT",
                ir: &gdedoutfebca4f4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EFUSE",
        address: 0x40022800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdefuseee44bd8c",
                version: "v1",
                block: "EFUSE",
                ir: &gdefuseee44bd8c::REGISTERS,
            },
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
        address: 0x52004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc293e9145",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc293e9145::REGISTERS,
            },
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
        address: 0x58000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdextiacb27c36",
                version: "v1",
                block: "EXTI",
                ir: &gdextiacb27c36::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FAC",
        address: 0x48024800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfac96d60f19",
                version: "v1",
                block: "FAC",
                ir: &gdfac96d60f19::REGISTERS,
            },
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
        address: 0x52002000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfmca3be0d1e",
                version: "v1",
                block: "FMC",
                ir: &gdfmca3be0d1e::REGISTERS,
            },
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
        address: 0x58004800,
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
        address: 0x58020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioa9b446375",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9b446375::REGISTERS,
            },
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
        address: 0x58020400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiob3479439a",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob3479439a::REGISTERS,
            },
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
        address: 0x58020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
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
        address: 0x58020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
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
        address: 0x58021000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
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
        address: 0x58021400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
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
        address: 0x58021800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
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
        address: 0x58021c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioc18dfc69f",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc18dfc69f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HPDF",
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdffd9de252",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffd9de252::REGISTERS,
            },
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
        name: "I2C2",
        address: 0x4000c000,
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
        name: "I2C3",
        address: 0x40005c00,
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
        name: "LPDTS",
        address: 0x58006800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlpdtsa3b40577",
                version: "v1",
                block: "LPDTS",
                ir: &gdlpdtsa3b40577::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "MDMA",
        address: 0x52000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdmdmab9a60aaf",
                version: "v1",
                block: "MDMA",
                ir: &gdmdmab9a60aaf::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPI0",
        address: 0x52005000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdospi0439e0312",
                version: "v1",
                block: "OSPI0",
                ir: &gdospi0439e0312::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPI1",
        address: 0x5200a000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdospi0439e0312",
                version: "v1",
                block: "OSPI0",
                ir: &gdospi0439e0312::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPIM",
        address: 0x5200b400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdospimeed3ce76",
                version: "v1",
                block: "OSPIM",
                ir: &gdospimeed3ce76::REGISTERS,
            },
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
        address: 0x58005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpmu04713c6c",
                version: "v1",
                block: "PMU",
                ir: &gdpmu04713c6c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMECCMU0",
        address: 0x52009000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrameccmu0d260ef4c",
                version: "v1",
                block: "RAMECCMU0",
                ir: &gdrameccmu0d260ef4c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMECCMU1",
        address: 0x48023000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrameccmu1ba654536",
                version: "v1",
                block: "RAMECCMU1",
                ir: &gdrameccmu1ba654536::REGISTERS,
            },
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
        address: 0x58024400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrcufed5390f",
                version: "v1",
                block: "RCU",
                ir: &gdrcufed5390f::REGISTERS,
            },
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
        address: 0x58004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrtcc8139290",
                version: "v1",
                block: "RTC",
                ir: &gdrtcc8139290::REGISTERS,
            },
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
                kind: "gdspi0a7377dd5",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a7377dd5::REGISTERS,
            },
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
                kind: "gdspi1356222e3",
                version: "v1",
                block: "SPI1",
                ir: &gdspi1356222e3::REGISTERS,
            },
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
                kind: "gdspi255cb8c1f",
                version: "v1",
                block: "SPI2",
                ir: &gdspi255cb8c1f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x40013400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi3e9b78823",
                version: "v1",
                block: "SPI3",
                ir: &gdspi3e9b78823::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI4",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi4af049e38",
                version: "v1",
                block: "SPI4",
                ir: &gdspi4af049e38::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi5c82f56e6",
                version: "v1",
                block: "SPI5",
                ir: &gdspi5c82f56e6::REGISTERS,
            },
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
        address: 0x58000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfgb64b8ea8",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgb64b8ea8::REGISTERS,
            },
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
        address: 0x40010000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer03afad14d",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer03afad14d::REGISTERS,
            },
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
                kind: "gdtimer1da3bc56a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1da3bc56a::REGISTERS,
            },
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
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
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
                kind: "gdtimer155d5134ba",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer155d5134ba::REGISTERS,
            },
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
                kind: "gdtimer155d5134ba",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer155d5134ba::REGISTERS,
            },
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
                kind: "gdtimer27201f8c9",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer27201f8c9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER22",
        address: 0x4000e000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1da3bc56a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1da3bc56a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER23",
        address: 0x4000e400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1da3bc56a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1da3bc56a::REGISTERS,
            },
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
                kind: "gdtimer27201f8c9",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer27201f8c9::REGISTERS,
            },
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
                kind: "gdtimer1da3bc56a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1da3bc56a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER40",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER41",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER42",
        address: 0x4001d800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER43",
        address: 0x4001dc00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER44",
        address: 0x4001f000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer1457881844",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer1457881844::REGISTERS,
            },
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
                kind: "gdtimer5330a987e",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer5330a987e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER50",
        address: 0x4000f000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5071732508",
                version: "v1",
                block: "TIMER50",
                ir: &gdtimer5071732508::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER51",
        address: 0x4000f400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5071732508",
                version: "v1",
                block: "TIMER50",
                ir: &gdtimer5071732508::REGISTERS,
            },
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
                kind: "gdtimer5330a987e",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer5330a987e::REGISTERS,
            },
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
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer03afad14d",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer03afad14d::REGISTERS,
            },
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
        address: 0x48024400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtmucbc214df",
                version: "v1",
                block: "TMU",
                ir: &gdtmucbc214df::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TRIGSEL",
        address: 0x40018400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrigsel9d4a38fd",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigsel9d4a38fd::REGISTERS,
            },
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
        address: 0x48021800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrng6bc6a907",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng6bc6a907::REGISTERS,
            },
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
                kind: "gduart330e38640",
                version: "v1",
                block: "UART3",
                ir: &gduart330e38640::REGISTERS,
            },
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
                kind: "gduart330e38640",
                version: "v1",
                block: "UART3",
                ir: &gduart330e38640::REGISTERS,
            },
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
                kind: "gduart330e38640",
                version: "v1",
                block: "UART3",
                ir: &gduart330e38640::REGISTERS,
            },
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
                kind: "gduart330e38640",
                version: "v1",
                block: "UART3",
                ir: &gduart330e38640::REGISTERS,
            },
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
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart0626fb765",
                version: "v1",
                block: "USART0",
                ir: &gdusart0626fb765::REGISTERS,
            },
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
                kind: "gdusart0626fb765",
                version: "v1",
                block: "USART0",
                ir: &gdusart0626fb765::REGISTERS,
            },
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
                kind: "gdusart0626fb765",
                version: "v1",
                block: "USART0",
                ir: &gdusart0626fb765::REGISTERS,
            },
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
        address: 0x40011400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart0626fb765",
                version: "v1",
                block: "USART0",
                ir: &gdusart0626fb765::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_DEVICE",
        address: 0x40040800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs0deviced0449d15",
                version: "v1",
                block: "USBHS0_DEVICE",
                ir: &gdusbhs0deviced0449d15::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_GLOBAL",
        address: 0x40040000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs0globalbee3a389",
                version: "v1",
                block: "USBHS0_GLOBAL",
                ir: &gdusbhs0globalbee3a389::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_HOST",
        address: 0x40040400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs0host663109ac",
                version: "v1",
                block: "USBHS0_HOST",
                ir: &gdusbhs0host663109ac::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_PWRCLK",
        address: 0x40040e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs0pwrclk0f97dd8b",
                version: "v1",
                block: "USBHS0_PWRCLK",
                ir: &gdusbhs0pwrclk0f97dd8b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_DEVICE",
        address: 0x40080800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs1device9d406887",
                version: "v1",
                block: "USBHS1_DEVICE",
                ir: &gdusbhs1device9d406887::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_GLOBAL",
        address: 0x40080000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs1globalb3d6824e",
                version: "v1",
                block: "USBHS1_GLOBAL",
                ir: &gdusbhs1globalb3d6824e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_HOST",
        address: 0x40080400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs1host14113081",
                version: "v1",
                block: "USBHS1_HOST",
                ir: &gdusbhs1host14113081::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_PWRCLK",
        address: 0x40080e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbhs1pwrclk29fc276e",
                version: "v1",
                block: "USBHS1_PWRCLK",
                ir: &gdusbhs1pwrclk29fc276e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "VREF",
        address: 0x58003c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdvref193fa1c3",
                version: "v1",
                block: "VREF",
                ir: &gdvref193fa1c3::REGISTERS,
            },
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
        address: 0x50003000,
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
        name: "AVD_PVD",
        number: 1,
    },
    Interrupt {
        name: "RTC_TAMPER_TIMESTAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WAKE",
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
        name: "TIMER0_TR_CM",
        number: 26,
    },
    Interrupt {
        name: "TIMER0_CAP",
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
        name: "TIMER7_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TR_CM",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CAP",
        number: 46,
    },
    Interrupt {
        name: "DMA0_CHANNEL7",
        number: 47,
    },
    Interrupt {
        name: "EXMC_GLOBAL",
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
        name: "TIMER5_DAC",
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
        name: "DMA1_CHANNEL5",
        number: 68,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 69,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 70,
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
        name: "USBHS0_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "USBHS0_EP1_IN",
        number: 75,
    },
    Interrupt {
        name: "USBHS0_WAKEUP",
        number: 76,
    },
    Interrupt {
        name: "USBHS0_GLOBAL",
        number: 77,
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
        name: "SPI3",
        number: 84,
    },
    Interrupt {
        name: "SPI4",
        number: 85,
    },
    Interrupt {
        name: "SPI5",
        number: 86,
    },
    Interrupt {
        name: "OSPI0",
        number: 92,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 95,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 96,
    },
    Interrupt {
        name: "DMAMUX0",
        number: 102,
    },
    Interrupt {
        name: "HPDF_GLOBAL0",
        number: 110,
    },
    Interrupt {
        name: "HPDF_GLOBAL1",
        number: 111,
    },
    Interrupt {
        name: "HPDF_GLOBAL2",
        number: 112,
    },
    Interrupt {
        name: "HPDF_GLOBAL3",
        number: 113,
    },
    Interrupt {
        name: "TIMER14",
        number: 116,
    },
    Interrupt {
        name: "TIMER15",
        number: 117,
    },
    Interrupt {
        name: "TIMER16",
        number: 118,
    },
    Interrupt {
        name: "MDMA_GLOBAL",
        number: 122,
    },
    Interrupt {
        name: "ADC2",
        number: 127,
    },
    Interrupt {
        name: "CPM_GLOBAL",
        number: 137,
    },
    Interrupt {
        name: "CTC",
        number: 144,
    },
    Interrupt {
        name: "ECC",
        number: 145,
    },
    Interrupt {
        name: "OSPI1",
        number: 150,
    },
    Interrupt {
        name: "FAC_GLOBAL",
        number: 153,
    },
    Interrupt {
        name: "TMU_GLOBAL",
        number: 154,
    },
    Interrupt {
        name: "TIMER22",
        number: 161,
    },
    Interrupt {
        name: "TIMER23",
        number: 162,
    },
    Interrupt {
        name: "TIMER40",
        number: 165,
    },
    Interrupt {
        name: "TIMER41",
        number: 166,
    },
    Interrupt {
        name: "TIMER42",
        number: 167,
    },
    Interrupt {
        name: "TIMER43",
        number: 168,
    },
    Interrupt {
        name: "TIMER44",
        number: 169,
    },
    Interrupt {
        name: "TIMER50",
        number: 170,
    },
    Interrupt {
        name: "TIMER51",
        number: 171,
    },
    Interrupt {
        name: "USBHS1_EP1_OUT",
        number: 172,
    },
    Interrupt {
        name: "USBHS1_EP1_IN",
        number: 173,
    },
    Interrupt {
        name: "USBHS1_WAKEUP",
        number: 174,
    },
    Interrupt {
        name: "USBHS1_GLOBAL",
        number: 175,
    },
    Interrupt {
        name: "CAN0_WK",
        number: 179,
    },
    Interrupt {
        name: "CAN0_BUFF",
        number: 180,
    },
    Interrupt {
        name: "CAN0_BUSOFF",
        number: 181,
    },
    Interrupt {
        name: "CAN0_ERROR",
        number: 182,
    },
    Interrupt {
        name: "CAN0_ERROR_FTX",
        number: 183,
    },
    Interrupt {
        name: "CAN0_WARNING_TX",
        number: 184,
    },
    Interrupt {
        name: "CAN0_WARNING_RX",
        number: 185,
    },
    Interrupt {
        name: "CAN1_WK",
        number: 186,
    },
    Interrupt {
        name: "CAN1_BUFF",
        number: 187,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 188,
    },
    Interrupt {
        name: "CAN1_ERROR",
        number: 189,
    },
    Interrupt {
        name: "CAN1_ERROR_FTX",
        number: 190,
    },
    Interrupt {
        name: "CAN1_WARNING_TX",
        number: 191,
    },
    Interrupt {
        name: "CAN1_WARNING_RX",
        number: 192,
    },
    Interrupt {
        name: "CAN2_WK",
        number: 193,
    },
    Interrupt {
        name: "CAN2_BUFF",
        number: 194,
    },
    Interrupt {
        name: "CAN2_BUSOFF",
        number: 195,
    },
    Interrupt {
        name: "CAN2_ERROR",
        number: 196,
    },
    Interrupt {
        name: "CAN2_ERROR_FTX",
        number: 197,
    },
    Interrupt {
        name: "CAN2_WARNING_TX",
        number: 198,
    },
    Interrupt {
        name: "CAN2_WARNING_RX",
        number: 199,
    },
    Interrupt {
        name: "EFUSE",
        number: 200,
    },
    Interrupt {
        name: "I2C0_WAKE",
        number: 201,
    },
    Interrupt {
        name: "I2C1_WAKE",
        number: 202,
    },
    Interrupt {
        name: "I2C2_WAKE",
        number: 203,
    },
    Interrupt {
        name: "I2C3_WAKE",
        number: 204,
    },
    Interrupt {
        name: "LPDTS",
        number: 205,
    },
    Interrupt {
        name: "LPDTS_WAKE",
        number: 206,
    },
    Interrupt {
        name: "TIMER0_DEC",
        number: 207,
    },
    Interrupt {
        name: "TIMER7_DEC",
        number: 208,
    },
    Interrupt {
        name: "TIMER1_DEC",
        number: 209,
    },
    Interrupt {
        name: "TIMER2_DEC",
        number: 210,
    },
    Interrupt {
        name: "TIMER3_DEC",
        number: 211,
    },
    Interrupt {
        name: "TIMER4_DEC",
        number: 212,
    },
    Interrupt {
        name: "TIMER22_DEC",
        number: 213,
    },
    Interrupt {
        name: "TIMER23_DEC",
        number: 214,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc06d279556_v1.rs"] pub mod gdadc06d279556;
#[path="../registers/gdadc1425a4aff_v1.rs"] pub mod gdadc1425a4aff;
#[path="../registers/gdadc2efea3dc8_v1.rs"] pub mod gdadc2efea3dc8;
#[path="../registers/gdcan0ab6ea0b5_v1.rs"] pub mod gdcan0ab6ea0b5;
#[path="../registers/gdcmpb718ae34_v1.rs"] pub mod gdcmpb718ae34;
#[path="../registers/gdcrc1ff07d05_v1.rs"] pub mod gdcrc1ff07d05;
#[path="../registers/gdctcdb80f1ce_v1.rs"] pub mod gdctcdb80f1ce;
#[path="../registers/gddac555b6194_v1.rs"] pub mod gddac555b6194;
#[path="../registers/gddbg21d11b7d_v1.rs"] pub mod gddbg21d11b7d;
#[path="../registers/gddma09f21797a_v1.rs"] pub mod gddma09f21797a;
#[path="../registers/gddmamuxeaace10d_v1.rs"] pub mod gddmamuxeaace10d;
#[path="../registers/gdedoutfebca4f4_v1.rs"] pub mod gdedoutfebca4f4;
#[path="../registers/gdefuseee44bd8c_v1.rs"] pub mod gdefuseee44bd8c;
#[path="../registers/gdexmc293e9145_v1.rs"] pub mod gdexmc293e9145;
#[path="../registers/gdextiacb27c36_v1.rs"] pub mod gdextiacb27c36;
#[path="../registers/gdfac96d60f19_v1.rs"] pub mod gdfac96d60f19;
#[path="../registers/gdfmca3be0d1e_v1.rs"] pub mod gdfmca3be0d1e;
#[path="../registers/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../registers/gdgpioa9b446375_v1.rs"] pub mod gdgpioa9b446375;
#[path="../registers/gdgpiob3479439a_v1.rs"] pub mod gdgpiob3479439a;
#[path="../registers/gdgpioc18dfc69f_v1.rs"] pub mod gdgpioc18dfc69f;
#[path="../registers/gdhpdffd9de252_v1.rs"] pub mod gdhpdffd9de252;
#[path="../registers/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../registers/gdlpdtsa3b40577_v1.rs"] pub mod gdlpdtsa3b40577;
#[path="../registers/gdmdmab9a60aaf_v1.rs"] pub mod gdmdmab9a60aaf;
#[path="../registers/gdospi0439e0312_v1.rs"] pub mod gdospi0439e0312;
#[path="../registers/gdospimeed3ce76_v1.rs"] pub mod gdospimeed3ce76;
#[path="../registers/gdpmu04713c6c_v1.rs"] pub mod gdpmu04713c6c;
#[path="../registers/gdrameccmu0d260ef4c_v1.rs"] pub mod gdrameccmu0d260ef4c;
#[path="../registers/gdrameccmu1ba654536_v1.rs"] pub mod gdrameccmu1ba654536;
#[path="../registers/gdrcufed5390f_v1.rs"] pub mod gdrcufed5390f;
#[path="../registers/gdrtcc8139290_v1.rs"] pub mod gdrtcc8139290;
#[path="../registers/gdspi0a7377dd5_v1.rs"] pub mod gdspi0a7377dd5;
#[path="../registers/gdspi1356222e3_v1.rs"] pub mod gdspi1356222e3;
#[path="../registers/gdspi255cb8c1f_v1.rs"] pub mod gdspi255cb8c1f;
#[path="../registers/gdspi3e9b78823_v1.rs"] pub mod gdspi3e9b78823;
#[path="../registers/gdspi4af049e38_v1.rs"] pub mod gdspi4af049e38;
#[path="../registers/gdspi5c82f56e6_v1.rs"] pub mod gdspi5c82f56e6;
#[path="../registers/gdsyscfgb64b8ea8_v1.rs"] pub mod gdsyscfgb64b8ea8;
#[path="../registers/gdtimer03afad14d_v1.rs"] pub mod gdtimer03afad14d;
#[path="../registers/gdtimer1457881844_v1.rs"] pub mod gdtimer1457881844;
#[path="../registers/gdtimer155d5134ba_v1.rs"] pub mod gdtimer155d5134ba;
#[path="../registers/gdtimer1da3bc56a_v1.rs"] pub mod gdtimer1da3bc56a;
#[path="../registers/gdtimer27201f8c9_v1.rs"] pub mod gdtimer27201f8c9;
#[path="../registers/gdtimer5071732508_v1.rs"] pub mod gdtimer5071732508;
#[path="../registers/gdtimer5330a987e_v1.rs"] pub mod gdtimer5330a987e;
#[path="../registers/gdtmucbc214df_v1.rs"] pub mod gdtmucbc214df;
#[path="../registers/gdtrigsel9d4a38fd_v1.rs"] pub mod gdtrigsel9d4a38fd;
#[path="../registers/gdtrng6bc6a907_v1.rs"] pub mod gdtrng6bc6a907;
#[path="../registers/gduart330e38640_v1.rs"] pub mod gduart330e38640;
#[path="../registers/gdusart0626fb765_v1.rs"] pub mod gdusart0626fb765;
#[path="../registers/gdusbhs0deviced0449d15_v1.rs"] pub mod gdusbhs0deviced0449d15;
#[path="../registers/gdusbhs0globalbee3a389_v1.rs"] pub mod gdusbhs0globalbee3a389;
#[path="../registers/gdusbhs0host663109ac_v1.rs"] pub mod gdusbhs0host663109ac;
#[path="../registers/gdusbhs0pwrclk0f97dd8b_v1.rs"] pub mod gdusbhs0pwrclk0f97dd8b;
#[path="../registers/gdusbhs1device9d406887_v1.rs"] pub mod gdusbhs1device9d406887;
#[path="../registers/gdusbhs1globalb3d6824e_v1.rs"] pub mod gdusbhs1globalb3d6824e;
#[path="../registers/gdusbhs1host14113081_v1.rs"] pub mod gdusbhs1host14113081;
#[path="../registers/gdusbhs1pwrclk29fc276e_v1.rs"] pub mod gdusbhs1pwrclk29fc276e;
#[path="../registers/gdvref193fa1c3_v1.rs"] pub mod gdvref193fa1c3;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
