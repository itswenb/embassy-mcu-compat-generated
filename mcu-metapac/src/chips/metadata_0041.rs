
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadca4c861d7",
                version: "v1",
                block: "ADC",
                ir: &gdadca4c861d7::REGISTERS,
            },
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
        address: 0x40006400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcanf0c54386",
                version: "v1",
                block: "CAN",
                ir: &gdcanf0c54386::REGISTERS,
            },
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
        address: 0x50060000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcaue9e51f0c",
                version: "v1",
                block: "CAU",
                ir: &gdcaue9e51f0c::REGISTERS,
            },
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
                kind: "gdcmpd90af10b",
                version: "v1",
                block: "CMP",
                ir: &gdcmpd90af10b::REGISTERS,
            },
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
                kind: "gdcrc67d273cb",
                version: "v1",
                block: "CRC",
                ir: &gdcrc67d273cb::REGISTERS,
            },
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
                kind: "gdctceaaaf458",
                version: "v1",
                block: "CTC",
                ir: &gdctceaaaf458::REGISTERS,
            },
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
                kind: "gddac7e57a629",
                version: "v1",
                block: "DAC",
                ir: &gddac7e57a629::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbgmcu02036f49",
                version: "v1",
                block: "DBGMCU",
                ir: &gddbgmcu02036f49::REGISTERS,
            },
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
                kind: "gddmaaff99f21",
                version: "v1",
                block: "DMA",
                ir: &gddmaaff99f21::REGISTERS,
            },
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
                kind: "gddmamux1c160dfc",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux1c160dfc::REGISTERS,
            },
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
                kind: "gdexti30fc9668",
                version: "v1",
                block: "EXTI",
                ir: &gdexti30fc9668::REGISTERS,
            },
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
                kind: "gdfmc8dbd8b5e",
                version: "v1",
                block: "FMC",
                ir: &gdfmc8dbd8b5e::REGISTERS,
            },
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
                kind: "gdfwdgt5932fb56",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgt5932fb56::REGISTERS,
            },
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
                kind: "gdgpioc0fba06c4",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc0fba06c4::REGISTERS,
            },
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
                kind: "gdgpiod7229d923",
                version: "v1",
                block: "GPIOD",
                ir: &gdgpiod7229d923::REGISTERS,
            },
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
                kind: "gdgpiof7c6237df",
                version: "v1",
                block: "GPIOF",
                ir: &gdgpiof7c6237df::REGISTERS,
            },
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
                kind: "gdi2c0cd973dc4",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0cd973dc4::REGISTERS,
            },
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
                kind: "gdi2c0cd973dc4",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0cd973dc4::REGISTERS,
            },
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
                kind: "gdi2c0cd973dc4",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0cd973dc4::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPTIMER0",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlptimer0fade8b7a",
                version: "v1",
                block: "LPTIMER0",
                ir: &gdlptimer0fade8b7a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPTIMER1",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlptimer0fade8b7a",
                version: "v1",
                block: "LPTIMER0",
                ir: &gdlptimer0fade8b7a::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPUART0",
        address: 0x40008000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlpuart058954338",
                version: "v1",
                block: "LPUART0",
                ir: &gdlpuart058954338::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlpuart058954338",
                version: "v1",
                block: "LPUART0",
                ir: &gdlpuart058954338::REGISTERS,
            },
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
                kind: "gdpmu1c5ddd6a",
                version: "v1",
                block: "PMU",
                ir: &gdpmu1c5ddd6a::REGISTERS,
            },
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
                kind: "gdrcuf0eb0adf",
                version: "v1",
                block: "RCU",
                ir: &gdrcuf0eb0adf::REGISTERS,
            },
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
                kind: "gdrtc30fffb52",
                version: "v1",
                block: "RTC",
                ir: &gdrtc30fffb52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SLCD",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdslcd6dc6af89",
                version: "v1",
                block: "SLCD",
                ir: &gdslcd6dc6af89::REGISTERS,
            },
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
                kind: "gdspi0cf000376",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0cf000376::REGISTERS,
            },
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
                kind: "gdspi19358bf74",
                version: "v1",
                block: "SPI1",
                ir: &gdspi19358bf74::REGISTERS,
            },
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
                kind: "gdsyscfg47a22be1",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg47a22be1::REGISTERS,
            },
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
                kind: "gdtimer0d0a204ab",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0d0a204ab::REGISTERS,
            },
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
                kind: "gdtimer180e32204",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer180e32204::REGISTERS,
            },
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
                kind: "gdtimer8dfb2bdb4",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer8dfb2bdb4::REGISTERS,
            },
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
                kind: "gdtimer14452fee2b",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer14452fee2b::REGISTERS,
            },
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
                kind: "gdtimer20f28b429",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer20f28b429::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER40",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer14452fee2b",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer14452fee2b::REGISTERS,
            },
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
        name: "TIMER6",
        address: 0x40001400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer6b465bb6b",
                version: "v1",
                block: "TIMER6",
                ir: &gdtimer6b465bb6b::REGISTERS,
            },
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
                kind: "gdtimer8dfb2bdb4",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer8dfb2bdb4::REGISTERS,
            },
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
        address: 0x50060800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrngbf61c352",
                version: "v1",
                block: "TRNG",
                ir: &gdtrngbf61c352::REGISTERS,
            },
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
                kind: "gduart37add471e",
                version: "v1",
                block: "UART3",
                ir: &gduart37add471e::REGISTERS,
            },
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
                kind: "gduart47d66af8a",
                version: "v1",
                block: "UART4",
                ir: &gduart47d66af8a::REGISTERS,
            },
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
                kind: "gdusart0184abb20",
                version: "v1",
                block: "USART0",
                ir: &gdusart0184abb20::REGISTERS,
            },
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
                kind: "gdusart0184abb20",
                version: "v1",
                block: "USART0",
                ir: &gdusart0184abb20::REGISTERS,
            },
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
                kind: "gdusbd3c6a50b5",
                version: "v1",
                block: "USBD",
                ir: &gdusbd3c6a50b5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "VREF",
        address: 0x40010030,
        registers: Some(
            PeripheralRegisters {
                kind: "gdvreff6814bb8",
                version: "v1",
                block: "VREF",
                ir: &gdvreff6814bb8::REGISTERS,
            },
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
                kind: "gdwwdgtdd622579",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgtdd622579::REGISTERS,
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
        name: "RTC_TAMPER_TIMESTAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WAKE",
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
        name: "EXTI0",
        number: 6,
    },
    Interrupt {
        name: "EXTI01",
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
        name: "DMA_CHANNEL0",
        number: 11,
    },
    Interrupt {
        name: "DMA_CHANNEL1",
        number: 12,
    },
    Interrupt {
        name: "DMA_CHANNEL2",
        number: 13,
    },
    Interrupt {
        name: "DMA_CHANNEL3",
        number: 14,
    },
    Interrupt {
        name: "DMA_CHANNEL4",
        number: 15,
    },
    Interrupt {
        name: "DMA_CHANNEL5",
        number: 16,
    },
    Interrupt {
        name: "DMA_CHANNEL6",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "USBD_HP_CAN0_TX",
        number: 19,
    },
    Interrupt {
        name: "USBD_LP_CAN0_RX0",
        number: 20,
    },
    Interrupt {
        name: "TIMER1",
        number: 21,
    },
    Interrupt {
        name: "TIMER2",
        number: 22,
    },
    Interrupt {
        name: "TIMER8",
        number: 23,
    },
    Interrupt {
        name: "TIMER11",
        number: 24,
    },
    Interrupt {
        name: "TIMER5",
        number: 25,
    },
    Interrupt {
        name: "TIMER6",
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
        name: "USART3",
        number: 29,
    },
    Interrupt {
        name: "UART4",
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
        name: "DAC",
        number: 37,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 39,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "USBD_WKUP",
        number: 42,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 43,
    },
    Interrupt {
        name: "TIMER0",
        number: 44,
    },
    Interrupt {
        name: "TIMER0_CAP",
        number: 45,
    },
    Interrupt {
        name: "TIMER14",
        number: 46,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 47,
    },
    Interrupt {
        name: "TIMER40",
        number: 48,
    },
    Interrupt {
        name: "CAN0_RX1",
        number: 49,
    },
    Interrupt {
        name: "CAN0_EWMC",
        number: 50,
    },
    Interrupt {
        name: "DMA_MUX",
        number: 55,
    },
    Interrupt {
        name: "CMP0",
        number: 56,
    },
    Interrupt {
        name: "CMP1",
        number: 57,
    },
    Interrupt {
        name: "I2C0_WAKE",
        number: 58,
    },
    Interrupt {
        name: "I2C2_WAKE",
        number: 59,
    },
    Interrupt {
        name: "USART0_WAKE",
        number: 60,
    },
    Interrupt {
        name: "LPUART0",
        number: 61,
    },
    Interrupt {
        name: "CAU",
        number: 62,
    },
    Interrupt {
        name: "TRNG",
        number: 63,
    },
    Interrupt {
        name: "SLCD",
        number: 64,
    },
    Interrupt {
        name: "USART1_WAKE",
        number: 65,
    },
    Interrupt {
        name: "I2C1_WAKE",
        number: 66,
    },
    Interrupt {
        name: "LPUART0_WAKE",
        number: 67,
    },
    Interrupt {
        name: "LPTIMER0",
        number: 68,
    },
    Interrupt {
        name: "LPUART1_WAKE",
        number: 69,
    },
    Interrupt {
        name: "LPTIMER1",
        number: 70,
    },
    Interrupt {
        name: "LPUART1",
        number: 71,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadca4c861d7_v1.rs"] pub mod gdadca4c861d7;
#[path="../registers/gdcanf0c54386_v1.rs"] pub mod gdcanf0c54386;
#[path="../registers/gdcaue9e51f0c_v1.rs"] pub mod gdcaue9e51f0c;
#[path="../registers/gdcmpd90af10b_v1.rs"] pub mod gdcmpd90af10b;
#[path="../registers/gdcrc67d273cb_v1.rs"] pub mod gdcrc67d273cb;
#[path="../registers/gdctceaaaf458_v1.rs"] pub mod gdctceaaaf458;
#[path="../registers/gddac7e57a629_v1.rs"] pub mod gddac7e57a629;
#[path="../registers/gddbgmcu02036f49_v1.rs"] pub mod gddbgmcu02036f49;
#[path="../registers/gddmaaff99f21_v1.rs"] pub mod gddmaaff99f21;
#[path="../registers/gddmamux1c160dfc_v1.rs"] pub mod gddmamux1c160dfc;
#[path="../registers/gdexti30fc9668_v1.rs"] pub mod gdexti30fc9668;
#[path="../registers/gdfmc8dbd8b5e_v1.rs"] pub mod gdfmc8dbd8b5e;
#[path="../registers/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../registers/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../registers/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../registers/gdgpioc0fba06c4_v1.rs"] pub mod gdgpioc0fba06c4;
#[path="../registers/gdgpiod7229d923_v1.rs"] pub mod gdgpiod7229d923;
#[path="../registers/gdgpiof7c6237df_v1.rs"] pub mod gdgpiof7c6237df;
#[path="../registers/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../registers/gdlptimer0fade8b7a_v1.rs"] pub mod gdlptimer0fade8b7a;
#[path="../registers/gdlpuart058954338_v1.rs"] pub mod gdlpuart058954338;
#[path="../registers/gdpmu1c5ddd6a_v1.rs"] pub mod gdpmu1c5ddd6a;
#[path="../registers/gdrcuf0eb0adf_v1.rs"] pub mod gdrcuf0eb0adf;
#[path="../registers/gdrtc30fffb52_v1.rs"] pub mod gdrtc30fffb52;
#[path="../registers/gdslcd6dc6af89_v1.rs"] pub mod gdslcd6dc6af89;
#[path="../registers/gdspi0cf000376_v1.rs"] pub mod gdspi0cf000376;
#[path="../registers/gdspi19358bf74_v1.rs"] pub mod gdspi19358bf74;
#[path="../registers/gdsyscfg47a22be1_v1.rs"] pub mod gdsyscfg47a22be1;
#[path="../registers/gdtimer0d0a204ab_v1.rs"] pub mod gdtimer0d0a204ab;
#[path="../registers/gdtimer14452fee2b_v1.rs"] pub mod gdtimer14452fee2b;
#[path="../registers/gdtimer180e32204_v1.rs"] pub mod gdtimer180e32204;
#[path="../registers/gdtimer20f28b429_v1.rs"] pub mod gdtimer20f28b429;
#[path="../registers/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../registers/gdtimer6b465bb6b_v1.rs"] pub mod gdtimer6b465bb6b;
#[path="../registers/gdtimer8dfb2bdb4_v1.rs"] pub mod gdtimer8dfb2bdb4;
#[path="../registers/gdtrngbf61c352_v1.rs"] pub mod gdtrngbf61c352;
#[path="../registers/gduart37add471e_v1.rs"] pub mod gduart37add471e;
#[path="../registers/gduart47d66af8a_v1.rs"] pub mod gduart47d66af8a;
#[path="../registers/gdusart0184abb20_v1.rs"] pub mod gdusart0184abb20;
#[path="../registers/gdusbd3c6a50b5_v1.rs"] pub mod gdusbd3c6a50b5;
#[path="../registers/gdvreff6814bb8_v1.rs"] pub mod gdvreff6814bb8;
#[path="../registers/gdwwdgtdd622579_v1.rs"] pub mod gdwwdgtdd622579;
