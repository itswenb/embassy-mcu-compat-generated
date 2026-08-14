
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcaec32a72",
                version: "v1",
                block: "ADC",
                ir: &gdadcaec32a72::REGISTERS,
            },
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
                kind: "gdadcaec32a72",
                version: "v1",
                block: "ADC",
                ir: &gdadcaec32a72::REGISTERS,
            },
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
                kind: "gdbkp33ccb7c2",
                version: "v1",
                block: "BKP",
                ir: &gdbkp33ccb7c2::REGISTERS,
            },
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
                kind: "gdcanf6d1de49",
                version: "v1",
                block: "CAN",
                ir: &gdcanf6d1de49::REGISTERS,
            },
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
                kind: "gdcanf6d1de49",
                version: "v1",
                block: "CAN",
                ir: &gdcanf6d1de49::REGISTERS,
            },
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
                kind: "gdcmpd1466266",
                version: "v1",
                block: "CMP",
                ir: &gdcmpd1466266::REGISTERS,
            },
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
        name: "DAC0",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac32e8d501",
                version: "v1",
                block: "DAC",
                ir: &gddac32e8d501::REGISTERS,
            },
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
                kind: "gddbg3f036c14",
                version: "v1",
                block: "DBG",
                ir: &gddbg3f036c14::REGISTERS,
            },
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
        name: "DMAMUX",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamux4c40dca9",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux4c40dca9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMAMUX_RG_CHXCFG_BASE",
        address: 0x40020900,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamuxrgchxcfgbase1b4097c0",
                version: "v1",
                block: "DMAMUX_RG_CHXCFG_BASE",
                ir: &gddmamuxrgchxcfgbase1b4097c0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMAMUX_RM_CHXCFG_BASE",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamuxrmchxcfgbase7797637e",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbase7797637e::REGISTERS,
            },
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
                kind: "gdexti29f744de",
                version: "v1",
                block: "EXTI",
                ir: &gdexti29f744de::REGISTERS,
            },
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
                kind: "gdfmcee4535d3",
                version: "v1",
                block: "FMC",
                ir: &gdfmcee4535d3::REGISTERS,
            },
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
                kind: "gdfwdgtc7bc9588",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtc7bc9588::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
        address: 0x48001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdi2cecd631a5",
                version: "v1",
                block: "I2C",
                ir: &gdi2cecd631a5::REGISTERS,
            },
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
                kind: "gdi2cecd631a5",
                version: "v1",
                block: "I2C",
                ir: &gdi2cecd631a5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "MFCOM",
        address: 0x40038400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdmfcomff1dc6ee",
                version: "v1",
                block: "MFCOM",
                ir: &gdmfcomff1dc6ee::REGISTERS,
            },
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
                kind: "gdob28e8bc27",
                version: "v1",
                block: "OB",
                ir: &gdob28e8bc27::REGISTERS,
            },
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
                kind: "gdpmubc000a45",
                version: "v1",
                block: "PMU",
                ir: &gdpmubc000a45::REGISTERS,
            },
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
                kind: "gdrcu1b817abe",
                version: "v1",
                block: "RCU",
                ir: &gdrcu1b817abe::REGISTERS,
            },
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
        name: "SPI0",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi5d70f94a",
                version: "v1",
                block: "SPI",
                ir: &gdspi5d70f94a::REGISTERS,
            },
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
                kind: "gdspi5d70f94a",
                version: "v1",
                block: "SPI",
                ir: &gdspi5d70f94a::REGISTERS,
            },
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
                kind: "gdsyscfg39dede78",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg39dede78::REGISTERS,
            },
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
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
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
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER19",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER20",
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
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
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
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
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
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
                kind: "gdtimer5e574d75",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e574d75::REGISTERS,
            },
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
                kind: "gdtrigseld16b35fa",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigseld16b35fa::REGISTERS,
            },
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
                kind: "gdusart6d0b41a3",
                version: "v1",
                block: "USART",
                ir: &gdusart6d0b41a3::REGISTERS,
            },
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
                kind: "gdusart6d0b41a3",
                version: "v1",
                block: "USART",
                ir: &gdusart6d0b41a3::REGISTERS,
            },
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
                kind: "gdusart6d0b41a3",
                version: "v1",
                block: "USART",
                ir: &gdusart6d0b41a3::REGISTERS,
            },
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
        name: "CAN0_MESSAGE",
        number: 19,
    },
    Interrupt {
        name: "CAN0_BUSOFF",
        number: 20,
    },
    Interrupt {
        name: "CAN0_ERROR",
        number: 21,
    },
    Interrupt {
        name: "CAN0_FASTERROR",
        number: 22,
    },
    Interrupt {
        name: "CAN0_TEC",
        number: 23,
    },
    Interrupt {
        name: "CAN0_REC",
        number: 24,
    },
    Interrupt {
        name: "CAN0_WKUP",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_BRK_UP_TRG_CMT",
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
        name: "TIMER19_BRK_UP_TRG_CMT",
        number: 29,
    },
    Interrupt {
        name: "TIMER19_CHANNEL",
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
        name: "EXTI10_15",
        number: 40,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 41,
    },
    Interrupt {
        name: "TAMPER",
        number: 42,
    },
    Interrupt {
        name: "TIMER20_BRK_UP_TRG_CMT",
        number: 43,
    },
    Interrupt {
        name: "TIMER20_CHANNEL",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_BRK_UP_TRG_CMT",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CHANNEL",
        number: 46,
    },
    Interrupt {
        name: "DMAMUX",
        number: 47,
    },
    Interrupt {
        name: "SRAMC_ECCSE",
        number: 48,
    },
    Interrupt {
        name: "CMP",
        number: 49,
    },
    Interrupt {
        name: "OVD",
        number: 51,
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
        name: "CAN1_WKUP",
        number: 62,
    },
    Interrupt {
        name: "CAN1_MESSAGE",
        number: 63,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 64,
    },
    Interrupt {
        name: "CAN1_ERROR",
        number: 65,
    },
    Interrupt {
        name: "CAN1_FASTERROR",
        number: 66,
    },
    Interrupt {
        name: "CAN1_TEC",
        number: 67,
    },
    Interrupt {
        name: "CAN1_REC",
        number: 68,
    },
    Interrupt {
        name: "FPU",
        number: 69,
    },
    Interrupt {
        name: "MFCOM",
        number: 70,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadcaec32a72_v1.rs"] pub mod gdadcaec32a72;
#[path="../registers/gdbkp33ccb7c2_v1.rs"] pub mod gdbkp33ccb7c2;
#[path="../registers/gdcanf6d1de49_v1.rs"] pub mod gdcanf6d1de49;
#[path="../registers/gdcmpd1466266_v1.rs"] pub mod gdcmpd1466266;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gddac32e8d501_v1.rs"] pub mod gddac32e8d501;
#[path="../registers/gddbg3f036c14_v1.rs"] pub mod gddbg3f036c14;
#[path="../registers/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../registers/gddmamux4c40dca9_v1.rs"] pub mod gddmamux4c40dca9;
#[path="../registers/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../registers/gddmamuxrmchxcfgbase7797637e_v1.rs"] pub mod gddmamuxrmchxcfgbase7797637e;
#[path="../registers/gdexti29f744de_v1.rs"] pub mod gdexti29f744de;
#[path="../registers/gdfmcee4535d3_v1.rs"] pub mod gdfmcee4535d3;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../registers/gdi2cecd631a5_v1.rs"] pub mod gdi2cecd631a5;
#[path="../registers/gdmfcomff1dc6ee_v1.rs"] pub mod gdmfcomff1dc6ee;
#[path="../registers/gdob28e8bc27_v1.rs"] pub mod gdob28e8bc27;
#[path="../registers/gdpmubc000a45_v1.rs"] pub mod gdpmubc000a45;
#[path="../registers/gdrcu1b817abe_v1.rs"] pub mod gdrcu1b817abe;
#[path="../registers/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../registers/gdspi5d70f94a_v1.rs"] pub mod gdspi5d70f94a;
#[path="../registers/gdsyscfg39dede78_v1.rs"] pub mod gdsyscfg39dede78;
#[path="../registers/gdtimer5e574d75_v1.rs"] pub mod gdtimer5e574d75;
#[path="../registers/gdtrigseld16b35fa_v1.rs"] pub mod gdtrigseld16b35fa;
#[path="../registers/gdusart6d0b41a3_v1.rs"] pub mod gdusart6d0b41a3;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
