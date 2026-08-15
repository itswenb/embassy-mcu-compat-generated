
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc07ce7e972",
                version: "v1",
                block: "ADC0",
                ir: &gdadc07ce7e972::REGISTERS,
            },
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
                kind: "gdadc446b6462",
                version: "v1",
                block: "ADC",
                ir: &gdadc446b6462::REGISTERS,
            },
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
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc446b6462",
                version: "v1",
                block: "ADC",
                ir: &gdadc446b6462::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "AXIIM",
        address: 0x51000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdaxiimafbe6630",
                version: "v1",
                block: "AXIIM",
                ir: &gdaxiimafbe6630::REGISTERS,
            },
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
                kind: "gdcana7cd7781",
                version: "v1",
                block: "CAN",
                ir: &gdcana7cd7781::REGISTERS,
            },
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
                kind: "gdcana7cd7781",
                version: "v1",
                block: "CAN",
                ir: &gdcana7cd7781::REGISTERS,
            },
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
                kind: "gdcana7cd7781",
                version: "v1",
                block: "CAN",
                ir: &gdcana7cd7781::REGISTERS,
            },
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
                kind: "gdcaufd3db2fb",
                version: "v1",
                block: "CAU",
                ir: &gdcaufd3db2fb::REGISTERS,
            },
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
        address: 0x58003800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcmp9ee5ac9c",
                version: "v1",
                block: "CMP",
                ir: &gdcmp9ee5ac9c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM_OSPI0",
        address: 0x52006000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcpdm37c60c16",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm37c60c16::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM_OSPI1",
        address: 0x5200b000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcpdm37c60c16",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm37c60c16::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "CPDM_SDIO0",
        address: 0x52008000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcpdm37c60c16",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm37c60c16::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdcpdm37c60c16",
                version: "v1",
                block: "CPDM",
                ir: &gdcpdm37c60c16::REGISTERS,
            },
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
        address: 0x58024c00,
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
        name: "CTC",
        address: 0x40008400,
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
        name: "DAC0",
        address: 0x40007400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddac523caf1a",
                version: "v1",
                block: "DAC",
                ir: &gddac523caf1a::REGISTERS,
            },
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
        address: 0xe00e1000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbgde2c0489",
                version: "v1",
                block: "DBG",
                ir: &gddbgde2c0489::REGISTERS,
            },
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
        address: 0x48020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddcid3cb6fbe",
                version: "v1",
                block: "DCI",
                ir: &gddcid3cb6fbe::REGISTERS,
            },
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
                kind: "gddmaf3ee856f",
                version: "v1",
                block: "DMA",
                ir: &gddmaf3ee856f::REGISTERS,
            },
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
                kind: "gddmaf3ee856f",
                version: "v1",
                block: "DMA",
                ir: &gddmaf3ee856f::REGISTERS,
            },
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
                kind: "gddmamux489b88ce",
                version: "v1",
                block: "DMAMUX",
                ir: &gddmamux489b88ce::REGISTERS,
            },
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
                kind: "gddmamuxrgchxcfgbase95458840",
                version: "v1",
                block: "DMAMUX_RG_CHXCFG_BASE",
                ir: &gddmamuxrgchxcfgbase95458840::REGISTERS,
            },
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
                kind: "gddmamuxrmchxcfgbase3c558169",
                version: "v1",
                block: "DMAMUX_RM_CHXCFG_BASE",
                ir: &gddmamuxrmchxcfgbase3c558169::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "DSI",
        address: 0x52003000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddsi76441603",
                version: "v1",
                block: "DSI",
                ir: &gddsi76441603::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDIM_AFMT",
        address: 0x4000c400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedimafmtc755000c",
                version: "v1",
                block: "EDIM_AFMT",
                ir: &gdedimafmtc755000c::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDIM_BISS",
        address: 0x4000d000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedimbiss1ab60e08",
                version: "v1",
                block: "EDIM_BISS",
                ir: &gdedimbiss1ab60e08::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDIM_ENDAT",
        address: 0x4000cc00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedimendat15ceb0e5",
                version: "v1",
                block: "EDIM_ENDAT",
                ir: &gdedimendat15ceb0e5::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDIM_HDSL",
        address: 0x4000d800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedimhdsl8f02a454",
                version: "v1",
                block: "EDIM_HDSL",
                ir: &gdedimhdsl8f02a454::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EDIM_TFMT",
        address: 0x4000c800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdedimtfmt3eab6256",
                version: "v1",
                block: "EDIM_TFMT",
                ir: &gdedimtfmt3eab6256::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdedout6974cab7",
                version: "v1",
                block: "EDOUT",
                ir: &gdedout6974cab7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET0",
        address: 0x40028000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenet38297eab",
                version: "v1",
                block: "ENET",
                ir: &gdenet38297eab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET1",
        address: 0x4002a000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenet38297eab",
                version: "v1",
                block: "ENET",
                ir: &gdenet38297eab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ESC",
        address: 0x5f000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdescd7927785",
                version: "v1",
                block: "ESC",
                ir: &gdescd7927785::REGISTERS,
            },
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
        address: 0x52004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc2e387b8a",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc2e387b8a::REGISTERS,
            },
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
        address: 0x58000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexti049f6534",
                version: "v1",
                block: "EXTI",
                ir: &gdexti049f6534::REGISTERS,
            },
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
        address: 0x48024000,
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
        name: "FWDGT",
        address: 0x58004800,
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
        address: 0x58020000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58020400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58020800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58020c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58021000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58021400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
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
        address: 0x58021800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPIOJ",
        address: 0x58022400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpio9a89eec9",
                version: "v1",
                block: "GPIO",
                ir: &gdgpio9a89eec9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "GPSI",
        address: 0x48020200,
        registers: Some(
            PeripheralRegisters {
                kind: "gdgpsi8a02f8c0",
                version: "v1",
                block: "GPSI",
                ir: &gdgpsi8a02f8c0::REGISTERS,
            },
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
        address: 0x48021400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhau7c50811e",
                version: "v1",
                block: "HAU",
                ir: &gdhau7c50811e::REGISTERS,
            },
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
                kind: "gdhpdffe1368b3",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffe1368b3::REGISTERS,
            },
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
                kind: "gdhpdffe1368b3",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffe1368b3::REGISTERS,
            },
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
                kind: "gdhpdffe1368b3",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffe1368b3::REGISTERS,
            },
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
                kind: "gdhpdffe1368b3",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffe1368b3::REGISTERS,
            },
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
                kind: "gdhpdffe1368b3",
                version: "v1",
                block: "HPDF",
                ir: &gdhpdffe1368b3::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdhwsem7ea96c46",
                version: "v1",
                block: "HWSEM",
                ir: &gdhwsem7ea96c46::REGISTERS,
            },
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
        name: "IPA",
        address: 0x52001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdipa61f09f1e",
                version: "v1",
                block: "IPA",
                ir: &gdipa61f09f1e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LAYER0",
        address: 0x50001084,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli1ba8a397",
                version: "v1",
                block: "TLI",
                ir: &gdtli1ba8a397::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "LAYER1",
        address: 0x50001104,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli1ba8a397",
                version: "v1",
                block: "TLI",
                ir: &gdtli1ba8a397::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdlpdts3307f0fd",
                version: "v1",
                block: "LPDTS",
                ir: &gdlpdts3307f0fd::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "MDIO",
        address: 0x40009400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdmdio596b2b27",
                version: "v1",
                block: "MDIO",
                ir: &gdmdio596b2b27::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdmdma7993bd09",
                version: "v1",
                block: "MDMA",
                ir: &gdmdma7993bd09::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "NVMC",
        address: 0x52002000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdnvmc9a5d07ef",
                version: "v1",
                block: "NVMC",
                ir: &gdnvmc9a5d07ef::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdospi71203c17",
                version: "v1",
                block: "OSPI",
                ir: &gdospi71203c17::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdospi71203c17",
                version: "v1",
                block: "OSPI",
                ir: &gdospi71203c17::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdospimad46f02a",
                version: "v1",
                block: "OSPIM",
                ir: &gdospimad46f02a::REGISTERS,
            },
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
        address: 0x48000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpkcauf9e1d63d",
                version: "v1",
                block: "PKCAU",
                ir: &gdpkcauf9e1d63d::REGISTERS,
            },
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
        address: 0x58005800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpmu64058cb9",
                version: "v1",
                block: "PMU",
                ir: &gdpmu64058cb9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RAMECCMU0",
        address: 0x52009000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrameccmu05ad7d731",
                version: "v1",
                block: "RAMECCMU0",
                ir: &gdrameccmu05ad7d731::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdrameccmu8cae911f",
                version: "v1",
                block: "RAMECCMU",
                ir: &gdrameccmu8cae911f::REGISTERS,
            },
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
        address: 0x58024400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrcu0591d20f",
                version: "v1",
                block: "RCU",
                ir: &gdrcu0591d20f::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RDCM",
        address: 0x40016400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrdcme04ef535",
                version: "v1",
                block: "RDCM",
                ir: &gdrdcme04ef535::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdrspdif22daf900",
                version: "v1",
                block: "RSPDIF",
                ir: &gdrspdif22daf900::REGISTERS,
            },
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
        address: 0x58004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrtc4f2c5d98",
                version: "v1",
                block: "RTC",
                ir: &gdrtc4f2c5d98::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "RTDEC0",
        address: 0x5200b800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrtdeca2be341e",
                version: "v1",
                block: "RTDEC",
                ir: &gdrtdeca2be341e::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdrtdeca2be341e",
                version: "v1",
                block: "RTDEC",
                ir: &gdrtdeca2be341e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI0",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsai81edec52",
                version: "v1",
                block: "SAI",
                ir: &gdsai81edec52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI1",
        address: 0x40015c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsai81edec52",
                version: "v1",
                block: "SAI",
                ir: &gdsai81edec52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI2",
        address: 0x40016000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsai81edec52",
                version: "v1",
                block: "SAI",
                ir: &gdsai81edec52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI3",
        address: 0x40016800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsai81edec52",
                version: "v1",
                block: "SAI",
                ir: &gdsai81edec52::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SDIO0",
        address: 0x52007000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsdiobd66f984",
                version: "v1",
                block: "SDIO",
                ir: &gdsdiobd66f984::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdsdiobd66f984",
                version: "v1",
                block: "SDIO",
                ir: &gdsdiobd66f984::REGISTERS,
            },
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
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
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
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
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
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI3",
        address: 0x40013400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI4",
        address: 0x40015000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SPI5",
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspia2feaee7",
                version: "v1",
                block: "SPI",
                ir: &gdspia2feaee7::REGISTERS,
            },
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
        address: 0x58000400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfgd19ad75d",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgd19ad75d::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER22",
        address: 0x4000e000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER23",
        address: 0x4000e400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER30",
        address: 0x4000e800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER31",
        address: 0x4000ec00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER40",
        address: 0x4001d000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER41",
        address: 0x4001d400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER42",
        address: 0x4001d800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER43",
        address: 0x4001dc00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER44",
        address: 0x4001f000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER50",
        address: 0x4000f000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER51",
        address: 0x4000f400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
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
        address: 0x40010400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimereb2fc375",
                version: "v1",
                block: "TIMER",
                ir: &gdtimereb2fc375::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TLI",
        address: 0x50001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli1ba8a397",
                version: "v1",
                block: "TLI",
                ir: &gdtli1ba8a397::REGISTERS,
            },
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
                kind: "gdtrigselc40c9296",
                version: "v1",
                block: "TRIGSEL",
                ir: &gdtrigselc40c9296::REGISTERS,
            },
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
                kind: "gdtrng89e226cf",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng89e226cf::REGISTERS,
            },
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
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
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
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART6",
        address: 0x40007800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "UART7",
        address: 0x40007c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
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
        address: 0x40011000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
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
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
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
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "USART5",
        address: 0x40011400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdusarte31b633e",
                version: "v1",
                block: "USART",
                ir: &gdusarte31b633e::REGISTERS,
            },
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
        address: 0x58003c00,
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
        address: 0x50003000,
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
        name: "VAVD_LVD_VOVD_VUVD",
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
        name: "NVMC",
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
        name: "ADC0_1_RDCM",
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
        name: "DMA0_CHANNEL7",
        number: 47,
    },
    Interrupt {
        name: "EXMC",
        number: 48,
    },
    Interrupt {
        name: "SDIO0",
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
        name: "TIMER5_DAC_UDR",
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
        name: "ENET0",
        number: 61,
    },
    Interrupt {
        name: "ENET0_WKUP",
        number: 62,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 68,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 69,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 70,
    },
    Interrupt {
        name: "USART5",
        number: 71,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 72,
    },
    Interrupt {
        name: "I2C2_ER",
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
        name: "UART6",
        number: 82,
    },
    Interrupt {
        name: "UART7",
        number: 83,
    },
    Interrupt {
        name: "SPI3",
        number: 84,
    },
    Interrupt {
        name: "SPI4",
        number: 85,
    },
    Interrupt {
        name: "SPI5",
        number: 86,
    },
    Interrupt {
        name: "SAI0",
        number: 87,
    },
    Interrupt {
        name: "TLI",
        number: 88,
    },
    Interrupt {
        name: "TLI_ER",
        number: 89,
    },
    Interrupt {
        name: "IPA",
        number: 90,
    },
    Interrupt {
        name: "SAI1",
        number: 91,
    },
    Interrupt {
        name: "OSPI0",
        number: 92,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 95,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 96,
    },
    Interrupt {
        name: "RSPDIF",
        number: 97,
    },
    Interrupt {
        name: "DMAMUX_OVR",
        number: 102,
    },
    Interrupt {
        name: "HPDF_INT4",
        number: 109,
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
        name: "SAI2",
        number: 114,
    },
    Interrupt {
        name: "TIMER14",
        number: 116,
    },
    Interrupt {
        name: "TIMER15",
        number: 117,
    },
    Interrupt {
        name: "TIMER16",
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
        name: "ADC2",
        number: 127,
    },
    Interrupt {
        name: "CMP0_1",
        number: 137,
    },
    Interrupt {
        name: "CTC",
        number: 144,
    },
    Interrupt {
        name: "RAMECCMU",
        number: 145,
    },
    Interrupt {
        name: "SAI3",
        number: 146,
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
        name: "FFT",
        number: 155,
    },
    Interrupt {
        name: "ESC_SYNC0",
        number: 156,
    },
    Interrupt {
        name: "ESC_SYNC1",
        number: 157,
    },
    Interrupt {
        name: "ESC",
        number: 158,
    },
    Interrupt {
        name: "TIMER22",
        number: 161,
    },
    Interrupt {
        name: "TIMER23",
        number: 162,
    },
    Interrupt {
        name: "TIMER30",
        number: 163,
    },
    Interrupt {
        name: "TIMER31",
        number: 164,
    },
    Interrupt {
        name: "TIMER40",
        number: 165,
    },
    Interrupt {
        name: "TIMER41",
        number: 166,
    },
    Interrupt {
        name: "TIMER42",
        number: 167,
    },
    Interrupt {
        name: "TIMER43",
        number: 168,
    },
    Interrupt {
        name: "TIMER44",
        number: 169,
    },
    Interrupt {
        name: "TIMER50",
        number: 170,
    },
    Interrupt {
        name: "TIMER51",
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
        name: "CAN0_WKUP",
        number: 179,
    },
    Interrupt {
        name: "CAN0_MESSAGE",
        number: 180,
    },
    Interrupt {
        name: "CAN0_BUSOFF",
        number: 181,
    },
    Interrupt {
        name: "CAN0_ERROR",
        number: 182,
    },
    Interrupt {
        name: "CAN0_FASTERROR",
        number: 183,
    },
    Interrupt {
        name: "CAN0_TEC",
        number: 184,
    },
    Interrupt {
        name: "CAN0_REC",
        number: 185,
    },
    Interrupt {
        name: "CAN1_WKUP",
        number: 186,
    },
    Interrupt {
        name: "CAN1_MESSAGE",
        number: 187,
    },
    Interrupt {
        name: "CAN1_BUSOFF",
        number: 188,
    },
    Interrupt {
        name: "CAN1_ERROR",
        number: 189,
    },
    Interrupt {
        name: "CAN1_FASTERROR",
        number: 190,
    },
    Interrupt {
        name: "CAN1_TEC",
        number: 191,
    },
    Interrupt {
        name: "CAN1_REC",
        number: 192,
    },
    Interrupt {
        name: "CAN2_WKUP",
        number: 193,
    },
    Interrupt {
        name: "CAN2_MESSAGE",
        number: 194,
    },
    Interrupt {
        name: "CAN2_BUSOFF",
        number: 195,
    },
    Interrupt {
        name: "CAN2_ERROR",
        number: 196,
    },
    Interrupt {
        name: "CAN2_FASTERROR",
        number: 197,
    },
    Interrupt {
        name: "CAN2_TEC",
        number: 198,
    },
    Interrupt {
        name: "CAN2_REC",
        number: 199,
    },
    Interrupt {
        name: "OTP",
        number: 200,
    },
    Interrupt {
        name: "I2C0_WKUP",
        number: 201,
    },
    Interrupt {
        name: "I2C1_WKUP",
        number: 202,
    },
    Interrupt {
        name: "I2C2_WKUP",
        number: 203,
    },
    Interrupt {
        name: "I2C3_WKUP",
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
        name: "TIMER0_DEC",
        number: 207,
    },
    Interrupt {
        name: "TIMER7_DEC",
        number: 208,
    },
    Interrupt {
        name: "TIMER1_DEC",
        number: 209,
    },
    Interrupt {
        name: "TIMER2_DEC",
        number: 210,
    },
    Interrupt {
        name: "TIMER3_DEC",
        number: 211,
    },
    Interrupt {
        name: "TIMER4_DEC",
        number: 212,
    },
    Interrupt {
        name: "TIMER22_DEC",
        number: 213,
    },
    Interrupt {
        name: "TIMER23_DEC",
        number: 214,
    },
    Interrupt {
        name: "TIMER30_DEC",
        number: 215,
    },
    Interrupt {
        name: "TIMER31_DEC",
        number: 216,
    },
    Interrupt {
        name: "CPU_DCACHE_ERR",
        number: 217,
    },
    Interrupt {
        name: "CPU_ICACHE_ERR",
        number: 218,
    },
    Interrupt {
        name: "VOVD_VUVD_FILTER",
        number: 219,
    },
    Interrupt {
        name: "TIMER0_CHANNEL_BRK",
        number: 221,
    },
    Interrupt {
        name: "TIMER7_CHANNEL_BRK",
        number: 222,
    },
    Interrupt {
        name: "DSI",
        number: 223,
    },
    Interrupt {
        name: "PKCAU",
        number: 224,
    },
    Interrupt {
        name: "ESC_BRIDGE",
        number: 225,
    },
    Interrupt {
        name: "EDIM",
        number: 226,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc07ce7e972_v1.rs"] pub mod gdadc07ce7e972;
#[path="../registers/gdadc446b6462_v1.rs"] pub mod gdadc446b6462;
#[path="../registers/gdaxiimafbe6630_v1.rs"] pub mod gdaxiimafbe6630;
#[path="../registers/gdcana7cd7781_v1.rs"] pub mod gdcana7cd7781;
#[path="../registers/gdcaufd3db2fb_v1.rs"] pub mod gdcaufd3db2fb;
#[path="../registers/gdcmp9ee5ac9c_v1.rs"] pub mod gdcmp9ee5ac9c;
#[path="../registers/gdcpdm37c60c16_v1.rs"] pub mod gdcpdm37c60c16;
#[path="../registers/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../registers/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../registers/gddac523caf1a_v1.rs"] pub mod gddac523caf1a;
#[path="../registers/gddbgde2c0489_v1.rs"] pub mod gddbgde2c0489;
#[path="../registers/gddcid3cb6fbe_v1.rs"] pub mod gddcid3cb6fbe;
#[path="../registers/gddmaf3ee856f_v1.rs"] pub mod gddmaf3ee856f;
#[path="../registers/gddmamux489b88ce_v1.rs"] pub mod gddmamux489b88ce;
#[path="../registers/gddmamuxrgchxcfgbase95458840_v1.rs"] pub mod gddmamuxrgchxcfgbase95458840;
#[path="../registers/gddmamuxrmchxcfgbase3c558169_v1.rs"] pub mod gddmamuxrmchxcfgbase3c558169;
#[path="../registers/gddsi76441603_v1.rs"] pub mod gddsi76441603;
#[path="../registers/gdedimafmtc755000c_v1.rs"] pub mod gdedimafmtc755000c;
#[path="../registers/gdedimbiss1ab60e08_v1.rs"] pub mod gdedimbiss1ab60e08;
#[path="../registers/gdedimendat15ceb0e5_v1.rs"] pub mod gdedimendat15ceb0e5;
#[path="../registers/gdedimhdsl8f02a454_v1.rs"] pub mod gdedimhdsl8f02a454;
#[path="../registers/gdedimtfmt3eab6256_v1.rs"] pub mod gdedimtfmt3eab6256;
#[path="../registers/gdedout6974cab7_v1.rs"] pub mod gdedout6974cab7;
#[path="../registers/gdenet38297eab_v1.rs"] pub mod gdenet38297eab;
#[path="../registers/gdescd7927785_v1.rs"] pub mod gdescd7927785;
#[path="../registers/gdexmc2e387b8a_v1.rs"] pub mod gdexmc2e387b8a;
#[path="../registers/gdexti049f6534_v1.rs"] pub mod gdexti049f6534;
#[path="../registers/gdfac70f604b5_v1.rs"] pub mod gdfac70f604b5;
#[path="../registers/gdfftc3717816_v1.rs"] pub mod gdfftc3717816;
#[path="../registers/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../registers/gdgpio9a89eec9_v1.rs"] pub mod gdgpio9a89eec9;
#[path="../registers/gdgpsi8a02f8c0_v1.rs"] pub mod gdgpsi8a02f8c0;
#[path="../registers/gdhau7c50811e_v1.rs"] pub mod gdhau7c50811e;
#[path="../registers/gdhpdffe1368b3_v1.rs"] pub mod gdhpdffe1368b3;
#[path="../registers/gdhwsem7ea96c46_v1.rs"] pub mod gdhwsem7ea96c46;
#[path="../registers/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../registers/gdipa61f09f1e_v1.rs"] pub mod gdipa61f09f1e;
#[path="../registers/gdlpdts3307f0fd_v1.rs"] pub mod gdlpdts3307f0fd;
#[path="../registers/gdmdio596b2b27_v1.rs"] pub mod gdmdio596b2b27;
#[path="../registers/gdmdma7993bd09_v1.rs"] pub mod gdmdma7993bd09;
#[path="../registers/gdnvmc9a5d07ef_v1.rs"] pub mod gdnvmc9a5d07ef;
#[path="../registers/gdospi71203c17_v1.rs"] pub mod gdospi71203c17;
#[path="../registers/gdospimad46f02a_v1.rs"] pub mod gdospimad46f02a;
#[path="../registers/gdpkcauf9e1d63d_v1.rs"] pub mod gdpkcauf9e1d63d;
#[path="../registers/gdpmu64058cb9_v1.rs"] pub mod gdpmu64058cb9;
#[path="../registers/gdrameccmu05ad7d731_v1.rs"] pub mod gdrameccmu05ad7d731;
#[path="../registers/gdrameccmu8cae911f_v1.rs"] pub mod gdrameccmu8cae911f;
#[path="../registers/gdrcu0591d20f_v1.rs"] pub mod gdrcu0591d20f;
#[path="../registers/gdrdcme04ef535_v1.rs"] pub mod gdrdcme04ef535;
#[path="../registers/gdrspdif22daf900_v1.rs"] pub mod gdrspdif22daf900;
#[path="../registers/gdrtc4f2c5d98_v1.rs"] pub mod gdrtc4f2c5d98;
#[path="../registers/gdrtdeca2be341e_v1.rs"] pub mod gdrtdeca2be341e;
#[path="../registers/gdsai81edec52_v1.rs"] pub mod gdsai81edec52;
#[path="../registers/gdsdiobd66f984_v1.rs"] pub mod gdsdiobd66f984;
#[path="../registers/gdspia2feaee7_v1.rs"] pub mod gdspia2feaee7;
#[path="../registers/gdsyscfgd19ad75d_v1.rs"] pub mod gdsyscfgd19ad75d;
#[path="../registers/gdtimereb2fc375_v1.rs"] pub mod gdtimereb2fc375;
#[path="../registers/gdtli1ba8a397_v1.rs"] pub mod gdtli1ba8a397;
#[path="../registers/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../registers/gdtrigselc40c9296_v1.rs"] pub mod gdtrigselc40c9296;
#[path="../registers/gdtrng89e226cf_v1.rs"] pub mod gdtrng89e226cf;
#[path="../registers/gdusarte31b633e_v1.rs"] pub mod gdusarte31b633e;
#[path="../registers/gdvref779f5a9e_v1.rs"] pub mod gdvref779f5a9e;
#[path="../registers/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
