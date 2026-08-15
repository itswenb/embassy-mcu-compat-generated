
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc0742bf082",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0742bf082::REGISTERS,
            },
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
                kind: "gdadc16b3105a2",
                version: "v1",
                block: "ADC1",
                ir: &gdadc16b3105a2::REGISTERS,
            },
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
                kind: "gdafio0c3529dd",
                version: "v1",
                block: "AFIO",
                ir: &gdafio0c3529dd::REGISTERS,
            },
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
                kind: "gdbkp16a620e0",
                version: "v1",
                block: "BKP",
                ir: &gdbkp16a620e0::REGISTERS,
            },
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
                kind: "gdcrc66a4f78d",
                version: "v1",
                block: "CRC",
                ir: &gdcrc66a4f78d::REGISTERS,
            },
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
                kind: "gdctcb7d69f86",
                version: "v1",
                block: "CTC",
                ir: &gdctcb7d69f86::REGISTERS,
            },
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
                kind: "gddac95a93bf8",
                version: "v1",
                block: "DAC",
                ir: &gddac95a93bf8::REGISTERS,
            },
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
                kind: "gddbgf31a76f7",
                version: "v1",
                block: "DBG",
                ir: &gddbgf31a76f7::REGISTERS,
            },
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
                kind: "gddma0310ff78e",
                version: "v1",
                block: "DMA0",
                ir: &gddma0310ff78e::REGISTERS,
            },
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
                kind: "gddma0310ff78e",
                version: "v1",
                block: "DMA0",
                ir: &gddma0310ff78e::REGISTERS,
            },
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
                kind: "gdexmc0c702b92",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc0c702b92::REGISTERS,
            },
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
                kind: "gdexti9c0c2c74",
                version: "v1",
                block: "EXTI",
                ir: &gdexti9c0c2c74::REGISTERS,
            },
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
                kind: "gdfmc24146b0d",
                version: "v1",
                block: "FMC",
                ir: &gdfmc24146b0d::REGISTERS,
            },
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
                kind: "gdfwdgtdc3d0d7a",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtdc3d0d7a::REGISTERS,
            },
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
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
            },
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
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
            },
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
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
            },
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
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
            },
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
                kind: "gdgpioac3e5c224",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioac3e5c224::REGISTERS,
            },
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
                kind: "gdi2c0e704b87e",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0e704b87e::REGISTERS,
            },
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
                kind: "gdi2c0e704b87e",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0e704b87e::REGISTERS,
            },
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
                kind: "gdpmueb69c330",
                version: "v1",
                block: "PMU",
                ir: &gdpmueb69c330::REGISTERS,
            },
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
                kind: "gdrcu07c5da32",
                version: "v1",
                block: "RCU",
                ir: &gdrcu07c5da32::REGISTERS,
            },
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
        name: "SPI2",
        address: 0x40003c00,
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
        name: "TIMER0",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer0a0aa2af0",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a0aa2af0::REGISTERS,
            },
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
                kind: "gdtimer1e3a3341a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1e3a3341a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER10",
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer90722414a",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer90722414a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER11",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer895e47fd0",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer895e47fd0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER12",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer90722414a",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer90722414a::REGISTERS,
            },
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
                kind: "gdtimer90722414a",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer90722414a::REGISTERS,
            },
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
                kind: "gdtimer1e3a3341a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1e3a3341a::REGISTERS,
            },
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
                kind: "gdtimer1e3a3341a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1e3a3341a::REGISTERS,
            },
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
                kind: "gdtimer1e3a3341a",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1e3a3341a::REGISTERS,
            },
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
        name: "TIMER7",
        address: 0x40013400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer0a0aa2af0",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0a0aa2af0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER8",
        address: 0x40014c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer895e47fd0",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer895e47fd0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER9",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer90722414a",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer90722414a::REGISTERS,
            },
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
                kind: "gduart384580f85",
                version: "v1",
                block: "UART3",
                ir: &gduart384580f85::REGISTERS,
            },
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
                kind: "gduart384580f85",
                version: "v1",
                block: "UART3",
                ir: &gduart384580f85::REGISTERS,
            },
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
                kind: "gdusart05f10d25f",
                version: "v1",
                block: "USART0",
                ir: &gdusart05f10d25f::REGISTERS,
            },
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
                kind: "gdusart05f10d25f",
                version: "v1",
                block: "USART0",
                ir: &gdusart05f10d25f::REGISTERS,
            },
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
                kind: "gdusart05f10d25f",
                version: "v1",
                block: "USART0",
                ir: &gdusart05f10d25f::REGISTERS,
            },
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
                kind: "gdusbfsdevicea4903788",
                version: "v1",
                block: "USBFS_DEVICE",
                ir: &gdusbfsdevicea4903788::REGISTERS,
            },
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
                kind: "gdusbfsglobal36d63ef4",
                version: "v1",
                block: "USBFS_GLOBAL",
                ir: &gdusbfsglobal36d63ef4::REGISTERS,
            },
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
        name: "TAMPER",
        number: 2,
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
        name: "RCU_CTC",
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
        name: "DMA0_CHANNEL6",
        number: 17,
    },
    Interrupt {
        name: "ADC0_1",
        number: 18,
    },
    Interrupt {
        name: "EXTI_LINE9_5",
        number: 23,
    },
    Interrupt {
        name: "TIMER0_BRK_TIMER8",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP_TIMER9",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT_TIMER10",
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
        name: "TIMER3",
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
        name: "EXTI_LINE15_10",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "USBFS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIMER7_BRK_TIMER11",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP_TIMER12",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TRG_CMT_TIMER13",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CHANNEL",
        number: 46,
    },
    Interrupt {
        name: "EXMC",
        number: 48,
    },
    Interrupt {
        name: "TIMER4",
        number: 50,
    },
    Interrupt {
        name: "SPI2",
        number: 51,
    },
    Interrupt {
        name: "UART3",
        number: 52,
    },
    Interrupt {
        name: "UART4",
        number: 53,
    },
    Interrupt {
        name: "TIMER5",
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
        name: "DMA1_CHANNEL3_4",
        number: 59,
    },
    Interrupt {
        name: "USBFS",
        number: 67,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc0742bf082_v1.rs"] pub mod gdadc0742bf082;
#[path="../registers/gdadc16b3105a2_v1.rs"] pub mod gdadc16b3105a2;
#[path="../registers/gdafio0c3529dd_v1.rs"] pub mod gdafio0c3529dd;
#[path="../registers/gdbkp16a620e0_v1.rs"] pub mod gdbkp16a620e0;
#[path="../registers/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../registers/gdctcb7d69f86_v1.rs"] pub mod gdctcb7d69f86;
#[path="../registers/gddac95a93bf8_v1.rs"] pub mod gddac95a93bf8;
#[path="../registers/gddbgf31a76f7_v1.rs"] pub mod gddbgf31a76f7;
#[path="../registers/gddma0310ff78e_v1.rs"] pub mod gddma0310ff78e;
#[path="../registers/gdexmc0c702b92_v1.rs"] pub mod gdexmc0c702b92;
#[path="../registers/gdexti9c0c2c74_v1.rs"] pub mod gdexti9c0c2c74;
#[path="../registers/gdfmc24146b0d_v1.rs"] pub mod gdfmc24146b0d;
#[path="../registers/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../registers/gdgpioac3e5c224_v1.rs"] pub mod gdgpioac3e5c224;
#[path="../registers/gdi2c0e704b87e_v1.rs"] pub mod gdi2c0e704b87e;
#[path="../registers/gdpmueb69c330_v1.rs"] pub mod gdpmueb69c330;
#[path="../registers/gdrcu07c5da32_v1.rs"] pub mod gdrcu07c5da32;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdspi0c6850d65_v1.rs"] pub mod gdspi0c6850d65;
#[path="../registers/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../registers/gdtimer1e3a3341a_v1.rs"] pub mod gdtimer1e3a3341a;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../registers/gdtimer90722414a_v1.rs"] pub mod gdtimer90722414a;
#[path="../registers/gduart384580f85_v1.rs"] pub mod gduart384580f85;
#[path="../registers/gdusart05f10d25f_v1.rs"] pub mod gdusart05f10d25f;
#[path="../registers/gdusbfsdevicea4903788_v1.rs"] pub mod gdusbfsdevicea4903788;
#[path="../registers/gdusbfsglobal36d63ef4_v1.rs"] pub mod gdusbfsglobal36d63ef4;
#[path="../registers/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../registers/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
