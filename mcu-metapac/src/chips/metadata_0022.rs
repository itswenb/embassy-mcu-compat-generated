
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc018ab876",
                version: "v1",
                block: "ADC",
                ir: &gdadc018ab876::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CEC",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcecade85f56",
                version: "v1",
                block: "CEC",
                ir: &gdcecade85f56::REGISTERS,
            },
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
        address: 0x4001001c,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmp23924063",
                version: "v1",
                block: "CMP",
                ir: &gdcmp23924063::REGISTERS,
            },
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
                kind: "gdcrc7d8cea52",
                version: "v1",
                block: "CRC",
                ir: &gdcrc7d8cea52::REGISTERS,
            },
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
                kind: "gddac2ce5879d",
                version: "v1",
                block: "DAC",
                ir: &gddac2ce5879d::REGISTERS,
            },
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
                kind: "gddbg1876a7bc",
                version: "v1",
                block: "DBG",
                ir: &gddbg1876a7bc::REGISTERS,
            },
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
                kind: "gddma9472c5b9",
                version: "v1",
                block: "DMA",
                ir: &gddma9472c5b9::REGISTERS,
            },
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
                kind: "gdextiab091bb7",
                version: "v1",
                block: "EXTI",
                ir: &gdextiab091bb7::REGISTERS,
            },
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
                kind: "gdfmcb49be91b",
                version: "v1",
                block: "FMC",
                ir: &gdfmcb49be91b::REGISTERS,
            },
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
                kind: "gdgpioa3815acae",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa3815acae::REGISTERS,
            },
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
                kind: "gdgpiob0e35583c",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob0e35583c::REGISTERS,
            },
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
                kind: "gdgpioc797149ba",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc797149ba::REGISTERS,
            },
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
                kind: "gdgpiod7a8beedc",
                version: "v1",
                block: "GPIOD",
                ir: &gdgpiod7a8beedc::REGISTERS,
            },
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
                kind: "gdgpiof4abb2202",
                version: "v1",
                block: "GPIOF",
                ir: &gdgpiof4abb2202::REGISTERS,
            },
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
                kind: "gdi2c004a0a4d9",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c004a0a4d9::REGISTERS,
            },
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
                kind: "gdi2c004a0a4d9",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c004a0a4d9::REGISTERS,
            },
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
                kind: "gdi2c004a0a4d9",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c004a0a4d9::REGISTERS,
            },
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
                kind: "gdpmu808687e1",
                version: "v1",
                block: "PMU",
                ir: &gdpmu808687e1::REGISTERS,
            },
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
                kind: "gdrcu70c8037a",
                version: "v1",
                block: "RCU",
                ir: &gdrcu70c8037a::REGISTERS,
            },
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
                kind: "gdrtce98f095e",
                version: "v1",
                block: "RTC",
                ir: &gdrtce98f095e::REGISTERS,
            },
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
                kind: "gdspi031a1ec7f",
                version: "v1",
                block: "SPI0",
                ir: &gdspi031a1ec7f::REGISTERS,
            },
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
                kind: "gdspi031a1ec7f",
                version: "v1",
                block: "SPI0",
                ir: &gdspi031a1ec7f::REGISTERS,
            },
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
                kind: "gdspi031a1ec7f",
                version: "v1",
                block: "SPI0",
                ir: &gdspi031a1ec7f::REGISTERS,
            },
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
                kind: "gdsyscfg501b84a6",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg501b84a6::REGISTERS,
            },
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
                kind: "gdtimer096c7099a",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer096c7099a::REGISTERS,
            },
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
                kind: "gdtimer1ac32c839",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1ac32c839::REGISTERS,
            },
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
                kind: "gdtimer131b1f9b88",
                version: "v1",
                block: "TIMER13",
                ir: &gdtimer131b1f9b88::REGISTERS,
            },
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
                kind: "gdtimer143daae142",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer143daae142::REGISTERS,
            },
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
                kind: "gdtimer1504ddc856",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer1504ddc856::REGISTERS,
            },
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
                kind: "gdtimer1504ddc856",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer1504ddc856::REGISTERS,
            },
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
                kind: "gdtimer1ac32c839",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1ac32c839::REGISTERS,
            },
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
                kind: "gdtimer539366dab",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer539366dab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TSI",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtsie9c86076",
                version: "v1",
                block: "TSI",
                ir: &gdtsie9c86076::REGISTERS,
            },
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
                kind: "gdusart06f0b00f6",
                version: "v1",
                block: "USART0",
                ir: &gdusart06f0b00f6::REGISTERS,
            },
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
                kind: "gdusart13604d2cd",
                version: "v1",
                block: "USART1",
                ir: &gdusart13604d2cd::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBD",
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbd0273a115",
                version: "v1",
                block: "USBD",
                ir: &gdusbd0273a115::REGISTERS,
            },
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
                kind: "gdwwdgta2f29825",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgta2f29825::REGISTERS,
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
        number: 2,
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
        name: "EXTI0_1",
        number: 5,
    },
    Interrupt {
        name: "EXTI2_3",
        number: 6,
    },
    Interrupt {
        name: "EXTI4_15",
        number: 7,
    },
    Interrupt {
        name: "TSI",
        number: 8,
    },
    Interrupt {
        name: "DMA_CHANNEL0",
        number: 9,
    },
    Interrupt {
        name: "DMA_CHANNEL1_2",
        number: 10,
    },
    Interrupt {
        name: "DMA_CHANNEL3_4",
        number: 11,
    },
    Interrupt {
        name: "ADC_CMP",
        number: 12,
    },
    Interrupt {
        name: "TIMER0_BRK_UP_TRG_COM",
        number: 13,
    },
    Interrupt {
        name: "TIMER0_CC",
        number: 14,
    },
    Interrupt {
        name: "TIMER1",
        number: 15,
    },
    Interrupt {
        name: "TIMER2",
        number: 16,
    },
    Interrupt {
        name: "TIMER5_DAC",
        number: 17,
    },
    Interrupt {
        name: "TIMER13",
        number: 19,
    },
    Interrupt {
        name: "TIMER14",
        number: 20,
    },
    Interrupt {
        name: "TIMER15",
        number: 21,
    },
    Interrupt {
        name: "TIMER16",
        number: 22,
    },
    Interrupt {
        name: "I2C0_EV",
        number: 23,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 24,
    },
    Interrupt {
        name: "SPI0",
        number: 25,
    },
    Interrupt {
        name: "SPI1",
        number: 26,
    },
    Interrupt {
        name: "USART0",
        number: 27,
    },
    Interrupt {
        name: "USART1",
        number: 28,
    },
    Interrupt {
        name: "CEC",
        number: 30,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 34,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 35,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 36,
    },
    Interrupt {
        name: "USBD_LP",
        number: 37,
    },
    Interrupt {
        name: "USBD_HP",
        number: 38,
    },
    Interrupt {
        name: "USBD_WKUP",
        number: 42,
    },
    Interrupt {
        name: "DMA_CHANNEL5_6",
        number: 48,
    },
    Interrupt {
        name: "SPI2",
        number: 51,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc018ab876_v1.rs"] pub mod gdadc018ab876;
#[path="../registers/gdcecade85f56_v1.rs"] pub mod gdcecade85f56;
#[path="../registers/gdcmp23924063_v1.rs"] pub mod gdcmp23924063;
#[path="../registers/gdcrc7d8cea52_v1.rs"] pub mod gdcrc7d8cea52;
#[path="../registers/gddac2ce5879d_v1.rs"] pub mod gddac2ce5879d;
#[path="../registers/gddbg1876a7bc_v1.rs"] pub mod gddbg1876a7bc;
#[path="../registers/gddma9472c5b9_v1.rs"] pub mod gddma9472c5b9;
#[path="../registers/gdextiab091bb7_v1.rs"] pub mod gdextiab091bb7;
#[path="../registers/gdfmcb49be91b_v1.rs"] pub mod gdfmcb49be91b;
#[path="../registers/gdfwdgte0a44d28_v1.rs"] pub mod gdfwdgte0a44d28;
#[path="../registers/gdgpioa3815acae_v1.rs"] pub mod gdgpioa3815acae;
#[path="../registers/gdgpiob0e35583c_v1.rs"] pub mod gdgpiob0e35583c;
#[path="../registers/gdgpioc797149ba_v1.rs"] pub mod gdgpioc797149ba;
#[path="../registers/gdgpiod7a8beedc_v1.rs"] pub mod gdgpiod7a8beedc;
#[path="../registers/gdgpiof4abb2202_v1.rs"] pub mod gdgpiof4abb2202;
#[path="../registers/gdi2c004a0a4d9_v1.rs"] pub mod gdi2c004a0a4d9;
#[path="../registers/gdpmu808687e1_v1.rs"] pub mod gdpmu808687e1;
#[path="../registers/gdrcu70c8037a_v1.rs"] pub mod gdrcu70c8037a;
#[path="../registers/gdrtce98f095e_v1.rs"] pub mod gdrtce98f095e;
#[path="../registers/gdspi031a1ec7f_v1.rs"] pub mod gdspi031a1ec7f;
#[path="../registers/gdsyscfg501b84a6_v1.rs"] pub mod gdsyscfg501b84a6;
#[path="../registers/gdtimer096c7099a_v1.rs"] pub mod gdtimer096c7099a;
#[path="../registers/gdtimer131b1f9b88_v1.rs"] pub mod gdtimer131b1f9b88;
#[path="../registers/gdtimer143daae142_v1.rs"] pub mod gdtimer143daae142;
#[path="../registers/gdtimer1504ddc856_v1.rs"] pub mod gdtimer1504ddc856;
#[path="../registers/gdtimer1ac32c839_v1.rs"] pub mod gdtimer1ac32c839;
#[path="../registers/gdtimer539366dab_v1.rs"] pub mod gdtimer539366dab;
#[path="../registers/gdtsie9c86076_v1.rs"] pub mod gdtsie9c86076;
#[path="../registers/gdusart06f0b00f6_v1.rs"] pub mod gdusart06f0b00f6;
#[path="../registers/gdusart13604d2cd_v1.rs"] pub mod gdusart13604d2cd;
#[path="../registers/gdusbd0273a115_v1.rs"] pub mod gdusbd0273a115;
#[path="../registers/gdwwdgta2f29825_v1.rs"] pub mod gdwwdgta2f29825;
