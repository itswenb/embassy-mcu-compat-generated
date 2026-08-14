
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc4e1ce4e3",
                version: "v1",
                block: "ADC",
                ir: &gdadc4e1ce4e3::REGISTERS,
            },
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
        address: 0x40012100,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc4e1ce4e3",
                version: "v1",
                block: "ADC",
                ir: &gdadc4e1ce4e3::REGISTERS,
            },
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
        address: 0x40012200,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc4e1ce4e3",
                version: "v1",
                block: "ADC",
                ir: &gdadc4e1ce4e3::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC_BASE",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadcbasec6505c26",
                version: "v1",
                block: "ADC_BASE",
                ir: &gdadcbasec6505c26::REGISTERS,
            },
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
        address: 0x40006400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcanc1d18d75",
                version: "v1",
                block: "CAN",
                ir: &gdcanc1d18d75::REGISTERS,
            },
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
        address: 0x40006800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcanc1d18d75",
                version: "v1",
                block: "CAN",
                ir: &gdcanc1d18d75::REGISTERS,
            },
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
        address: 0x50060000,
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
        name: "CRC",
        address: 0x40023000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdcrc3d3f2740",
                version: "v1",
                block: "CRC",
                ir: &gdcrc3d3f2740::REGISTERS,
            },
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
        address: 0x40006c00,
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
                kind: "gddac4ad47e29",
                version: "v1",
                block: "DAC",
                ir: &gddac4ad47e29::REGISTERS,
            },
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
                kind: "gddbg546af4b0",
                version: "v1",
                block: "DBG",
                ir: &gddbg546af4b0::REGISTERS,
            },
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
        address: 0x50050000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddci6728f4f7",
                version: "v1",
                block: "DCI",
                ir: &gddci6728f4f7::REGISTERS,
            },
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
        address: 0x40026000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma35406330",
                version: "v1",
                block: "DMA",
                ir: &gddma35406330::REGISTERS,
            },
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
        address: 0x40026400,
        registers: Some(
            PeripheralRegisters {
                kind: "gddma35406330",
                version: "v1",
                block: "DMA",
                ir: &gddma35406330::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET",
        address: 0x40028000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenet4408cf6f",
                version: "v1",
                block: "ENET",
                ir: &gdenet4408cf6f::REGISTERS,
            },
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
                kind: "gdexmc7164c8e0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc7164c8e0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXMC_NAND",
        address: 0x70000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc7164c8e0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc7164c8e0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXMC_NOR_PSRAM",
        address: 0x60000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc7164c8e0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc7164c8e0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXMC_PCCARD",
        address: 0x90000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc7164c8e0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc7164c8e0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "EXMC_SDRAM",
        address: 0xc0000000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexmc7164c8e0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc7164c8e0::REGISTERS,
            },
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
        address: 0x40013c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdexti9d7fc170",
                version: "v1",
                block: "EXTI",
                ir: &gdexti9d7fc170::REGISTERS,
            },
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
        address: 0x40023c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfmce61e6de3",
                version: "v1",
                block: "FMC",
                ir: &gdfmce61e6de3::REGISTERS,
            },
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
                kind: "gdfwdgtd0ad6e6f",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtd0ad6e6f::REGISTERS,
            },
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
        address: 0x40020000,
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
        address: 0x40020400,
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
        address: 0x40020800,
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
        name: "GPIOD",
        address: 0x40020c00,
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
        name: "GPIOE",
        address: 0x40021000,
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
        address: 0x40021400,
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
        name: "GPIOG",
        address: 0x40021800,
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
        name: "GPIOH",
        address: 0x40021c00,
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
        name: "GPIOI",
        address: 0x40022000,
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
        name: "HAU",
        address: 0x50060400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhaub97c00c8",
                version: "v1",
                block: "HAU",
                ir: &gdhaub97c00c8::REGISTERS,
            },
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
                kind: "gdi2c2acc0e94",
                version: "v1",
                block: "I2C",
                ir: &gdi2c2acc0e94::REGISTERS,
            },
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
                kind: "gdi2c7d47e9dc",
                version: "v1",
                block: "I2C",
                ir: &gdi2c7d47e9dc::REGISTERS,
            },
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
        address: 0x40005c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c7d47e9dc",
                version: "v1",
                block: "I2C",
                ir: &gdi2c7d47e9dc::REGISTERS,
            },
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
        address: 0x40008000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c157576b1",
                version: "v1",
                block: "I2C",
                ir: &gdi2c157576b1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C4",
        address: 0x40008400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c157576b1",
                version: "v1",
                block: "I2C",
                ir: &gdi2c157576b1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2C5",
        address: 0x40008800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2c157576b1",
                version: "v1",
                block: "I2C",
                ir: &gdi2c157576b1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S1_ADD",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2s32f828a0",
                version: "v1",
                block: "I2S",
                ir: &gdi2s32f828a0::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S2_ADD",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdi2s32f828a0",
                version: "v1",
                block: "I2S",
                ir: &gdi2s32f828a0::REGISTERS,
            },
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
        address: 0x4002b000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdipad2e9ee25",
                version: "v1",
                block: "IPA",
                ir: &gdipad2e9ee25::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdiref4b25e655",
                version: "v1",
                block: "IREF",
                ir: &gdiref4b25e655::REGISTERS,
            },
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
        address: 0x40016800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli94583ddb",
                version: "v1",
                block: "TLI",
                ir: &gdtli94583ddb::REGISTERS,
            },
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
        address: 0x40016880,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli94583ddb",
                version: "v1",
                block: "TLI",
                ir: &gdtli94583ddb::REGISTERS,
            },
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
        address: 0x1ffec000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdobda811d1d",
                version: "v1",
                block: "OB",
                ir: &gdobda811d1d::REGISTERS,
            },
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
        address: 0x50061000,
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
        address: 0x40007000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdpmu36bfb084",
                version: "v1",
                block: "PMU",
                ir: &gdpmu36bfb084::REGISTERS,
            },
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
        address: 0x40023800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdrcu5c7c9b49",
                version: "v1",
                block: "RCU",
                ir: &gdrcu5c7c9b49::REGISTERS,
            },
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
                kind: "gdrtcea88a5d1",
                version: "v1",
                block: "RTC",
                ir: &gdrtcea88a5d1::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SAI",
        address: 0x40015800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsaib6e01957",
                version: "v1",
                block: "SAI",
                ir: &gdsaib6e01957::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "SDIO",
        address: 0x40012c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsdioa16a5588",
                version: "v1",
                block: "SDIO",
                ir: &gdsdioa16a5588::REGISTERS,
            },
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
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
        address: 0x40015400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi84bb0d40",
                version: "v1",
                block: "SPI",
                ir: &gdspi84bb0d40::REGISTERS,
            },
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
        address: 0x40013800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdsyscfgf9c313df",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfgf9c313df::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER10",
        address: 0x40014800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER11",
        address: 0x40001800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER12",
        address: 0x40001c00,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER8",
        address: 0x40014000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "TIMER9",
        address: 0x40014400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtimer94a0e296",
                version: "v1",
                block: "TIMER",
                ir: &gdtimer94a0e296::REGISTERS,
            },
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
        address: 0x40016800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtli94583ddb",
                version: "v1",
                block: "TLI",
                ir: &gdtli94583ddb::REGISTERS,
            },
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
        address: 0x50060800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdtrng13872700",
                version: "v1",
                block: "TRNG",
                ir: &gdtrng13872700::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
                kind: "gdusartd5126b39",
                version: "v1",
                block: "USART",
                ir: &gdusartd5126b39::REGISTERS,
            },
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
        name: "LVD",
        number: 1,
    },
    Interrupt {
        name: "TAMPER_STAMP",
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
        name: "ADC",
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
        name: "TIMER0_BRK_TIMER8",
        number: 24,
    },
    Interrupt {
        name: "TIMER0_UP_TIMER9",
        number: 25,
    },
    Interrupt {
        name: "TIMER0_TRG_CMT_TIMER10",
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
        name: "SDIO",
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
        name: "TIMER5_DAC",
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
        name: "ENET",
        number: 61,
    },
    Interrupt {
        name: "ENET_WKUP",
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
        name: "USBFS",
        number: 67,
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
        name: "DCI",
        number: 78,
    },
    Interrupt {
        name: "TRNG",
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
        name: "SAI",
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
        name: "PKCAU",
        number: 91,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 92,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 93,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 94,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 95,
    },
    Interrupt {
        name: "I2C5_EV",
        number: 96,
    },
    Interrupt {
        name: "I2C5_ER",
        number: 97,
    },
    Interrupt {
        name: "I2C3_WKUP",
        number: 98,
    },
    Interrupt {
        name: "I2C4_WKUP",
        number: 99,
    },
    Interrupt {
        name: "I2C5_WKUP",
        number: 100,
    },
    Interrupt {
        name: "SYSCFG_SRAM_ECC_ER",
        number: 101,
    },
    Interrupt {
        name: "HAU",
        number: 102,
    },
    Interrupt {
        name: "CAU",
        number: 103,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc4e1ce4e3_v1.rs"] pub mod gdadc4e1ce4e3;
#[path="../registers/gdadcbasec6505c26_v1.rs"] pub mod gdadcbasec6505c26;
#[path="../registers/gdcanc1d18d75_v1.rs"] pub mod gdcanc1d18d75;
#[path="../registers/gdcau0732936f_v1.rs"] pub mod gdcau0732936f;
#[path="../registers/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../registers/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../registers/gddac4ad47e29_v1.rs"] pub mod gddac4ad47e29;
#[path="../registers/gddbg546af4b0_v1.rs"] pub mod gddbg546af4b0;
#[path="../registers/gddci6728f4f7_v1.rs"] pub mod gddci6728f4f7;
#[path="../registers/gddma35406330_v1.rs"] pub mod gddma35406330;
#[path="../registers/gdenet4408cf6f_v1.rs"] pub mod gdenet4408cf6f;
#[path="../registers/gdexmc7164c8e0_v1.rs"] pub mod gdexmc7164c8e0;
#[path="../registers/gdexti9d7fc170_v1.rs"] pub mod gdexti9d7fc170;
#[path="../registers/gdfmce61e6de3_v1.rs"] pub mod gdfmce61e6de3;
#[path="../registers/gdfwdgtd0ad6e6f_v1.rs"] pub mod gdfwdgtd0ad6e6f;
#[path="../registers/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../registers/gdhaub97c00c8_v1.rs"] pub mod gdhaub97c00c8;
#[path="../registers/gdi2c157576b1_v1.rs"] pub mod gdi2c157576b1;
#[path="../registers/gdi2c2acc0e94_v1.rs"] pub mod gdi2c2acc0e94;
#[path="../registers/gdi2c7d47e9dc_v1.rs"] pub mod gdi2c7d47e9dc;
#[path="../registers/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../registers/gdipad2e9ee25_v1.rs"] pub mod gdipad2e9ee25;
#[path="../registers/gdiref4b25e655_v1.rs"] pub mod gdiref4b25e655;
#[path="../registers/gdobda811d1d_v1.rs"] pub mod gdobda811d1d;
#[path="../registers/gdpkcauf9e1d63d_v1.rs"] pub mod gdpkcauf9e1d63d;
#[path="../registers/gdpmu36bfb084_v1.rs"] pub mod gdpmu36bfb084;
#[path="../registers/gdrcu5c7c9b49_v1.rs"] pub mod gdrcu5c7c9b49;
#[path="../registers/gdrtcea88a5d1_v1.rs"] pub mod gdrtcea88a5d1;
#[path="../registers/gdsaib6e01957_v1.rs"] pub mod gdsaib6e01957;
#[path="../registers/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../registers/gdspi84bb0d40_v1.rs"] pub mod gdspi84bb0d40;
#[path="../registers/gdsyscfgf9c313df_v1.rs"] pub mod gdsyscfgf9c313df;
#[path="../registers/gdtimer94a0e296_v1.rs"] pub mod gdtimer94a0e296;
#[path="../registers/gdtli94583ddb_v1.rs"] pub mod gdtli94583ddb;
#[path="../registers/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../registers/gdusartd5126b39_v1.rs"] pub mod gdusartd5126b39;
#[path="../registers/gdwwdgt59a14ef4_v1.rs"] pub mod gdwwdgt59a14ef4;
