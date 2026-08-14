
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcc334f3cc",
                version: "v1",
                block: "ADC",
                ir: &gdadcc334f3cc::REGISTERS,
            },
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
                kind: "gdadcc334f3cc",
                version: "v1",
                block: "ADC",
                ir: &gdadcc334f3cc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x40013c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcc334f3cc",
                version: "v1",
                block: "ADC",
                ir: &gdadcc334f3cc::REGISTERS,
            },
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
                kind: "gdafiocfb569a7",
                version: "v1",
                block: "AFIO",
                ir: &gdafiocfb569a7::REGISTERS,
            },
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
                kind: "gdbkpddaa24e5",
                version: "v1",
                block: "BKP",
                ir: &gdbkpddaa24e5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN0",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan09590032f",
                version: "v1",
                block: "CAN0",
                ir: &gdcan09590032f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN1",
        address: 0x40015c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan8ce81596",
                version: "v1",
                block: "CAN",
                ir: &gdcan8ce81596::REGISTERS,
            },
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
        address: 0x40023400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcauc12c326c",
                version: "v1",
                block: "CAU",
                ir: &gdcauc12c326c::REGISTERS,
            },
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
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmp70d4db57",
                version: "v1",
                block: "CMP",
                ir: &gdcmp70d4db57::REGISTERS,
            },
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
        address: 0xe0045000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbgc321d162",
                version: "v1",
                block: "DBG",
                ir: &gddbgc321d162::REGISTERS,
            },
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
                kind: "gddmae208530b",
                version: "v1",
                block: "DMA",
                ir: &gddmae208530b::REGISTERS,
            },
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
                kind: "gddmae208530b",
                version: "v1",
                block: "DMA",
                ir: &gddmae208530b::REGISTERS,
            },
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
                kind: "gddmamuxcd4c69ea",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamuxcd4c69ea::REGISTERS,
            },
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
                kind: "gddmamuxrgchxcfgbased84fcfa6",
                version: "v1",
                block: "DMAMUX_RG_CHXCFG_BASE",
                ir: &gddmamuxrgchxcfgbased84fcfa6::REGISTERS,
            },
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
                kind: "gddmamuxrmchxcfgbasef37c083c",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbasef37c083c::REGISTERS,
            },
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
                kind: "gdexmcf139886b",
                version: "v1",
                block: "EXMC",
                ir: &gdexmcf139886b::REGISTERS,
            },
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
                kind: "gdextia39263ea",
                version: "v1",
                block: "EXTI",
                ir: &gdextia39263ea::REGISTERS,
            },
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
                kind: "gdfmc71a20e5f",
                version: "v1",
                block: "FMC",
                ir: &gdfmc71a20e5f::REGISTERS,
            },
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
        address: 0x40010800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpiob9f95038",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiob9f95038::REGISTERS,
            },
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
                kind: "gdgpiob9f95038",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiob9f95038::REGISTERS,
            },
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
                kind: "gdgpiob9f95038",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiob9f95038::REGISTERS,
            },
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
                kind: "gdgpiob9f95038",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiob9f95038::REGISTERS,
            },
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
                kind: "gdgpiob9f95038",
                version: "v1",
                block: "GPIO",
                ir: &gdgpiob9f95038::REGISTERS,
            },
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
        address: 0x40023800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhau6f90c013",
                version: "v1",
                block: "HAU",
                ir: &gdhau6f90c013::REGISTERS,
            },
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
                kind: "gdi2cecd631a5",
                version: "v1",
                block: "I2C",
                ir: &gdi2cecd631a5::REGISTERS,
            },
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
                kind: "gdi2cecd631a5",
                version: "v1",
                block: "I2C",
                ir: &gdi2cecd631a5::REGISTERS,
            },
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
                kind: "gdob138fbadf",
                version: "v1",
                block: "OB",
                ir: &gdob138fbadf::REGISTERS,
            },
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
                kind: "gdpmuf6fd2d81",
                version: "v1",
                block: "PMU",
                ir: &gdpmuf6fd2d81::REGISTERS,
            },
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
                kind: "gdrcu97a76383",
                version: "v1",
                block: "RCU",
                ir: &gdrcu97a76383::REGISTERS,
            },
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
                kind: "gdrtc250e9b91",
                version: "v1",
                block: "RTC",
                ir: &gdrtc250e9b91::REGISTERS,
            },
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
                kind: "gdspidfb3af1d",
                version: "v1",
                block: "SPI",
                ir: &gdspidfb3af1d::REGISTERS,
            },
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
                kind: "gdspidfb3af1d",
                version: "v1",
                block: "SPI",
                ir: &gdspidfb3af1d::REGISTERS,
            },
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
                kind: "gdspidfb3af1d",
                version: "v1",
                block: "SPI",
                ir: &gdspidfb3af1d::REGISTERS,
            },
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
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfga124fcf6",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfga124fcf6::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
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
                kind: "gdtimer0a6eba78",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer0a6eba78::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TRIGSEL",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrigsel280e29a3",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigsel280e29a3::REGISTERS,
            },
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
        address: 0x40023c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrng4a6beb42",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng4a6beb42::REGISTERS,
            },
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
                kind: "gdusartd892c3f9",
                version: "v1",
                block: "USART",
                ir: &gdusartd892c3f9::REGISTERS,
            },
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
                kind: "gdusartd892c3f9",
                version: "v1",
                block: "USART",
                ir: &gdusartd892c3f9::REGISTERS,
            },
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
                kind: "gdusartd892c3f9",
                version: "v1",
                block: "USART",
                ir: &gdusartd892c3f9::REGISTERS,
            },
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
                kind: "gdusartd892c3f9",
                version: "v1",
                block: "USART",
                ir: &gdusartd892c3f9::REGISTERS,
            },
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
                kind: "gdusartd892c3f9",
                version: "v1",
                block: "USART",
                ir: &gdusartd892c3f9::REGISTERS,
            },
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
                kind: "gdwwdgt59a14ef4",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt59a14ef4::REGISTERS,
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
        name: "LVD_VAVD",
        number: 1,
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
        name: "ADC0_1",
        number: 18,
    },
    Interrupt {
        name: "CAN0_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN0_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN0_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN0_EWMC",
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
        name: "TIMER0_TRG_CMT",
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
        name: "USBFS_WKUP",
        number: 42,
    },
    Interrupt {
        name: "TIMER7_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TRG_CMT",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CHANNEL",
        number: 46,
    },
    Interrupt {
        name: "ADC2",
        number: 47,
    },
    Interrupt {
        name: "RCU_CKFM",
        number: 49,
    },
    Interrupt {
        name: "CMP_WAKEUP",
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
        name: "DMA1_CHANNEL3",
        number: 59,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 60,
    },
    Interrupt {
        name: "DAC",
        number: 61,
    },
    Interrupt {
        name: "PMU_VUVD_VOVD",
        number: 62,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN1_EWMC",
        number: 66,
    },
    Interrupt {
        name: "SRAM_ECC",
        number: 67,
    },
    Interrupt {
        name: "FPU",
        number: 68,
    },
    Interrupt {
        name: "CMP",
        number: 69,
    },
    Interrupt {
        name: "DMAMUX",
        number: 70,
    },
    Interrupt {
        name: "CAU",
        number: 71,
    },
    Interrupt {
        name: "HAU",
        number: 72,
    },
    Interrupt {
        name: "TRNG",
        number: 73,
    },
    Interrupt {
        name: "USBFS",
        number: 74,
    },
    Interrupt {
        name: "TIMER4",
        number: 75,
    },
    Interrupt {
        name: "TIMER15",
        number: 76,
    },
    Interrupt {
        name: "TIMER16",
        number: 77,
    },
    Interrupt {
        name: "TIMER0_BRK_CHANNEL",
        number: 78,
    },
    Interrupt {
        name: "TIMER7_BRK_CHANNEL",
        number: 79,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadcc334f3cc_v1.rs"] pub mod gdadcc334f3cc;
#[path="../registers/gdafiocfb569a7_v1.rs"] pub mod gdafiocfb569a7;
#[path="../registers/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../registers/gdcan09590032f_v1.rs"] pub mod gdcan09590032f;
#[path="../registers/gdcan8ce81596_v1.rs"] pub mod gdcan8ce81596;
#[path="../registers/gdcauc12c326c_v1.rs"] pub mod gdcauc12c326c;
#[path="../registers/gdcmp70d4db57_v1.rs"] pub mod gdcmp70d4db57;
#[path="../registers/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../registers/gddbgc321d162_v1.rs"] pub mod gddbgc321d162;
#[path="../registers/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../registers/gddmamuxcd4c69ea_v1.rs"] pub mod gddmamuxcd4c69ea;
#[path="../registers/gddmamuxrgchxcfgbased84fcfa6_v1.rs"] pub mod gddmamuxrgchxcfgbased84fcfa6;
#[path="../registers/gddmamuxrmchxcfgbasef37c083c_v1.rs"] pub mod gddmamuxrmchxcfgbasef37c083c;
#[path="../registers/gdexmcf139886b_v1.rs"] pub mod gdexmcf139886b;
#[path="../registers/gdextia39263ea_v1.rs"] pub mod gdextia39263ea;
#[path="../registers/gdfmc71a20e5f_v1.rs"] pub mod gdfmc71a20e5f;
#[path="../registers/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../registers/gdgpiob9f95038_v1.rs"] pub mod gdgpiob9f95038;
#[path="../registers/gdhau6f90c013_v1.rs"] pub mod gdhau6f90c013;
#[path="../registers/gdi2cecd631a5_v1.rs"] pub mod gdi2cecd631a5;
#[path="../registers/gdob138fbadf_v1.rs"] pub mod gdob138fbadf;
#[path="../registers/gdpmuf6fd2d81_v1.rs"] pub mod gdpmuf6fd2d81;
#[path="../registers/gdrcu97a76383_v1.rs"] pub mod gdrcu97a76383;
#[path="../registers/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../registers/gdspidfb3af1d_v1.rs"] pub mod gdspidfb3af1d;
#[path="../registers/gdsyscfga124fcf6_v1.rs"] pub mod gdsyscfga124fcf6;
#[path="../registers/gdtimer0a6eba78_v1.rs"] pub mod gdtimer0a6eba78;
#[path="../registers/gdtrigsel280e29a3_v1.rs"] pub mod gdtrigsel280e29a3;
#[path="../registers/gdtrng4a6beb42_v1.rs"] pub mod gdtrng4a6beb42;
#[path="../registers/gdusartd892c3f9_v1.rs"] pub mod gdusartd892c3f9;
#[path="../registers/gdwwdgt59a14ef4_v1.rs"] pub mod gdwwdgt59a14ef4;
