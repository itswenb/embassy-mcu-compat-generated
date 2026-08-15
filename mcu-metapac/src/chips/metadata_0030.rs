
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc05f794981",
                version: "v1",
                block: "ADC0",
                ir: &gdadc05f794981::REGISTERS,
            },
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
                kind: "gdadc05f794981",
                version: "v1",
                block: "ADC0",
                ir: &gdadc05f794981::REGISTERS,
            },
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
                kind: "gdadc05f794981",
                version: "v1",
                block: "ADC0",
                ir: &gdadc05f794981::REGISTERS,
            },
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
                kind: "gdafio220b0195",
                version: "v1",
                block: "AFIO",
                ir: &gdafio220b0195::REGISTERS,
            },
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
        name: "CAN0",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan0f4404570",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0f4404570::REGISTERS,
            },
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
                kind: "gdcan0f4404570",
                version: "v1",
                block: "CAN0",
                ir: &gdcan0f4404570::REGISTERS,
            },
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
                kind: "gdcaue3998bcd",
                version: "v1",
                block: "CAU",
                ir: &gdcaue3998bcd::REGISTERS,
            },
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
                kind: "gdcmpfc67f344",
                version: "v1",
                block: "CMP",
                ir: &gdcmpfc67f344::REGISTERS,
            },
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
                kind: "gdcrc2255b0ef",
                version: "v1",
                block: "CRC",
                ir: &gdcrc2255b0ef::REGISTERS,
            },
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
                kind: "gdctc0cad8643",
                version: "v1",
                block: "CTC",
                ir: &gdctc0cad8643::REGISTERS,
            },
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
                kind: "gddac4d974090",
                version: "v1",
                block: "DAC",
                ir: &gddac4d974090::REGISTERS,
            },
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
                kind: "gddbg56ea3fe0",
                version: "v1",
                block: "DBG",
                ir: &gddbg56ea3fe0::REGISTERS,
            },
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
                kind: "gddma011392832",
                version: "v1",
                block: "DMA0",
                ir: &gddma011392832::REGISTERS,
            },
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
                kind: "gddma011392832",
                version: "v1",
                block: "DMA0",
                ir: &gddma011392832::REGISTERS,
            },
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
                kind: "gddmamux84b48a9d",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux84b48a9d::REGISTERS,
            },
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
                kind: "gdexmce9c487de",
                version: "v1",
                block: "EXMC",
                ir: &gdexmce9c487de::REGISTERS,
            },
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
                kind: "gdexti11a1be47",
                version: "v1",
                block: "EXTI",
                ir: &gdexti11a1be47::REGISTERS,
            },
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
                kind: "gdfmc32a1a410",
                version: "v1",
                block: "FMC",
                ir: &gdfmc32a1a410::REGISTERS,
            },
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
                kind: "gdgpioa519aa242",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa519aa242::REGISTERS,
            },
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
                kind: "gdgpioa519aa242",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa519aa242::REGISTERS,
            },
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
                kind: "gdgpioa519aa242",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa519aa242::REGISTERS,
            },
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
                kind: "gdgpioa519aa242",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa519aa242::REGISTERS,
            },
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
                kind: "gdgpioa519aa242",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa519aa242::REGISTERS,
            },
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
                kind: "gdhau0e6a9f22",
                version: "v1",
                block: "HAU",
                ir: &gdhau0e6a9f22::REGISTERS,
            },
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
                kind: "gdi2c0932dc70f",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0932dc70f::REGISTERS,
            },
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
                kind: "gdi2c0932dc70f",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0932dc70f::REGISTERS,
            },
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
                kind: "gdpmu82ca625b",
                version: "v1",
                block: "PMU",
                ir: &gdpmu82ca625b::REGISTERS,
            },
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
                kind: "gdrcudff54106",
                version: "v1",
                block: "RCU",
                ir: &gdrcudff54106::REGISTERS,
            },
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
                kind: "gdspi0ad5e2dff",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0ad5e2dff::REGISTERS,
            },
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
                kind: "gdspi0ad5e2dff",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0ad5e2dff::REGISTERS,
            },
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
                kind: "gdspi0ad5e2dff",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0ad5e2dff::REGISTERS,
            },
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
                kind: "gdsyscfg14885d82",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg14885d82::REGISTERS,
            },
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
                kind: "gdtimer0debe3394",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0debe3394::REGISTERS,
            },
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
                kind: "gdtimer12e5cd301",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer12e5cd301::REGISTERS,
            },
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
                kind: "gdtimer15296d56bf",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15296d56bf::REGISTERS,
            },
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
                kind: "gdtimer15296d56bf",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15296d56bf::REGISTERS,
            },
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
                kind: "gdtimer12e5cd301",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer12e5cd301::REGISTERS,
            },
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
                kind: "gdtimer12e5cd301",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer12e5cd301::REGISTERS,
            },
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
                kind: "gdtimer12e5cd301",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer12e5cd301::REGISTERS,
            },
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
                kind: "gdtimer0debe3394",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0debe3394::REGISTERS,
            },
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
                kind: "gdtrigsel7c239a51",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigsel7c239a51::REGISTERS,
            },
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
                kind: "gdtrng8cefada9",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng8cefada9::REGISTERS,
            },
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
                kind: "gduart31f2ca0ee",
                version: "v1",
                block: "UART3",
                ir: &gduart31f2ca0ee::REGISTERS,
            },
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
                kind: "gduart31f2ca0ee",
                version: "v1",
                block: "UART3",
                ir: &gduart31f2ca0ee::REGISTERS,
            },
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
                kind: "gdusart0b635b392",
                version: "v1",
                block: "USART0",
                ir: &gdusart0b635b392::REGISTERS,
            },
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
                kind: "gdusart0b635b392",
                version: "v1",
                block: "USART0",
                ir: &gdusart0b635b392::REGISTERS,
            },
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
                kind: "gdusart0b635b392",
                version: "v1",
                block: "USART0",
                ir: &gdusart0b635b392::REGISTERS,
            },
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
                kind: "gdusbfsdevicec9f07fda",
                version: "v1",
                block: "USBFS_DEVICE",
                ir: &gdusbfsdevicec9f07fda::REGISTERS,
            },
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
                kind: "gdusbfsglobald97a6bbd",
                version: "v1",
                block: "USBFS_GLOBAL",
                ir: &gdusbfsglobald97a6bbd::REGISTERS,
            },
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
                kind: "gdwwdgt5c5bacde",
                version: "v1",
                block: "WWDGT",
                ir: &gdwwdgt5c5bacde::REGISTERS,
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
        name: "ADC1",
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
        name: "EXTI_LINE9_5",
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
        name: "TIMER0_CC",
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
        name: "TIMER7_CC",
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
        name: "VUVD_VOVD",
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
            #[path="../registers/gdadc05f794981_v1.rs"] pub mod gdadc05f794981;
