
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "gdadca4c861d7",
            version: "v1",
            block: "ADC",
            ir: &gdadca4c861d7::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                af: None,
            },
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
                pin: "PC0",
                signal: "IN10",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
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
        name: "CAN",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "gdcanf0c54386",
            version: "v1",
            block: "CAN",
            ir: &gdcanf0c54386::REGISTERS,
        }),
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
        pins: &[],
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
            kind: "gddbgmcu02036f49",
            version: "v1",
            block: "DBGMCU",
            ir: &gddbgmcu02036f49::REGISTERS,
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
            version: "gd752f98b8d3cb",
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
            version: "g0x1",
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
                pin: "PA12",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SDA",
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
                pin: "PB6",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "S",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
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
                pin: "PA12",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "I2C2",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
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
                pin: "PB7",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
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
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
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
        name: "LPTIM1",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "gdlptimer0fade8b7a",
            version: "v1",
            block: "LPTIMER0",
            ir: &gdlptimer0fade8b7a::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ETI0",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "ETI0",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "IN0",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "gdlptimer0fade8b7a",
            version: "v1",
            block: "LPTIMER0",
            ir: &gdlptimer0fade8b7a::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC3",
                signal: "ETI0",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "OUT",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPTIM2",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART1",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "gdlpuart058954338",
            version: "v1",
            block: "LPUART0",
            ir: &gdlpuart058954338::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "LPUART1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "LPUART1",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                af: None,
            },
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
                pin: "PC12",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
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
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(59),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LPUART1",
        }],
        afio: None,
    },
    Peripheral {
        name: "LPUART2",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "gdlpuart058954338",
            version: "v1",
            block: "LPUART0",
            ir: &gdlpuart058954338::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CTS",
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
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(61),
            },
        ],
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
            version: "g0x1",
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
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
                pin: "PA0",
                signal: "TAMP1",
                af: None,
            },
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
                pin: "PB15",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "TAMP0",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "TS",
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
            kind: "gdslcd6dc6af89",
            version: "v1",
            block: "SLCD",
            ir: &gdslcd6dc6af89::REGISTERS,
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
                pin: "PA12",
                signal: "MOSI",
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
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "NS",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "NSS",
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
                pin: "PA13",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
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
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MISO",
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
                pin: "PC11",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SC",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "NSS",
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
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "CCIPR",
                field: "TIM1SEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APBENR2",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR2",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "BKIN",
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
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(65),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(69),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_UP_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_UP_BRK",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_UP_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_UP_BRK",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer8dfb2bdb4",
            version: "v1",
            block: "TIMER8",
            ir: &gdtimer8dfb2bdb4::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM15",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer14452fee2b",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer14452fee2b::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "BKIN",
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
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(72),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(73),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(74),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
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
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APBRSTR1",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH4",
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
                pin: "PA7",
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH4",
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
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(29),
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
        ],
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
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TIM3",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH4",
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
        name: "TIM41",
        address: 0x4001d000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer14452fee2b",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer14452fee2b::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(76),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(80),
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(78),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(79),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer5183dba8f",
            version: "v1",
            block: "TIMER5",
            ir: &gdtimer5183dba8f::REGISTERS,
        }),
        rcc: None,
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
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer6b465bb6b",
            version: "v1",
            block: "TIMER6",
            ir: &gdtimer6b465bb6b::REGISTERS,
        }),
        rcc: None,
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
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM9",
        address: 0x40014c00,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer8dfb2bdb4",
            version: "v1",
            block: "TIMER8",
            ir: &gdtimer8dfb2bdb4::REGISTERS,
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
                signal: "CH",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH1",
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
            kind: "gduart47d66af8a",
            version: "v1",
            block: "UART4",
            ir: &gduart47d66af8a::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "UART5",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
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
                pin: "PA11",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RTS",
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
                pin: "PA0",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
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
                pin: "PD1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "CK",
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
            kind: "gdvreff6814bb8",
            version: "v1",
            block: "VREF",
            ir: &gdvreff6814bb8::REGISTERS,
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
        name: "USBD_HP_CAN_TX",
        number: 19,
    },
    Interrupt {
        name: "USBD_LP_CAN_RX0",
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
        name: "TIM1_TRG_COM_UP_BRK",
        number: 44,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 45,
    },
    Interrupt {
        name: "TIM15",
        number: 46,
    },
    Interrupt {
        name: "EXTI10_15",
        number: 47,
    },
    Interrupt {
        name: "TIM41",
        number: 48,
    },
    Interrupt {
        name: "CAN_RX1",
        number: 49,
    },
    Interrupt {
        name: "CAN_EWMC",
        number: 50,
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
        name: "LPUART1",
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
        name: "LPUART1_WKUP",
        number: 67,
    },
    Interrupt {
        name: "LPTIM1",
        number: 68,
    },
    Interrupt {
        name: "LPUART2_WKUP",
        number: 69,
    },
    Interrupt {
        name: "LPTIM2",
        number: 70,
    },
    Interrupt {
        name: "LPUART2",
        number: 71,
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
    Pin { name: "PA0" },
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
    Pin { name: "PC0" },
    Pin { name: "PC1" },
    Pin { name: "PC10" },
    Pin { name: "PC11" },
    Pin { name: "PC12" },
    Pin { name: "PC2" },
    Pin { name: "PC3" },
    Pin { name: "PC4" },
    Pin { name: "PC5" },
    Pin { name: "PC6" },
    Pin { name: "PC7" },
    Pin { name: "PC8" },
    Pin { name: "PC9" },
    Pin { name: "PD0" },
    Pin { name: "PD1" },
    Pin { name: "PD2" },
    Pin { name: "PD4" },
    Pin { name: "PD5" },
    Pin { name: "PD6" },
    Pin { name: "PD8" },
    Pin { name: "PD9" },
];
#[path = "../registers/bdma_v1.rs"]
pub mod bdma;
#[path = "../registers/dmamux_gd752f98b8d3cb.rs"]
pub mod dmamux;
#[path = "../registers/flash_g0x1.rs"]
pub mod flash;
#[path = "../registers/gdadca4c861d7_v1.rs"]
pub mod gdadca4c861d7;
#[path = "../registers/gdcanf0c54386_v1.rs"]
pub mod gdcanf0c54386;
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
#[path = "../registers/gddbgmcu02036f49_v1.rs"]
pub mod gddbgmcu02036f49;
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
#[path = "../registers/gdlptimer0fade8b7a_v1.rs"]
pub mod gdlptimer0fade8b7a;
#[path = "../registers/gdlpuart058954338_v1.rs"]
pub mod gdlpuart058954338;
#[path = "../registers/gdrtc30fffb52_v1.rs"]
pub mod gdrtc30fffb52;
#[path = "../registers/gdslcd6dc6af89_v1.rs"]
pub mod gdslcd6dc6af89;
#[path = "../registers/gdspi0cf000376_v1.rs"]
pub mod gdspi0cf000376;
#[path = "../registers/gdspi19358bf74_v1.rs"]
pub mod gdspi19358bf74;
#[path = "../registers/gdtimer14452fee2b_v1.rs"]
pub mod gdtimer14452fee2b;
#[path = "../registers/gdtimer5183dba8f_v1.rs"]
pub mod gdtimer5183dba8f;
#[path = "../registers/gdtimer6b465bb6b_v1.rs"]
pub mod gdtimer6b465bb6b;
#[path = "../registers/gdtimer8dfb2bdb4_v1.rs"]
pub mod gdtimer8dfb2bdb4;
#[path = "../registers/gdtrngbf61c352_v1.rs"]
pub mod gdtrngbf61c352;
#[path = "../registers/gduart37add471e_v1.rs"]
pub mod gduart37add471e;
#[path = "../registers/gduart47d66af8a_v1.rs"]
pub mod gduart47d66af8a;
#[path = "../registers/gdusart0184abb20_v1.rs"]
pub mod gdusart0184abb20;
#[path = "../registers/gdusbd3c6a50b5_v1.rs"]
pub mod gdusbd3c6a50b5;
#[path = "../registers/gdvreff6814bb8_v1.rs"]
pub mod gdvreff6814bb8;
#[path = "../registers/gdwwdgtdd622579_v1.rs"]
pub mod gdwwdgtdd622579;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../registers/pwr_g0.rs"]
pub mod pwr;
#[path = "../registers/rcc_g0x1.rs"]
pub mod rcc;
#[path = "../registers/syscfg_g0.rs"]
pub mod syscfg;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
