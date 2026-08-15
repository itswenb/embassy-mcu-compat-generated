
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc09259a092",
                version: "v1",
                block: "ADC0",
                ir: &gdadc09259a092::REGISTERS,
            },
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
                kind: "gdadc1c4c42f4e",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1c4c42f4e::REGISTERS,
            },
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
                kind: "gdbkp726e313a",
                version: "v1",
                block: "BKP",
                ir: &gdbkp726e313a::REGISTERS,
            },
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
                kind: "gdcan0e1070584",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0e1070584::REGISTERS,
            },
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
                kind: "gdcan0e1070584",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0e1070584::REGISTERS,
            },
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
                kind: "gdcmp8f451a3d",
                version: "v1",
                block: "CMP",
                ir: &gdcmp8f451a3d::REGISTERS,
            },
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
                kind: "gdcrc8a4036fe",
                version: "v1",
                block: "CRC",
                ir: &gdcrc8a4036fe::REGISTERS,
            },
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
                kind: "gddac78befc29",
                version: "v1",
                block: "DAC",
                ir: &gddac78befc29::REGISTERS,
            },
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
                kind: "gddbg2b068fbb",
                version: "v1",
                block: "DBG",
                ir: &gddbg2b068fbb::REGISTERS,
            },
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
                kind: "gddma0bac0399b",
                version: "v1",
                block: "DMA0",
                ir: &gddma0bac0399b::REGISTERS,
            },
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
        name: "DMAMUX",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamux7bf7f916",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux7bf7f916::REGISTERS,
            },
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
                kind: "gdextibdc5df6b",
                version: "v1",
                block: "EXTI",
                ir: &gdextibdc5df6b::REGISTERS,
            },
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
                kind: "gdfmcd32845c2",
                version: "v1",
                block: "FMC",
                ir: &gdfmcd32845c2::REGISTERS,
            },
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
                kind: "gdfwdgt9ccc125f",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgt9ccc125f::REGISTERS,
            },
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
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
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
                kind: "gdgpiob3a01cf30",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob3a01cf30::REGISTERS,
            },
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
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
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
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
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
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
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
                kind: "gdgpioa9804d271",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa9804d271::REGISTERS,
            },
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
                kind: "gdi2c000bb4e12",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c000bb4e12::REGISTERS,
            },
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
                kind: "gdi2c19c81d997",
                version: "v1",
                block: "I2C1",
                ir: &gdi2c19c81d997::REGISTERS,
            },
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
                kind: "gdmfcomdfea6a59",
                version: "v1",
                block: "MFCOM",
                ir: &gdmfcomdfea6a59::REGISTERS,
            },
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
                kind: "gdpmu2d47d9c6",
                version: "v1",
                block: "PMU",
                ir: &gdpmu2d47d9c6::REGISTERS,
            },
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
                kind: "gdrcue53fc96d",
                version: "v1",
                block: "RCU",
                ir: &gdrcue53fc96d::REGISTERS,
            },
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
                kind: "gdspi0c6850d65",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0c6850d65::REGISTERS,
            },
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
                kind: "gdspi14e571efb",
                version: "v1",
                block: "SPI1",
                ir: &gdspi14e571efb::REGISTERS,
            },
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
                kind: "gdsyscfg9b2fb855",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg9b2fb855::REGISTERS,
            },
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
                kind: "gdtimer0a62f20ab",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a62f20ab::REGISTERS,
            },
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
                kind: "gdtimer1cb2db824",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1cb2db824::REGISTERS,
            },
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
                kind: "gdtimer0a62f20ab",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a62f20ab::REGISTERS,
            },
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
                kind: "gdtimer0a62f20ab",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a62f20ab::REGISTERS,
            },
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
                kind: "gdtimer0a62f20ab",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a62f20ab::REGISTERS,
            },
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
                kind: "gdtrigselbd6404a1",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigselbd6404a1::REGISTERS,
            },
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
                kind: "gdusart042a368e4",
                version: "v1",
                block: "USART0",
                ir: &gdusart042a368e4::REGISTERS,
            },
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
                kind: "gdusart042a368e4",
                version: "v1",
                block: "USART0",
                ir: &gdusart042a368e4::REGISTERS,
            },
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
                kind: "gdusart042a368e4",
                version: "v1",
                block: "USART0",
                ir: &gdusart042a368e4::REGISTERS,
            },
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
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "CAN0_MF",
        number: 19,
    },
    Interrupt {
        name: "CAN0_BUSOFF",
        number: 20,
    },
    Interrupt {
        name: "CAN0_ER",
        number: 21,
    },
    Interrupt {
        name: "CAN0_ERFT",
        number: 22,
    },
    Interrupt {
        name: "CAN0_TW",
        number: 23,
    },
    Interrupt {
        name: "CAN0_RW",
        number: 24,
    },
    Interrupt {
        name: "CAN0_WAKE",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_BRK_UP_TR_CM",
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
        name: "TIMER19_BRK_UP_TR_CM",
        number: 29,
    },
    Interrupt {
        name: "TIMER19_CAP",
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
        name: "TIMER20_BRK_UP_TR_CM",
        number: 43,
    },
    Interrupt {
        name: "TIMER20_CAP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_BRK_UP_TR_CM",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CAP",
        number: 46,
    },
    Interrupt {
        name: "DMA_MUX",
        number: 47,
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
        name: "CAN1_WAKE",
        number: 62,
    },
    Interrupt {
        name: "CAN1_MF",
        number: 63,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 64,
    },
    Interrupt {
        name: "CAN1_ER",
        number: 65,
    },
    Interrupt {
        name: "CAN1_ERFT",
        number: 66,
    },
    Interrupt {
        name: "CAN1_TW",
        number: 67,
    },
    Interrupt {
        name: "CAN1_RW",
        number: 68,
    },
    Interrupt {
        name: "MFCOM",
        number: 70,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc09259a092_v1.rs"] pub mod gdadc09259a092;
#[path="../registers/gdadc1c4c42f4e_v1.rs"] pub mod gdadc1c4c42f4e;
#[path="../registers/gdbkp726e313a_v1.rs"] pub mod gdbkp726e313a;
#[path="../registers/gdcan0e1070584_v1.rs"] pub mod gdcan0e1070584;
#[path="../registers/gdcmp8f451a3d_v1.rs"] pub mod gdcmp8f451a3d;
#[path="../registers/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../registers/gddac78befc29_v1.rs"] pub mod gddac78befc29;
#[path="../registers/gddbg2b068fbb_v1.rs"] pub mod gddbg2b068fbb;
#[path="../registers/gddma0bac0399b_v1.rs"] pub mod gddma0bac0399b;
#[path="../registers/gddma13e943824_v1.rs"] pub mod gddma13e943824;
#[path="../registers/gddmamux7bf7f916_v1.rs"] pub mod gddmamux7bf7f916;
#[path="../registers/gdextibdc5df6b_v1.rs"] pub mod gdextibdc5df6b;
#[path="../registers/gdfmcd32845c2_v1.rs"] pub mod gdfmcd32845c2;
#[path="../registers/gdfwdgt9ccc125f_v1.rs"] pub mod gdfwdgt9ccc125f;
#[path="../registers/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../registers/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../registers/gdi2c000bb4e12_v1.rs"] pub mod gdi2c000bb4e12;
#[path="../registers/gdi2c19c81d997_v1.rs"] pub mod gdi2c19c81d997;
#[path="../registers/gdmfcomdfea6a59_v1.rs"] pub mod gdmfcomdfea6a59;
#[path="../registers/gdpmu2d47d9c6_v1.rs"] pub mod gdpmu2d47d9c6;
#[path="../registers/gdrcue53fc96d_v1.rs"] pub mod gdrcue53fc96d;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdspi0c6850d65_v1.rs"] pub mod gdspi0c6850d65;
#[path="../registers/gdspi14e571efb_v1.rs"] pub mod gdspi14e571efb;
#[path="../registers/gdsyscfg9b2fb855_v1.rs"] pub mod gdsyscfg9b2fb855;
#[path="../registers/gdtimer0a62f20ab_v1.rs"] pub mod gdtimer0a62f20ab;
#[path="../registers/gdtimer1cb2db824_v1.rs"] pub mod gdtimer1cb2db824;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtimer642c906a4_v1.rs"] pub mod gdtimer642c906a4;
#[path="../registers/gdtrigselbd6404a1_v1.rs"] pub mod gdtrigselbd6404a1;
#[path="../registers/gdusart042a368e4_v1.rs"] pub mod gdusart042a368e4;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
