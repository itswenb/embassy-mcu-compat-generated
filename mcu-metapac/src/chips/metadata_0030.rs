
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc939a9095",
                version: "v1",
                block: "ADC",
                ir: &gdadc939a9095::REGISTERS,
            },
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
                kind: "gdcmpe12ecc50",
                version: "v1",
                block: "CMP",
                ir: &gdcmpe12ecc50::REGISTERS,
            },
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
        name: "DBG",
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg2bf2258f",
                version: "v1",
                block: "DBG",
                ir: &gddbg2bf2258f::REGISTERS,
            },
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
                kind: "gddma203b2e8a",
                version: "v1",
                block: "DMA",
                ir: &gddma203b2e8a::REGISTERS,
            },
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
                kind: "gddmachxcntbased390cdb4",
                version: "v1",
                block: "DMA_CHXCNT_BASE",
                ir: &gddmachxcntbased390cdb4::REGISTERS,
            },
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
                kind: "gddmachxctlbase9fc231ae",
                version: "v1",
                block: "DMA_CHXCTL_BASE",
                ir: &gddmachxctlbase9fc231ae::REGISTERS,
            },
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
                kind: "gddmachxmaddrbase53fbca93",
                version: "v1",
                block: "DMA_CHXMADDR_BASE",
                ir: &gddmachxmaddrbase53fbca93::REGISTERS,
            },
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
                kind: "gddmachxpaddrbase24a24737",
                version: "v1",
                block: "DMA_CHXPADDR_BASE",
                ir: &gddmachxpaddrbase24a24737::REGISTERS,
            },
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
                kind: "gdexti7b9b36c7",
                version: "v1",
                block: "EXTI",
                ir: &gdexti7b9b36c7::REGISTERS,
            },
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
                kind: "gdfmcb0f6c6c8",
                version: "v1",
                block: "FMC",
                ir: &gdfmcb0f6c6c8::REGISTERS,
            },
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
                kind: "gdgpioe3950126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe3950126::REGISTERS,
            },
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
                kind: "gdgpioe3950126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe3950126::REGISTERS,
            },
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
                kind: "gdgpioe3950126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe3950126::REGISTERS,
            },
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
                kind: "gdgpioe3950126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe3950126::REGISTERS,
            },
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
                kind: "gdgpioe3950126",
                version: "v1",
                block: "GPIO",
                ir: &gdgpioe3950126::REGISTERS,
            },
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
                kind: "gdi2cd2b8dbf2",
                version: "v1",
                block: "I2C",
                ir: &gdi2cd2b8dbf2::REGISTERS,
            },
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
                kind: "gdi2cd2b8dbf2",
                version: "v1",
                block: "I2C",
                ir: &gdi2cd2b8dbf2::REGISTERS,
            },
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
                kind: "gdob8669b02b",
                version: "v1",
                block: "OB",
                ir: &gdob8669b02b::REGISTERS,
            },
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
                kind: "gdpmuda3a6d7f",
                version: "v1",
                block: "PMU",
                ir: &gdpmuda3a6d7f::REGISTERS,
            },
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
                kind: "gdrcuc6ee4fe0",
                version: "v1",
                block: "RCU",
                ir: &gdrcuc6ee4fe0::REGISTERS,
            },
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
                kind: "gdrtc218478ea",
                version: "v1",
                block: "RTC",
                ir: &gdrtc218478ea::REGISTERS,
            },
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
                kind: "gdspib2f7804e",
                version: "v1",
                block: "SPI",
                ir: &gdspib2f7804e::REGISTERS,
            },
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
                kind: "gdspib2f7804e",
                version: "v1",
                block: "SPI",
                ir: &gdspib2f7804e::REGISTERS,
            },
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
                kind: "gdsyscfg4d7d59e9",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg4d7d59e9::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdtimer58330829",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer58330829::REGISTERS,
            },
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
                kind: "gdusartf414f7c3",
                version: "v1",
                block: "USART",
                ir: &gdusartf414f7c3::REGISTERS,
            },
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
                kind: "gdusartf414f7c3",
                version: "v1",
                block: "USART",
                ir: &gdusartf414f7c3::REGISTERS,
            },
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
        number: 2,
    },
    Interrupt {
        name: "FMC",
        number: 3,
    },
    Interrupt {
        name: "RCU_CTC",
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
        name: "TIMER0_CHANNEL",
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
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 34,
    },
    Interrupt {
        name: "DMA_CHANNEL5_6",
        number: 48,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc939a9095_v1.rs"] pub mod gdadc939a9095;
#[path="../registers/gdcmpe12ecc50_v1.rs"] pub mod gdcmpe12ecc50;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../registers/gddbg2bf2258f_v1.rs"] pub mod gddbg2bf2258f;
#[path="../registers/gddma203b2e8a_v1.rs"] pub mod gddma203b2e8a;
#[path="../registers/gddmachxcntbased390cdb4_v1.rs"] pub mod gddmachxcntbased390cdb4;
#[path="../registers/gddmachxctlbase9fc231ae_v1.rs"] pub mod gddmachxctlbase9fc231ae;
#[path="../registers/gddmachxmaddrbase53fbca93_v1.rs"] pub mod gddmachxmaddrbase53fbca93;
#[path="../registers/gddmachxpaddrbase24a24737_v1.rs"] pub mod gddmachxpaddrbase24a24737;
#[path="../registers/gdexti7b9b36c7_v1.rs"] pub mod gdexti7b9b36c7;
#[path="../registers/gdfmcb0f6c6c8_v1.rs"] pub mod gdfmcb0f6c6c8;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpioe3950126_v1.rs"] pub mod gdgpioe3950126;
#[path="../registers/gdi2cd2b8dbf2_v1.rs"] pub mod gdi2cd2b8dbf2;
#[path="../registers/gdob8669b02b_v1.rs"] pub mod gdob8669b02b;
#[path="../registers/gdpmuda3a6d7f_v1.rs"] pub mod gdpmuda3a6d7f;
#[path="../registers/gdrcuc6ee4fe0_v1.rs"] pub mod gdrcuc6ee4fe0;
#[path="../registers/gdrtc218478ea_v1.rs"] pub mod gdrtc218478ea;
#[path="../registers/gdspib2f7804e_v1.rs"] pub mod gdspib2f7804e;
#[path="../registers/gdsyscfg4d7d59e9_v1.rs"] pub mod gdsyscfg4d7d59e9;
#[path="../registers/gdtimer58330829_v1.rs"] pub mod gdtimer58330829;
#[path="../registers/gdusartf414f7c3_v1.rs"] pub mod gdusartf414f7c3;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
