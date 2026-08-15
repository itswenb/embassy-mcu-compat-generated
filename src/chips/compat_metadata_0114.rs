
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "gdadc34b106d5",
            version: "v1",
            block: "ADC",
            ir: &gdadc34b106d5::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN0",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(5),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "CAU",
        address: 0x50060000,
        registers: Some(PeripheralRegisters {
            kind: "gdcaue9e51f0c",
            version: "v1",
            block: "CAU",
            ir: &gdcaue9e51f0c::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdcmpd90af10b",
            version: "v1",
            block: "CMP",
            ir: &gdcmpd90af10b::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdcrc67d273cb",
            version: "v1",
            block: "CRC",
            ir: &gdcrc67d273cb::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CRS",
        address: 0x4000c800,
        registers: Some(PeripheralRegisters {
            kind: "gdctceaaaf458",
            version: "v1",
            block: "CTC",
            ir: &gdctceaaaf458::REGISTERS,
        }),
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PA8",
            signal: "SYNC",
            af: None,
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "gddac7e57a629",
            version: "v1",
            block: "DAC",
            ir: &gddac7e57a629::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(6),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0x40015800,
        registers: Some(PeripheralRegisters {
            kind: "gddbgmcu738c6f06",
            version: "v1",
            block: "DBGMCU",
            ir: &gddbgmcu738c6f06::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "bdma",
            version: "v1",
            block: "DMA",
            ir: &bdma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "DMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "DMAMUX1",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "gdbba0ac7e1f00",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdexti30fc9668",
            version: "v1",
            block: "EXTI",
            ir: &gdexti30fc9668::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "g0x0",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHBENR",
                field: "FLASHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "FLASHRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x48000000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOB",
        address: 0x48000400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "GPIO",
            kernel_clock: Clock("GPIO"),
            enable: Some(PeripheralRccRegister {
                register: "GPIOENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "GPIORSTR",
                field: "GPIOBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOC",
        address: 0x48000800,
        registers: Some(PeripheralRegisters {
            kind: "gdgpioc0fba06c4",
            version: "v1",
            block: "GPIOC",
            ir: &gdgpioc0fba06c4::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdgpiod7229d923",
            version: "v1",
            block: "GPIOD",
            ir: &gdgpiod7229d923::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdgpiof7c6237df",
            version: "v1",
            block: "GPIOF",
            ir: &gdgpiof7c6237df::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "gdi2c0cd973dc4",
            version: "v1",
            block: "I2C0",
            ir: &gdi2c0cd973dc4::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(10),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(11),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "gdi2c0cd973dc4",
            version: "v1",
            block: "I2C0",
            ir: &gdi2c0cd973dc4::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "I2C2",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(13),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2C3",
        address: 0x4000c000,
        registers: Some(PeripheralRegisters {
            kind: "gdi2c0cd973dc4",
            version: "v1",
            block: "I2C0",
            ir: &gdi2c0cd973dc4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(15),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v2",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdlptimer81986a0b",
            version: "v1",
            block: "LPTIMER",
            ir: &gdlptimer81986a0b::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LPUART",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "gdlpuart3ad1937d",
            version: "v1",
            block: "LPUART",
            ir: &gdlpuart3ad1937d::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "g0",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "PWRRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "g0x0",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC_CRS",
        }],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x50060800,
        registers: Some(PeripheralRegisters {
            kind: "gdtrngbf61c352",
            version: "v1",
            block: "TRNG",
            ir: &gdtrngbf61c352::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdrtc30fffb52",
            version: "v1",
            block: "RTC",
            ir: &gdrtc30fffb52::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TAMP2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TAMP1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SLCD",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "gdslcd8086d68f",
            version: "v1",
            block: "SLCD",
            ir: &gdslcd8086d68f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "gdspi0cf000376",
            version: "v1",
            block: "SPI0",
            ir: &gdspi0cf000376::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "NS",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "gdspi19358bf74",
            version: "v1",
            block: "SPI1",
            ir: &gdspi19358bf74::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SCK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(19),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "g0",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "SYSCFGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer83f762be9",
            version: "v1",
            block: "TIMER8",
            ir: &gdtimer83f762be9::REGISTERS,
        }),
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PB14",
            signal: "CH2",
            af: None,
        }],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer15f311eaa",
            version: "v1",
            block: "TIMER1",
            ir: &gdtimer15f311eaa::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(26),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(28),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(30),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_GP16",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(33),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(34),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(37),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(42),
        }],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM6",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM6",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APBENR1",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(43),
        }],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM7",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM9",
        address: 0x40014c00,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer83f762be9",
            version: "v1",
            block: "TIMER8",
            ir: &gdtimer83f762be9::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "gduart37add471e",
            version: "v1",
            block: "UART3",
            ir: &gduart37add471e::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(54),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(55),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "gduart4f478961c",
            version: "v1",
            block: "UART4",
            ir: &gduart4f478961c::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(56),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(57),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "gdusart0184abb20",
            version: "v1",
            block: "USART0",
            ir: &gdusart0184abb20::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RTS/USART0_DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(50),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(51),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "gdusart0184abb20",
            version: "v1",
            block: "USART0",
            ir: &gdusart0184abb20::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RTS/USART1_DE",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(52),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(53),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
        afio: None,
    },
    Peripheral {
        name: "USBD",
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "gdusbd3c6a50b5",
            version: "v1",
            block: "USBD",
            ir: &gdusbd3c6a50b5::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdvref8ca405d5",
            version: "v1",
            block: "VREF",
            ir: &gdvref8ca405d5::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "gdwwdgtdd622579",
            version: "v1",
            block: "WWDGT",
            ir: &gdwwdgtdd622579::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "WWDG",
            },
            PeripheralInterrupt {
                signal: "RST",
                interrupt: "WWDG",
            },
        ],
        afio: None,
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 0,
    },
    Interrupt { name: "LVD", number: 1 },
    Interrupt {
        name: "TAMPER_STAMP",
        number: 2,
    },
    Interrupt {
        name: "RTC_WKUP",
        number: 3,
    },
    Interrupt {
        name: "FLASH",
        number: 4,
    },
    Interrupt {
        name: "RCC_CRS",
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
        name: "DMA1_CHANNEL1",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 17,
    },
    Interrupt {
        name: "ADC1",
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
        name: "TIM2",
        number: 21,
    },
    Interrupt {
        name: "TIM3",
        number: 22,
    },
    Interrupt {
        name: "TIM9",
        number: 23,
    },
    Interrupt {
        name: "TIM12",
        number: 24,
    },
    Interrupt {
        name: "TIM6",
        number: 25,
    },
    Interrupt {
        name: "TIM7",
        number: 26,
    },
    Interrupt {
        name: "USART1",
        number: 27,
    },
    Interrupt {
        name: "USART2",
        number: 28,
    },
    Interrupt {
        name: "UART4",
        number: 29,
    },
    Interrupt {
        name: "UART5",
        number: 30,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 31,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 32,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 33,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 34,
    },
    Interrupt {
        name: "SPI1",
        number: 35,
    },
    Interrupt {
        name: "SPI2",
        number: 36,
    },
    Interrupt {
        name: "DAC1",
        number: 37,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 39,
    },
    Interrupt {
        name: "I2C3_ER",
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
        name: "DMAMUX1",
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
        name: "I2C1_WKUP",
        number: 58,
    },
    Interrupt {
        name: "I2C3_WKUP",
        number: 59,
    },
    Interrupt {
        name: "USART1_WKUP",
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
        name: "RNG",
        number: 63,
    },
    Interrupt {
        name: "SLCD",
        number: 64,
    },
    Interrupt {
        name: "USART2_WKUP",
        number: 65,
    },
    Interrupt {
        name: "I2C2_WKUP",
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
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA1" },
    Pin { name: "PA10" },
    Pin { name: "PA11" },
    Pin { name: "PA12" },
    Pin { name: "PA13" },
    Pin { name: "PA14" },
    Pin { name: "PA15" },
    Pin { name: "PA2" },
    Pin { name: "PA3" },
    Pin { name: "PA4" },
    Pin { name: "PA5" },
    Pin { name: "PA6" },
    Pin { name: "PA7" },
    Pin { name: "PA8" },
    Pin { name: "PA9" },
    Pin { name: "PB0" },
    Pin { name: "PB1" },
    Pin { name: "PB10" },
    Pin { name: "PB11" },
    Pin { name: "PB12" },
    Pin { name: "PB13" },
    Pin { name: "PB14" },
    Pin { name: "PB15" },
    Pin { name: "PB3" },
    Pin { name: "PB4" },
    Pin { name: "PB5" },
    Pin { name: "PB6" },
    Pin { name: "PB7" },
    Pin { name: "PB8" },
    Pin { name: "PB9" },
    Pin { name: "PC10" },
    Pin { name: "PC11" },
    Pin { name: "PC12" },
    Pin { name: "PC3" },
    Pin { name: "PC6" },
    Pin { name: "PC7" },
];
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/dmamux_gdbba0ac7e1f00.rs"]
pub mod dmamux;
#[path = "../registers/flash_g0x0.rs"]
pub mod flash;
#[path = "../registers/gdadc34b106d5_v1.rs"]
pub mod gdadc34b106d5;
#[path = "../registers/gdcaue9e51f0c_v1.rs"]
pub mod gdcaue9e51f0c;
#[path = "../registers/gdcmpd90af10b_v1.rs"]
pub mod gdcmpd90af10b;
#[path = "../registers/gdcrc67d273cb_v1.rs"]
pub mod gdcrc67d273cb;
#[path = "../registers/gdctceaaaf458_v1.rs"]
pub mod gdctceaaaf458;
#[path = "../registers/gddac7e57a629_v1.rs"]
pub mod gddac7e57a629;
#[path = "../registers/gddbgmcu738c6f06_v1.rs"]
pub mod gddbgmcu738c6f06;
#[path = "../registers/gdexti30fc9668_v1.rs"]
pub mod gdexti30fc9668;
#[path = "../registers/gdgpioc0fba06c4_v1.rs"]
pub mod gdgpioc0fba06c4;
#[path = "../registers/gdgpiod7229d923_v1.rs"]
pub mod gdgpiod7229d923;
#[path = "../registers/gdgpiof7c6237df_v1.rs"]
pub mod gdgpiof7c6237df;
#[path = "../registers/gdi2c0cd973dc4_v1.rs"]
pub mod gdi2c0cd973dc4;
#[path = "../registers/gdlptimer81986a0b_v1.rs"]
pub mod gdlptimer81986a0b;
#[path = "../registers/gdlpuart3ad1937d_v1.rs"]
pub mod gdlpuart3ad1937d;
#[path = "../registers/gdrtc30fffb52_v1.rs"]
pub mod gdrtc30fffb52;
#[path = "../registers/gdslcd8086d68f_v1.rs"]
pub mod gdslcd8086d68f;
#[path = "../registers/gdspi0cf000376_v1.rs"]
pub mod gdspi0cf000376;
#[path = "../registers/gdspi19358bf74_v1.rs"]
pub mod gdspi19358bf74;
#[path = "../registers/gdtimer15f311eaa_v1.rs"]
pub mod gdtimer15f311eaa;
#[path = "../registers/gdtimer83f762be9_v1.rs"]
pub mod gdtimer83f762be9;
#[path = "../registers/gdtrngbf61c352_v1.rs"]
pub mod gdtrngbf61c352;
#[path = "../registers/gduart37add471e_v1.rs"]
pub mod gduart37add471e;
#[path = "../registers/gduart4f478961c_v1.rs"]
pub mod gduart4f478961c;
#[path = "../registers/gdusart0184abb20_v1.rs"]
pub mod gdusart0184abb20;
#[path = "../registers/gdusbd3c6a50b5_v1.rs"]
pub mod gdusbd3c6a50b5;
#[path = "../registers/gdvref8ca405d5_v1.rs"]
pub mod gdvref8ca405d5;
#[path = "../registers/gdwwdgtdd622579_v1.rs"]
pub mod gdwwdgtdd622579;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_g0.rs"]
pub mod pwr;
#[path = "../registers/rcc_g0x0.rs"]
pub mod rcc;
#[path = "../registers/syscfg_g0.rs"]
pub mod syscfg;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
