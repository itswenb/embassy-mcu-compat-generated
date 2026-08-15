
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x50000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc0e44d6214",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0e44d6214::REGISTERS,
            },
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
                kind: "gdadc1d46cf375",
                version: "v1",
                block: "ADC1",
                ir: &gdadc1d46cf375::REGISTERS,
            },
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
                kind: "gdadc2ffb56179",
                version: "v1",
                block: "ADC2",
                ir: &gdadc2ffb56179::REGISTERS,
            },
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
                kind: "gdadc3412cd97d",
                version: "v1",
                block: "ADC3",
                ir: &gdadc3412cd97d::REGISTERS,
            },
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
                kind: "gdcan00d9f65e8",
                version: "v1",
                block: "CAN0",
                ir: &gdcan00d9f65e8::REGISTERS,
            },
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
                kind: "gdcan00d9f65e8",
                version: "v1",
                block: "CAN0",
                ir: &gdcan00d9f65e8::REGISTERS,
            },
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
                kind: "gdcan00d9f65e8",
                version: "v1",
                block: "CAN0",
                ir: &gdcan00d9f65e8::REGISTERS,
            },
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
                kind: "gdcaue3899f2b",
                version: "v1",
                block: "CAU",
                ir: &gdcaue3899f2b::REGISTERS,
            },
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
                kind: "gdclabbe1d8c5",
                version: "v1",
                block: "CLA",
                ir: &gdclabbe1d8c5::REGISTERS,
            },
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
                kind: "gdcmp5553b816",
                version: "v1",
                block: "CMP",
                ir: &gdcmp5553b816::REGISTERS,
            },
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
                kind: "gdcpdm0270b1b1",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm0270b1b1::REGISTERS,
            },
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
                kind: "gdcrccf345d41",
                version: "v1",
                block: "CRC",
                ir: &gdcrccf345d41::REGISTERS,
            },
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
                kind: "gddac07216986f",
                version: "v1",
                block: "DAC0",
                ir: &gddac07216986f::REGISTERS,
            },
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
                kind: "gddac07216986f",
                version: "v1",
                block: "DAC0",
                ir: &gddac07216986f::REGISTERS,
            },
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
                kind: "gddac07216986f",
                version: "v1",
                block: "DAC0",
                ir: &gddac07216986f::REGISTERS,
            },
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
                kind: "gddac07216986f",
                version: "v1",
                block: "DAC0",
                ir: &gddac07216986f::REGISTERS,
            },
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
                kind: "gddbg91aeb23f",
                version: "v1",
                block: "DBG",
                ir: &gddbg91aeb23f::REGISTERS,
            },
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
                kind: "gddma0ff389860",
                version: "v1",
                block: "DMA0",
                ir: &gddma0ff389860::REGISTERS,
            },
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
                kind: "gddma0ff389860",
                version: "v1",
                block: "DMA0",
                ir: &gddma0ff389860::REGISTERS,
            },
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
                kind: "gddmamux1fdc5150",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux1fdc5150::REGISTERS,
            },
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
                kind: "gdexmc15e63ec9",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc15e63ec9::REGISTERS,
            },
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
                kind: "gdextiaf81da6f",
                version: "v1",
                block: "EXTI",
                ir: &gdextiaf81da6f::REGISTERS,
            },
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
                kind: "gdfac8bc94bdf",
                version: "v1",
                block: "FAC",
                ir: &gdfac8bc94bdf::REGISTERS,
            },
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
                kind: "gdfft4a1b8727",
                version: "v1",
                block: "FFT",
                ir: &gdfft4a1b8727::REGISTERS,
            },
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
                kind: "gdfmcd425e751",
                version: "v1",
                block: "FMC",
                ir: &gdfmcd425e751::REGISTERS,
            },
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
                kind: "gdgpioa041672dd",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa041672dd::REGISTERS,
            },
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
                kind: "gdgpiob2a39d6c5",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob2a39d6c5::REGISTERS,
            },
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
                kind: "gdgpioc62f10237",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc62f10237::REGISTERS,
            },
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
                kind: "gdgpioc62f10237",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc62f10237::REGISTERS,
            },
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
                kind: "gdgpioc62f10237",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc62f10237::REGISTERS,
            },
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
                kind: "gdgpioc62f10237",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc62f10237::REGISTERS,
            },
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
                kind: "gdgpioc62f10237",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpioc62f10237::REGISTERS,
            },
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
                kind: "gdhpdf7e0cd818",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdf7e0cd818::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_COMMON",
        address: 0x40015b80,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimercommone8f80cd9",
                version: "v1",
                block: "HRTIMER_COMMON",
                ir: &gdhrtimercommone8f80cd9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_MASTER_TIMER",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimermastertimer46dd88dd",
                version: "v1",
                block: "HRTIMER_MASTER_TIMER",
                ir: &gdhrtimermastertimer46dd88dd::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER0",
        address: 0x40015880,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer067581449",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER0",
                ir: &gdhrtimerslavetimer067581449::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER1",
        address: 0x40015900,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer14d8109aa",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER1",
                ir: &gdhrtimerslavetimer14d8109aa::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER2",
        address: 0x40015980,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer20b2b3389",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER2",
                ir: &gdhrtimerslavetimer20b2b3389::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER3",
        address: 0x40015a00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer34217e6d5",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER3",
                ir: &gdhrtimerslavetimer34217e6d5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER4",
        address: 0x40015a80,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer44f2ae72f",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER4",
                ir: &gdhrtimerslavetimer44f2ae72f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER5",
        address: 0x40015b00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer5093157b0",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER5",
                ir: &gdhrtimerslavetimer5093157b0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER6",
        address: 0x40016000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer66a7539e7",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER6",
                ir: &gdhrtimerslavetimer66a7539e7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HRTIMER_SLAVE_TIMER7",
        address: 0x40016080,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhrtimerslavetimer7b4a9b9e4",
                version: "v1",
                block: "HRTIMER_SLAVE_TIMER7",
                ir: &gdhrtimerslavetimer7b4a9b9e4::REGISTERS,
            },
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
        name: "I2C3",
        address: 0x40005c00,
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
        name: "LPTIMER",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdlptimer1f47bc76",
                version: "v1",
                block: "LPTIMER",
                ir: &gdlptimer1f47bc76::REGISTERS,
            },
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
                kind: "gdpmu38e55ba3",
                version: "v1",
                block: "PMU",
                ir: &gdpmu38e55ba3::REGISTERS,
            },
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
                kind: "gdqspic71428c9",
                version: "v1",
                block: "QSPI",
                ir: &gdqspic71428c9::REGISTERS,
            },
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
                kind: "gdrcu35f5457f",
                version: "v1",
                block: "RCU",
                ir: &gdrcu35f5457f::REGISTERS,
            },
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
                kind: "gdrtca0f4d2cf",
                version: "v1",
                block: "RTC",
                ir: &gdrtca0f4d2cf::REGISTERS,
            },
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
                kind: "gdspi0d1cf2b57",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0d1cf2b57::REGISTERS,
            },
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
                kind: "gdspi16544be1a",
                version: "v1",
                block: "SPI1",
                ir: &gdspi16544be1a::REGISTERS,
            },
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
                kind: "gdspi2112e160c",
                version: "v1",
                block: "SPI2",
                ir: &gdspi2112e160c::REGISTERS,
            },
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
                kind: "gdsyscfgc16069c6",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgc16069c6::REGISTERS,
            },
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
                kind: "gdtimer00fb2a8b3",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer00fb2a8b3::REGISTERS,
            },
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
                kind: "gdtimer103a746fb",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer103a746fb::REGISTERS,
            },
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
                kind: "gdtimer14eb20ecc0",
                version: "v1",
                block: "TIMER14",
                ir: &gdtimer14eb20ecc0::REGISTERS,
            },
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
                kind: "gdtimer15f7745da8",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15f7745da8::REGISTERS,
            },
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
                kind: "gdtimer15f7745da8",
                version: "v1",
                block: "TIMER15",
                ir: &gdtimer15f7745da8::REGISTERS,
            },
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
                kind: "gdtimer00fb2a8b3",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer00fb2a8b3::REGISTERS,
            },
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
                kind: "gdtimer291200e8c",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer291200e8c::REGISTERS,
            },
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
                kind: "gdtimer291200e8c",
                version: "v1",
                block: "TIMER2",
                ir: &gdtimer291200e8c::REGISTERS,
            },
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
                kind: "gdtimer103a746fb",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer103a746fb::REGISTERS,
            },
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
                kind: "gdtimer58fe8734a",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer58fe8734a::REGISTERS,
            },
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
                kind: "gdtimer58fe8734a",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer58fe8734a::REGISTERS,
            },
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
                kind: "gdtimer00fb2a8b3",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer00fb2a8b3::REGISTERS,
            },
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
                kind: "gdtmuf06b7fd1",
                version: "v1",
                block: "TMU",
                ir: &gdtmuf06b7fd1::REGISTERS,
            },
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
                kind: "gdtrigsel75c0668b",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigsel75c0668b::REGISTERS,
            },
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
                kind: "gdtrnga8e0c4c0",
                version: "v1",
                block: "TRNG",
                ir: &gdtrnga8e0c4c0::REGISTERS,
            },
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
                kind: "gduart38ee66329",
                version: "v1",
                block: "UART3",
                ir: &gduart38ee66329::REGISTERS,
            },
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
                kind: "gduart38ee66329",
                version: "v1",
                block: "UART3",
                ir: &gduart38ee66329::REGISTERS,
            },
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
                kind: "gdusart001f99729",
                version: "v1",
                block: "USART0",
                ir: &gdusart001f99729::REGISTERS,
            },
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
                kind: "gdusart001f99729",
                version: "v1",
                block: "USART0",
                ir: &gdusart001f99729::REGISTERS,
            },
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
                kind: "gdusart001f99729",
                version: "v1",
                block: "USART0",
                ir: &gdusart001f99729::REGISTERS,
            },
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
                kind: "gdvref193fa1c3",
                version: "v1",
                block: "VREF",
                ir: &gdvref193fa1c3::REGISTERS,
            },
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
        name: "LVD_AVD_OVD",
        number: 1,
    },
    Interrupt {
        name: "RTC_TAMPER_TIMESTAMP_LXTAL_STUCK",
        number: 2,
    },
    Interrupt {
        name: "RTC_WAKE",
        number: 3,
    },
    Interrupt {
        name: "FMC_GLOBAL",
        number: 4,
    },
    Interrupt {
        name: "RCU_GLOBAL",
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
        name: "TIMER0_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_TR_CM_DEC_ZERO",
        number: 26,
    },
    Interrupt {
        name: "TIMER0_CAP",
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
        name: "I2C0_EV_WAKE",
        number: 31,
    },
    Interrupt {
        name: "I2C0_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C1_EV_WAKE",
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
        name: "TIMER7_BRK_TE_ZE",
        number: 43,
    },
    Interrupt {
        name: "TIMER7_UP",
        number: 44,
    },
    Interrupt {
        name: "TIMER7_TR_CM_DEC_ZERO",
        number: 45,
    },
    Interrupt {
        name: "TIMER7_CAP",
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
        name: "TIMER5_DAC_0_2",
        number: 54,
    },
    Interrupt {
        name: "TIMER6_DAC_1_3",
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
        name: "UVD2_OVD2",
        number: 63,
    },
    Interrupt {
        name: "CMP0_1_2_3",
        number: 64,
    },
    Interrupt {
        name: "CMP4_5_6_7",
        number: 65,
    },
    Interrupt {
        name: "CMP_GLOBAL",
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
        name: "TIMER19_BRK_TE_ZE",
        number: 77,
    },
    Interrupt {
        name: "TIMER19_UP",
        number: 78,
    },
    Interrupt {
        name: "TIMER19_TR_CM_DEC_ZERO",
        number: 79,
    },
    Interrupt {
        name: "TIMER19_CAP",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "I2C2_EV_WAKE",
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
        name: "I2C3_EV_WAKE",
        number: 92,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 93,
    },
    Interrupt {
        name: "DMA_MUX",
        number: 94,
    },
    Interrupt {
        name: "QSPI",
        number: 95,
    },
    Interrupt {
        name: "FFT_GLOBAL",
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
        name: "FAC_GLOBAL",
        number: 101,
    },
    Interrupt {
        name: "HPDF_GLOBAL0",
        number: 102,
    },
    Interrupt {
        name: "HPDF_GLOBAL1",
        number: 103,
    },
    Interrupt {
        name: "HPDF_GLOBAL2",
        number: 104,
    },
    Interrupt {
        name: "HPDF_GLOBAL3",
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
        name: "CAN0_WK",
        number: 109,
    },
    Interrupt {
        name: "CAN0_BUFF",
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
        name: "CAN0_ERROR_FTX",
        number: 113,
    },
    Interrupt {
        name: "CAN0_WARNING_TX",
        number: 114,
    },
    Interrupt {
        name: "CAN0_WARNING_RX",
        number: 115,
    },
    Interrupt {
        name: "CAN1_WK",
        number: 116,
    },
    Interrupt {
        name: "CAN1_BUFF",
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
        name: "CAN1_ERROR_FTX",
        number: 120,
    },
    Interrupt {
        name: "CAN1_WARNING_TX",
        number: 121,
    },
    Interrupt {
        name: "CAN1_WARNING_RX",
        number: 122,
    },
    Interrupt {
        name: "CAN2_WK",
        number: 123,
    },
    Interrupt {
        name: "CAN2_BUFF",
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
        name: "CAN2_ERROR_FTX",
        number: 127,
    },
    Interrupt {
        name: "CAN2_WARNING_TX",
        number: 128,
    },
    Interrupt {
        name: "CAN2_WARNING_RX",
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
            #[path="../registers/gdadc0e44d6214_v1.rs"] pub mod gdadc0e44d6214;
#[path="../registers/gdadc1d46cf375_v1.rs"] pub mod gdadc1d46cf375;
#[path="../registers/gdadc2ffb56179_v1.rs"] pub mod gdadc2ffb56179;
#[path="../registers/gdadc3412cd97d_v1.rs"] pub mod gdadc3412cd97d;
#[path="../registers/gdcan00d9f65e8_v1.rs"] pub mod gdcan00d9f65e8;
#[path="../registers/gdcaue3899f2b_v1.rs"] pub mod gdcaue3899f2b;
#[path="../registers/gdclabbe1d8c5_v1.rs"] pub mod gdclabbe1d8c5;
#[path="../registers/gdcmp5553b816_v1.rs"] pub mod gdcmp5553b816;
#[path="../registers/gdcpdm0270b1b1_v1.rs"] pub mod gdcpdm0270b1b1;
#[path="../registers/gdcrccf345d41_v1.rs"] pub mod gdcrccf345d41;
#[path="../registers/gddac07216986f_v1.rs"] pub mod gddac07216986f;
#[path="../registers/gddbg91aeb23f_v1.rs"] pub mod gddbg91aeb23f;
#[path="../registers/gddma0ff389860_v1.rs"] pub mod gddma0ff389860;
#[path="../registers/gddmamux1fdc5150_v1.rs"] pub mod gddmamux1fdc5150;
#[path="../registers/gdexmc15e63ec9_v1.rs"] pub mod gdexmc15e63ec9;
#[path="../registers/gdextiaf81da6f_v1.rs"] pub mod gdextiaf81da6f;
#[path="../registers/gdfac8bc94bdf_v1.rs"] pub mod gdfac8bc94bdf;
#[path="../registers/gdfft4a1b8727_v1.rs"] pub mod gdfft4a1b8727;
#[path="../registers/gdfmcd425e751_v1.rs"] pub mod gdfmcd425e751;
#[path="../registers/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../registers/gdgpioa041672dd_v1.rs"] pub mod gdgpioa041672dd;
#[path="../registers/gdgpiob2a39d6c5_v1.rs"] pub mod gdgpiob2a39d6c5;
#[path="../registers/gdgpioc62f10237_v1.rs"] pub mod gdgpioc62f10237;
#[path="../registers/gdhpdf7e0cd818_v1.rs"] pub mod gdhpdf7e0cd818;
#[path="../registers/gdhrtimercommone8f80cd9_v1.rs"] pub mod gdhrtimercommone8f80cd9;
#[path="../registers/gdhrtimermastertimer46dd88dd_v1.rs"] pub mod gdhrtimermastertimer46dd88dd;
#[path="../registers/gdhrtimerslavetimer067581449_v1.rs"] pub mod gdhrtimerslavetimer067581449;
#[path="../registers/gdhrtimerslavetimer14d8109aa_v1.rs"] pub mod gdhrtimerslavetimer14d8109aa;
#[path="../registers/gdhrtimerslavetimer20b2b3389_v1.rs"] pub mod gdhrtimerslavetimer20b2b3389;
#[path="../registers/gdhrtimerslavetimer34217e6d5_v1.rs"] pub mod gdhrtimerslavetimer34217e6d5;
#[path="../registers/gdhrtimerslavetimer44f2ae72f_v1.rs"] pub mod gdhrtimerslavetimer44f2ae72f;
#[path="../registers/gdhrtimerslavetimer5093157b0_v1.rs"] pub mod gdhrtimerslavetimer5093157b0;
#[path="../registers/gdhrtimerslavetimer66a7539e7_v1.rs"] pub mod gdhrtimerslavetimer66a7539e7;
#[path="../registers/gdhrtimerslavetimer7b4a9b9e4_v1.rs"] pub mod gdhrtimerslavetimer7b4a9b9e4;
#[path="../registers/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../registers/gdlptimer1f47bc76_v1.rs"] pub mod gdlptimer1f47bc76;
#[path="../registers/gdpmu38e55ba3_v1.rs"] pub mod gdpmu38e55ba3;
#[path="../registers/gdqspic71428c9_v1.rs"] pub mod gdqspic71428c9;
#[path="../registers/gdrcu35f5457f_v1.rs"] pub mod gdrcu35f5457f;
#[path="../registers/gdrtca0f4d2cf_v1.rs"] pub mod gdrtca0f4d2cf;
#[path="../registers/gdspi0d1cf2b57_v1.rs"] pub mod gdspi0d1cf2b57;
#[path="../registers/gdspi16544be1a_v1.rs"] pub mod gdspi16544be1a;
#[path="../registers/gdspi2112e160c_v1.rs"] pub mod gdspi2112e160c;
#[path="../registers/gdsyscfgc16069c6_v1.rs"] pub mod gdsyscfgc16069c6;
#[path="../registers/gdtimer00fb2a8b3_v1.rs"] pub mod gdtimer00fb2a8b3;
#[path="../registers/gdtimer103a746fb_v1.rs"] pub mod gdtimer103a746fb;
#[path="../registers/gdtimer14eb20ecc0_v1.rs"] pub mod gdtimer14eb20ecc0;
#[path="../registers/gdtimer15f7745da8_v1.rs"] pub mod gdtimer15f7745da8;
#[path="../registers/gdtimer291200e8c_v1.rs"] pub mod gdtimer291200e8c;
#[path="../registers/gdtimer58fe8734a_v1.rs"] pub mod gdtimer58fe8734a;
#[path="../registers/gdtmuf06b7fd1_v1.rs"] pub mod gdtmuf06b7fd1;
#[path="../registers/gdtrigsel75c0668b_v1.rs"] pub mod gdtrigsel75c0668b;
#[path="../registers/gdtrnga8e0c4c0_v1.rs"] pub mod gdtrnga8e0c4c0;
#[path="../registers/gduart38ee66329_v1.rs"] pub mod gduart38ee66329;
#[path="../registers/gdusart001f99729_v1.rs"] pub mod gdusart001f99729;
#[path="../registers/gdvref193fa1c3_v1.rs"] pub mod gdvref193fa1c3;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
