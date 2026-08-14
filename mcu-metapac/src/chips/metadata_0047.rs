
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc07b93ae04",
                version: "v1",
                block: "ADC0",
                ir: &gdadc07b93ae04::REGISTERS,
            },
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
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc2e242f52",
                version: "v1",
                block: "ADC",
                ir: &gdadc2e242f52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN",
        address: 0x4001a000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan60cfeb95",
                version: "v1",
                block: "CAN",
                ir: &gdcan60cfeb95::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CFMU",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcfmu50e649be",
                version: "v1",
                block: "CFMU",
                ir: &gdcfmu50e649be::REGISTERS,
            },
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
                kind: "gdcmp73832aeb",
                version: "v1",
                block: "CMP",
                ir: &gdcmp73832aeb::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPTIMER0",
        address: 0x40000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcptimeree6db8d9",
                version: "v1",
                block: "CPTIMER",
                ir: &gdcptimeree6db8d9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPTIMER1",
        address: 0x40000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcptimeree6db8d9",
                version: "v1",
                block: "CPTIMER",
                ir: &gdcptimeree6db8d9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPTIMERW",
        address: 0x4000e000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcptimerw3395205e",
                version: "v1",
                block: "CPTIMERW",
                ir: &gdcptimerw3395205e::REGISTERS,
            },
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
                kind: "gdcrcba782523",
                version: "v1",
                block: "CRC",
                ir: &gdcrcba782523::REGISTERS,
            },
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
                kind: "gddac4a56ea36",
                version: "v1",
                block: "DAC",
                ir: &gddac4a56ea36::REGISTERS,
            },
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
                kind: "gddbgbc5f12bf",
                version: "v1",
                block: "DBG",
                ir: &gddbgbc5f12bf::REGISTERS,
            },
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
                kind: "gddmaa3a4fde0",
                version: "v1",
                block: "DMA",
                ir: &gddmaa3a4fde0::REGISTERS,
            },
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
                kind: "gddmaa3a4fde0",
                version: "v1",
                block: "DMA",
                ir: &gddmaa3a4fde0::REGISTERS,
            },
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
        name: "EVIC",
        address: 0x40018400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdevic5169c252",
                version: "v1",
                block: "EVIC",
                ir: &gdevic5169c252::REGISTERS,
            },
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
                kind: "gdexti13fbcd68",
                version: "v1",
                block: "EXTI",
                ir: &gdexti13fbcd68::REGISTERS,
            },
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
                kind: "gdfmc5edddd6f",
                version: "v1",
                block: "FMC",
                ir: &gdfmc5edddd6f::REGISTERS,
            },
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
                kind: "gdfwdgt9caf0879",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgt9caf0879::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
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
        address: 0x48001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9495ea81",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9495ea81::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPION",
        address: 0x48004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpionff502c14",
                version: "v1",
                block: "GPION",
                ir: &gdgpionff502c14::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPTIMER0",
        address: 0x40016000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgptimer95f98022",
                version: "v1",
                block: "GPTIMER",
                ir: &gdgptimer95f98022::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPTIMER1",
        address: 0x40016100,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgptimer95f98022",
                version: "v1",
                block: "GPTIMER",
                ir: &gdgptimer95f98022::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTOC0",
        address: 0x48004800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgtoc78b2467f",
                version: "v1",
                block: "GTOC",
                ir: &gdgtoc78b2467f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTOC1",
        address: 0x48004810,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgtoc78b2467f",
                version: "v1",
                block: "GTOC",
                ir: &gdgtoc78b2467f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTOC2",
        address: 0x48004820,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgtoc78b2467f",
                version: "v1",
                block: "GTOC",
                ir: &gdgtoc78b2467f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTOC3",
        address: 0x48004830,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgtoc78b2467f",
                version: "v1",
                block: "GTOC",
                ir: &gdgtoc78b2467f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C",
        address: 0x40005400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c8ede78f7",
                version: "v1",
                block: "I2C",
                ir: &gdi2c8ede78f7::REGISTERS,
            },
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
                kind: "gdobecba84d3",
                version: "v1",
                block: "OB",
                ir: &gdobecba84d3::REGISTERS,
            },
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
                kind: "gdpmu3925f692",
                version: "v1",
                block: "PMU",
                ir: &gdpmu3925f692::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "POC",
        address: 0x48004400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpocc3ca8581",
                version: "v1",
                block: "POC",
                ir: &gdpocc3ca8581::REGISTERS,
            },
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
                kind: "gdrcu5a71bf8b",
                version: "v1",
                block: "RCU",
                ir: &gdrcu5a71bf8b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI",
        address: 0x40013000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi59cf958b",
                version: "v1",
                block: "SPI",
                ir: &gdspi59cf958b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SVPWM",
        address: 0x48024000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsvpwm8128db8b",
                version: "v1",
                block: "SVPWM",
                ir: &gdsvpwm8128db8b::REGISTERS,
            },
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
                kind: "gdsyscfge224963c",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfge224963c::REGISTERS,
            },
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
                kind: "gdtimer9c3b27bc",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer9c3b27bc::REGISTERS,
            },
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
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer9c3b27bc",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer9c3b27bc::REGISTERS,
            },
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
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer9c3b27bc",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer9c3b27bc::REGISTERS,
            },
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
                kind: "gdtimer9c3b27bc",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer9c3b27bc::REGISTERS,
            },
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
                kind: "gdtmu6e5ec85c",
                version: "v1",
                block: "TMU",
                ir: &gdtmu6e5ec85c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART0",
        address: 0x40004c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gduartaa5a7938",
                version: "v1",
                block: "UART",
                ir: &gduartaa5a7938::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART1",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "gduartaa5a7938",
                version: "v1",
                block: "UART",
                ir: &gduartaa5a7938::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART2",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "gduartaa5a7938",
                version: "v1",
                block: "UART",
                ir: &gduartaa5a7938::REGISTERS,
            },
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
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gduartaa5a7938",
                version: "v1",
                block: "UART",
                ir: &gduartaa5a7938::REGISTERS,
            },
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
                kind: "gdwwdgt6968988b",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt6968988b::REGISTERS,
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
        name: "LVD1",
        number: 1,
    },
    Interrupt {
        name: "LVD2",
        number: 2,
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
        name: "ADC0",
        number: 18,
    },
    Interrupt {
        name: "CAN_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN_EWMC",
        number: 22,
    },
    Interrupt {
        name: "EXTI5_9",
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
        name: "GPTIMER0",
        number: 30,
    },
    Interrupt {
        name: "I2C_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C_ER",
        number: 32,
    },
    Interrupt {
        name: "SPI",
        number: 35,
    },
    Interrupt {
        name: "UART0",
        number: 37,
    },
    Interrupt {
        name: "UART1",
        number: 38,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 40,
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
        name: "TIMER7_CHANNEL",
        number: 46,
    },
    Interrupt {
        name: "TMU",
        number: 47,
    },
    Interrupt {
        name: "GPTIMER1",
        number: 50,
    },
    Interrupt {
        name: "UART2",
        number: 52,
    },
    Interrupt {
        name: "UART3",
        number: 53,
    },
    Interrupt {
        name: "CPTIMER0",
        number: 54,
    },
    Interrupt {
        name: "CPTIMER1",
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
        number: 61,
    },
    Interrupt {
        name: "DMAMUX_OVERRUN",
        number: 62,
    },
    Interrupt {
        name: "CPTIMERW",
        number: 63,
    },
    Interrupt {
        name: "CFMU",
        number: 65,
    },
    Interrupt {
        name: "I2C_WKUP",
        number: 66,
    },
    Interrupt {
        name: "FWDGT",
        number: 67,
    },
    Interrupt {
        name: "CMP0",
        number: 70,
    },
    Interrupt {
        name: "CMP1",
        number: 71,
    },
    Interrupt {
        name: "CMP2",
        number: 72,
    },
    Interrupt {
        name: "CMP3",
        number: 73,
    },
    Interrupt {
        name: "ADC2",
        number: 75,
    },
    Interrupt {
        name: "POC",
        number: 77,
    },
    Interrupt {
        name: "GTOC0",
        number: 79,
    },
    Interrupt {
        name: "GTOC1",
        number: 80,
    },
    Interrupt {
        name: "GTOC2",
        number: 81,
    },
    Interrupt {
        name: "GTOC3",
        number: 82,
    },
    Interrupt {
        name: "CMP0_EXTI",
        number: 85,
    },
    Interrupt {
        name: "CMP1_EXTI",
        number: 86,
    },
    Interrupt {
        name: "CMP2_EXTI",
        number: 87,
    },
    Interrupt {
        name: "CMP3_EXTI",
        number: 88,
    },
    Interrupt {
        name: "SRAMC_ECC",
        number: 92,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc07b93ae04_v1.rs"] pub mod gdadc07b93ae04;
#[path="../registers/gdadc2e242f52_v1.rs"] pub mod gdadc2e242f52;
#[path="../registers/gdcan60cfeb95_v1.rs"] pub mod gdcan60cfeb95;
#[path="../registers/gdcfmu50e649be_v1.rs"] pub mod gdcfmu50e649be;
#[path="../registers/gdcmp73832aeb_v1.rs"] pub mod gdcmp73832aeb;
#[path="../registers/gdcptimeree6db8d9_v1.rs"] pub mod gdcptimeree6db8d9;
#[path="../registers/gdcptimerw3395205e_v1.rs"] pub mod gdcptimerw3395205e;
#[path="../registers/gdcrcba782523_v1.rs"] pub mod gdcrcba782523;
#[path="../registers/gddac4a56ea36_v1.rs"] pub mod gddac4a56ea36;
#[path="../registers/gddbgbc5f12bf_v1.rs"] pub mod gddbgbc5f12bf;
#[path="../registers/gddmaa3a4fde0_v1.rs"] pub mod gddmaa3a4fde0;
#[path="../registers/gddmamux4c40dca9_v1.rs"] pub mod gddmamux4c40dca9;
#[path="../registers/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../registers/gddmamuxrmchxcfgbase7797637e_v1.rs"] pub mod gddmamuxrmchxcfgbase7797637e;
#[path="../registers/gdevic5169c252_v1.rs"] pub mod gdevic5169c252;
#[path="../registers/gdexti13fbcd68_v1.rs"] pub mod gdexti13fbcd68;
#[path="../registers/gdfmc5edddd6f_v1.rs"] pub mod gdfmc5edddd6f;
#[path="../registers/gdfwdgt9caf0879_v1.rs"] pub mod gdfwdgt9caf0879;
#[path="../registers/gdgpio9495ea81_v1.rs"] pub mod gdgpio9495ea81;
#[path="../registers/gdgpionff502c14_v1.rs"] pub mod gdgpionff502c14;
#[path="../registers/gdgptimer95f98022_v1.rs"] pub mod gdgptimer95f98022;
#[path="../registers/gdgtoc78b2467f_v1.rs"] pub mod gdgtoc78b2467f;
#[path="../registers/gdi2c8ede78f7_v1.rs"] pub mod gdi2c8ede78f7;
#[path="../registers/gdobecba84d3_v1.rs"] pub mod gdobecba84d3;
#[path="../registers/gdpmu3925f692_v1.rs"] pub mod gdpmu3925f692;
#[path="../registers/gdpocc3ca8581_v1.rs"] pub mod gdpocc3ca8581;
#[path="../registers/gdrcu5a71bf8b_v1.rs"] pub mod gdrcu5a71bf8b;
#[path="../registers/gdspi59cf958b_v1.rs"] pub mod gdspi59cf958b;
#[path="../registers/gdsvpwm8128db8b_v1.rs"] pub mod gdsvpwm8128db8b;
#[path="../registers/gdsyscfge224963c_v1.rs"] pub mod gdsyscfge224963c;
#[path="../registers/gdtimer9c3b27bc_v1.rs"] pub mod gdtimer9c3b27bc;
#[path="../registers/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../registers/gduartaa5a7938_v1.rs"] pub mod gduartaa5a7938;
#[path="../registers/gdwwdgt6968988b_v1.rs"] pub mod gdwwdgt6968988b;