#[path="../registers/gdafio220b0195_v1.rs"] pub mod gdafio220b0195;
#[path="../registers/gdbkp16a620e0_v1.rs"] pub mod gdbkp16a620e0;
#[path="../registers/gdcan0f4404570_v1.rs"] pub mod gdcan0f4404570;
#[path="../registers/gdcaue3998bcd_v1.rs"] pub mod gdcaue3998bcd;
#[path="../registers/gdcmpfc67f344_v1.rs"] pub mod gdcmpfc67f344;
#[path="../registers/gdcrc2255b0ef_v1.rs"] pub mod gdcrc2255b0ef;
#[path="../registers/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../registers/gddac4d974090_v1.rs"] pub mod gddac4d974090;
#[path="../registers/gddbg56ea3fe0_v1.rs"] pub mod gddbg56ea3fe0;
#[path="../registers/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../registers/gddmamux84b48a9d_v1.rs"] pub mod gddmamux84b48a9d;
#[path="../registers/gdexmce9c487de_v1.rs"] pub mod gdexmce9c487de;
#[path="../registers/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../registers/gdfmc32a1a410_v1.rs"] pub mod gdfmc32a1a410;
#[path="../registers/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../registers/gdgpioa519aa242_v1.rs"] pub mod gdgpioa519aa242;
#[path="../registers/gdhau0e6a9f22_v1.rs"] pub mod gdhau0e6a9f22;
#[path="../registers/gdi2c0932dc70f_v1.rs"] pub mod gdi2c0932dc70f;
#[path="../registers/gdpmu82ca625b_v1.rs"] pub mod gdpmu82ca625b;
#[path="../registers/gdrcudff54106_v1.rs"] pub mod gdrcudff54106;
#[path="../registers/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../registers/gdspi0ad5e2dff_v1.rs"] pub mod gdspi0ad5e2dff;
#[path="../registers/gdsyscfg14885d82_v1.rs"] pub mod gdsyscfg14885d82;
#[path="../registers/gdtimer0debe3394_v1.rs"] pub mod gdtimer0debe3394;
#[path="../registers/gdtimer12e5cd301_v1.rs"] pub mod gdtimer12e5cd301;
#[path="../registers/gdtimer15296d56bf_v1.rs"] pub mod gdtimer15296d56bf;
#[path="../registers/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../registers/gdtrigsel7c239a51_v1.rs"] pub mod gdtrigsel7c239a51;
#[path="../registers/gdtrng8cefada9_v1.rs"] pub mod gdtrng8cefada9;
#[path="../registers/gduart31f2ca0ee_v1.rs"] pub mod gduart31f2ca0ee;
#[path="../registers/gdusart0b635b392_v1.rs"] pub mod gdusart0b635b392;
#[path="../registers/gdusbfsdevicec9f07fda_v1.rs"] pub mod gdusbfsdevicec9f07fda;
#[path="../registers/gdusbfsglobald97a6bbd_v1.rs"] pub mod gdusbfsglobald97a6bbd;
#[path="../registers/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../registers/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../registers/gdwwdgt5c5bacde_v1.rs"] pub mod gdwwdgt5c5bacde;
