
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012000,
        registers: Some(PeripheralRegisters {
            kind: "gdadc0644c59d8",
            version: "v1",
            block: "ADC0",
            ir: &gdadc0644c59d8::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC0",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "ADC0",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x40012100,
        registers: Some(PeripheralRegisters {
            kind: "gdadc0644c59d8",
            version: "v1",
            block: "ADC0",
            ir: &gdadc0644c59d8::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0x40012200,
        registers: Some(PeripheralRegisters {
            kind: "gdadc0644c59d8",
            version: "v1",
            block: "ADC0",
            ir: &gdadc0644c59d8::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PF10",
                signal: "IN8",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "IN9",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "IN14",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "IN4",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "IN5",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "IN6",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "IN7",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "ADC2",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC_Common",
        address: 0x40012300,
        registers: Some(PeripheralRegisters {
            kind: "gdadccommon6f53c1c8",
            version: "v1",
            block: "ADC_Common",
            ir: &gdadccommon6f53c1c8::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "gdcan06b36baa3",
            version: "v1",
            block: "CAN0",
            ir: &gdcan06b36baa3::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PH13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PI9",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "CAN2",
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "gdcan06b36baa3",
            version: "v1",
            block: "CAN0",
            ir: &gdcan06b36baa3::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN2_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN2_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN2_SCE",
            },
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN2_TX",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "v1",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "CRCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "CRCRST",
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
        name: "CRS",
        address: 0x40006c00,
        registers: Some(PeripheralRegisters {
            kind: "gdctc47444a2c",
            version: "v1",
            block: "CTC",
            ir: &gdctc47444a2c::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "SYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SYNC",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DAC1",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v2",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "DACRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DAC",
            channel: Some("DMA1_CH5"),
            dmamux: None,
            remap: &[],
            dma: None,
            request: Some(7),
        }],
        triggers: &[
            PeripheralTrigger {
                signal: "DAC_CHX_TRG0",
                source: "TIM6_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG1",
                source: "TIM8_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG2",
                source: "TIM7_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG3",
                source: "TIM5_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG4",
                source: "TIM2_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG5",
                source: "TIM4_TRGO",
            },
            PeripheralTrigger {
                signal: "DAC_CHX_TRG6",
                source: "EXTI9_TRG",
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6_DAC1",
        }],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe0042000,
        registers: Some(PeripheralRegisters {
            kind: "gddbg50e0203e",
            version: "v1",
            block: "DBG",
            ir: &gddbg50e0203e::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DCMI",
        address: 0x50050000,
        registers: Some(PeripheralRegisters {
            kind: "dcmi",
            version: "v1",
            block: "DCMI",
            ir: &dcmi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK2",
            kernel_clock: Clock("HCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "AHB2ENR",
                field: "DCMIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB2RSTR",
                field: "DCMIRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "HSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "VSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "D13",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D13",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "VSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PH10",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PH11",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PH14",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PH15",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PH6",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PH7",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PH8",
                signal: "HSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PI0",
                signal: "D13",
                af: None,
            },
            PeripheralPin {
                pin: "PI1",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PI2",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PI3",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PI4",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PI5",
                signal: "VSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PI6",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PI7",
                signal: "D7",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "DCI",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "DCI",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "DCMI",
        }],
        afio: None,
    },
    Peripheral {
        name: "DMA1",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v2",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMA1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "DMA1_CHANNEL0",
            },
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
        name: "DMA2",
        address: 0x40026400,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v2",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "DMA2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "DMA2_CHANNEL0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "ENET_DMA",
        address: 0x40029000,
        registers: Some(PeripheralRegisters {
            kind: "gdenetdma7fbba2f4",
            version: "v1",
            block: "ENET_DMA",
            ir: &gdenetdma7fbba2f4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET_MAC",
        address: 0x40028000,
        registers: Some(PeripheralRegisters {
            kind: "gdenetmac93552dd1",
            version: "v1",
            block: "ENET_MAC",
            ir: &gdenetmac93552dd1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET_MAC_FCTH",
        address: 0x40029080,
        registers: Some(PeripheralRegisters {
            kind: "gdenetmacfcth8ada9e21",
            version: "v1",
            block: "ENET_MAC_FCTH",
            ir: &gdenetmacfcth8ada9e21::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET_MSC",
        address: 0x40028100,
        registers: Some(PeripheralRegisters {
            kind: "gdenetmsc10390666",
            version: "v1",
            block: "ENET_MSC",
            ir: &gdenetmsc10390666::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET_PTP",
        address: 0x40028700,
        registers: Some(PeripheralRegisters {
            kind: "gdenetptp5c8a2d48",
            version: "v1",
            block: "ENET_PTP",
            ir: &gdenetptp5c8a2d48::REGISTERS,
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
        address: 0x40013c00,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "gd3c4cfb0b7eef",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI11",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI12",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI13",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI14",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_10",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI6",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI7",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI9",
                interrupt: "EXTI9_5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "FLASH",
        address: 0x40023c00,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "f4",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
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
        name: "FMC",
        address: 0xa0000000,
        registers: Some(PeripheralRegisters {
            kind: "fmc",
            version: "v1x3",
            block: "FMC",
            ir: &fmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK3",
            kernel_clock: Clock("HCLK3"),
            enable: Some(PeripheralRccRegister {
                register: "AHB3ENR",
                field: "FMCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB3RSTR",
                field: "FMCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "SDNWE",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDCKE1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NL",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NL/EXMC_NADV",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDNWE",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SDNE0",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "SDCKE0",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SDNE0",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SDCKE0",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16/EXMC_CLE",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17/EXMC_ALE",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NCE1",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE0",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "A0",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "A1",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "INTR",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SDNRAS",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "A6",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "A7",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "A8",
                af: None,
            },
            PeripheralPin {
                pin: "PF15",
                signal: "A9",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "A2",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "A3",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "A4",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "A5",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NIORD",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "NIOWR",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "NREG",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CD",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "NIOWR",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CD",
                af: None,
            },
            PeripheralPin {
                pin: "PG0",
                signal: "A10",
                af: None,
            },
            PeripheralPin {
                pin: "PG1",
                signal: "A11",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NCE3_0",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NE2",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "NCE3_1",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "NE3",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "A24",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "A25",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "SDNCAS",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "A12",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "A13",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "A14",
                af: None,
            },
            PeripheralPin {
                pin: "PG5",
                signal: "A15",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "INT1",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT2",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SDCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NCE2",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NE1",
                af: None,
            },
            PeripheralPin {
                pin: "PH10",
                signal: "D18",
                af: None,
            },
            PeripheralPin {
                pin: "PH11",
                signal: "D19",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "D20",
                af: None,
            },
            PeripheralPin {
                pin: "PH13",
                signal: "D21",
                af: None,
            },
            PeripheralPin {
                pin: "PH14",
                signal: "D22",
                af: None,
            },
            PeripheralPin {
                pin: "PH15",
                signal: "D23",
                af: None,
            },
            PeripheralPin {
                pin: "PH2",
                signal: "SDCKE0",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "SDNE0",
                af: None,
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SDNWE",
                af: None,
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SDNE1",
                af: None,
            },
            PeripheralPin {
                pin: "PH7",
                signal: "SDCKE1",
                af: None,
            },
            PeripheralPin {
                pin: "PH8",
                signal: "D16",
                af: None,
            },
            PeripheralPin {
                pin: "PH9",
                signal: "D17",
                af: None,
            },
            PeripheralPin {
                pin: "PI0",
                signal: "D24",
                af: None,
            },
            PeripheralPin {
                pin: "PI1",
                signal: "D25",
                af: None,
            },
            PeripheralPin {
                pin: "PI10",
                signal: "D31",
                af: None,
            },
            PeripheralPin {
                pin: "PI2",
                signal: "D26",
                af: None,
            },
            PeripheralPin {
                pin: "PI3",
                signal: "D27",
                af: None,
            },
            PeripheralPin {
                pin: "PI4",
                signal: "NBL2",
                af: None,
            },
            PeripheralPin {
                pin: "PI5",
                signal: "NBL3",
                af: None,
            },
            PeripheralPin {
                pin: "PI6",
                signal: "D28",
                af: None,
            },
            PeripheralPin {
                pin: "PI7",
                signal: "D29",
                af: None,
            },
            PeripheralPin {
                pin: "PI9",
                signal: "D30",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FMC",
        }],
        afio: None,
    },
    Peripheral {
        name: "FS_DEVICE",
        address: 0x50000800,
        registers: Some(PeripheralRegisters {
            kind: "gdfsdeviceb377b28b",
            version: "v1",
            block: "FS_DEVICE",
            ir: &gdfsdeviceb377b28b::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FS_GLOBAL",
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "gdfsglobale74e6f0e",
            version: "v1",
            block: "FS_GLOBAL",
            ir: &gdfsglobale74e6f0e::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FS_HOST",
        address: 0x50000400,
        registers: Some(PeripheralRegisters {
            kind: "gdfshost44621b1c",
            version: "v1",
            block: "FS_HOST",
            ir: &gdfshost44621b1c::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FS_PWRCLK",
        address: 0x50000e00,
        registers: Some(PeripheralRegisters {
            kind: "gdfspwrclk87dcd48b",
            version: "v1",
            block: "FS_PWRCLK",
            ir: &gdfspwrclk87dcd48b::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
        address: 0x40020400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
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
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOCRST",
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
        name: "GPIOD",
        address: 0x40020c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIODEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIODRST",
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
        name: "GPIOE",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOERST",
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
        name: "GPIOF",
        address: 0x40021400,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOFRST",
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
        name: "GPIOG",
        address: 0x40021800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOGRST",
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
        name: "GPIOH",
        address: 0x40021c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOHEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOHRST",
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
        name: "GPIOI",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v2",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK1",
            kernel_clock: Clock("HCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "AHB1ENR",
                field: "GPIOIEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHB1RSTR",
                field: "GPIOIRST",
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
        name: "HS_DEVICE",
        address: 0x40040800,
        registers: Some(PeripheralRegisters {
            kind: "gdhsdevicec9d69f15",
            version: "v1",
            block: "HS_DEVICE",
            ir: &gdhsdevicec9d69f15::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HS_GLOBAL",
        address: 0x40040000,
        registers: Some(PeripheralRegisters {
            kind: "gdhsglobalc406147a",
            version: "v1",
            block: "HS_GLOBAL",
            ir: &gdhsglobalc406147a::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HS_HOST",
        address: 0x40040400,
        registers: Some(PeripheralRegisters {
            kind: "gdhshostc2377b4a",
            version: "v1",
            block: "HS_HOST",
            ir: &gdhshostc2377b4a::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HS_PWRCLK",
        address: 0x40040e00,
        registers: Some(PeripheralRegisters {
            kind: "gdhspwrclk9376d26f",
            version: "v1",
            block: "HS_PWRCLK",
            ir: &gdhspwrclk9376d26f::REGISTERS,
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
            kind: "i2c",
            version: "v1",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
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
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
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
            kind: "i2c",
            version: "v1",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SDA",
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
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PH4",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PH5",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SMBA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
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
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v1",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PH10",
                signal: "TXFRAME",
                af: None,
            },
            PeripheralPin {
                pin: "PH7",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PH8",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PH9",
                signal: "SMBA",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "I2S1_add",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "gdspi0a39abaa4",
            version: "v1",
            block: "SPI0",
            ir: &gdspi0a39abaa4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S2_add",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "gdspi0a39abaa4",
            version: "v1",
            block: "SPI0",
            ir: &gdspi0a39abaa4::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IPA",
        address: 0x4002b000,
        registers: Some(PeripheralRegisters {
            kind: "gdipae676fed9",
            version: "v1",
            block: "IPA",
            ir: &gdipae676fed9::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IREF",
        address: 0x4000c400,
        registers: Some(PeripheralRegisters {
            kind: "gdiref361590d6",
            version: "v1",
            block: "IREF",
            ir: &gdiref361590d6::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v1",
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
        name: "LTDC",
        address: 0x40016800,
        registers: Some(PeripheralRegisters {
            kind: "gdtli3a8126bb",
            version: "v1",
            block: "TLI",
            ir: &gdtli3a8126bb::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "R4",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "R5",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B5",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B3",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "G7",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "G3",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "B4",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "R7",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "B0",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "G0",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "G1",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "G3",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B1",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "B4",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "R7",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PH10",
                signal: "R4",
                af: None,
            },
            PeripheralPin {
                pin: "PH11",
                signal: "R5",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "R6",
                af: None,
            },
            PeripheralPin {
                pin: "PH13",
                signal: "G2",
                af: None,
            },
            PeripheralPin {
                pin: "PH14",
                signal: "G3",
                af: None,
            },
            PeripheralPin {
                pin: "PH15",
                signal: "G4",
                af: None,
            },
            PeripheralPin {
                pin: "PH2",
                signal: "R0",
                af: None,
            },
            PeripheralPin {
                pin: "PH3",
                signal: "R1",
                af: None,
            },
            PeripheralPin {
                pin: "PH8",
                signal: "R2",
                af: None,
            },
            PeripheralPin {
                pin: "PH9",
                signal: "R3",
                af: None,
            },
            PeripheralPin {
                pin: "PI0",
                signal: "G5",
                af: None,
            },
            PeripheralPin {
                pin: "PI1",
                signal: "G6",
                af: None,
            },
            PeripheralPin {
                pin: "PI10",
                signal: "HSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PI2",
                signal: "G7",
                af: None,
            },
            PeripheralPin {
                pin: "PI4",
                signal: "B4",
                af: None,
            },
            PeripheralPin {
                pin: "PI5",
                signal: "B5",
                af: None,
            },
            PeripheralPin {
                pin: "PI6",
                signal: "B6",
                af: None,
            },
            PeripheralPin {
                pin: "PI7",
                signal: "B7",
                af: None,
            },
            PeripheralPin {
                pin: "PI9",
                signal: "VSYNC",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "LTDC_ER",
            },
            PeripheralInterrupt {
                signal: "LO",
                interrupt: "LTDC",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "f4",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
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
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "f4",
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
            kind: "gdtrngb48807ab",
            version: "v1",
            block: "TRNG",
            ir: &gdtrngb48807ab::REGISTERS,
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
            kind: "gdrtc34bd68c7",
            version: "v1",
            block: "RTC",
            ir: &gdrtc34bd68c7::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "TAMP0",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "TAMP1",
                af: None,
            },
            PeripheralPin {
                pin: "PI8",
                signal: "TS",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTC_ALARM",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "RTC_WKUP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "SDIO",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "sdmmc",
            version: "v1",
            block: "SDMMC",
            ir: &sdmmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Mux(PeripheralRccRegister {
                register: "DCKCFGR",
                field: "SDIOSEL",
            }),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SDIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SDIORST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CMD",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "SDIO",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "SDIO",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDIO",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
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
                pin: "PA5",
                signal: "SCK",
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
                pin: "PA7",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
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
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
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
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
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
                pin: "PC6",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PI0",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PI1",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PI2",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PI3",
                signal: "MOSI",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
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
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
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
                pin: "PB0",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
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
                pin: "PC5",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI4",
        address: 0x40013400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "NSS",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI4",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0x40015000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PH5",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PH6",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PH7",
                signal: "MISO",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI5",
        }],
        afio: None,
    },
    Peripheral {
        name: "SPI6",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v2_i2s",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SPI6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "SPI6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PG10",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "NSS",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI6",
        }],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "syscfg",
            version: "f4",
            block: "SYSCFG",
            ir: &syscfg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "SYSCFGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
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
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "BKIN",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA2_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM10",
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM10RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_UP_TIM10",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP_TIM10",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM11",
        address: 0x40014800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM11EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM11RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_TRG_COM_TIM11",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM12",
        address: 0x40001800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PH6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PH9",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_BRK_TIM12",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM13",
        address: 0x40001c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM13EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM13RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_UP_TIM13",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM14",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_1CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM14EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM14RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
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
                pin: "PA5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CH3",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(3),
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
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
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
                pin: "PA7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
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
                pin: "PB5",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH2",
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
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
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
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(2),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM5",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_GP32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PH10",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PH11",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PH12",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PI0",
                signal: "CH4",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(6),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer519fda6d7",
            version: "v1",
            block: "TIMER5",
            ir: &gdtimer519fda6d7::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            remap: &[],
            dma: None,
            request: Some(7),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_BASIC",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(1),
            },
        ],
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
        name: "TIM8",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_ADV",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH2",
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
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PH13",
                signal: "CH1N",
                af: None,
            },
            PeripheralPin {
                pin: "PH14",
                signal: "CH2N",
                af: None,
            },
            PeripheralPin {
                pin: "PH15",
                signal: "CH3N",
                af: None,
            },
            PeripheralPin {
                pin: "PI2",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PI3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PI4",
                signal: "BKIN",
                af: None,
            },
            PeripheralPin {
                pin: "PI5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PI6",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PI7",
                signal: "CH3",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(0),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "TG",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK_TIM12",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM_TIM14",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP_TIM13",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM9",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v1",
            block: "TIM_2CH",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2_TIM"),
            enable: Some(PeripheralRccRegister {
                register: "APB2ENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2RSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH2",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_BRK_TIM9",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_BRK_TIM9",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "gduart38ecaf091",
            version: "v1",
            block: "UART3",
            ir: &gduart38ecaf091::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
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
                pin: "PC3",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART4",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "gduart38ecaf091",
            version: "v1",
            block: "UART3",
            ir: &gduart38ecaf091::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART5",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART7",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "gduart38ecaf091",
            version: "v1",
            block: "UART3",
            ir: &gduart38ecaf091::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PE7",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART7",
        }],
        afio: None,
    },
    Peripheral {
        name: "UART8",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "gduart38ecaf091",
            version: "v1",
            block: "UART3",
            ir: &gduart38ecaf091::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PE0",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH0"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART8",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "gdusart06fc75967",
            version: "v1",
            block: "USART0",
            ir: &gdusart06fc75967::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
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
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
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
            kind: "gdusart06fc75967",
            version: "v1",
            block: "USART0",
            ir: &gdusart06fc75967::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
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
                pin: "PC3",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
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
                pin: "PD6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
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
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "gdusart06fc75967",
            version: "v1",
            block: "USART0",
            ir: &gdusart06fc75967::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
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
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
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
                channel: Some("DMA1_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(4),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(7),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
        afio: None,
    },
    Peripheral {
        name: "USART6",
        address: 0x40011400,
        registers: Some(PeripheralRegisters {
            kind: "gdusart06fc75967",
            version: "v1",
            block: "USART0",
            ir: &gdusart06fc75967::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CK",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "RX",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                remap: &[],
                dma: None,
                request: Some(5),
            },
        ],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
        afio: None,
    },
    Peripheral {
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v1",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1ENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1RSTR",
                field: "WWDGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
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
        name: "DMA1_CHANNEL0",
        number: 11,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 12,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 13,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 14,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 15,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 16,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 17,
    },
    Interrupt {
        name: "ADC",
        number: 18,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 19,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 20,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 21,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 22,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK_TIM9",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP_TIM10",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM_TIM11",
        number: 26,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 27,
    },
    Interrupt {
        name: "TIM2",
        number: 28,
    },
    Interrupt {
        name: "TIM3",
        number: 29,
    },
    Interrupt {
        name: "TIM4",
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
        name: "USART1",
        number: 37,
    },
    Interrupt {
        name: "USART2",
        number: 38,
    },
    Interrupt {
        name: "USART3",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_10",
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
        name: "TIM8_BRK_TIM12",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP_TIM13",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM_TIM14",
        number: 45,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 46,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 47,
    },
    Interrupt {
        name: "FMC",
        number: 48,
    },
    Interrupt {
        name: "SDIO",
        number: 49,
    },
    Interrupt {
        name: "TIM5",
        number: 50,
    },
    Interrupt {
        name: "SPI3",
        number: 51,
    },
    Interrupt {
        name: "UART4",
        number: 52,
    },
    Interrupt {
        name: "UART5",
        number: 53,
    },
    Interrupt {
        name: "TIM6_DAC1",
        number: 54,
    },
    Interrupt {
        name: "TIM7",
        number: 55,
    },
    Interrupt {
        name: "DMA2_CHANNEL0",
        number: 56,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 57,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 58,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 59,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 60,
    },
    Interrupt {
        name: "ENET",
        number: 61,
    },
    Interrupt {
        name: "ENET_WKUP",
        number: 62,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 63,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 64,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 65,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 66,
    },
    Interrupt {
        name: "USBFS",
        number: 67,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 68,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 69,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 70,
    },
    Interrupt {
        name: "USART6",
        number: 71,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 73,
    },
    Interrupt {
        name: "USBHS_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "USBHS_EP1_IN",
        number: 75,
    },
    Interrupt {
        name: "USBHS_WKUP",
        number: 76,
    },
    Interrupt {
        name: "USBHS",
        number: 77,
    },
    Interrupt {
        name: "DCMI",
        number: 78,
    },
    Interrupt {
        name: "RNG",
        number: 80,
    },
    Interrupt {
        name: "FPU",
        number: 81,
    },
    Interrupt {
        name: "UART7",
        number: 82,
    },
    Interrupt {
        name: "UART8",
        number: 83,
    },
    Interrupt {
        name: "SPI4",
        number: 84,
    },
    Interrupt {
        name: "SPI5",
        number: 85,
    },
    Interrupt {
        name: "SPI6",
        number: 86,
    },
    Interrupt {
        name: "LTDC",
        number: 88,
    },
    Interrupt {
        name: "LTDC_ER",
        number: 89,
    },
    Interrupt {
        name: "IPA",
        number: 90,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
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
    Pin { name: "PB2" },
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
    Pin { name: "PC13" },
    Pin { name: "PC14" },
    Pin { name: "PC15" },
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
    Pin { name: "PD10" },
    Pin { name: "PD11" },
    Pin { name: "PD12" },
    Pin { name: "PD13" },
    Pin { name: "PD14" },
    Pin { name: "PD15" },
    Pin { name: "PD2" },
    Pin { name: "PD3" },
    Pin { name: "PD4" },
    Pin { name: "PD5" },
    Pin { name: "PD6" },
    Pin { name: "PD7" },
    Pin { name: "PD8" },
    Pin { name: "PD9" },
    Pin { name: "PE0" },
    Pin { name: "PE1" },
    Pin { name: "PE10" },
    Pin { name: "PE11" },
    Pin { name: "PE12" },
    Pin { name: "PE13" },
    Pin { name: "PE14" },
    Pin { name: "PE15" },
    Pin { name: "PE2" },
    Pin { name: "PE3" },
    Pin { name: "PE4" },
    Pin { name: "PE5" },
    Pin { name: "PE6" },
    Pin { name: "PE7" },
    Pin { name: "PE8" },
    Pin { name: "PE9" },
    Pin { name: "PF0" },
    Pin { name: "PF1" },
    Pin { name: "PF10" },
    Pin { name: "PF11" },
    Pin { name: "PF12" },
    Pin { name: "PF13" },
    Pin { name: "PF14" },
    Pin { name: "PF15" },
    Pin { name: "PF2" },
    Pin { name: "PF3" },
    Pin { name: "PF4" },
    Pin { name: "PF5" },
    Pin { name: "PF6" },
    Pin { name: "PF7" },
    Pin { name: "PF8" },
    Pin { name: "PF9" },
    Pin { name: "PG0" },
    Pin { name: "PG1" },
    Pin { name: "PG10" },
    Pin { name: "PG11" },
    Pin { name: "PG12" },
    Pin { name: "PG13" },
    Pin { name: "PG14" },
    Pin { name: "PG15" },
    Pin { name: "PG2" },
    Pin { name: "PG3" },
    Pin { name: "PG4" },
    Pin { name: "PG5" },
    Pin { name: "PG6" },
    Pin { name: "PG7" },
    Pin { name: "PG8" },
    Pin { name: "PG9" },
    Pin { name: "PH0" },
    Pin { name: "PH1" },
    Pin { name: "PH10" },
    Pin { name: "PH11" },
    Pin { name: "PH12" },
    Pin { name: "PH13" },
    Pin { name: "PH14" },
    Pin { name: "PH15" },
    Pin { name: "PH2" },
    Pin { name: "PH3" },
    Pin { name: "PH4" },
    Pin { name: "PH5" },
    Pin { name: "PH6" },
    Pin { name: "PH7" },
    Pin { name: "PH8" },
    Pin { name: "PH9" },
    Pin { name: "PI0" },
    Pin { name: "PI1" },
    Pin { name: "PI10" },
    Pin { name: "PI11" },
    Pin { name: "PI2" },
    Pin { name: "PI3" },
    Pin { name: "PI4" },
    Pin { name: "PI5" },
    Pin { name: "PI6" },
    Pin { name: "PI7" },
    Pin { name: "PI8" },
    Pin { name: "PI9" },
];
#[path = "../registers/crc_v1.rs"]
pub mod crc;
#[path = "../registers/dac_v2.rs"]
pub mod dac;
#[path = "../registers/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../registers/dma_v2.rs"]
pub mod dma;
#[path = "../registers/exti_gd3c4cfb0b7eef.rs"]
pub mod exti;
#[path = "../registers/flash_f4.rs"]
pub mod flash;
#[path = "../registers/fmc_v1x3.rs"]
pub mod fmc;
#[path = "../registers/gdadc0644c59d8_v1.rs"]
pub mod gdadc0644c59d8;
#[path = "../registers/gdadccommon6f53c1c8_v1.rs"]
pub mod gdadccommon6f53c1c8;
#[path = "../registers/gdcan06b36baa3_v1.rs"]
pub mod gdcan06b36baa3;
#[path = "../registers/gdctc47444a2c_v1.rs"]
pub mod gdctc47444a2c;
#[path = "../registers/gddbg50e0203e_v1.rs"]
pub mod gddbg50e0203e;
#[path = "../registers/gdenetdma7fbba2f4_v1.rs"]
pub mod gdenetdma7fbba2f4;
#[path = "../registers/gdenetmac93552dd1_v1.rs"]
pub mod gdenetmac93552dd1;
#[path = "../registers/gdenetmacfcth8ada9e21_v1.rs"]
pub mod gdenetmacfcth8ada9e21;
#[path = "../registers/gdenetmsc10390666_v1.rs"]
pub mod gdenetmsc10390666;
#[path = "../registers/gdenetptp5c8a2d48_v1.rs"]
pub mod gdenetptp5c8a2d48;
#[path = "../registers/gdfsdeviceb377b28b_v1.rs"]
pub mod gdfsdeviceb377b28b;
#[path = "../registers/gdfsglobale74e6f0e_v1.rs"]
pub mod gdfsglobale74e6f0e;
#[path = "../registers/gdfshost44621b1c_v1.rs"]
pub mod gdfshost44621b1c;
#[path = "../registers/gdfspwrclk87dcd48b_v1.rs"]
pub mod gdfspwrclk87dcd48b;
#[path = "../registers/gdhsdevicec9d69f15_v1.rs"]
pub mod gdhsdevicec9d69f15;
#[path = "../registers/gdhsglobalc406147a_v1.rs"]
pub mod gdhsglobalc406147a;
#[path = "../registers/gdhshostc2377b4a_v1.rs"]
pub mod gdhshostc2377b4a;
#[path = "../registers/gdhspwrclk9376d26f_v1.rs"]
pub mod gdhspwrclk9376d26f;
#[path = "../registers/gdipae676fed9_v1.rs"]
pub mod gdipae676fed9;
#[path = "../registers/gdiref361590d6_v1.rs"]
pub mod gdiref361590d6;
#[path = "../registers/gdrtc34bd68c7_v1.rs"]
pub mod gdrtc34bd68c7;
#[path = "../registers/gdspi0a39abaa4_v1.rs"]
pub mod gdspi0a39abaa4;
#[path = "../registers/gdtimer519fda6d7_v1.rs"]
pub mod gdtimer519fda6d7;
#[path = "../registers/gdtli3a8126bb_v1.rs"]
pub mod gdtli3a8126bb;
#[path = "../registers/gdtrngb48807ab_v1.rs"]
pub mod gdtrngb48807ab;
#[path = "../registers/gduart38ecaf091_v1.rs"]
pub mod gduart38ecaf091;
#[path = "../registers/gdusart06fc75967_v1.rs"]
pub mod gdusart06fc75967;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/i2c_v1_gd2531cd0166de.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../registers/pwr_f4.rs"]
pub mod pwr;
#[path = "../registers/rcc_f4.rs"]
pub mod rcc;
#[path = "../registers/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../registers/spi_v2_i2s.rs"]
pub mod spi;
#[path = "../registers/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;
