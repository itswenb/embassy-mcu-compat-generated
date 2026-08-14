
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadca6e20c2c",
                version: "v1",
                block: "ADC",
                ir: &gdadca6e20c2c::REGISTERS,
            },
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
                kind: "gdcmp13366a93",
                version: "v1",
                block: "CMP",
                ir: &gdcmp13366a93::REGISTERS,
            },
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
        name: "DBG",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbgc0a1e7bf",
                version: "v1",
                block: "DBG",
                ir: &gddbgc0a1e7bf::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA",
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma35e38e2e",
                version: "v1",
                block: "DMA",
                ir: &gddma35e38e2e::REGISTERS,
            },
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
                kind: "gddmamux3017f39e",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux3017f39e::REGISTERS,
            },
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
                kind: "gddmamuxrmchxcfgbasebc255481",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbasebc255481::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA_CHXCNT_BASE",
        address: 0x4002000c,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmachxcntbase65c5fd05",
                version: "v1",
                block: "DMA_CHXCNT_BASE",
                ir: &gddmachxcntbase65c5fd05::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA_CHXCTL_BASE",
        address: 0x40020008,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmachxctlbase70e585cf",
                version: "v1",
                block: "DMA_CHXCTL_BASE",
                ir: &gddmachxctlbase70e585cf::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA_CHXMADDR_BASE",
        address: 0x40020014,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmachxmaddrbase5e865b4b",
                version: "v1",
                block: "DMA_CHXMADDR_BASE",
                ir: &gddmachxmaddrbase5e865b4b::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA_CHXPADDR_BASE",
        address: 0x40020010,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmachxpaddrbasea8e95eb7",
                version: "v1",
                block: "DMA_CHXPADDR_BASE",
                ir: &gddmachxpaddrbasea8e95eb7::REGISTERS,
            },
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
                kind: "gdextid7fe0966",
                version: "v1",
                block: "EXTI",
                ir: &gdextid7fe0966::REGISTERS,
            },
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
                kind: "gdfmccf889ee9",
                version: "v1",
                block: "FMC",
                ir: &gdfmccf889ee9::REGISTERS,
            },
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
                kind: "gdgpiod3b44485",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiod3b44485::REGISTERS,
            },
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
                kind: "gdgpiod3b44485",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiod3b44485::REGISTERS,
            },
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
                kind: "gdgpiod3b44485",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiod3b44485::REGISTERS,
            },
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
                kind: "gdgpiod3b44485",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiod3b44485::REGISTERS,
            },
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
                kind: "gdgpiod3b44485",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiod3b44485::REGISTERS,
            },
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
                kind: "gdi2c4e73acde",
                version: "v1",
                block: "I2C",
                ir: &gdi2c4e73acde::REGISTERS,
            },
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
                kind: "gdi2c4e73acde",
                version: "v1",
                block: "I2C",
                ir: &gdi2c4e73acde::REGISTERS,
            },
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
                kind: "gdpmu4410be00",
                version: "v1",
                block: "PMU",
                ir: &gdpmu4410be00::REGISTERS,
            },
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
                kind: "gdrcuffcd57c8",
                version: "v1",
                block: "RCU",
                ir: &gdrcuffcd57c8::REGISTERS,
            },
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
                kind: "gdrtce6f00754",
                version: "v1",
                block: "RTC",
                ir: &gdrtce6f00754::REGISTERS,
            },
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
                kind: "gdspi3e72f252",
                version: "v1",
                block: "SPI",
                ir: &gdspi3e72f252::REGISTERS,
            },
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
                kind: "gdspi3e72f252",
                version: "v1",
                block: "SPI",
                ir: &gdspi3e72f252::REGISTERS,
            },
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
                kind: "gdsyscfgf7d28bd5",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgf7d28bd5::REGISTERS,
            },
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
                kind: "gdtimer46598974",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer46598974::REGISTERS,
            },
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
                kind: "gdtimer46598974",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer46598974::REGISTERS,
            },
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
                kind: "gdtimer46598974",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer46598974::REGISTERS,
            },
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
                kind: "gdtimer46598974",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer46598974::REGISTERS,
            },
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
                kind: "gdtimer46598974",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer46598974::REGISTERS,
            },
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
                kind: "gdusart34eadece",
                version: "v1",
                block: "USART",
                ir: &gdusart34eadece::REGISTERS,
            },
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
                kind: "gdusart34eadece",
                version: "v1",
                block: "USART",
                ir: &gdusart34eadece::REGISTERS,
            },
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
                kind: "gdusart34eadece",
                version: "v1",
                block: "USART",
                ir: &gdusart34eadece::REGISTERS,
            },
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
                kind: "gdwwdgtf694703e",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgtf694703e::REGISTERS,
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
        name: "TIMESTAMP",
        number: 1,
    },
    Interrupt {
        name: "FMC",
        number: 3,
    },
    Interrupt {
        name: "RCU",
        number: 4,
    },
    Interrupt {
        name: "EXTI0",
        number: 5,
    },
    Interrupt {
        name: "EXTI1",
        number: 6,
    },
    Interrupt {
        name: "EXTI2",
        number: 7,
    },
    Interrupt {
        name: "EXTI3",
        number: 8,
    },
    Interrupt {
        name: "EXTI4",
        number: 9,
    },
    Interrupt {
        name: "DMA_CHANNEL0",
        number: 10,
    },
    Interrupt {
        name: "DMA_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "ADC",
        number: 13,
    },
    Interrupt {
        name: "USART0",
        number: 14,
    },
    Interrupt {
        name: "USART1",
        number: 15,
    },
    Interrupt {
        name: "USART2",
        number: 16,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 17,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 18,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 19,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 20,
    },
    Interrupt {
        name: "SPI0",
        number: 21,
    },
    Interrupt {
        name: "SPI1",
        number: 22,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 23,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT_UP_BRK",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_CHANNEL",
        number: 26,
    },
    Interrupt {
        name: "TIMER2",
        number: 27,
    },
    Interrupt {
        name: "TIMER13",
        number: 28,
    },
    Interrupt {
        name: "TIMER15",
        number: 29,
    },
    Interrupt {
        name: "TIMER16",
        number: 30,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 31,
    },
    Interrupt {
        name: "DMAMUX",
        number: 33,
    },
    Interrupt {
        name: "CMP0",
        number: 34,
    },
    Interrupt {
        name: "CMP1",
        number: 35,
    },
    Interrupt {
        name: "I2C0_WKUP",
        number: 36,
    },
    Interrupt {
        name: "I2C1_WKUP",
        number: 37,
    },
    Interrupt {
        name: "USART0_WKUP",
        number: 38,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadca6e20c2c_v1.rs"] pub mod gdadca6e20c2c;
#[path="../registers/gdcmp13366a93_v1.rs"] pub mod gdcmp13366a93;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gddbgc0a1e7bf_v1.rs"] pub mod gddbgc0a1e7bf;
#[path="../registers/gddma35e38e2e_v1.rs"] pub mod gddma35e38e2e;
#[path="../registers/gddmachxcntbase65c5fd05_v1.rs"] pub mod gddmachxcntbase65c5fd05;
#[path="../registers/gddmachxctlbase70e585cf_v1.rs"] pub mod gddmachxctlbase70e585cf;
#[path="../registers/gddmachxmaddrbase5e865b4b_v1.rs"] pub mod gddmachxmaddrbase5e865b4b;
#[path="../registers/gddmachxpaddrbasea8e95eb7_v1.rs"] pub mod gddmachxpaddrbasea8e95eb7;
#[path="../registers/gddmamux3017f39e_v1.rs"] pub mod gddmamux3017f39e;
#[path="../registers/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../registers/gddmamuxrmchxcfgbasebc255481_v1.rs"] pub mod gddmamuxrmchxcfgbasebc255481;
#[path="../registers/gdextid7fe0966_v1.rs"] pub mod gdextid7fe0966;
#[path="../registers/gdfmccf889ee9_v1.rs"] pub mod gdfmccf889ee9;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpiod3b44485_v1.rs"] pub mod gdgpiod3b44485;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdpmu4410be00_v1.rs"] pub mod gdpmu4410be00;
#[path="../registers/gdrcuffcd57c8_v1.rs"] pub mod gdrcuffcd57c8;
#[path="../registers/gdrtce6f00754_v1.rs"] pub mod gdrtce6f00754;
#[path="../registers/gdspi3e72f252_v1.rs"] pub mod gdspi3e72f252;
#[path="../registers/gdsyscfgf7d28bd5_v1.rs"] pub mod gdsyscfgf7d28bd5;
#[path="../registers/gdtimer46598974_v1.rs"] pub mod gdtimer46598974;
#[path="../registers/gdusart34eadece_v1.rs"] pub mod gdusart34eadece;
#[path="../registers/gdwwdgtf694703e_v1.rs"] pub mod gdwwdgtf694703e;
