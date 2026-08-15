
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadce30ea086",
                version: "v1",
                block: "ADC",
                ir: &gdadce30ea086::REGISTERS,
            },
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
                kind: "gdadce30ea086",
                version: "v1",
                block: "ADC",
                ir: &gdadce30ea086::REGISTERS,
            },
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
                kind: "gdafioa27eefcd",
                version: "v1",
                block: "AFIO",
                ir: &gdafioa27eefcd::REGISTERS,
            },
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
                kind: "gdbkp7944b1bc",
                version: "v1",
                block: "BKP",
                ir: &gdbkp7944b1bc::REGISTERS,
            },
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
                kind: "gdcan486a8ac4",
                version: "v1",
                block: "CAN",
                ir: &gdcan486a8ac4::REGISTERS,
            },
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
                kind: "gdcan486a8ac4",
                version: "v1",
                block: "CAN",
                ir: &gdcan486a8ac4::REGISTERS,
            },
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
                kind: "gdcrc3d3f2740",
                version: "v1",
                block: "CRC",
                ir: &gdcrc3d3f2740::REGISTERS,
            },
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
                kind: "gddac55126433",
                version: "v1",
                block: "DAC",
                ir: &gddac55126433::REGISTERS,
            },
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
                kind: "gddbg8cc9fb0b",
                version: "v1",
                block: "DBG",
                ir: &gddbg8cc9fb0b::REGISTERS,
            },
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
        name: "EXMC",
        address: 0xa0000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc9f914e53",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc9f914e53::REGISTERS,
            },
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
                kind: "gdextif95225bb",
                version: "v1",
                block: "EXTI",
                ir: &gdextif95225bb::REGISTERS,
            },
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
                kind: "gdfmcdc407917",
                version: "v1",
                block: "FMC",
                ir: &gdfmcdc407917::REGISTERS,
            },
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
                kind: "gdgpioc14eca7d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioc14eca7d::REGISTERS,
            },
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
                kind: "gdgpioc14eca7d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioc14eca7d::REGISTERS,
            },
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
                kind: "gdgpioc14eca7d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioc14eca7d::REGISTERS,
            },
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
                kind: "gdgpioc14eca7d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioc14eca7d::REGISTERS,
            },
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
                kind: "gdgpioc14eca7d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioc14eca7d::REGISTERS,
            },
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
                kind: "gdi2c734aaed5",
                version: "v1",
                block: "I2C",
                ir: &gdi2c734aaed5::REGISTERS,
            },
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
                kind: "gdi2c734aaed5",
                version: "v1",
                block: "I2C",
                ir: &gdi2c734aaed5::REGISTERS,
            },
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
                kind: "gdob09cb4d52",
                version: "v1",
                block: "OB",
                ir: &gdob09cb4d52::REGISTERS,
            },
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
                kind: "gdpmu97892901",
                version: "v1",
                block: "PMU",
                ir: &gdpmu97892901::REGISTERS,
            },
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
                kind: "gdrcu5361e546",
                version: "v1",
                block: "RCU",
                ir: &gdrcu5361e546::REGISTERS,
            },
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
                kind: "gdspi239d2229",
                version: "v1",
                block: "SPI",
                ir: &gdspi239d2229::REGISTERS,
            },
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
                kind: "gdspi239d2229",
                version: "v1",
                block: "SPI",
                ir: &gdspi239d2229::REGISTERS,
            },
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
                kind: "gdspi239d2229",
                version: "v1",
                block: "SPI",
                ir: &gdspi239d2229::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdtimerb17b3660",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerb17b3660::REGISTERS,
            },
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
                kind: "gdusartd2819c58",
                version: "v1",
                block: "USART",
                ir: &gdusartd2819c58::REGISTERS,
            },
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
                kind: "gdusartd2819c58",
                version: "v1",
                block: "USART",
                ir: &gdusartd2819c58::REGISTERS,
            },
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
                kind: "gdusartd2819c58",
                version: "v1",
                block: "USART",
                ir: &gdusartd2819c58::REGISTERS,
            },
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
                kind: "gdusartd2819c58",
                version: "v1",
                block: "USART",
                ir: &gdusartd2819c58::REGISTERS,
            },
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
                kind: "gdusartd2819c58",
                version: "v1",
                block: "USART",
                ir: &gdusartd2819c58::REGISTERS,
            },
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
        number: 19,
    },
    Interrupt {
        name: "LVD",
        number: 20,
    },
    Interrupt {
        name: "TAMPER",
        number: 21,
    },
    Interrupt {
        name: "RTC",
        number: 22,
    },
    Interrupt {
        name: "FMC",
        number: 23,
    },
    Interrupt {
        name: "RCU_CTC",
        number: 24,
    },
    Interrupt {
        name: "EXTI0",
        number: 25,
    },
    Interrupt {
        name: "EXTI1",
        number: 26,
    },
    Interrupt {
        name: "EXTI2",
        number: 27,
    },
    Interrupt {
        name: "EXTI3",
        number: 28,
    },
    Interrupt {
        name: "EXTI4",
        number: 29,
    },
    Interrupt {
        name: "DMA0_CHANNEL0",
        number: 30,
    },
    Interrupt {
        name: "DMA0_CHANNEL1",
        number: 31,
    },
    Interrupt {
        name: "DMA0_CHANNEL2",
        number: 32,
    },
    Interrupt {
        name: "DMA0_CHANNEL3",
        number: 33,
    },
    Interrupt {
        name: "DMA0_CHANNEL4",
        number: 34,
    },
    Interrupt {
        name: "DMA0_CHANNEL5",
        number: 35,
    },
    Interrupt {
        name: "DMA0_CHANNEL6",
        number: 36,
    },
    Interrupt {
        name: "ADC0_1",
        number: 37,
    },
    Interrupt {
        name: "CAN0_TX",
        number: 38,
    },
    Interrupt {
        name: "CAN0_RX0",
        number: 39,
    },
    Interrupt {
        name: "CAN0_RX1",
        number: 40,
    },
    Interrupt {
        name: "CAN0_EWMC",
        number: 41,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 42,
    },
    Interrupt {
        name: "TIMER0_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIMER0_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT",
        number: 45,
    },
    Interrupt {
        name: "TIMER0_CHANNEL",
        number: 46,
    },
    Interrupt {
        name: "TIMER1",
        number: 47,
    },
    Interrupt {
        name: "TIMER2",
        number: 48,
    },
    Interrupt {
        name: "TIMER3",
        number: 49,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 50,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 51,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 52,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 53,
    },
    Interrupt {
        name: "SPI0",
        number: 54,
    },
    Interrupt {
        name: "SPI1",
        number: 55,
    },
    Interrupt {
        name: "USART0",
        number: 56,
    },
    Interrupt {
        name: "USART1",
        number: 57,
    },
    Interrupt {
        name: "USART2",
        number: 58,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 59,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 60,
    },
    Interrupt {
        name: "USBFS_WKUP",
        number: 61,
    },
    Interrupt {
        name: "TIMER4",
        number: 69,
    },
    Interrupt {
        name: "SPI2",
        number: 70,
    },
    Interrupt {
        name: "UART3",
        number: 71,
    },
    Interrupt {
        name: "UART4",
        number: 72,
    },
    Interrupt {
        name: "TIMER5",
        number: 73,
    },
    Interrupt {
        name: "TIMER6",
        number: 74,
    },
    Interrupt {
        name: "DMA1_CHANNEL0",
        number: 75,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 76,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 77,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 78,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 79,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 82,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 83,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 84,
    },
    Interrupt {
        name: "CAN1_EWMC",
        number: 85,
    },
    Interrupt {
        name: "USBFS",
        number: 86,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadce30ea086_v1.rs"] pub mod gdadce30ea086;
#[path="../registers/gdafioa27eefcd_v1.rs"] pub mod gdafioa27eefcd;
#[path="../registers/gdbkp7944b1bc_v1.rs"] pub mod gdbkp7944b1bc;
#[path="../registers/gdcan486a8ac4_v1.rs"] pub mod gdcan486a8ac4;
#[path="../registers/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../registers/gddac55126433_v1.rs"] pub mod gddac55126433;
#[path="../registers/gddbg8cc9fb0b_v1.rs"] pub mod gddbg8cc9fb0b;
#[path="../registers/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../registers/gdexmc9f914e53_v1.rs"] pub mod gdexmc9f914e53;
#[path="../registers/gdextif95225bb_v1.rs"] pub mod gdextif95225bb;
#[path="../registers/gdfmcdc407917_v1.rs"] pub mod gdfmcdc407917;
#[path="../registers/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../registers/gdgpioc14eca7d_v1.rs"] pub mod gdgpioc14eca7d;
#[path="../registers/gdi2c734aaed5_v1.rs"] pub mod gdi2c734aaed5;
#[path="../registers/gdob09cb4d52_v1.rs"] pub mod gdob09cb4d52;
#[path="../registers/gdpmu97892901_v1.rs"] pub mod gdpmu97892901;
#[path="../registers/gdrcu5361e546_v1.rs"] pub mod gdrcu5361e546;
#[path="../registers/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../registers/gdspi239d2229_v1.rs"] pub mod gdspi239d2229;
#[path="../registers/gdtimerb17b3660_v1.rs"] pub mod gdtimerb17b3660;
#[path="../registers/gdusartd2819c58_v1.rs"] pub mod gdusartd2819c58;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
