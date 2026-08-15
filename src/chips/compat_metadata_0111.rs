
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "gdadc06d279556",
            version: "v1",
            block: "ADC0",
            ir: &gdadc06d279556::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN16",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN17",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC0",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(9),
        }],
        triggers: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
        afio: None,
    },
    Peripheral {
        name: "ADC2",
        address: 0x40012800,
        registers: Some(PeripheralRegisters {
            kind: "gdadc1425a4aff",
            version: "v1",
            block: "ADC1",
            ir: &gdadc1425a4aff::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(10),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC3",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "gdadc2efea3dc8",
            version: "v1",
            block: "ADC2",
            ir: &gdadc2efea3dc8::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC2",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(123),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "AXI",
        address: 0x51000000,
        registers: Some(PeripheralRegisters {
            kind: "gdaxi7000de15",
            version: "v1",
            block: "AXI",
            ir: &gdaxi7000de15::REGISTERS,
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
        address: 0x4001a000,
        registers: Some(PeripheralRegisters {
            kind: "gdcan0ab6ea0b5",
            version: "v1",
            block: "CAN0",
            ir: &gdcan0ab6ea0b5::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CAN0",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(186),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN2",
        address: 0x4001b000,
        registers: Some(PeripheralRegisters {
            kind: "gdcan0ab6ea0b5",
            version: "v1",
            block: "CAN0",
            ir: &gdcan0ab6ea0b5::REGISTERS,
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
        dma_channels: &[PeripheralDmaChannel {
            signal: "CAN1",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(187),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAN3",
        address: 0x4001c000,
        registers: Some(PeripheralRegisters {
            kind: "gdcan0ab6ea0b5",
            version: "v1",
            block: "CAN0",
            ir: &gdcan0ab6ea0b5::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "TX",
                af: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "CAN2",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(188),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CAU",
        address: 0x48021000,
        registers: Some(PeripheralRegisters {
            kind: "gdcau3fafd38d",
            version: "v1",
            block: "CAU",
            ir: &gdcau3fafd38d::REGISTERS,
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
        address: 0x58003800,
        registers: Some(PeripheralRegisters {
            kind: "gdcmp65ef540c",
            version: "v1",
            block: "CMP",
            ir: &gdcmp65ef540c::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "MUX_OUT0",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MUX_OUT1",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "MUX_OUT2",
                af: None,
            },
            PeripheralPin {
                pin: "PE15",
                signal: "MUX_OUT4",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MUX_OUT3",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "MUX_OUT5",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "MUX_OUT6",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "MUX_OUT7",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM_SDIO0",
        address: 0x52008000,
        registers: Some(PeripheralRegisters {
            kind: "gdcpdmsdio04a9ee533",
            version: "v1",
            block: "CPDM_SDIO0",
            ir: &gdcpdmsdio04a9ee533::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM_SDIO1",
        address: 0x48022800,
        registers: Some(PeripheralRegisters {
            kind: "gdcpdmsdio04a9ee533",
            version: "v1",
            block: "CPDM_SDIO0",
            ir: &gdcpdmsdio04a9ee533::REGISTERS,
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
        address: 0x58024c00,
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
        address: 0x40008400,
        registers: Some(PeripheralRegisters {
            kind: "gdctcdb80f1ce",
            version: "v1",
            block: "CTC",
            ir: &gdctcdb80f1ce::REGISTERS,
        }),
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PB3",
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
            kind: "gddac555b6194",
            version: "v1",
            block: "DAC",
            ir: &gddac555b6194::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(81),
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(82),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DBGMCU",
        address: 0xe00e1000,
        registers: Some(PeripheralRegisters {
            kind: "gddbgefa81966",
            version: "v1",
            block: "DBG",
            ir: &gddbgefa81966::REGISTERS,
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
        address: 0x48020000,
        registers: Some(PeripheralRegisters {
            kind: "gddci5ae31085",
            version: "v1",
            block: "DCI",
            ir: &gddci5ae31085::REGISTERS,
        }),
        rcc: None,
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
                pin: "PB13",
                signal: "D2",
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
                pin: "PD12",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "D13",
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
                pin: "PE3",
                signal: "PIXCLK",
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
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "DCI",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(89),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
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
        address: 0x40020400,
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
        name: "DMAMUX1",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "gddmamuxeaace10d",
            version: "v1",
            block: "DMAMUX",
            ir: &gddmamuxeaace10d::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDOUT",
        address: 0x40018800,
        registers: Some(PeripheralRegisters {
            kind: "gdedoutfebca4f4",
            version: "v1",
            block: "EDOUT",
            ir: &gdedoutfebca4f4::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdefuse25c60075",
            version: "v1",
            block: "EFUSE",
            ir: &gdefuse25c60075::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0_DMA",
        address: 0x40029000,
        registers: Some(PeripheralRegisters {
            kind: "gdenet0dma7d3e05fd",
            version: "v1",
            block: "ENET0_DMA",
            ir: &gdenet0dma7d3e05fd::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0_MAC",
        address: 0x40028000,
        registers: Some(PeripheralRegisters {
            kind: "gdenet0macd2855220",
            version: "v1",
            block: "ENET0_MAC",
            ir: &gdenet0macd2855220::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0_MAC_FCTH",
        address: 0x40029080,
        registers: Some(PeripheralRegisters {
            kind: "gdenet0macfcthffd74812",
            version: "v1",
            block: "ENET0_MAC_FCTH",
            ir: &gdenet0macfcthffd74812::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0_MSC",
        address: 0x40028100,
        registers: Some(PeripheralRegisters {
            kind: "gdenet0msc2451d465",
            version: "v1",
            block: "ENET0_MSC",
            ir: &gdenet0msc2451d465::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0_PTP",
        address: 0x40028700,
        registers: Some(PeripheralRegisters {
            kind: "gdenet0ptpc700182c",
            version: "v1",
            block: "ENET0_PTP",
            ir: &gdenet0ptpc700182c::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1_DMA",
        address: 0x4002b000,
        registers: Some(PeripheralRegisters {
            kind: "gdenet1dmabfdb3976",
            version: "v1",
            block: "ENET1_DMA",
            ir: &gdenet1dmabfdb3976::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1_MAC",
        address: 0x4002a000,
        registers: Some(PeripheralRegisters {
            kind: "gdenet1maceef08a3b",
            version: "v1",
            block: "ENET1_MAC",
            ir: &gdenet1maceef08a3b::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1_MAC_FCTH",
        address: 0x4002b080,
        registers: Some(PeripheralRegisters {
            kind: "gdenet1macfcthacf2ccdd",
            version: "v1",
            block: "ENET1_MAC_FCTH",
            ir: &gdenet1macfcthacf2ccdd::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1_MSC",
        address: 0x4002a100,
        registers: Some(PeripheralRegisters {
            kind: "gdenet1msc4852d4b8",
            version: "v1",
            block: "ENET1_MSC",
            ir: &gdenet1msc4852d4b8::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1_PTP",
        address: 0x4002a700,
        registers: Some(PeripheralRegisters {
            kind: "gdenet1ptpedbe1f92",
            version: "v1",
            block: "ENET1_PTP",
            ir: &gdenet1ptpedbe1f92::REGISTERS,
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
        address: 0x58000000,
        registers: Some(PeripheralRegisters {
            kind: "gdextic827d627",
            version: "v1",
            block: "EXTI",
            ir: &gdextic827d627::REGISTERS,
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
        ],
        afio: None,
    },
    Peripheral {
        name: "FAC",
        address: 0x48024800,
        registers: Some(PeripheralRegisters {
            kind: "gdfac96d60f19",
            version: "v1",
            block: "FAC",
            ir: &gdfac96d60f19::REGISTERS,
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
        address: 0x52002000,
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
        address: 0x52004000,
        registers: Some(PeripheralRegisters {
            kind: "gdexmc293e9145",
            version: "v1",
            block: "EXMC",
            ir: &gdexmc293e9145::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "A19",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "INT",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "D8",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "D9",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "A22",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "SDNWE",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "D10",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "NCE",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SDCKE1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "D11",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDNE1",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NADV",
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
                signal: "A25",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SDNWE",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "NBL2",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "D6",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "A25",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "D12",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "A22",
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
                pin: "PC6",
                signal: "NWAIT",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "NE0",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "INT",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "NE1",
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
                pin: "PD1",
                signal: "D7",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16/EXMC_CLE",
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
                pin: "PD2",
                signal: "D7",
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
                signal: "NCE",
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
                signal: "D24",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "D25",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "D26",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "D27",
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
                signal: "D31",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NE2",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "D29",
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
                signal: "NE2",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "D28",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "INT",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "SDCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "D30",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "NE1",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOA",
        address: 0x58020000,
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
        address: 0x58020400,
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
        address: 0x58020800,
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
        address: 0x58020c00,
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
        address: 0x58021000,
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
        address: 0x58021400,
        registers: Some(PeripheralRegisters {
            kind: "gdgpioc18dfc69f",
            version: "v1",
            block: "GPIOC",
            ir: &gdgpioc18dfc69f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOG",
        address: 0x58021800,
        registers: Some(PeripheralRegisters {
            kind: "gdgpioc18dfc69f",
            version: "v1",
            block: "GPIOC",
            ir: &gdgpioc18dfc69f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOH",
        address: 0x58021c00,
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
        name: "GPIOJ",
        address: 0x58022400,
        registers: Some(PeripheralRegisters {
            kind: "gdgpioc18dfc69f",
            version: "v1",
            block: "GPIOC",
            ir: &gdgpioc18dfc69f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOK",
        address: 0x58022800,
        registers: Some(PeripheralRegisters {
            kind: "gdgpioc18dfc69f",
            version: "v1",
            block: "GPIOC",
            ir: &gdgpioc18dfc69f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HAU",
        address: 0x48021400,
        registers: Some(PeripheralRegisters {
            kind: "gdhaub8125197",
            version: "v1",
            block: "HAU",
            ir: &gdhaub8125197::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdhpdffd9de252",
            version: "v1",
            block: "HPDF",
            ir: &gdhpdffd9de252::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HWSEM",
        address: 0x58026400,
        registers: Some(PeripheralRegisters {
            kind: "gdhwsem5325a440",
            version: "v1",
            block: "HWSEM",
            ir: &gdhwsem5325a440::REGISTERS,
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
                pin: "PB4",
                signal: "SMBA",
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
                pin: "PB6",
                signal: "SDA",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(43),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(44),
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
                pin: "PB11",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
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
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(45),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(46),
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
                pin: "PC9",
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
                request: Some(87),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(88),
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
        name: "I2C4",
        address: 0x40005c00,
        registers: Some(PeripheralRegisters {
            kind: "gdi2c0cd973dc4",
            version: "v1",
            block: "I2C0",
            ir: &gdi2c0cd973dc4::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "SMBA",
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
            PeripheralPin {
                pin: "PB9",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SDA",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "SMBA",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "SCL",
                af: None,
            },
            PeripheralPin {
                pin: "PF15",
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
                request: Some(184),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(185),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "IPA",
        address: 0x52001000,
        registers: Some(PeripheralRegisters {
            kind: "gdipae01bd374",
            version: "v1",
            block: "IPA",
            ir: &gdipae01bd374::REGISTERS,
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
        address: 0x58004800,
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
        name: "LPDTS",
        address: 0x58006800,
        registers: Some(PeripheralRegisters {
            kind: "gdlpdtsa3b40577",
            version: "v1",
            block: "LPDTS",
            ir: &gdlpdtsa3b40577::REGISTERS,
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
        address: 0x50001000,
        registers: Some(PeripheralRegisters {
            kind: "gdtli3a8126bb",
            version: "v1",
            block: "TLI",
            ir: &gdtli3a8126bb::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA1",
                signal: "R2",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "B4",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "G7",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "R4",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "G7",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "R5",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "B6",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "R3",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "R1",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "B5",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "VSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "R4",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "G2",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "VSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "B3",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "R6",
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "R5",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "G1",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "R3",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "R6",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "G0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "R6",
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "G4",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "G5",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "G7",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "B5",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "B5",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "B6",
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "B7",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "G2",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "R5",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "G5",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "B1",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "R2",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "B4",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "R6",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "R7",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "R3",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "HSYNC",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "G6",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "G3",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "B1",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "B3",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "B2",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "B7",
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
                pin: "PD7",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "R0",
                af: None,
            },
            PeripheralPin {
                pin: "PE1",
                signal: "R6",
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
                signal: "HSYNC",
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
                pin: "PF4",
                signal: "PIXCLK",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "G7",
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
                pin: "PG11",
                signal: "B3",
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
                pin: "PG13",
                signal: "R0",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "B0",
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
                pin: "PG8",
                signal: "G7",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "MDIO",
        address: 0x40009400,
        registers: Some(PeripheralRegisters {
            kind: "gdmdio2685003f",
            version: "v1",
            block: "MDIO",
            ir: &gdmdio2685003f::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "MDMA",
        address: 0x52000000,
        registers: Some(PeripheralRegisters {
            kind: "gdmdmab9a60aaf",
            version: "v1",
            block: "MDMA",
            ir: &gdmdmab9a60aaf::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPI0",
        address: 0x52005000,
        registers: Some(PeripheralRegisters {
            kind: "gdospi0439e0312",
            version: "v1",
            block: "OSPI0",
            ir: &gdospi0439e0312::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPI1",
        address: 0x5200a000,
        registers: Some(PeripheralRegisters {
            kind: "gdospi0439e0312",
            version: "v1",
            block: "OSPI0",
            ir: &gdospi0439e0312::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "OSPIM",
        address: 0x5200b400,
        registers: Some(PeripheralRegisters {
            kind: "gdospim63e4b4c4",
            version: "v1",
            block: "OSPIM",
            ir: &gdospim63e4b4c4::REGISTERS,
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
        address: 0x58005800,
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
        pins: &[
            PeripheralPin {
                pin: "PA7",
                signal: "DEEPSLEEP",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "DEEPSLEEP",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "SLEEP",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SLEEP",
                af: None,
            },
        ],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMECCMU0",
        address: 0x52009000,
        registers: Some(PeripheralRegisters {
            kind: "gdrameccmu0d260ef4c",
            version: "v1",
            block: "RAMECCMU0",
            ir: &gdrameccmu0d260ef4c::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMECCMU1",
        address: 0x48023000,
        registers: Some(PeripheralRegisters {
            kind: "gdrameccmu1ba654536",
            version: "v1",
            block: "RAMECCMU1",
            ir: &gdrameccmu1ba654536::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RCC",
        address: 0x58024400,
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
            interrupt: "RCC",
        }],
        afio: None,
    },
    Peripheral {
        name: "RNG",
        address: 0x48021800,
        registers: Some(PeripheralRegisters {
            kind: "gdtrng6bc6a907",
            version: "v1",
            block: "TRNG",
            ir: &gdtrng6bc6a907::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RSPDIF",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "gdrspdif9ce23832",
            version: "v1",
            block: "RSPDIF",
            ir: &gdrspdif9ce23832::REGISTERS,
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
        address: 0x58004000,
        registers: Some(PeripheralRegisters {
            kind: "gdrtcc8139290",
            version: "v1",
            block: "RTC",
            ir: &gdrtcc8139290::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "REFIN",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "REFIN",
                af: None,
            },
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
                pin: "PB2",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TAMP1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "OUT",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "TAMP0",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
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
        name: "RTDEC0",
        address: 0x5200b800,
        registers: Some(PeripheralRegisters {
            kind: "gdrtdec0fa1e67ae",
            version: "v1",
            block: "RTDEC0",
            ir: &gdrtdec0fa1e67ae::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RTDEC1",
        address: 0x5200bc00,
        registers: Some(PeripheralRegisters {
            kind: "gdrtdec1b5caa4c1",
            version: "v1",
            block: "RTDEC1",
            ir: &gdrtdec1b5caa4c1::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x40015800,
        registers: Some(PeripheralRegisters {
            kind: "gdsai06e25733b",
            version: "v1",
            block: "SAI0",
            ir: &gdsai06e25733b::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "DAT1",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "DAT1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS0",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK1",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS1",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "MCLK0",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "B0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(101),
            },
            PeripheralDmaChannel {
                signal: "B1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(102),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI2",
        address: 0x40015c00,
        registers: Some(PeripheralRegisters {
            kind: "gdsai06e25733b",
            version: "v1",
            block: "SAI0",
            ir: &gdsai06e25733b::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MCLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "FS1",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCK1",
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "FS1",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "FS1",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "DAT1",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "FS0",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SCK0",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PD8",
                signal: "CLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PD9",
                signal: "CLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE0",
                signal: "MCLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "SCK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE13",
                signal: "FS1",
                af: None,
            },
            PeripheralPin {
                pin: "PE14",
                signal: "MCLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "MCLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "FS1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "B0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(178),
            },
            PeripheralDmaChannel {
                signal: "B1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(179),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI3",
        address: 0x40016000,
        registers: Some(PeripheralRegisters {
            kind: "gdsai06e25733b",
            version: "v1",
            block: "SAI0",
            ir: &gdsai06e25733b::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PD6",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE2",
                signal: "MCLK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "DAT1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "FS0",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "SCK0",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "DAT0",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SD0",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "DAT2",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "SD1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "MCLK1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "SCK1",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "FS1",
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "B0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(180),
            },
            PeripheralDmaChannel {
                signal: "B1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(181),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SDIO0",
        address: 0x52007000,
        registers: Some(PeripheralRegisters {
            kind: "gdsdio042a58275",
            version: "v1",
            block: "SDIO0",
            ir: &gdsdio042a58275::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SDIO1",
        address: 0x48022400,
        registers: Some(PeripheralRegisters {
            kind: "gdsdio042a58275",
            version: "v1",
            block: "SDIO0",
            ir: &gdsdio042a58275::REGISTERS,
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
            kind: "gdspi0a7377dd5",
            version: "v1",
            block: "SPI0",
            ir: &gdspi0a7377dd5::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "NSS",
                af: None,
            },
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
                pin: "PD6",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PG10",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PG11",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "MISO",
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
            kind: "gdspi1356222e3",
            version: "v1",
            block: "SPI1",
            ir: &gdspi1356222e3::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SCK",
                af: None,
            },
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
                pin: "PB11",
                signal: "NSS",
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
                pin: "PB15",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
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
                signal: "MOSI",
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
                pin: "PG2",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "MOSI",
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
                request: Some(49),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(50),
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
            kind: "gdspi255cb8c1f",
            version: "v1",
            block: "SPI2",
            ir: &gdspi255cb8c1f::REGISTERS,
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
                pin: "PC10",
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
                pin: "PC12",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(75),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(76),
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
            kind: "gdspi3e9b78823",
            version: "v1",
            block: "SPI3",
            ir: &gdspi3e9b78823::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "NSS",
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
                pin: "PE8",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "IO3",
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
                request: Some(97),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(98),
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
            kind: "gdspi4af049e38",
            version: "v1",
            block: "SPI4",
            ir: &gdspi4af049e38::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PF11",
                signal: "MOSI",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "IO2",
                af: None,
            },
            PeripheralPin {
                pin: "PF15",
                signal: "IO3",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MOSI",
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
                request: Some(99),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(100),
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
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "gdspi5c82f56e6",
            version: "v1",
            block: "SPI5",
            ir: &gdspi5c82f56e6::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
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
                pin: "PC12",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "MISO",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "SCK",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(182),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(183),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SYSCFG",
        address: 0x58000400,
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
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MCH0",
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
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "MCH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "MCH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PE12",
                signal: "MCH2",
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
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PE5",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE8",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PG5",
                signal: "ETR",
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
                request: Some(11),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(12),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(13),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(14),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(21),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(15),
            },
            PeripheralDmaChannel {
                signal: "MCH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(16),
            },
            PeripheralDmaChannel {
                signal: "MCH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(17),
            },
            PeripheralDmaChannel {
                signal: "MCH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(18),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(20),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(19),
            },
        ],
        triggers: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
        ],
        afio: None,
    },
    Peripheral {
        name: "TIM15",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
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
            PeripheralPin {
                pin: "PC11",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PE3",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PE4",
                signal: "MCH0",
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
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(109),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(110),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(114),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(111),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(113),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(112),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM16",
        address: 0x40014400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer155d5134ba",
            version: "v1",
            block: "TIMER15",
            ir: &gdtimer155d5134ba::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF10",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MCH0",
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
                request: Some(115),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(116),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(118),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM17",
        address: 0x40014800,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer155d5134ba",
            version: "v1",
            block: "TIMER15",
            ir: &gdtimer155d5134ba::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "MCH0",
                af: None,
            },
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
                pin: "PF7",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "BRKIN0",
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
                request: Some(119),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(120),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(122),
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
                signal: "CH2",
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
                pin: "PA14",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "ETR",
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
                pin: "PA2",
                signal: "CH4",
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
                pin: "PB3",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH2",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(22),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(23),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(24),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(25),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(27),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(26),
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
        name: "TIM23",
        address: 0x4000e000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1da3bc56a",
            version: "v1",
            block: "TIMER1",
            ir: &gdtimer1da3bc56a::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PF2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PF3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PG12",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "ETR",
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
                request: Some(128),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(129),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(130),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(131),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(134),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(132),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM24",
        address: 0x4000e400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1da3bc56a",
            version: "v1",
            block: "TIMER1",
            ir: &gdtimer1da3bc56a::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PF11",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PF14",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "ETR",
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
                request: Some(135),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(136),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(137),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(138),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(141),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(139),
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
                pin: "PB4",
                signal: "CH2",
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
                pin: "PC7",
                signal: "CH3",
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
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD1",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH1",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(29),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(30),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(31),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(32),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(35),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(33),
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
        name: "TIM31",
        address: 0x4000e800,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer27201f8c9",
            version: "v1",
            block: "TIMER2",
            ir: &gdtimer27201f8c9::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PG5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PG6",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG7",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "ETR",
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
                request: Some(142),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(143),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(144),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(145),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(148),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(146),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM32",
        address: 0x4000ec00,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer27201f8c9",
            version: "v1",
            block: "TIMER2",
            ir: &gdtimer27201f8c9::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PG0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PG1",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "ETR",
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
                request: Some(149),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(150),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(151),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(152),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(155),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(154),
            },
        ],
        triggers: &[],
        interrupts: &[],
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
                pin: "PB5",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
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
                pin: "PD11",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH2",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(36),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(37),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(38),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(39),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(41),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(42),
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
        name: "TIM41",
        address: 0x4001d000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
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
                request: Some(156),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(189),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(158),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(157),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(190),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(159),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM42",
        address: 0x4001d400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD12",
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
                request: Some(160),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(191),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(162),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(161),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(192),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(163),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM43",
        address: 0x4001d800,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PD13",
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
                request: Some(164),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(193),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(166),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(165),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(194),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(167),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM44",
        address: 0x4001dc00,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[PeripheralPin {
            pin: "PD14",
            signal: "CH2",
            af: None,
        }],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(168),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(195),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(170),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(169),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(196),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(171),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM45",
        address: 0x4001f000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer1457881844",
            version: "v1",
            block: "TIMER14",
            ir: &gdtimer1457881844::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD15",
                signal: "CH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG13",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PG14",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PG15",
                signal: "BRKIN0",
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
                request: Some(172),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(197),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(174),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(173),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(198),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(175),
            },
        ],
        triggers: &[],
        interrupts: &[],
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
                pin: "PA0",
                signal: "CH2",
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
                pin: "PA2",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
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
                request: Some(68),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(69),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(70),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(71),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(74),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(72),
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
        name: "TIM51",
        address: 0x4000f000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer5071732508",
            version: "v1",
            block: "TIMER50",
            ir: &gdtimer5071732508::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(176),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM52",
        address: 0x4000f400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer5071732508",
            version: "v1",
            block: "TIMER50",
            ir: &gdtimer5071732508::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(177),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer5330a987e",
            version: "v1",
            block: "TIMER5",
            ir: &gdtimer5330a987e::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(83),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer5330a987e",
            version: "v1",
            block: "TIMER5",
            ir: &gdtimer5330a987e::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: None,
            dmamux: Some("DMAMUX1"),
            remap: &[],
            dma: None,
            request: Some(84),
        }],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIM8",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "gdtimer03afad14d",
            version: "v1",
            block: "TIMER0",
            ir: &gdtimer03afad14d::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MCH0",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "MCH1",
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
                pin: "PC7",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PD0",
                signal: "CH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD10",
                signal: "MCH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD11",
                signal: "MCH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH1",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "MCH3",
                af: None,
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH4",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "MCH1",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
                signal: "MCH2",
                af: None,
            },
            PeripheralPin {
                pin: "PG2",
                signal: "BRKIN0",
                af: None,
            },
            PeripheralPin {
                pin: "PG3",
                signal: "BRKIN2",
                af: None,
            },
            PeripheralPin {
                pin: "PG4",
                signal: "BRKIN1",
                af: None,
            },
            PeripheralPin {
                pin: "PG8",
                signal: "ETR",
                af: None,
            },
            PeripheralPin {
                pin: "PG9",
                signal: "BRKIN1",
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
                request: Some(57),
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(58),
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(59),
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(60),
            },
            PeripheralDmaChannel {
                signal: "CMT",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(67),
            },
            PeripheralDmaChannel {
                signal: "MCH0",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(61),
            },
            PeripheralDmaChannel {
                signal: "MCH1",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(62),
            },
            PeripheralDmaChannel {
                signal: "MCH2",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(63),
            },
            PeripheralDmaChannel {
                signal: "MCH3",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(64),
            },
            PeripheralDmaChannel {
                signal: "TRG",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(66),
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(65),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TMU",
        address: 0x48024400,
        registers: Some(PeripheralRegisters {
            kind: "gdtmucbc214df",
            version: "v1",
            block: "TMU",
            ir: &gdtmucbc214df::REGISTERS,
        }),
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
        registers: Some(PeripheralRegisters {
            kind: "gdtrigseldfb10546",
            version: "v1",
            block: "TRIGSEL",
            ir: &gdtrigseldfb10546::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "gduart330e38640",
            version: "v1",
            block: "UART3",
            ir: &gduart330e38640::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "RX",
                af: None,
            },
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
                pin: "PA13",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
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
                pin: "PC5",
                signal: "CTS",
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
                pin: "PF4",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
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
                request: Some(77),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(78),
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
            kind: "gduart330e38640",
            version: "v1",
            block: "UART3",
            ir: &gduart330e38640::REGISTERS,
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
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(79),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(80),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART7",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "gduart330e38640",
            version: "v1",
            block: "UART3",
            ir: &gduart330e38640::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CTS",
                af: None,
            },
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
                pin: "PE9",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PF8",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PF9",
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
                request: Some(93),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(94),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART8",
        address: 0x40007c00,
        registers: Some(PeripheralRegisters {
            kind: "gduart330e38640",
            version: "v1",
            block: "UART3",
            ir: &gduart330e38640::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PD15",
                signal: "RTS",
                af: None,
            },
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(95),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(96),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART1",
        address: 0x40011000,
        registers: Some(PeripheralRegisters {
            kind: "gdusart0626fb765",
            version: "v1",
            block: "USART0",
            ir: &gdusart0626fb765::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
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
                pin: "PB13",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
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
            PeripheralPin {
                pin: "PF4",
                signal: "TX",
                af: None,
            },
            PeripheralPin {
                pin: "PF5",
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
                request: Some(51),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(52),
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
            kind: "gdusart0626fb765",
            version: "v1",
            block: "USART0",
            ir: &gdusart0626fb765::REGISTERS,
        }),
        rcc: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                af: None,
            },
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
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
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
                pin: "PD3",
                signal: "CTS",
                af: None,
            },
            PeripheralPin {
                pin: "PD4",
                signal: "DE",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(53),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(54),
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
            kind: "gdusart0626fb765",
            version: "v1",
            block: "USART0",
            ir: &gdusart0626fb765::REGISTERS,
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
                signal: "DE",
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
                signal: "DE",
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
            PeripheralPin {
                pin: "PF4",
                signal: "DE",
                af: None,
            },
            PeripheralPin {
                pin: "PF4",
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
                request: Some(55),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(56),
            },
        ],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART6",
        address: 0x40011400,
        registers: Some(PeripheralRegisters {
            kind: "gdusart0626fb765",
            version: "v1",
            block: "USART0",
            ir: &gdusart0626fb765::REGISTERS,
        }),
        rcc: None,
        pins: &[
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
                signal: "DE",
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
                signal: "DE",
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
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(85),
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: None,
                dmamux: Some("DMAMUX1"),
                remap: &[],
                dma: None,
                request: Some(86),
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
        name: "USBHS0_DEVICE",
        address: 0x40040800,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs0deviced0449d15",
            version: "v1",
            block: "USBHS0_DEVICE",
            ir: &gdusbhs0deviced0449d15::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_GLOBAL",
        address: 0x40040000,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs0globalbee3a389",
            version: "v1",
            block: "USBHS0_GLOBAL",
            ir: &gdusbhs0globalbee3a389::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_HOST",
        address: 0x40040400,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs0host663109ac",
            version: "v1",
            block: "USBHS0_HOST",
            ir: &gdusbhs0host663109ac::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS0_PWRCLK",
        address: 0x40040e00,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs0pwrclk0f97dd8b",
            version: "v1",
            block: "USBHS0_PWRCLK",
            ir: &gdusbhs0pwrclk0f97dd8b::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_DEVICE",
        address: 0x40080800,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs1device9d406887",
            version: "v1",
            block: "USBHS1_DEVICE",
            ir: &gdusbhs1device9d406887::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_GLOBAL",
        address: 0x40080000,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs1globalb3d6824e",
            version: "v1",
            block: "USBHS1_GLOBAL",
            ir: &gdusbhs1globalb3d6824e::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_HOST",
        address: 0x40080400,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs1host14113081",
            version: "v1",
            block: "USBHS1_HOST",
            ir: &gdusbhs1host14113081::REGISTERS,
        }),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USBHS1_PWRCLK",
        address: 0x40080e00,
        registers: Some(PeripheralRegisters {
            kind: "gdusbhs1pwrclk29fc276e",
            version: "v1",
            block: "USBHS1_PWRCLK",
            ir: &gdusbhs1pwrclk29fc276e::REGISTERS,
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
        address: 0x58003c00,
        registers: Some(PeripheralRegisters {
            kind: "gdvref193fa1c3",
            version: "v1",
            block: "VREF",
            ir: &gdvref193fa1c3::REGISTERS,
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
        address: 0x50003000,
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
    Interrupt {
        name: "VAVD_LVD_VOVD",
        number: 1,
    },
    Interrupt {
        name: "TAMPER_STAMP_LXTAL",
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
    Interrupt { name: "RCC", number: 5 },
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
        name: "ADC1_2",
        number: 18,
    },
    Interrupt {
        name: "EXTI5_9",
        number: 23,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 24,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 25,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
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
        name: "EXTI10_15",
        number: 40,
    },
    Interrupt {
        name: "RTC_ALARM",
        number: 41,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 43,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 44,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
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
        name: "SDIO0",
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
        name: "TIM6_DAC1_UDR",
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
        name: "ENET0",
        number: 61,
    },
    Interrupt {
        name: "ENET0_WKUP",
        number: 62,
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
        name: "USBHS0_EP1_OUT",
        number: 74,
    },
    Interrupt {
        name: "USBHS0_EP1_IN",
        number: 75,
    },
    Interrupt {
        name: "USBHS0_WKUP",
        number: 76,
    },
    Interrupt {
        name: "USBHS0",
        number: 77,
    },
    Interrupt {
        name: "DCMI",
        number: 78,
    },
    Interrupt {
        name: "CAU",
        number: 79,
    },
    Interrupt {
        name: "HAU_RNG",
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
        name: "SAI1",
        number: 87,
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
    Interrupt {
        name: "SAI2",
        number: 91,
    },
    Interrupt {
        name: "OSPI0",
        number: 92,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 95,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 96,
    },
    Interrupt {
        name: "RSPDIF",
        number: 97,
    },
    Interrupt {
        name: "DMAMUX1_OVR",
        number: 102,
    },
    Interrupt {
        name: "HPDF_INT0",
        number: 110,
    },
    Interrupt {
        name: "HPDF_INT1",
        number: 111,
    },
    Interrupt {
        name: "HPDF_INT2",
        number: 112,
    },
    Interrupt {
        name: "HPDF_INT3",
        number: 113,
    },
    Interrupt {
        name: "SAI3",
        number: 114,
    },
    Interrupt {
        name: "TIM15",
        number: 116,
    },
    Interrupt {
        name: "TIM16",
        number: 117,
    },
    Interrupt {
        name: "TIM17",
        number: 118,
    },
    Interrupt {
        name: "MDIO",
        number: 120,
    },
    Interrupt {
        name: "MDMA",
        number: 122,
    },
    Interrupt {
        name: "SDIO1",
        number: 124,
    },
    Interrupt {
        name: "HWSEM",
        number: 125,
    },
    Interrupt {
        name: "ADC3",
        number: 127,
    },
    Interrupt {
        name: "CMP0_1",
        number: 137,
    },
    Interrupt {
        name: "CRS",
        number: 144,
    },
    Interrupt {
        name: "RAMECCMU",
        number: 145,
    },
    Interrupt {
        name: "OSPI1",
        number: 150,
    },
    Interrupt {
        name: "RTDEC0",
        number: 151,
    },
    Interrupt {
        name: "RTDEC1",
        number: 152,
    },
    Interrupt {
        name: "FAC",
        number: 153,
    },
    Interrupt {
        name: "TMU",
        number: 154,
    },
    Interrupt {
        name: "TIM23",
        number: 161,
    },
    Interrupt {
        name: "TIM24",
        number: 162,
    },
    Interrupt {
        name: "TIM31",
        number: 163,
    },
    Interrupt {
        name: "TIM32",
        number: 164,
    },
    Interrupt {
        name: "TIM41",
        number: 165,
    },
    Interrupt {
        name: "TIM42",
        number: 166,
    },
    Interrupt {
        name: "TIM43",
        number: 167,
    },
    Interrupt {
        name: "TIM44",
        number: 168,
    },
    Interrupt {
        name: "TIM45",
        number: 169,
    },
    Interrupt {
        name: "TIM51",
        number: 170,
    },
    Interrupt {
        name: "TIM52",
        number: 171,
    },
    Interrupt {
        name: "USBHS1_EP1_OUT",
        number: 172,
    },
    Interrupt {
        name: "USBHS1_EP1_IN",
        number: 173,
    },
    Interrupt {
        name: "USBHS1_WKUP",
        number: 174,
    },
    Interrupt {
        name: "USBHS1",
        number: 175,
    },
    Interrupt {
        name: "ENET1",
        number: 176,
    },
    Interrupt {
        name: "ENET1_WKUP",
        number: 177,
    },
    Interrupt {
        name: "CAN1_WKUP",
        number: 179,
    },
    Interrupt {
        name: "CAN1_MESSAGE",
        number: 180,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 181,
    },
    Interrupt {
        name: "CAN1_ERROR",
        number: 182,
    },
    Interrupt {
        name: "CAN1_FASTERROR",
        number: 183,
    },
    Interrupt {
        name: "CAN1_TEC",
        number: 184,
    },
    Interrupt {
        name: "CAN1_REC",
        number: 185,
    },
    Interrupt {
        name: "CAN2_WKUP",
        number: 186,
    },
    Interrupt {
        name: "CAN2_MESSAGE",
        number: 187,
    },
    Interrupt {
        name: "CAN2_BUSOFF",
        number: 188,
    },
    Interrupt {
        name: "CAN2_ERROR",
        number: 189,
    },
    Interrupt {
        name: "CAN2_FASTERROR",
        number: 190,
    },
    Interrupt {
        name: "CAN2_TEC",
        number: 191,
    },
    Interrupt {
        name: "CAN2_REC",
        number: 192,
    },
    Interrupt {
        name: "CAN3_WKUP",
        number: 193,
    },
    Interrupt {
        name: "CAN3_MESSAGE",
        number: 194,
    },
    Interrupt {
        name: "CAN3_BUSOFF",
        number: 195,
    },
    Interrupt {
        name: "CAN3_ERROR",
        number: 196,
    },
    Interrupt {
        name: "CAN3_FASTERROR",
        number: 197,
    },
    Interrupt {
        name: "CAN3_TEC",
        number: 198,
    },
    Interrupt {
        name: "CAN3_REC",
        number: 199,
    },
    Interrupt {
        name: "EFUSE",
        number: 200,
    },
    Interrupt {
        name: "I2C1_WKUP",
        number: 201,
    },
    Interrupt {
        name: "I2C2_WKUP",
        number: 202,
    },
    Interrupt {
        name: "I2C3_WKUP",
        number: 203,
    },
    Interrupt {
        name: "I2C4_WKUP",
        number: 204,
    },
    Interrupt {
        name: "LPDTS",
        number: 205,
    },
    Interrupt {
        name: "LPDTS_WKUP",
        number: 206,
    },
    Interrupt {
        name: "TIM1_DEC",
        number: 207,
    },
    Interrupt {
        name: "TIM8_DEC",
        number: 208,
    },
    Interrupt {
        name: "TIM2_DEC",
        number: 209,
    },
    Interrupt {
        name: "TIM3_DEC",
        number: 210,
    },
    Interrupt {
        name: "TIM4_DEC",
        number: 211,
    },
    Interrupt {
        name: "TIM5_DEC",
        number: 212,
    },
    Interrupt {
        name: "TIM23_DEC",
        number: 213,
    },
    Interrupt {
        name: "TIM24_DEC",
        number: 214,
    },
    Interrupt {
        name: "TIM31_DEC",
        number: 215,
    },
    Interrupt {
        name: "TIM32_DEC",
        number: 216,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH0",
        dma: "DMA1",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(0),
    },
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(1),
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(2),
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(3),
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(4),
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(5),
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(6),
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(7),
    },
    DmaChannel {
        name: "DMA2_CH0",
        dma: "DMA2",
        channel: 0,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(8),
    },
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 1,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(9),
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 2,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(10),
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 3,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(11),
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 4,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(12),
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 5,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(13),
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 6,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(14),
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 7,
        dmamux: Some("DMAMUX1"),
        dmamux_channel: Some(15),
    },
];
pub(crate) static PINS: &[Pin] = &[
    Pin { name: "PA0" },
    Pin { name: "PA1" },
    Pin { name: "PA10" },
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
];
#[path = "../registers/crc_v1.rs"]
pub mod crc;
#[path = "../registers/dma_v2.rs"]
pub mod dma;
#[path = "../registers/flash_f4.rs"]
pub mod flash;
#[path = "../registers/gdadc06d279556_v1.rs"]
pub mod gdadc06d279556;
#[path = "../registers/gdadc1425a4aff_v1.rs"]
pub mod gdadc1425a4aff;
#[path = "../registers/gdadc2efea3dc8_v1.rs"]
pub mod gdadc2efea3dc8;
#[path = "../registers/gdaxi7000de15_v1.rs"]
pub mod gdaxi7000de15;
#[path = "../registers/gdcan0ab6ea0b5_v1.rs"]
pub mod gdcan0ab6ea0b5;
#[path = "../registers/gdcau3fafd38d_v1.rs"]
pub mod gdcau3fafd38d;
#[path = "../registers/gdcmp65ef540c_v1.rs"]
pub mod gdcmp65ef540c;
#[path = "../registers/gdcpdmsdio04a9ee533_v1.rs"]
pub mod gdcpdmsdio04a9ee533;
#[path = "../registers/gdctcdb80f1ce_v1.rs"]
pub mod gdctcdb80f1ce;
#[path = "../registers/gddac555b6194_v1.rs"]
pub mod gddac555b6194;
#[path = "../registers/gddbgefa81966_v1.rs"]
pub mod gddbgefa81966;
#[path = "../registers/gddci5ae31085_v1.rs"]
pub mod gddci5ae31085;
#[path = "../registers/gddmamuxeaace10d_v1.rs"]
pub mod gddmamuxeaace10d;
#[path = "../registers/gdedoutfebca4f4_v1.rs"]
pub mod gdedoutfebca4f4;
#[path = "../registers/gdefuse25c60075_v1.rs"]
pub mod gdefuse25c60075;
#[path = "../registers/gdenet0dma7d3e05fd_v1.rs"]
pub mod gdenet0dma7d3e05fd;
#[path = "../registers/gdenet0macd2855220_v1.rs"]
pub mod gdenet0macd2855220;
#[path = "../registers/gdenet0macfcthffd74812_v1.rs"]
pub mod gdenet0macfcthffd74812;
#[path = "../registers/gdenet0msc2451d465_v1.rs"]
pub mod gdenet0msc2451d465;
#[path = "../registers/gdenet0ptpc700182c_v1.rs"]
pub mod gdenet0ptpc700182c;
#[path = "../registers/gdenet1dmabfdb3976_v1.rs"]
pub mod gdenet1dmabfdb3976;
#[path = "../registers/gdenet1maceef08a3b_v1.rs"]
pub mod gdenet1maceef08a3b;
#[path = "../registers/gdenet1macfcthacf2ccdd_v1.rs"]
pub mod gdenet1macfcthacf2ccdd;
#[path = "../registers/gdenet1msc4852d4b8_v1.rs"]
pub mod gdenet1msc4852d4b8;
#[path = "../registers/gdenet1ptpedbe1f92_v1.rs"]
pub mod gdenet1ptpedbe1f92;
#[path = "../registers/gdexmc293e9145_v1.rs"]
pub mod gdexmc293e9145;
#[path = "../registers/gdextic827d627_v1.rs"]
pub mod gdextic827d627;
#[path = "../registers/gdfac96d60f19_v1.rs"]
pub mod gdfac96d60f19;
#[path = "../registers/gdgpioc18dfc69f_v1.rs"]
pub mod gdgpioc18dfc69f;
#[path = "../registers/gdhaub8125197_v1.rs"]
pub mod gdhaub8125197;
#[path = "../registers/gdhpdffd9de252_v1.rs"]
pub mod gdhpdffd9de252;
#[path = "../registers/gdhwsem5325a440_v1.rs"]
pub mod gdhwsem5325a440;
#[path = "../registers/gdi2c0cd973dc4_v1.rs"]
pub mod gdi2c0cd973dc4;
#[path = "../registers/gdipae01bd374_v1.rs"]
pub mod gdipae01bd374;
#[path = "../registers/gdlpdtsa3b40577_v1.rs"]
pub mod gdlpdtsa3b40577;
#[path = "../registers/gdmdio2685003f_v1.rs"]
pub mod gdmdio2685003f;
#[path = "../registers/gdmdmab9a60aaf_v1.rs"]
pub mod gdmdmab9a60aaf;
#[path = "../registers/gdospi0439e0312_v1.rs"]
pub mod gdospi0439e0312;
#[path = "../registers/gdospim63e4b4c4_v1.rs"]
pub mod gdospim63e4b4c4;
#[path = "../registers/gdrameccmu0d260ef4c_v1.rs"]
pub mod gdrameccmu0d260ef4c;
#[path = "../registers/gdrameccmu1ba654536_v1.rs"]
pub mod gdrameccmu1ba654536;
#[path = "../registers/gdrspdif9ce23832_v1.rs"]
pub mod gdrspdif9ce23832;
#[path = "../registers/gdrtcc8139290_v1.rs"]
pub mod gdrtcc8139290;
#[path = "../registers/gdrtdec0fa1e67ae_v1.rs"]
pub mod gdrtdec0fa1e67ae;
#[path = "../registers/gdrtdec1b5caa4c1_v1.rs"]
pub mod gdrtdec1b5caa4c1;
#[path = "../registers/gdsai06e25733b_v1.rs"]
pub mod gdsai06e25733b;
#[path = "../registers/gdsdio042a58275_v1.rs"]
pub mod gdsdio042a58275;
#[path = "../registers/gdspi0a7377dd5_v1.rs"]
pub mod gdspi0a7377dd5;
#[path = "../registers/gdspi1356222e3_v1.rs"]
pub mod gdspi1356222e3;
#[path = "../registers/gdspi255cb8c1f_v1.rs"]
pub mod gdspi255cb8c1f;
#[path = "../registers/gdspi3e9b78823_v1.rs"]
pub mod gdspi3e9b78823;
#[path = "../registers/gdspi4af049e38_v1.rs"]
pub mod gdspi4af049e38;
#[path = "../registers/gdspi5c82f56e6_v1.rs"]
pub mod gdspi5c82f56e6;
#[path = "../registers/gdtimer03afad14d_v1.rs"]
pub mod gdtimer03afad14d;
#[path = "../registers/gdtimer1457881844_v1.rs"]
pub mod gdtimer1457881844;
#[path = "../registers/gdtimer155d5134ba_v1.rs"]
pub mod gdtimer155d5134ba;
#[path = "../registers/gdtimer1da3bc56a_v1.rs"]
pub mod gdtimer1da3bc56a;
#[path = "../registers/gdtimer27201f8c9_v1.rs"]
pub mod gdtimer27201f8c9;
#[path = "../registers/gdtimer5071732508_v1.rs"]
pub mod gdtimer5071732508;
#[path = "../registers/gdtimer5330a987e_v1.rs"]
pub mod gdtimer5330a987e;
#[path = "../registers/gdtli3a8126bb_v1.rs"]
pub mod gdtli3a8126bb;
#[path = "../registers/gdtmucbc214df_v1.rs"]
pub mod gdtmucbc214df;
#[path = "../registers/gdtrigseldfb10546_v1.rs"]
pub mod gdtrigseldfb10546;
#[path = "../registers/gdtrng6bc6a907_v1.rs"]
pub mod gdtrng6bc6a907;
#[path = "../registers/gduart330e38640_v1.rs"]
pub mod gduart330e38640;
#[path = "../registers/gdusart0626fb765_v1.rs"]
pub mod gdusart0626fb765;
#[path = "../registers/gdusbhs0deviced0449d15_v1.rs"]
pub mod gdusbhs0deviced0449d15;
#[path = "../registers/gdusbhs0globalbee3a389_v1.rs"]
pub mod gdusbhs0globalbee3a389;
#[path = "../registers/gdusbhs0host663109ac_v1.rs"]
pub mod gdusbhs0host663109ac;
#[path = "../registers/gdusbhs0pwrclk0f97dd8b_v1.rs"]
pub mod gdusbhs0pwrclk0f97dd8b;
#[path = "../registers/gdusbhs1device9d406887_v1.rs"]
pub mod gdusbhs1device9d406887;
#[path = "../registers/gdusbhs1globalb3d6824e_v1.rs"]
pub mod gdusbhs1globalb3d6824e;
#[path = "../registers/gdusbhs1host14113081_v1.rs"]
pub mod gdusbhs1host14113081;
#[path = "../registers/gdusbhs1pwrclk29fc276e_v1.rs"]
pub mod gdusbhs1pwrclk29fc276e;
#[path = "../registers/gdvref193fa1c3_v1.rs"]
pub mod gdvref193fa1c3;
#[path = "../registers/gpio_v2.rs"]
pub mod gpio;
#[path = "../registers/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../registers/pwr_f4.rs"]
pub mod pwr;
#[path = "../registers/rcc_f4.rs"]
pub mod rcc;
#[path = "../registers/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../registers/timer_v1.rs"]
pub mod timer;
#[path = "../registers/wwdg_v1.rs"]
pub mod wwdg;
