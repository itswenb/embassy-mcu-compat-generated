
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadca9050599",
                version: "v1",
                block: "ADC",
                ir: &gdadca9050599::REGISTERS,
            },
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
        address: 0x4c060000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcau95f6f36a",
                version: "v1",
                block: "CAU",
                ir: &gdcau95f6f36a::REGISTERS,
            },
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
        name: "DBG",
        address: 0xe0044000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg4e46e6a0",
                version: "v1",
                block: "DBG",
                ir: &gddbg4e46e6a0::REGISTERS,
            },
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
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma1cf53887",
                version: "v1",
                block: "DMA",
                ir: &gddma1cf53887::REGISTERS,
            },
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
                kind: "gdefuse4f36246c",
                version: "v1",
                block: "EFUSE",
                ir: &gdefuse4f36246c::REGISTERS,
            },
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
        address: 0x40013c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdextia48fbf2e",
                version: "v1",
                block: "EXTI",
                ir: &gdextia48fbf2e::REGISTERS,
            },
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
                kind: "gdfmc523fbb53",
                version: "v1",
                block: "FMC",
                ir: &gdfmc523fbb53::REGISTERS,
            },
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
        address: 0x40020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioe6fca7d9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe6fca7d9::REGISTERS,
            },
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
        address: 0x40020400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioe6fca7d9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe6fca7d9::REGISTERS,
            },
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
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpioe6fca7d9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe6fca7d9::REGISTERS,
            },
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
        address: 0x4c060400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhaub97c00c8",
                version: "v1",
                block: "HAU",
                ir: &gdhaub97c00c8::REGISTERS,
            },
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
        name: "PKCAU",
        address: 0x4c061000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpkcauf9e1d63d",
                version: "v1",
                block: "PKCAU",
                ir: &gdpkcauf9e1d63d::REGISTERS,
            },
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
                kind: "gdpmu8ef89808",
                version: "v1",
                block: "PMU",
                ir: &gdpmu8ef89808::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "QSPI",
        address: 0x40025800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdqspi9ca258b7",
                version: "v1",
                block: "QSPI",
                ir: &gdqspi9ca258b7::REGISTERS,
            },
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
        address: 0x40023800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrcu0b3fa95b",
                version: "v1",
                block: "RCU",
                ir: &gdrcu0b3fa95b::REGISTERS,
            },
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
                kind: "gdrtc7c0d047d",
                version: "v1",
                block: "RTC",
                ir: &gdrtc7c0d047d::REGISTERS,
            },
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
                kind: "gdspi25816acd",
                version: "v1",
                block: "SPI",
                ir: &gdspi25816acd::REGISTERS,
            },
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
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfg51b128a8",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg51b128a8::REGISTERS,
            },
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
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
        address: 0x40018000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
        address: 0x40018400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
                kind: "gdtimer5e62b6e6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer5e62b6e6::REGISTERS,
            },
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
        address: 0x4c060800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrng1f3ad225",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng1f3ad225::REGISTERS,
            },
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
        address: 0x40004400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart7f24e647",
                version: "v1",
                block: "USART",
                ir: &gdusart7f24e647::REGISTERS,
            },
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
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart7f24e647",
                version: "v1",
                block: "USART",
                ir: &gdusart7f24e647::REGISTERS,
            },
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
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusart7f24e647",
                version: "v1",
                block: "USART",
                ir: &gdusart7f24e647::REGISTERS,
            },
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
        name: "TAMPER_STAMP",
        number: 21,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 22,
    },
    Interrupt {
        name: "FMC",
        number: 23,
    },
    Interrupt {
        name: "RCU",
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
        name: "DMA_CHANNEL0",
        number: 30,
    },
    Interrupt {
        name: "DMA_CHANNEL1",
        number: 31,
    },
    Interrupt {
        name: "DMA_CHANNEL2",
        number: 32,
    },
    Interrupt {
        name: "DMA_CHANNEL3",
        number: 33,
    },
    Interrupt {
        name: "DMA_CHANNEL4",
        number: 34,
    },
    Interrupt {
        name: "DMA_CHANNEL5",
        number: 35,
    },
    Interrupt {
        name: "DMA_CHANNEL6",
        number: 36,
    },
    Interrupt {
        name: "DMA_CHANNEL7",
        number: 37,
    },
    Interrupt {
        name: "ADC",
        number: 38,
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
        name: "TIMER0_CMT",
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
        name: "SPI",
        number: 54,
    },
    Interrupt {
        name: "USART0",
        number: 56,
    },
    Interrupt {
        name: "UART1",
        number: 57,
    },
    Interrupt {
        name: "UART2",
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
        name: "VLVDF",
        number: 61,
    },
    Interrupt {
        name: "TIMER15",
        number: 63,
    },
    Interrupt {
        name: "TIMER16",
        number: 64,
    },
    Interrupt {
        name: "I2C0_WKUP",
        number: 70,
    },
    Interrupt {
        name: "USART0_WKUP",
        number: 71,
    },
    Interrupt {
        name: "TIMER5",
        number: 73,
    },
    Interrupt {
        name: "WIFI_TRIGGER",
        number: 74,
    },
    Interrupt {
        name: "WIFI_MAC",
        number: 75,
    },
    Interrupt {
        name: "WIFI_TX",
        number: 76,
    },
    Interrupt {
        name: "WIFI_RX",
        number: 77,
    },
    Interrupt {
        name: "LA",
        number: 83,
    },
    Interrupt {
        name: "WIFI_WKUP",
        number: 84,
    },
    Interrupt {
        name: "BLE_WKUP",
        number: 85,
    },
    Interrupt {
        name: "PLATFORM_WAKE",
        number: 86,
    },
    Interrupt {
        name: "ISO_BT_STAMP0",
        number: 87,
    },
    Interrupt {
        name: "ISO_BT_STAMP1",
        number: 88,
    },
    Interrupt {
        name: "ISO_BT_STAMP2",
        number: 89,
    },
    Interrupt {
        name: "ISO_BT_STAMP3",
        number: 90,
    },
    Interrupt {
        name: "ISO_BT_STAMP4",
        number: 91,
    },
    Interrupt {
        name: "ISO_BT_STAMP5",
        number: 92,
    },
    Interrupt {
        name: "ISO_BT_STAMP6",
        number: 93,
    },
    Interrupt {
        name: "ISO_BT_STAMP7",
        number: 94,
    },
    Interrupt {
        name: "PMU",
        number: 95,
    },
    Interrupt {
        name: "CAU",
        number: 98,
    },
    Interrupt {
        name: "HAU_TRNG",
        number: 99,
    },
    Interrupt {
        name: "WIFI_INT",
        number: 101,
    },
    Interrupt {
        name: "WIFI_SW_TRIG",
        number: 102,
    },
    Interrupt {
        name: "WIFI_FINE_TIMER_TARGET",
        number: 103,
    },
    Interrupt {
        name: "WIFI_STAMP_TARGET1",
        number: 104,
    },
    Interrupt {
        name: "WIFI_STAMP_TARGET2",
        number: 105,
    },
    Interrupt {
        name: "WIFI_STAMP_TARGET3",
        number: 106,
    },
    Interrupt {
        name: "WIFI_ENCRYPTION_ENGINE",
        number: 107,
    },
    Interrupt {
        name: "WIFI_SLEEP_MODE",
        number: 108,
    },
    Interrupt {
        name: "WIFI_HALF_SLOT",
        number: 109,
    },
    Interrupt {
        name: "WIFI_FIFO_ACTIVITY",
        number: 110,
    },
    Interrupt {
        name: "WIFI_ERROR",
        number: 111,
    },
    Interrupt {
        name: "WIFI_FREQ_SELECT",
        number: 112,
    },
    Interrupt {
        name: "EFUSE",
        number: 113,
    },
    Interrupt {
        name: "QSPI",
        number: 114,
    },
    Interrupt {
        name: "PKCAU",
        number: 115,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadca9050599_v1.rs"] pub mod gdadca9050599;
#[path="../registers/gdcau95f6f36a_v1.rs"] pub mod gdcau95f6f36a;
#[path="../registers/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../registers/gddbg4e46e6a0_v1.rs"] pub mod gddbg4e46e6a0;
#[path="../registers/gddma1cf53887_v1.rs"] pub mod gddma1cf53887;
#[path="../registers/gdefuse4f36246c_v1.rs"] pub mod gdefuse4f36246c;
#[path="../registers/gdextia48fbf2e_v1.rs"] pub mod gdextia48fbf2e;
#[path="../registers/gdfmc523fbb53_v1.rs"] pub mod gdfmc523fbb53;
#[path="../registers/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../registers/gdgpioe6fca7d9_v1.rs"] pub mod gdgpioe6fca7d9;
#[path="../registers/gdhaub97c00c8_v1.rs"] pub mod gdhaub97c00c8;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdpkcauf9e1d63d_v1.rs"] pub mod gdpkcauf9e1d63d;
#[path="../registers/gdpmu8ef89808_v1.rs"] pub mod gdpmu8ef89808;
#[path="../registers/gdqspi9ca258b7_v1.rs"] pub mod gdqspi9ca258b7;
#[path="../registers/gdrcu0b3fa95b_v1.rs"] pub mod gdrcu0b3fa95b;
#[path="../registers/gdrtc7c0d047d_v1.rs"] pub mod gdrtc7c0d047d;
#[path="../registers/gdspi25816acd_v1.rs"] pub mod gdspi25816acd;
#[path="../registers/gdsyscfg51b128a8_v1.rs"] pub mod gdsyscfg51b128a8;
#[path="../registers/gdtimer5e62b6e6_v1.rs"] pub mod gdtimer5e62b6e6;
#[path="../registers/gdtrng1f3ad225_v1.rs"] pub mod gdtrng1f3ad225;
#[path="../registers/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
