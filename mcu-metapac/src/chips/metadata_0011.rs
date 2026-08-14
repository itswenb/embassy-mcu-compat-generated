
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc517245d2",
                version: "v1",
                block: "ADC",
                ir: &gdadc517245d2::REGISTERS,
            },
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
                kind: "gdcmpf9aa0875",
                version: "v1",
                block: "CMP",
                ir: &gdcmpf9aa0875::REGISTERS,
            },
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
                kind: "gddbg1b652252",
                version: "v1",
                block: "DBG",
                ir: &gddbg1b652252::REGISTERS,
            },
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
                kind: "gddma0d635cd0",
                version: "v1",
                block: "DMA",
                ir: &gddma0d635cd0::REGISTERS,
            },
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
                kind: "gddmachxcntbase50b58da3",
                version: "v1",
                block: "DMA_CHXCNT_BASE",
                ir: &gddmachxcntbase50b58da3::REGISTERS,
            },
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
                kind: "gddmachxctlbase49e16ead",
                version: "v1",
                block: "DMA_CHXCTL_BASE",
                ir: &gddmachxctlbase49e16ead::REGISTERS,
            },
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
                kind: "gddmachxmaddrbase0f5bcd0c",
                version: "v1",
                block: "DMA_CHXMADDR_BASE",
                ir: &gddmachxmaddrbase0f5bcd0c::REGISTERS,
            },
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
                kind: "gddmachxpaddrbasec13afd42",
                version: "v1",
                block: "DMA_CHXPADDR_BASE",
                ir: &gddmachxpaddrbasec13afd42::REGISTERS,
            },
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
                kind: "gdexti59df8d27",
                version: "v1",
                block: "EXTI",
                ir: &gdexti59df8d27::REGISTERS,
            },
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
                kind: "gdfmc95e51906",
                version: "v1",
                block: "FMC",
                ir: &gdfmc95e51906::REGISTERS,
            },
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
                kind: "gdi2c2414824a",
                version: "v1",
                block: "I2C",
                ir: &gdi2c2414824a::REGISTERS,
            },
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
                kind: "gdi2c2414824a",
                version: "v1",
                block: "I2C",
                ir: &gdi2c2414824a::REGISTERS,
            },
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
                kind: "gdobd8b8edf3",
                version: "v1",
                block: "OB",
                ir: &gdobd8b8edf3::REGISTERS,
            },
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
                kind: "gdpmu9ff5b6df",
                version: "v1",
                block: "PMU",
                ir: &gdpmu9ff5b6df::REGISTERS,
            },
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
                kind: "gdrcucca4dd31",
                version: "v1",
                block: "RCU",
                ir: &gdrcucca4dd31::REGISTERS,
            },
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
                kind: "gdspic7bc14a6",
                version: "v1",
                block: "SPI",
                ir: &gdspic7bc14a6::REGISTERS,
            },
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
                kind: "gdspic7bc14a6",
                version: "v1",
                block: "SPI",
                ir: &gdspic7bc14a6::REGISTERS,
            },
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
                kind: "gdsyscfg8890ad57",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg8890ad57::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
                kind: "gdtimer7ebd44eb",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer7ebd44eb::REGISTERS,
            },
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
        name: "TIMER2",
        number: 16,
    },
    Interrupt {
        name: "TIMER5",
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
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 34,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc517245d2_v1.rs"] pub mod gdadc517245d2;
#[path="../registers/gdcmpf9aa0875_v1.rs"] pub mod gdcmpf9aa0875;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gddbg1b652252_v1.rs"] pub mod gddbg1b652252;
#[path="../registers/gddma0d635cd0_v1.rs"] pub mod gddma0d635cd0;
#[path="../registers/gddmachxcntbase50b58da3_v1.rs"] pub mod gddmachxcntbase50b58da3;
#[path="../registers/gddmachxctlbase49e16ead_v1.rs"] pub mod gddmachxctlbase49e16ead;
#[path="../registers/gddmachxmaddrbase0f5bcd0c_v1.rs"] pub mod gddmachxmaddrbase0f5bcd0c;
#[path="../registers/gddmachxpaddrbasec13afd42_v1.rs"] pub mod gddmachxpaddrbasec13afd42;
#[path="../registers/gdexti59df8d27_v1.rs"] pub mod gdexti59df8d27;
#[path="../registers/gdfmc95e51906_v1.rs"] pub mod gdfmc95e51906;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../registers/gdi2c2414824a_v1.rs"] pub mod gdi2c2414824a;
#[path="../registers/gdobd8b8edf3_v1.rs"] pub mod gdobd8b8edf3;
#[path="../registers/gdpmu9ff5b6df_v1.rs"] pub mod gdpmu9ff5b6df;
#[path="../registers/gdrcucca4dd31_v1.rs"] pub mod gdrcucca4dd31;
#[path="../registers/gdrtc218478ea_v1.rs"] pub mod gdrtc218478ea;
#[path="../registers/gdspic7bc14a6_v1.rs"] pub mod gdspic7bc14a6;
#[path="../registers/gdsyscfg8890ad57_v1.rs"] pub mod gdsyscfg8890ad57;
#[path="../registers/gdtimer7ebd44eb_v1.rs"] pub mod gdtimer7ebd44eb;
#[path="../registers/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
