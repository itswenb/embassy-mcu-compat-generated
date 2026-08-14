
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc35fc0029",
                version: "v1",
                block: "ADC",
                ir: &gdadc35fc0029::REGISTERS,
            },
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
                kind: "gdcauf29b21d6",
                version: "v1",
                block: "CAU",
                ir: &gdcauf29b21d6::REGISTERS,
            },
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
                kind: "gddbg9afe2c92",
                version: "v1",
                block: "DBG",
                ir: &gddbg9afe2c92::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DCI",
        address: 0x4c050000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddci6728f4f7",
                version: "v1",
                block: "DCI",
                ir: &gddci6728f4f7::REGISTERS,
            },
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
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma03e09269",
                version: "v1",
                block: "DMA",
                ir: &gddma03e09269::REGISTERS,
            },
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
        address: 0x40026400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma03e09269",
                version: "v1",
                block: "DMA",
                ir: &gddma03e09269::REGISTERS,
            },
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
                kind: "gdefusef9100cc2",
                version: "v1",
                block: "EFUSE",
                ir: &gdefusef9100cc2::REGISTERS,
            },
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
                kind: "gdexti42cdb862",
                version: "v1",
                block: "EXTI",
                ir: &gdexti42cdb862::REGISTERS,
            },
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
                kind: "gdfmc21870ef0",
                version: "v1",
                block: "FMC",
                ir: &gdfmc21870ef0::REGISTERS,
            },
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
                kind: "gdgpio5d0b827e",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio5d0b827e::REGISTERS,
            },
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
                kind: "gdgpio5d0b827e",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio5d0b827e::REGISTERS,
            },
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
                kind: "gdgpio5d0b827e",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio5d0b827e::REGISTERS,
            },
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
        name: "HPDF",
        address: 0x40016000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfc666e7e5",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfc666e7e5::REGISTERS,
            },
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
        name: "I2S1_ADD",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2s32f828a0",
                version: "v1",
                block: "I2S",
                ir: &gdi2s32f828a0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ICACHE",
        address: 0x40080000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdicache04a9739a",
                version: "v1",
                block: "ICACHE",
                ir: &gdicache04a9739a::REGISTERS,
            },
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
                kind: "gdpkcau5848bf43",
                version: "v1",
                block: "PKCAU",
                ir: &gdpkcau5848bf43::REGISTERS,
            },
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
                kind: "gdpmufda9360e",
                version: "v1",
                block: "PMU",
                ir: &gdpmufda9360e::REGISTERS,
            },
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
                kind: "gdqspib6e42f6d",
                version: "v1",
                block: "QSPI",
                ir: &gdqspib6e42f6d::REGISTERS,
            },
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
                kind: "gdrcu3ec51a72",
                version: "v1",
                block: "RCU",
                ir: &gdrcu3ec51a72::REGISTERS,
            },
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
                kind: "gdrtcc5f2b32e",
                version: "v1",
                block: "RTC",
                ir: &gdrtcc5f2b32e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SDIO",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsdioa16a5588",
                version: "v1",
                block: "SDIO",
                ir: &gdsdioa16a5588::REGISTERS,
            },
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
                kind: "gdspi1c2f4e1e",
                version: "v1",
                block: "SPI",
                ir: &gdspi1c2f4e1e::REGISTERS,
            },
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
                kind: "gdspi1c2f4e1e",
                version: "v1",
                block: "SPI",
                ir: &gdspi1c2f4e1e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SQPI",
        address: 0x40025400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsqpi47688f21",
                version: "v1",
                block: "SQPI",
                ir: &gdsqpi47688f21::REGISTERS,
            },
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
                kind: "gdsyscfg24aecb07",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg24aecb07::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
                kind: "gdtimera05861d6",
                version: "v1",
                block: "TIMER",
                ir: &gdtimera05861d6::REGISTERS,
            },
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
        name: "TSI",
        address: 0x40024000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtsi75cc2319",
                version: "v1",
                block: "TSI",
                ir: &gdtsi75cc2319::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZBMPC0",
        address: 0x400a0800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtzbmpcafecfd82",
                version: "v1",
                block: "TZBMPC",
                ir: &gdtzbmpcafecfd82::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZBMPC1",
        address: 0x400a0c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtzbmpcafecfd82",
                version: "v1",
                block: "TZBMPC",
                ir: &gdtzbmpcafecfd82::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZBMPC2",
        address: 0x400b0000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtzbmpcf4d2e8f3",
                version: "v1",
                block: "TZBMPC",
                ir: &gdtzbmpcf4d2e8f3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZBMPC3",
        address: 0x400b0400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtzbmpcfe6b3775",
                version: "v1",
                block: "TZBMPC",
                ir: &gdtzbmpcfe6b3775::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZIAC",
        address: 0x400a0400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtziac6b01a7ab",
                version: "v1",
                block: "TZIAC",
                ir: &gdtziac6b01a7ab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TZSPC",
        address: 0x400a0000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtzspcada29d50",
                version: "v1",
                block: "TZSPC",
                ir: &gdtzspcada29d50::REGISTERS,
            },
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
        name: "USART2",
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
        name: "DMA0_CHANNEL7",
        number: 18,
    },
    Interrupt {
        name: "ADC",
        number: 19,
    },
    Interrupt {
        name: "TAMPER_STAMP_S",
        number: 20,
    },
    Interrupt {
        name: "RTC_WKUP_S",
        number: 21,
    },
    Interrupt {
        name: "RTC_ALARM_S",
        number: 22,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 23,
    },
    Interrupt {
        name: "TIMER0_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_CMT",
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
        name: "EXTI10_15",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "VLVDF",
        number: 42,
    },
    Interrupt {
        name: "TIMER15",
        number: 44,
    },
    Interrupt {
        name: "TIMER16",
        number: 45,
    },
    Interrupt {
        name: "SDIO",
        number: 49,
    },
    Interrupt {
        name: "TIMER4",
        number: 50,
    },
    Interrupt {
        name: "I2C0_WKUP",
        number: 51,
    },
    Interrupt {
        name: "USART0_WKUP",
        number: 52,
    },
    Interrupt {
        name: "USART2_WKUP",
        number: 53,
    },
    Interrupt {
        name: "TIMER5",
        number: 54,
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
        name: "DMA1_CHANNEL5",
        number: 61,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 62,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 63,
    },
    Interrupt {
        name: "WIFI11N_WKUP",
        number: 66,
    },
    Interrupt {
        name: "USBFS",
        number: 67,
    },
    Interrupt {
        name: "USBFS_WKUP",
        number: 76,
    },
    Interrupt {
        name: "DCI",
        number: 78,
    },
    Interrupt {
        name: "CAU",
        number: 79,
    },
    Interrupt {
        name: "HAU_TRNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "HPDF_INT0",
        number: 89,
    },
    Interrupt {
        name: "HPDF_INT1",
        number: 90,
    },
    Interrupt {
        name: "WIFI11N_INT0",
        number: 91,
    },
    Interrupt {
        name: "WIFI11N_INT1",
        number: 92,
    },
    Interrupt {
        name: "WIFI11N_INT2",
        number: 93,
    },
    Interrupt {
        name: "EFUSE",
        number: 94,
    },
    Interrupt {
        name: "QSPI",
        number: 95,
    },
    Interrupt {
        name: "PKCAU",
        number: 96,
    },
    Interrupt {
        name: "TSI",
        number: 97,
    },
    Interrupt {
        name: "ICACHE",
        number: 98,
    },
    Interrupt {
        name: "TZIAC_S",
        number: 99,
    },
    Interrupt {
        name: "FMC_S",
        number: 100,
    },
    Interrupt {
        name: "QSPI_S",
        number: 101,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc35fc0029_v1.rs"] pub mod gdadc35fc0029;
#[path="../registers/gdcauf29b21d6_v1.rs"] pub mod gdcauf29b21d6;
#[path="../registers/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../registers/gddbg9afe2c92_v1.rs"] pub mod gddbg9afe2c92;
#[path="../registers/gddci6728f4f7_v1.rs"] pub mod gddci6728f4f7;
#[path="../registers/gddma03e09269_v1.rs"] pub mod gddma03e09269;
#[path="../registers/gdefusef9100cc2_v1.rs"] pub mod gdefusef9100cc2;
#[path="../registers/gdexti42cdb862_v1.rs"] pub mod gdexti42cdb862;
#[path="../registers/gdfmc21870ef0_v1.rs"] pub mod gdfmc21870ef0;
#[path="../registers/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../registers/gdgpio5d0b827e_v1.rs"] pub mod gdgpio5d0b827e;
#[path="../registers/gdhaub97c00c8_v1.rs"] pub mod gdhaub97c00c8;
#[path="../registers/gdhpdfc666e7e5_v1.rs"] pub mod gdhpdfc666e7e5;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../registers/gdicache04a9739a_v1.rs"] pub mod gdicache04a9739a;
#[path="../registers/gdpkcau5848bf43_v1.rs"] pub mod gdpkcau5848bf43;
#[path="../registers/gdpmufda9360e_v1.rs"] pub mod gdpmufda9360e;
#[path="../registers/gdqspib6e42f6d_v1.rs"] pub mod gdqspib6e42f6d;
#[path="../registers/gdrcu3ec51a72_v1.rs"] pub mod gdrcu3ec51a72;
#[path="../registers/gdrtcc5f2b32e_v1.rs"] pub mod gdrtcc5f2b32e;
#[path="../registers/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../registers/gdspi1c2f4e1e_v1.rs"] pub mod gdspi1c2f4e1e;
#[path="../registers/gdsqpi47688f21_v1.rs"] pub mod gdsqpi47688f21;
#[path="../registers/gdsyscfg24aecb07_v1.rs"] pub mod gdsyscfg24aecb07;
#[path="../registers/gdtimera05861d6_v1.rs"] pub mod gdtimera05861d6;
#[path="../registers/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../registers/gdtsi75cc2319_v1.rs"] pub mod gdtsi75cc2319;
#[path="../registers/gdtzbmpcafecfd82_v1.rs"] pub mod gdtzbmpcafecfd82;
#[path="../registers/gdtzbmpcf4d2e8f3_v1.rs"] pub mod gdtzbmpcf4d2e8f3;
#[path="../registers/gdtzbmpcfe6b3775_v1.rs"] pub mod gdtzbmpcfe6b3775;
#[path="../registers/gdtziac6b01a7ab_v1.rs"] pub mod gdtziac6b01a7ab;
#[path="../registers/gdtzspcada29d50_v1.rs"] pub mod gdtzspcada29d50;
#[path="../registers/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
