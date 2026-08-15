
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc088c1e886",
                version: "v1",
                block: "ADC0",
                ir: &gdadc088c1e886::REGISTERS,
            },
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
                kind: "gdadc25692136a",
                version: "v1",
                block: "ADC2",
                ir: &gdadc25692136a::REGISTERS,
            },
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
                kind: "gdcan8d97a339",
                version: "v1",
                block: "CAN",
                ir: &gdcan8d97a339::REGISTERS,
            },
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
                kind: "gdcfmud735e759",
                version: "v1",
                block: "CFMU",
                ir: &gdcfmud735e759::REGISTERS,
            },
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
                kind: "gdcmp6cf4a780",
                version: "v1",
                block: "CMP",
                ir: &gdcmp6cf4a780::REGISTERS,
            },
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
                kind: "gdcptimer0f537712b",
                version: "v1",
                block: "CPTIMER0",
                ir: &gdcptimer0f537712b::REGISTERS,
            },
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
                kind: "gdcptimer0f537712b",
                version: "v1",
                block: "CPTIMER0",
                ir: &gdcptimer0f537712b::REGISTERS,
            },
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
                kind: "gdcptimerwc742ef6c",
                version: "v1",
                block: "CPTIMERW",
                ir: &gdcptimerwc742ef6c::REGISTERS,
            },
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
                kind: "gdcrc553be872",
                version: "v1",
                block: "CRC",
                ir: &gdcrc553be872::REGISTERS,
            },
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
                kind: "gddacb75238e9",
                version: "v1",
                block: "DAC",
                ir: &gddacb75238e9::REGISTERS,
            },
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
                kind: "gddbg217d467b",
                version: "v1",
                block: "DBG",
                ir: &gddbg217d467b::REGISTERS,
            },
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
                kind: "gddma0586e39d1",
                version: "v1",
                block: "DMA0",
                ir: &gddma0586e39d1::REGISTERS,
            },
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
                kind: "gddma1b108675d",
                version: "v1",
                block: "DMA1",
                ir: &gddma1b108675d::REGISTERS,
            },
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
                kind: "gddmamux77665c6a",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux77665c6a::REGISTERS,
            },
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
                kind: "gdevicf10e9e33",
                version: "v1",
                block: "EVIC",
                ir: &gdevicf10e9e33::REGISTERS,
            },
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
                kind: "gdextibab4ad71",
                version: "v1",
                block: "EXTI",
                ir: &gdextibab4ad71::REGISTERS,
            },
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
                kind: "gdfmcd2f13365",
                version: "v1",
                block: "FMC",
                ir: &gdfmcd2f13365::REGISTERS,
            },
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
                kind: "gdfwdgte0a44d28",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgte0a44d28::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpioaf9033ab6",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioaf9033ab6::REGISTERS,
            },
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
                kind: "gdgpionfd68a396",
                version: "v1",
                block: "GPION",
                ir: &gdgpionfd68a396::REGISTERS,
            },
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
                kind: "gdgptimer002f92dbb",
                version: "v1",
                block: "GPTIMER0",
                ir: &gdgptimer002f92dbb::REGISTERS,
            },
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
                kind: "gdgptimer002f92dbb",
                version: "v1",
                block: "GPTIMER0",
                ir: &gdgptimer002f92dbb::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GTOC",
        address: 0x48004800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgtoc1d40c5d1",
                version: "v1",
                block: "GTOC",
                ir: &gdgtoc1d40c5d1::REGISTERS,
            },
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
                kind: "gdi2ca6cc3474",
                version: "v1",
                block: "I2C",
                ir: &gdi2ca6cc3474::REGISTERS,
            },
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
                kind: "gdpmu4ecf2e55",
                version: "v1",
                block: "PMU",
                ir: &gdpmu4ecf2e55::REGISTERS,
            },
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
                kind: "gdpocca2dbf68",
                version: "v1",
                block: "POC",
                ir: &gdpocca2dbf68::REGISTERS,
            },
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
                kind: "gdrcu714c8771",
                version: "v1",
                block: "RCU",
                ir: &gdrcu714c8771::REGISTERS,
            },
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
                kind: "gdspiea15830d",
                version: "v1",
                block: "SPI",
                ir: &gdspiea15830d::REGISTERS,
            },
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
                kind: "gdsvpwmc75a0f03",
                version: "v1",
                block: "SVPWM",
                ir: &gdsvpwmc75a0f03::REGISTERS,
            },
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
                kind: "gdsyscfg8db51c8b",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg8db51c8b::REGISTERS,
            },
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
                kind: "gdtimer000cb8605",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer000cb8605::REGISTERS,
            },
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
                kind: "gdtimer1ed17b6a8",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1ed17b6a8::REGISTERS,
            },
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
                kind: "gdtimer1ed17b6a8",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1ed17b6a8::REGISTERS,
            },
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
                kind: "gdtimer000cb8605",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer000cb8605::REGISTERS,
            },
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
                kind: "gdtmuca711897",
                version: "v1",
                block: "TMU",
                ir: &gdtmuca711897::REGISTERS,
            },
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
                kind: "gduart0d4cac493",
                version: "v1",
                block: "UART0",
                ir: &gduart0d4cac493::REGISTERS,
            },
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
                kind: "gduart0d4cac493",
                version: "v1",
                block: "UART0",
                ir: &gduart0d4cac493::REGISTERS,
            },
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
                kind: "gduart0d4cac493",
                version: "v1",
                block: "UART0",
                ir: &gduart0d4cac493::REGISTERS,
            },
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
                kind: "gduart0d4cac493",
                version: "v1",
                block: "UART0",
                ir: &gduart0d4cac493::REGISTERS,
            },
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
                kind: "gdwwdgt7328a167",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt7328a167::REGISTERS,
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
        name: "TIMER0_TRG_CM",
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
        name: "EXTI_LINE15_10",
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
        name: "TIMER7_TR_CM",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CC",
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
        name: "DMA_MUX",
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
        name: "I2C_WAKEUP_FROM_EXTI_LINE23",
        number: 66,
    },
    Interrupt {
        name: "FWDGT_FROM_EXTI_LINE22",
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
        name: "EVIC",
        number: 78,
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
        name: "CMP0_FROM_EXTI",
        number: 85,
    },
    Interrupt {
        name: "CMP1_FROM_EXTI",
        number: 86,
    },
    Interrupt {
        name: "CMP2_FROM_EXTI",
        number: 87,
    },
    Interrupt {
        name: "CMP3_FROM_EXTI",
        number: 88,
    },
    Interrupt {
        name: "SRAM_ECC",
        number: 92,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc088c1e886_v1.rs"] pub mod gdadc088c1e886;
#[path="../registers/gdadc25692136a_v1.rs"] pub mod gdadc25692136a;
#[path="../registers/gdcan8d97a339_v1.rs"] pub mod gdcan8d97a339;
#[path="../registers/gdcfmud735e759_v1.rs"] pub mod gdcfmud735e759;
#[path="../registers/gdcmp6cf4a780_v1.rs"] pub mod gdcmp6cf4a780;
#[path="../registers/gdcptimer0f537712b_v1.rs"] pub mod gdcptimer0f537712b;
#[path="../registers/gdcptimerwc742ef6c_v1.rs"] pub mod gdcptimerwc742ef6c;
#[path="../registers/gdcrc553be872_v1.rs"] pub mod gdcrc553be872;
#[path="../registers/gddacb75238e9_v1.rs"] pub mod gddacb75238e9;
#[path="../registers/gddbg217d467b_v1.rs"] pub mod gddbg217d467b;
#[path="../registers/gddma0586e39d1_v1.rs"] pub mod gddma0586e39d1;
#[path="../registers/gddma1b108675d_v1.rs"] pub mod gddma1b108675d;
#[path="../registers/gddmamux77665c6a_v1.rs"] pub mod gddmamux77665c6a;
#[path="../registers/gdevicf10e9e33_v1.rs"] pub mod gdevicf10e9e33;
#[path="../registers/gdextibab4ad71_v1.rs"] pub mod gdextibab4ad71;
#[path="../registers/gdfmcd2f13365_v1.rs"] pub mod gdfmcd2f13365;
#[path="../registers/gdfwdgte0a44d28_v1.rs"] pub mod gdfwdgte0a44d28;
#[path="../registers/gdgpioaf9033ab6_v1.rs"] pub mod gdgpioaf9033ab6;
#[path="../registers/gdgpionfd68a396_v1.rs"] pub mod gdgpionfd68a396;
#[path="../registers/gdgptimer002f92dbb_v1.rs"] pub mod gdgptimer002f92dbb;
#[path="../registers/gdgtoc1d40c5d1_v1.rs"] pub mod gdgtoc1d40c5d1;
#[path="../registers/gdi2ca6cc3474_v1.rs"] pub mod gdi2ca6cc3474;
#[path="../registers/gdpmu4ecf2e55_v1.rs"] pub mod gdpmu4ecf2e55;
#[path="../registers/gdpocca2dbf68_v1.rs"] pub mod gdpocca2dbf68;
#[path="../registers/gdrcu714c8771_v1.rs"] pub mod gdrcu714c8771;
#[path="../registers/gdspiea15830d_v1.rs"] pub mod gdspiea15830d;
#[path="../registers/gdsvpwmc75a0f03_v1.rs"] pub mod gdsvpwmc75a0f03;
#[path="../registers/gdsyscfg8db51c8b_v1.rs"] pub mod gdsyscfg8db51c8b;
#[path="../registers/gdtimer000cb8605_v1.rs"] pub mod gdtimer000cb8605;
#[path="../registers/gdtimer1ed17b6a8_v1.rs"] pub mod gdtimer1ed17b6a8;
#[path="../registers/gdtmuca711897_v1.rs"] pub mod gdtmuca711897;
#[path="../registers/gduart0d4cac493_v1.rs"] pub mod gduart0d4cac493;
#[path="../registers/gdwwdgt7328a167_v1.rs"] pub mod gdwwdgt7328a167;
