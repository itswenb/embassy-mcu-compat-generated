
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcebf84092",
                version: "v1",
                block: "ADC",
                ir: &gdadcebf84092::REGISTERS,
            },
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
                kind: "gdadcebf84092",
                version: "v1",
                block: "ADC",
                ir: &gdadcebf84092::REGISTERS,
            },
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
                kind: "gdafio3260312d",
                version: "v1",
                block: "AFIO",
                ir: &gdafio3260312d::REGISTERS,
            },
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
                kind: "gdbkpddaa24e5",
                version: "v1",
                block: "BKP",
                ir: &gdbkpddaa24e5::REGISTERS,
            },
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
                kind: "gdcanc1d18d75",
                version: "v1",
                block: "CAN",
                ir: &gdcanc1d18d75::REGISTERS,
            },
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
                kind: "gdcanc1d18d75",
                version: "v1",
                block: "CAN",
                ir: &gdcanc1d18d75::REGISTERS,
            },
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
                kind: "gdcanc1d18d75",
                version: "v1",
                block: "CAN",
                ir: &gdcanc1d18d75::REGISTERS,
            },
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
                kind: "gdcmpa4fd7736",
                version: "v1",
                block: "CMP",
                ir: &gdcmpa4fd7736::REGISTERS,
            },
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
                kind: "gdcrc491c92d0",
                version: "v1",
                block: "CRC",
                ir: &gdcrc491c92d0::REGISTERS,
            },
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
                kind: "gdctc6d9ce461",
                version: "v1",
                block: "CTC",
                ir: &gdctc6d9ce461::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC0",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac2c324d90",
                version: "v1",
                block: "DAC",
                ir: &gddac2c324d90::REGISTERS,
            },
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
                kind: "gddbg0c729a76",
                version: "v1",
                block: "DBG",
                ir: &gddbg0c729a76::REGISTERS,
            },
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
                kind: "gddmae208530b",
                version: "v1",
                block: "DMA",
                ir: &gddmae208530b::REGISTERS,
            },
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
                kind: "gddmae208530b",
                version: "v1",
                block: "DMA",
                ir: &gddmae208530b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET",
        address: 0x40028000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenet4408cf6f",
                version: "v1",
                block: "ENET",
                ir: &gdenet4408cf6f::REGISTERS,
            },
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
                kind: "gdexmc6eb28b9f",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc6eb28b9f::REGISTERS,
            },
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
                kind: "gdexti9fc5df87",
                version: "v1",
                block: "EXTI",
                ir: &gdexti9fc5df87::REGISTERS,
            },
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
                kind: "gdfmc09ec7384",
                version: "v1",
                block: "FMC",
                ir: &gdfmc09ec7384::REGISTERS,
            },
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
                kind: "gdfwdgt77bb718d",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgt77bb718d::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdgpio114d8126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio114d8126::REGISTERS,
            },
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
                kind: "gdi2c59ded4aa",
                version: "v1",
                block: "I2C",
                ir: &gdi2c59ded4aa::REGISTERS,
            },
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
                kind: "gdi2c59ded4aa",
                version: "v1",
                block: "I2C",
                ir: &gdi2c59ded4aa::REGISTERS,
            },
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
                kind: "gdi2c2566026ac",
                version: "v1",
                block: "I2C2",
                ir: &gdi2c2566026ac::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S1_ADD",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2s32f828a0",
                version: "v1",
                block: "I2S",
                ir: &gdi2s32f828a0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S2_ADD",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2s32f828a0",
                version: "v1",
                block: "I2S",
                ir: &gdi2s32f828a0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OB",
        address: 0x1ffff800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdob721aff1c",
                version: "v1",
                block: "OB",
                ir: &gdob721aff1c::REGISTERS,
            },
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
                kind: "gdpmu5b735bb1",
                version: "v1",
                block: "PMU",
                ir: &gdpmu5b735bb1::REGISTERS,
            },
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
                kind: "gdrcu162113ac",
                version: "v1",
                block: "RCU",
                ir: &gdrcu162113ac::REGISTERS,
            },
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
                kind: "gdrtc250e9b91",
                version: "v1",
                block: "RTC",
                ir: &gdrtc250e9b91::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SHRTIMER0",
        address: 0x40017400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdshrtimerea23ae38",
                version: "v1",
                block: "SHRTIMER",
                ir: &gdshrtimerea23ae38::REGISTERS,
            },
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
                kind: "gdspi20dc9722",
                version: "v1",
                block: "SPI",
                ir: &gdspi20dc9722::REGISTERS,
            },
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
                kind: "gdspi20dc9722",
                version: "v1",
                block: "SPI",
                ir: &gdspi20dc9722::REGISTERS,
            },
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
                kind: "gdspi20dc9722",
                version: "v1",
                block: "SPI",
                ir: &gdspi20dc9722::REGISTERS,
            },
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
                kind: "gdsqpi47688f21",
                version: "v1",
                block: "SQPI",
                ir: &gdsqpi47688f21::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtimer894282e9",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer894282e9::REGISTERS,
            },
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
                kind: "gdtmu810f0d96",
                version: "v1",
                block: "TMU",
                ir: &gdtmu810f0d96::REGISTERS,
            },
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
                kind: "gdusartf581e00c",
                version: "v1",
                block: "USART",
                ir: &gdusartf581e00c::REGISTERS,
            },
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
                kind: "gdusartf581e00c",
                version: "v1",
                block: "USART",
                ir: &gdusartf581e00c::REGISTERS,
            },
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
                kind: "gdusartf581e00c",
                version: "v1",
                block: "USART",
                ir: &gdusartf581e00c::REGISTERS,
            },
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
                kind: "gdusartf581e00c",
                version: "v1",
                block: "USART",
                ir: &gdusartf581e00c::REGISTERS,
            },
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
                kind: "gdusartf581e00c",
                version: "v1",
                block: "USART",
                ir: &gdusartf581e00c::REGISTERS,
            },
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
                kind: "gdusart58135de6a",
                version: "v1",
                block: "USART5",
                ir: &gdusart58135de6a::REGISTERS,
            },
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
                kind: "gdwwdgt30374593",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt30374593::REGISTERS,
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
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI1",
        number: 7,
    },
    Interrupt {
        name: "EXTI2",
        number: 8,
    },
    Interrupt {
        name: "EXTI3",
        number: 9,
    },
    Interrupt {
        name: "EXTI4",
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
        name: "EXTI5_9",
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
        name: "SPI1_I2S1ADD",
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
        name: "EXTI10_15",
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
        name: "TIMER7_CHANNEL",
        number: 46,
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
        name: "SPI2_I2S2ADD",
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
        name: "USBHS_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "USBHS_EP1_IN",
        number: 75,
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
            #[path="../registers/gdadcebf84092_v1.rs"] pub mod gdadcebf84092;
#[path="../registers/gdafio3260312d_v1.rs"] pub mod gdafio3260312d;
#[path="../registers/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../registers/gdcanc1d18d75_v1.rs"] pub mod gdcanc1d18d75;
#[path="../registers/gdcmpa4fd7736_v1.rs"] pub mod gdcmpa4fd7736;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../registers/gddac2c324d90_v1.rs"] pub mod gddac2c324d90;
#[path="../registers/gddbg0c729a76_v1.rs"] pub mod gddbg0c729a76;
#[path="../registers/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../registers/gdenet4408cf6f_v1.rs"] pub mod gdenet4408cf6f;
#[path="../registers/gdexmc6eb28b9f_v1.rs"] pub mod gdexmc6eb28b9f;
#[path="../registers/gdexti9fc5df87_v1.rs"] pub mod gdexti9fc5df87;
#[path="../registers/gdfmc09ec7384_v1.rs"] pub mod gdfmc09ec7384;
#[path="../registers/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../registers/gdgpio114d8126_v1.rs"] pub mod gdgpio114d8126;
#[path="../registers/gdi2c2566026ac_v1.rs"] pub mod gdi2c2566026ac;
#[path="../registers/gdi2c59ded4aa_v1.rs"] pub mod gdi2c59ded4aa;
#[path="../registers/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../registers/gdob721aff1c_v1.rs"] pub mod gdob721aff1c;
#[path="../registers/gdpmu5b735bb1_v1.rs"] pub mod gdpmu5b735bb1;
#[path="../registers/gdrcu162113ac_v1.rs"] pub mod gdrcu162113ac;
#[path="../registers/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../registers/gdshrtimerea23ae38_v1.rs"] pub mod gdshrtimerea23ae38;
#[path="../registers/gdspi20dc9722_v1.rs"] pub mod gdspi20dc9722;
#[path="../registers/gdsqpi47688f21_v1.rs"] pub mod gdsqpi47688f21;
#[path="../registers/gdtimer894282e9_v1.rs"] pub mod gdtimer894282e9;
#[path="../registers/gdtmu810f0d96_v1.rs"] pub mod gdtmu810f0d96;
#[path="../registers/gdusart58135de6a_v1.rs"] pub mod gdusart58135de6a;
#[path="../registers/gdusartf581e00c_v1.rs"] pub mod gdusartf581e00c;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
