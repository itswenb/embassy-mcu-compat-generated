
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcf7ecbfdb",
                version: "v1",
                block: "ADC",
                ir: &gdadcf7ecbfdb::REGISTERS,
            },
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
                kind: "gdcau0732936f",
                version: "v1",
                block: "CAU",
                ir: &gdcau0732936f::REGISTERS,
            },
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
        name: "CTC",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdctc99079953",
                version: "v1",
                block: "CTC",
                ir: &gdctc99079953::REGISTERS,
            },
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
                kind: "gddac79dc5606",
                version: "v1",
                block: "DAC",
                ir: &gddac79dc5606::REGISTERS,
            },
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
                kind: "gddbg4cf9fb40",
                version: "v1",
                block: "DBG",
                ir: &gddbg4cf9fb40::REGISTERS,
            },
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
        name: "DMAMUX",
        address: 0x40020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddmamuxed8f0489",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamuxed8f0489::REGISTERS,
            },
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
                kind: "gddmamuxrmchxcfgbase89587415",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbase89587415::REGISTERS,
            },
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
                kind: "gdexti2655b085",
                version: "v1",
                block: "EXTI",
                ir: &gdexti2655b085::REGISTERS,
            },
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
                kind: "gdfmc30d3804f",
                version: "v1",
                block: "FMC",
                ir: &gdfmc30d3804f::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
                kind: "gdgpio45754e8d",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio45754e8d::REGISTERS,
            },
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
        name: "I2C2",
        address: 0x4000c000,
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
        name: "LPTIMER",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlptimer213ed3b9",
                version: "v1",
                block: "LPTIMER",
                ir: &gdlptimer213ed3b9::REGISTERS,
            },
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
                kind: "gdlpuart39bfce16",
                version: "v1",
                block: "LPUART",
                ir: &gdlpuart39bfce16::REGISTERS,
            },
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
                kind: "gdob52d5f4ba",
                version: "v1",
                block: "OB",
                ir: &gdob52d5f4ba::REGISTERS,
            },
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
                kind: "gdpmu0e670ce1",
                version: "v1",
                block: "PMU",
                ir: &gdpmu0e670ce1::REGISTERS,
            },
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
                kind: "gdrcue5f64fe8",
                version: "v1",
                block: "RCU",
                ir: &gdrcue5f64fe8::REGISTERS,
            },
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
                kind: "gdrtca0051ad5",
                version: "v1",
                block: "RTC",
                ir: &gdrtca0051ad5::REGISTERS,
            },
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
                kind: "gdslcdf5e2d73f",
                version: "v1",
                block: "SLCD",
                ir: &gdslcdf5e2d73f::REGISTERS,
            },
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
                kind: "gdsyscfgd86e92d4",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgd86e92d4::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtimer3aab94f3",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer3aab94f3::REGISTERS,
            },
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
                kind: "gdtrng13872700",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng13872700::REGISTERS,
            },
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
        name: "UART4",
        address: 0x40005000,
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
        address: 0x40013800,
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
        name: "USART1",
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
        name: "VREF",
        address: 0x40010030,
        registers: Some(
            PeripheralRegisters {
                kind: "gdvrefff788331",
                version: "v1",
                block: "VREF",
                ir: &gdvrefff788331::REGISTERS,
            },
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
        name: "LVD",
        number: 1,
    },
    Interrupt {
        name: "TAMPER_STAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
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
        name: "USBD_HP",
        number: 19,
    },
    Interrupt {
        name: "USBD_LP",
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
        name: "UART3",
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
        name: "EXTI10_15",
        number: 47,
    },
    Interrupt {
        name: "DMAMUX",
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
        name: "I2C0_WKUP",
        number: 58,
    },
    Interrupt {
        name: "I2C2_WKUP",
        number: 59,
    },
    Interrupt {
        name: "USART0_WKUP",
        number: 60,
    },
    Interrupt {
        name: "LPUART",
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
        name: "USART1_WKUP",
        number: 65,
    },
    Interrupt {
        name: "I2C1_WKUP",
        number: 66,
    },
    Interrupt {
        name: "LPUART_WKUP",
        number: 67,
    },
    Interrupt {
        name: "LPTIMER",
        number: 68,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadcf7ecbfdb_v1.rs"] pub mod gdadcf7ecbfdb;
#[path="../registers/gdcau0732936f_v1.rs"] pub mod gdcau0732936f;
#[path="../registers/gdcmp13366a93_v1.rs"] pub mod gdcmp13366a93;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gdctc99079953_v1.rs"] pub mod gdctc99079953;
#[path="../registers/gddac79dc5606_v1.rs"] pub mod gddac79dc5606;
#[path="../registers/gddbg4cf9fb40_v1.rs"] pub mod gddbg4cf9fb40;
#[path="../registers/gddma203b2e8a_v1.rs"] pub mod gddma203b2e8a;
#[path="../registers/gddmachxcntbased390cdb4_v1.rs"] pub mod gddmachxcntbased390cdb4;
#[path="../registers/gddmachxctlbase9fc231ae_v1.rs"] pub mod gddmachxctlbase9fc231ae;
#[path="../registers/gddmachxmaddrbase53fbca93_v1.rs"] pub mod gddmachxmaddrbase53fbca93;
#[path="../registers/gddmachxpaddrbase24a24737_v1.rs"] pub mod gddmachxpaddrbase24a24737;
#[path="../registers/gddmamuxed8f0489_v1.rs"] pub mod gddmamuxed8f0489;
#[path="../registers/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../registers/gddmamuxrmchxcfgbase89587415_v1.rs"] pub mod gddmamuxrmchxcfgbase89587415;
#[path="../registers/gdexti2655b085_v1.rs"] pub mod gdexti2655b085;
#[path="../registers/gdfmc30d3804f_v1.rs"] pub mod gdfmc30d3804f;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdlptimer213ed3b9_v1.rs"] pub mod gdlptimer213ed3b9;
#[path="../registers/gdlpuart39bfce16_v1.rs"] pub mod gdlpuart39bfce16;
#[path="../registers/gdob52d5f4ba_v1.rs"] pub mod gdob52d5f4ba;
#[path="../registers/gdpmu0e670ce1_v1.rs"] pub mod gdpmu0e670ce1;
#[path="../registers/gdrcue5f64fe8_v1.rs"] pub mod gdrcue5f64fe8;
#[path="../registers/gdrtca0051ad5_v1.rs"] pub mod gdrtca0051ad5;
#[path="../registers/gdslcdf5e2d73f_v1.rs"] pub mod gdslcdf5e2d73f;
#[path="../registers/gdspi3e72f252_v1.rs"] pub mod gdspi3e72f252;
#[path="../registers/gdsyscfgd86e92d4_v1.rs"] pub mod gdsyscfgd86e92d4;
#[path="../registers/gdtimer3aab94f3_v1.rs"] pub mod gdtimer3aab94f3;
#[path="../registers/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../registers/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../registers/gdvrefff788331_v1.rs"] pub mod gdvrefff788331;
#[path="../registers/gdwwdgtf694703e_v1.rs"] pub mod gdwwdgtf694703e;
