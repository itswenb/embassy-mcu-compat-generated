
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc01210ae24",
                version: "v1",
                block: "ADC0",
                ir: &gdadc01210ae24::REGISTERS,
            },
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
        address: 0x50000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc7aed6f31",
                version: "v1",
                block: "ADC",
                ir: &gdadc7aed6f31::REGISTERS,
            },
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
        address: 0x50000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc7aed6f31",
                version: "v1",
                block: "ADC",
                ir: &gdadc7aed6f31::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0x50000c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc7aed6f31",
                version: "v1",
                block: "ADC",
                ir: &gdadc7aed6f31::REGISTERS,
            },
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
        address: 0x4001a000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan22e45210",
                version: "v1",
                block: "CAN",
                ir: &gdcan22e45210::REGISTERS,
            },
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
        address: 0x4001b000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan22e45210",
                version: "v1",
                block: "CAN",
                ir: &gdcan22e45210::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN2",
        address: 0x4001c000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcan22e45210",
                version: "v1",
                block: "CAN",
                ir: &gdcan22e45210::REGISTERS,
            },
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
        address: 0x48021000,
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
        name: "CLA",
        address: 0x40038000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdclaa852113c",
                version: "v1",
                block: "CLA",
                ir: &gdclaa852113c::REGISTERS,
            },
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
                kind: "gdcmp4263a684",
                version: "v1",
                block: "CMP",
                ir: &gdcmp4263a684::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM",
        address: 0x48022800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcpdm49df1052",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm49df1052::REGISTERS,
            },
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
        name: "DAC0",
        address: 0x50001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac4a2b738c",
                version: "v1",
                block: "DAC",
                ir: &gddac4a2b738c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x50001400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac4a2b738c",
                version: "v1",
                block: "DAC",
                ir: &gddac4a2b738c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC2",
        address: 0x50001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac4a2b738c",
                version: "v1",
                block: "DAC",
                ir: &gddac4a2b738c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC3",
        address: 0x50001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac4a2b738c",
                version: "v1",
                block: "DAC",
                ir: &gddac4a2b738c::REGISTERS,
            },
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
                kind: "gddbge900170a",
                version: "v1",
                block: "DBG",
                ir: &gddbge900170a::REGISTERS,
            },
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
                kind: "gddmamuxd5ba02be",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamuxd5ba02be::REGISTERS,
            },
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
                kind: "gddmamuxrmchxcfgbase2ebc8a75",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbase2ebc8a75::REGISTERS,
            },
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
                kind: "gdexmc6ac29ae0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc6ac29ae0::REGISTERS,
            },
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
                kind: "gdexti8937c1c8",
                version: "v1",
                block: "EXTI",
                ir: &gdexti8937c1c8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FAC",
        address: 0x48024800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfac70f604b5",
                version: "v1",
                block: "FAC",
                ir: &gdfac70f604b5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FFT",
        address: 0x40025000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfftc3717816",
                version: "v1",
                block: "FFT",
                ir: &gdfftc3717816::REGISTERS,
            },
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
                kind: "gdfmc84e2d009",
                version: "v1",
                block: "FMC",
                ir: &gdfmc84e2d009::REGISTERS,
            },
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
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
        address: 0x48001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOG",
        address: 0x48001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio2c42bb33",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio2c42bb33::REGISTERS,
            },
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
        address: 0x40017000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfdafb56e7",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfdafb56e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HPDF_FLT0",
        address: 0x40017100,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfdafb56e7",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfdafb56e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HPDF_FLT1",
        address: 0x40017180,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfdafb56e7",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfdafb56e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HPDF_FLT2",
        address: 0x40017200,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfdafb56e7",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfdafb56e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HPDF_FLT3",
        address: 0x40017280,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhpdfdafb56e7",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdfdafb56e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER0",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerc7445402",
                version: "v1",
                block: "HRTIMER",
                ir: &gdhrtimerc7445402::REGISTERS,
            },
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
        name: "I2C3",
        address: 0x40005c00,
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
                kind: "gdlptimer265af638",
                version: "v1",
                block: "LPTIMER",
                ir: &gdlptimer265af638::REGISTERS,
            },
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
                kind: "gdpmu1dc33268",
                version: "v1",
                block: "PMU",
                ir: &gdpmu1dc33268::REGISTERS,
            },
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
        address: 0xa0001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdqspi6bd3ed8c",
                version: "v1",
                block: "QSPI",
                ir: &gdqspi6bd3ed8c::REGISTERS,
            },
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
                kind: "gdrcu8f196476",
                version: "v1",
                block: "RCU",
                ir: &gdrcu8f196476::REGISTERS,
            },
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
                kind: "gdrtcd5d8d4bc",
                version: "v1",
                block: "RTC",
                ir: &gdrtcd5d8d4bc::REGISTERS,
            },
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
                kind: "gdspiea8c377b",
                version: "v1",
                block: "SPI",
                ir: &gdspiea8c377b::REGISTERS,
            },
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
                kind: "gdspiea8c377b",
                version: "v1",
                block: "SPI",
                ir: &gdspiea8c377b::REGISTERS,
            },
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
                kind: "gdspiea8c377b",
                version: "v1",
                block: "SPI",
                ir: &gdspiea8c377b::REGISTERS,
            },
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
                kind: "gdsyscfgee183683",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgee183683::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER19",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
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
                kind: "gdtimerd311b1a2",
                version: "v1",
                block: "TIMER",
                ir: &gdtimerd311b1a2::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TMU",
        address: 0x48024400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtmu6e5ec85c",
                version: "v1",
                block: "TMU",
                ir: &gdtmu6e5ec85c::REGISTERS,
            },
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
        address: 0x40018400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrigsel37febbbf",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigsel37febbbf::REGISTERS,
            },
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
        address: 0x48021800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrng03d0dc9e",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng03d0dc9e::REGISTERS,
            },
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
                kind: "gdusartc72580ea",
                version: "v1",
                block: "USART",
                ir: &gdusartc72580ea::REGISTERS,
            },
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
                kind: "gdusartc72580ea",
                version: "v1",
                block: "USART",
                ir: &gdusartc72580ea::REGISTERS,
            },
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
                kind: "gdusartc72580ea",
                version: "v1",
                block: "USART",
                ir: &gdusartc72580ea::REGISTERS,
            },
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
                kind: "gdusartc72580ea",
                version: "v1",
                block: "USART",
                ir: &gdusartc72580ea::REGISTERS,
            },
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
                kind: "gdusartc72580ea",
                version: "v1",
                block: "USART",
                ir: &gdusartc72580ea::REGISTERS,
            },
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
        address: 0x40017800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdvref779f5a9e",
                version: "v1",
                block: "VREF",
                ir: &gdvref779f5a9e::REGISTERS,
            },
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
        name: "LVD_VAVD_VOVD_VUVD",
        number: 1,
    },
    Interrupt {
        name: "TAMPER",
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
        name: "ADC0_1",
        number: 18,
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
        name: "TIMER0_TRG_CMT_IDX",
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
        name: "I2C0_EV_WKUP",
        number: 31,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_EV_WKUP",
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
        name: "TIMER7_BRK_TRS_IDX",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TRG_CMT_IDX",
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
        name: "SYSCFG",
        number: 48,
    },
    Interrupt {
        name: "LPTIMER",
        number: 49,
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
        name: "TIMER5_DAC0_2",
        number: 54,
    },
    Interrupt {
        name: "TIMER6_DAC1_3",
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
        name: "ADC3",
        number: 61,
    },
    Interrupt {
        name: "VUVD1_VOVD1",
        number: 63,
    },
    Interrupt {
        name: "CMP0_3",
        number: 64,
    },
    Interrupt {
        name: "CMP4_7",
        number: 65,
    },
    Interrupt {
        name: "CMP",
        number: 66,
    },
    Interrupt {
        name: "HRTIMER_IRQ0",
        number: 67,
    },
    Interrupt {
        name: "HRTIMER_IRQ1",
        number: 68,
    },
    Interrupt {
        name: "HRTIMER_IRQ2",
        number: 69,
    },
    Interrupt {
        name: "HRTIMER_IRQ3",
        number: 70,
    },
    Interrupt {
        name: "HRTIMER_IRQ4",
        number: 71,
    },
    Interrupt {
        name: "HRTIMER_IRQ5",
        number: 72,
    },
    Interrupt {
        name: "HRTIMER_IRQ6",
        number: 73,
    },
    Interrupt {
        name: "HRTIMER_IRQ7",
        number: 74,
    },
    Interrupt {
        name: "HRTIMER_IRQ8",
        number: 75,
    },
    Interrupt {
        name: "HRTIMER_IRQ9",
        number: 76,
    },
    Interrupt {
        name: "TIMER19_BRK_TRS_IDX",
        number: 77,
    },
    Interrupt {
        name: "TIMER19_UP",
        number: 78,
    },
    Interrupt {
        name: "TIMER19_TRG_CMT_IDX",
        number: 79,
    },
    Interrupt {
        name: "TIMER19_CHANNEL",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "I2C2_EV_WKUP",
        number: 82,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 83,
    },
    Interrupt {
        name: "CAU",
        number: 85,
    },
    Interrupt {
        name: "TRNG",
        number: 90,
    },
    Interrupt {
        name: "I2C3_EV_WKUP",
        number: 92,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 93,
    },
    Interrupt {
        name: "DMAMUX_OVR",
        number: 94,
    },
    Interrupt {
        name: "QSPI",
        number: 95,
    },
    Interrupt {
        name: "FFT",
        number: 96,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 97,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 98,
    },
    Interrupt {
        name: "CLA",
        number: 99,
    },
    Interrupt {
        name: "TMU",
        number: 100,
    },
    Interrupt {
        name: "FAC",
        number: 101,
    },
    Interrupt {
        name: "HPDF0",
        number: 102,
    },
    Interrupt {
        name: "HPDF1",
        number: 103,
    },
    Interrupt {
        name: "HPDF2",
        number: 104,
    },
    Interrupt {
        name: "HPDF3",
        number: 105,
    },
    Interrupt {
        name: "TIMER14",
        number: 106,
    },
    Interrupt {
        name: "TIMER15",
        number: 107,
    },
    Interrupt {
        name: "TIMER16",
        number: 108,
    },
    Interrupt {
        name: "CAN0_WKUP",
        number: 109,
    },
    Interrupt {
        name: "CAN0_MESSAGE",
        number: 110,
    },
    Interrupt {
        name: "CAN0_BUSOFF",
        number: 111,
    },
    Interrupt {
        name: "CAN0_ERROR",
        number: 112,
    },
    Interrupt {
        name: "CAN0_FASTERROR",
        number: 113,
    },
    Interrupt {
        name: "CAN0_TEC",
        number: 114,
    },
    Interrupt {
        name: "CAN0_REC",
        number: 115,
    },
    Interrupt {
        name: "CAN1_WKUP",
        number: 116,
    },
    Interrupt {
        name: "CAN1_MESSAGE",
        number: 117,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 118,
    },
    Interrupt {
        name: "CAN1_ERROR",
        number: 119,
    },
    Interrupt {
        name: "CAN1_FASTERROR",
        number: 120,
    },
    Interrupt {
        name: "CAN1_TEC",
        number: 121,
    },
    Interrupt {
        name: "CAN1_REC",
        number: 122,
    },
    Interrupt {
        name: "CAN2_WKUP",
        number: 123,
    },
    Interrupt {
        name: "CAN2_MESSAGE",
        number: 124,
    },
    Interrupt {
        name: "CAN2_BUSOFF",
        number: 125,
    },
    Interrupt {
        name: "CAN2_ERROR",
        number: 126,
    },
    Interrupt {
        name: "CAN2_FASTERROR",
        number: 127,
    },
    Interrupt {
        name: "CAN2_TEC",
        number: 128,
    },
    Interrupt {
        name: "CAN2_REC",
        number: 129,
    },
    Interrupt {
        name: "TIMER0_DEC",
        number: 130,
    },
    Interrupt {
        name: "TIMER1_DEC",
        number: 131,
    },
    Interrupt {
        name: "TIMER2_DEC",
        number: 132,
    },
    Interrupt {
        name: "TIMER3_DEC",
        number: 133,
    },
    Interrupt {
        name: "TIMER4_DEC",
        number: 134,
    },
    Interrupt {
        name: "TIMER7_DEC",
        number: 135,
    },
    Interrupt {
        name: "TIMER19_DEC",
        number: 136,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc01210ae24_v1.rs"] pub mod gdadc01210ae24;
#[path="../registers/gdadc7aed6f31_v1.rs"] pub mod gdadc7aed6f31;
#[path="../registers/gdcan22e45210_v1.rs"] pub mod gdcan22e45210;
#[path="../registers/gdcau0732936f_v1.rs"] pub mod gdcau0732936f;
#[path="../registers/gdclaa852113c_v1.rs"] pub mod gdclaa852113c;
#[path="../registers/gdcmp4263a684_v1.rs"] pub mod gdcmp4263a684;
#[path="../registers/gdcpdm49df1052_v1.rs"] pub mod gdcpdm49df1052;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gddac4a2b738c_v1.rs"] pub mod gddac4a2b738c;
#[path="../registers/gddbge900170a_v1.rs"] pub mod gddbge900170a;
#[path="../registers/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../registers/gddmamuxd5ba02be_v1.rs"] pub mod gddmamuxd5ba02be;
#[path="../registers/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../registers/gddmamuxrmchxcfgbase2ebc8a75_v1.rs"] pub mod gddmamuxrmchxcfgbase2ebc8a75;
#[path="../registers/gdexmc6ac29ae0_v1.rs"] pub mod gdexmc6ac29ae0;
#[path="../registers/gdexti8937c1c8_v1.rs"] pub mod gdexti8937c1c8;
#[path="../registers/gdfac70f604b5_v1.rs"] pub mod gdfac70f604b5;
#[path="../registers/gdfftc3717816_v1.rs"] pub mod gdfftc3717816;
#[path="../registers/gdfmc84e2d009_v1.rs"] pub mod gdfmc84e2d009;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpio2c42bb33_v1.rs"] pub mod gdgpio2c42bb33;
#[path="../registers/gdhpdfdafb56e7_v1.rs"] pub mod gdhpdfdafb56e7;
#[path="../registers/gdhrtimerc7445402_v1.rs"] pub mod gdhrtimerc7445402;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdlptimer265af638_v1.rs"] pub mod gdlptimer265af638;
#[path="../registers/gdpmu1dc33268_v1.rs"] pub mod gdpmu1dc33268;
#[path="../registers/gdqspi6bd3ed8c_v1.rs"] pub mod gdqspi6bd3ed8c;
#[path="../registers/gdrcu8f196476_v1.rs"] pub mod gdrcu8f196476;
#[path="../registers/gdrtcd5d8d4bc_v1.rs"] pub mod gdrtcd5d8d4bc;
#[path="../registers/gdspiea8c377b_v1.rs"] pub mod gdspiea8c377b;
#[path="../registers/gdsyscfgee183683_v1.rs"] pub mod gdsyscfgee183683;
#[path="../registers/gdtimerd311b1a2_v1.rs"] pub mod gdtimerd311b1a2;
#[path="../registers/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../registers/gdtrigsel37febbbf_v1.rs"] pub mod gdtrigsel37febbbf;
#[path="../registers/gdtrng03d0dc9e_v1.rs"] pub mod gdtrng03d0dc9e;
#[path="../registers/gdusartc72580ea_v1.rs"] pub mod gdusartc72580ea;
#[path="../registers/gdvref779f5a9e_v1.rs"] pub mod gdvref779f5a9e;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
