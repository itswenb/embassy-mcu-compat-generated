
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcb7217899",
                version: "v1",
                block: "ADC",
                ir: &gdadcb7217899::REGISTERS,
            },
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
                kind: "gdcec9fb29752",
                version: "v1",
                block: "CEC",
                ir: &gdcec9fb29752::REGISTERS,
            },
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
                kind: "gdcmp6176059a",
                version: "v1",
                block: "CMP",
                ir: &gdcmp6176059a::REGISTERS,
            },
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
        name: "CTC",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdctc57a0fbe5",
                version: "v1",
                block: "CTC",
                ir: &gdctc57a0fbe5::REGISTERS,
            },
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
                kind: "gddacc6b1bb98",
                version: "v1",
                block: "DAC",
                ir: &gddacc6b1bb98::REGISTERS,
            },
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
                kind: "gddbg7f4c1511",
                version: "v1",
                block: "DBG",
                ir: &gddbg7f4c1511::REGISTERS,
            },
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
                kind: "gddma0f758611",
                version: "v1",
                block: "DMA",
                ir: &gddma0f758611::REGISTERS,
            },
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
                kind: "gdexti6214ef6d",
                version: "v1",
                block: "EXTI",
                ir: &gdexti6214ef6d::REGISTERS,
            },
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
                kind: "gdfmc20e9ec99",
                version: "v1",
                block: "FMC",
                ir: &gdfmc20e9ec99::REGISTERS,
            },
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
                kind: "gdfwdgtcbc843d4",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtcbc843d4::REGISTERS,
            },
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
                kind: "gdgpioa54a0be2b",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa54a0be2b::REGISTERS,
            },
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
                kind: "gdgpiobcfe71f6a",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiobcfe71f6a::REGISTERS,
            },
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
                kind: "gdgpioc47392aee",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc47392aee::REGISTERS,
            },
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
                kind: "gdgpiodc257f1c6",
                version: "v1",
                block: "GPIOD",
                ir: &gdgpiodc257f1c6::REGISTERS,
            },
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
                kind: "gdgpiof564f1005",
                version: "v1",
                block: "GPIOF",
                ir: &gdgpiof564f1005::REGISTERS,
            },
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
                kind: "gdi2c00d742485",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c00d742485::REGISTERS,
            },
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
                kind: "gdi2c00d742485",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c00d742485::REGISTERS,
            },
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
                kind: "gdpmuf173c0ef",
                version: "v1",
                block: "PMU",
                ir: &gdpmuf173c0ef::REGISTERS,
            },
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
                kind: "gdrcu5258fdf2",
                version: "v1",
                block: "RCU",
                ir: &gdrcu5258fdf2::REGISTERS,
            },
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
                kind: "gdrtc7ef316ca",
                version: "v1",
                block: "RTC",
                ir: &gdrtc7ef316ca::REGISTERS,
            },
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
                kind: "gdspi04f926fdd",
                version: "v1",
                block: "SPI0",
                ir: &gdspi04f926fdd::REGISTERS,
            },
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
                kind: "gdspi04f926fdd",
                version: "v1",
                block: "SPI0",
                ir: &gdspi04f926fdd::REGISTERS,
            },
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
                kind: "gdsyscfg595878d5",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg595878d5::REGISTERS,
            },
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
                kind: "gdtimer0d9a58b68",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0d9a58b68::REGISTERS,
            },
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
                kind: "gdtimer16a9c1bb0",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer16a9c1bb0::REGISTERS,
            },
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
                kind: "gdtimer1309b6b8cc",
                version: "v1",
                block: "TIMER13",
                ir: &gdtimer1309b6b8cc::REGISTERS,
            },
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
                kind: "gdtimer144dec44bb",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer144dec44bb::REGISTERS,
            },
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
                kind: "gdtimer15dc6fd783",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15dc6fd783::REGISTERS,
            },
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
                kind: "gdtimer15dc6fd783",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15dc6fd783::REGISTERS,
            },
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
                kind: "gdtimer16a9c1bb0",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer16a9c1bb0::REGISTERS,
            },
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
                kind: "gdtimer5183dba8f",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer5183dba8f::REGISTERS,
            },
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
                kind: "gdtsid83e70fb",
                version: "v1",
                block: "TSI",
                ir: &gdtsid83e70fb::REGISTERS,
            },
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
                kind: "gdusart08bc22e17",
                version: "v1",
                block: "USART0",
                ir: &gdusart08bc22e17::REGISTERS,
            },
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
                kind: "gdusart08bc22e17",
                version: "v1",
                block: "USART0",
                ir: &gdusart08bc22e17::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBFS_DEVICE",
        address: 0x50000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfsdevice6d1906cf",
                version: "v1",
                block: "USBFS_DEVICE",
                ir: &gdusbfsdevice6d1906cf::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBFS_GLOBAL",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfsglobal1a7549aa",
                version: "v1",
                block: "USBFS_GLOBAL",
                ir: &gdusbfsglobal1a7549aa::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBFS_HOST",
        address: 0x50000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfshost5f42a79e",
                version: "v1",
                block: "USBFS_HOST",
                ir: &gdusbfshost5f42a79e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBFS_PWRCLK",
        address: 0x50000e00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusbfspwrclk2ac667f0",
                version: "v1",
                block: "USBFS_PWRCLK",
                ir: &gdusbfspwrclk2ac667f0::REGISTERS,
            },
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
                kind: "gdwwdgtfa76971a",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgtfa76971a::REGISTERS,
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
        name: "USBFS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "DMA_CHANNEL5_6",
        number: 48,
    },
    Interrupt {
        name: "USBFS",
        number: 67,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadcb7217899_v1.rs"] pub mod gdadcb7217899;
#[path="../registers/gdcec9fb29752_v1.rs"] pub mod gdcec9fb29752;
#[path="../registers/gdcmp6176059a_v1.rs"] pub mod gdcmp6176059a;
#[path="../registers/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../registers/gdctc57a0fbe5_v1.rs"] pub mod gdctc57a0fbe5;
#[path="../registers/gddacc6b1bb98_v1.rs"] pub mod gddacc6b1bb98;
#[path="../registers/gddbg7f4c1511_v1.rs"] pub mod gddbg7f4c1511;
#[path="../registers/gddma0f758611_v1.rs"] pub mod gddma0f758611;
#[path="../registers/gdexti6214ef6d_v1.rs"] pub mod gdexti6214ef6d;
#[path="../registers/gdfmc20e9ec99_v1.rs"] pub mod gdfmc20e9ec99;
#[path="../registers/gdfwdgtcbc843d4_v1.rs"] pub mod gdfwdgtcbc843d4;
#[path="../registers/gdgpioa54a0be2b_v1.rs"] pub mod gdgpioa54a0be2b;
#[path="../registers/gdgpiobcfe71f6a_v1.rs"] pub mod gdgpiobcfe71f6a;
#[path="../registers/gdgpioc47392aee_v1.rs"] pub mod gdgpioc47392aee;
#[path="../registers/gdgpiodc257f1c6_v1.rs"] pub mod gdgpiodc257f1c6;
#[path="../registers/gdgpiof564f1005_v1.rs"] pub mod gdgpiof564f1005;
#[path="../registers/gdi2c00d742485_v1.rs"] pub mod gdi2c00d742485;
#[path="../registers/gdpmuf173c0ef_v1.rs"] pub mod gdpmuf173c0ef;
#[path="../registers/gdrcu5258fdf2_v1.rs"] pub mod gdrcu5258fdf2;
#[path="../registers/gdrtc7ef316ca_v1.rs"] pub mod gdrtc7ef316ca;
#[path="../registers/gdspi04f926fdd_v1.rs"] pub mod gdspi04f926fdd;
#[path="../registers/gdsyscfg595878d5_v1.rs"] pub mod gdsyscfg595878d5;
#[path="../registers/gdtimer0d9a58b68_v1.rs"] pub mod gdtimer0d9a58b68;
#[path="../registers/gdtimer1309b6b8cc_v1.rs"] pub mod gdtimer1309b6b8cc;
#[path="../registers/gdtimer144dec44bb_v1.rs"] pub mod gdtimer144dec44bb;
#[path="../registers/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../registers/gdtimer16a9c1bb0_v1.rs"] pub mod gdtimer16a9c1bb0;
#[path="../registers/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../registers/gdtsid83e70fb_v1.rs"] pub mod gdtsid83e70fb;
#[path="../registers/gdusart08bc22e17_v1.rs"] pub mod gdusart08bc22e17;
#[path="../registers/gdusbfsdevice6d1906cf_v1.rs"] pub mod gdusbfsdevice6d1906cf;
#[path="../registers/gdusbfsglobal1a7549aa_v1.rs"] pub mod gdusbfsglobal1a7549aa;
#[path="../registers/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../registers/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../registers/gdwwdgtfa76971a_v1.rs"] pub mod gdwwdgtfa76971a;
