
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "ADC0",
        address: 0x40012000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadc0644c59d8",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0644c59d8::REGISTERS,
            },
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
                kind: "gdadc0644c59d8",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0644c59d8::REGISTERS,
            },
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
                kind: "gdadc0644c59d8",
                version: "v1",
                block: "ADC0",
                ir: &gdadc0644c59d8::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ADC_Common",
        address: 0x40012300,
        registers: Some(
            PeripheralRegisters {
                kind: "gdadccommon6f53c1c8",
                version: "v1",
                block: "ADC_Common",
                ir: &gdadccommon6f53c1c8::REGISTERS,
            },
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
                kind: "gdcan06b36baa3",
                version: "v1",
                block: "CAN0",
                ir: &gdcan06b36baa3::REGISTERS,
            },
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
                kind: "gdcan06b36baa3",
                version: "v1",
                block: "CAN0",
                ir: &gdcan06b36baa3::REGISTERS,
            },
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
                kind: "gdcrc66a4f78d",
                version: "v1",
                block: "CRC",
                ir: &gdcrc66a4f78d::REGISTERS,
            },
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
                kind: "gdctc47444a2c",
                version: "v1",
                block: "CTC",
                ir: &gdctc47444a2c::REGISTERS,
            },
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
                kind: "gddac4f1e7381",
                version: "v1",
                block: "DAC",
                ir: &gddac4f1e7381::REGISTERS,
            },
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
        address: 0xe0042000,
        registers: Some(
            PeripheralRegisters {
                kind: "gddbg50e0203e",
                version: "v1",
                block: "DBG",
                ir: &gddbg50e0203e::REGISTERS,
            },
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
                kind: "gddci704bb188",
                version: "v1",
                block: "DCI",
                ir: &gddci704bb188::REGISTERS,
            },
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
                kind: "gddma024ec4b91",
                version: "v1",
                block: "DMA0",
                ir: &gddma024ec4b91::REGISTERS,
            },
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
                kind: "gddma024ec4b91",
                version: "v1",
                block: "DMA0",
                ir: &gddma024ec4b91::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "ENET_DMA",
        address: 0x40029000,
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetdma7fbba2f4",
                version: "v1",
                block: "ENET_DMA",
                ir: &gdenetdma7fbba2f4::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmac93552dd1",
                version: "v1",
                block: "ENET_MAC",
                ir: &gdenetmac93552dd1::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmacfcth8ada9e21",
                version: "v1",
                block: "ENET_MAC_FCTH",
                ir: &gdenetmacfcth8ada9e21::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetmsc10390666",
                version: "v1",
                block: "ENET_MSC",
                ir: &gdenetmsc10390666::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdenetptp5c8a2d48",
                version: "v1",
                block: "ENET_PTP",
                ir: &gdenetptp5c8a2d48::REGISTERS,
            },
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
                kind: "gdexmc55cf46b0",
                version: "v1",
                block: "EXMC",
                ir: &gdexmc55cf46b0::REGISTERS,
            },
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
                kind: "gdexti2861ec2a",
                version: "v1",
                block: "EXTI",
                ir: &gdexti2861ec2a::REGISTERS,
            },
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
                kind: "gdfmcf2343cdc",
                version: "v1",
                block: "FMC",
                ir: &gdfmcf2343cdc::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "FS_DEVICE",
        address: 0x50000800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdfsdeviceb377b28b",
                version: "v1",
                block: "FS_DEVICE",
                ir: &gdfsdeviceb377b28b::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdfsglobale74e6f0e",
                version: "v1",
                block: "FS_GLOBAL",
                ir: &gdfsglobale74e6f0e::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdfshost44621b1c",
                version: "v1",
                block: "FS_HOST",
                ir: &gdfshost44621b1c::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdfspwrclk87dcd48b",
                version: "v1",
                block: "FS_PWRCLK",
                ir: &gdfspwrclk87dcd48b::REGISTERS,
            },
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
                kind: "gdfwdgtb5a65d35",
                version: "v1",
                block: "FWDGT",
                ir: &gdfwdgtb5a65d35::REGISTERS,
            },
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
                kind: "gdgpioa54e55541",
                version: "v1",
                block: "GPIOA",
                ir: &gdgpioa54e55541::REGISTERS,
            },
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
                kind: "gdgpiob0a8ce2af",
                version: "v1",
                block: "GPIOB",
                ir: &gdgpiob0a8ce2af::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
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
                kind: "gdgpiocc25656a9",
                version: "v1",
                block: "GPIOC",
                ir: &gdgpiocc25656a9::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "HS_DEVICE",
        address: 0x40040800,
        registers: Some(
            PeripheralRegisters {
                kind: "gdhsdevicec9d69f15",
                version: "v1",
                block: "HS_DEVICE",
                ir: &gdhsdevicec9d69f15::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdhsglobalc406147a",
                version: "v1",
                block: "HS_GLOBAL",
                ir: &gdhsglobalc406147a::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdhshostc2377b4a",
                version: "v1",
                block: "HS_HOST",
                ir: &gdhshostc2377b4a::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdhspwrclk9376d26f",
                version: "v1",
                block: "HS_PWRCLK",
                ir: &gdhspwrclk9376d26f::REGISTERS,
            },
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
                kind: "gdi2c0116537ab",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0116537ab::REGISTERS,
            },
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
                kind: "gdi2c0116537ab",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0116537ab::REGISTERS,
            },
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
                kind: "gdi2c0116537ab",
                version: "v1",
                block: "I2C0",
                ir: &gdi2c0116537ab::REGISTERS,
            },
        ),
        rcc: None,
        pins: &[],
        dma_channels: &[],
        triggers: &[],
        interrupts: &[],
        afio: None,
    },
    Peripheral {
        name: "I2S1_add",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
        ),
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
        registers: Some(
            PeripheralRegisters {
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdipae676fed9",
                version: "v1",
                block: "IPA",
                ir: &gdipae676fed9::REGISTERS,
            },
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
                kind: "gdiref361590d6",
                version: "v1",
                block: "IREF",
                ir: &gdiref361590d6::REGISTERS,
            },
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
                kind: "gdpmu52565809",
                version: "v1",
                block: "PMU",
                ir: &gdpmu52565809::REGISTERS,
            },
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
                kind: "gdrcud76b9bb4",
                version: "v1",
                block: "RCU",
                ir: &gdrcud76b9bb4::REGISTERS,
            },
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
                kind: "gdrtc34bd68c7",
                version: "v1",
                block: "RTC",
                ir: &gdrtc34bd68c7::REGISTERS,
            },
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
                kind: "gdsdio6b548c0d",
                version: "v1",
                block: "SDIO",
                ir: &gdsdio6b548c0d::REGISTERS,
            },
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
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdspi0a39abaa4",
                version: "v1",
                block: "SPI0",
                ir: &gdspi0a39abaa4::REGISTERS,
            },
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
                kind: "gdspi528277832",
                version: "v1",
                block: "SPI5",
                ir: &gdspi528277832::REGISTERS,
            },
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
                kind: "gdsyscfg417a49de",
                version: "v1",
                block: "SYSCFG",
                ir: &gdsyscfg417a49de::REGISTERS,
            },
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
                kind: "gdtimer0ac749699",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0ac749699::REGISTERS,
            },
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
                kind: "gdtimer1f33d033d",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1f33d033d::REGISTERS,
            },
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
                kind: "gdtimer9b6e04d24",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9b6e04d24::REGISTERS,
            },
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
                kind: "gdtimer895e47fd0",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer895e47fd0::REGISTERS,
            },
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
                kind: "gdtimer9b6e04d24",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9b6e04d24::REGISTERS,
            },
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
                kind: "gdtimer9b6e04d24",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9b6e04d24::REGISTERS,
            },
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
                kind: "gdtimer1f33d033d",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1f33d033d::REGISTERS,
            },
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
                kind: "gdtimer1f33d033d",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1f33d033d::REGISTERS,
            },
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
                kind: "gdtimer1f33d033d",
                version: "v1",
                block: "TIMER1",
                ir: &gdtimer1f33d033d::REGISTERS,
            },
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
                kind: "gdtimer519fda6d7",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer519fda6d7::REGISTERS,
            },
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
                kind: "gdtimer519fda6d7",
                version: "v1",
                block: "TIMER5",
                ir: &gdtimer519fda6d7::REGISTERS,
            },
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
                kind: "gdtimer0ac749699",
                version: "v1",
                block: "TIMER0",
                ir: &gdtimer0ac749699::REGISTERS,
            },
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
                kind: "gdtimer895e47fd0",
                version: "v1",
                block: "TIMER8",
                ir: &gdtimer895e47fd0::REGISTERS,
            },
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
                kind: "gdtimer9b6e04d24",
                version: "v1",
                block: "TIMER9",
                ir: &gdtimer9b6e04d24::REGISTERS,
            },
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
                kind: "gdtli3a8126bb",
                version: "v1",
                block: "TLI",
                ir: &gdtli3a8126bb::REGISTERS,
            },
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
                kind: "gdtrngb48807ab",
                version: "v1",
                block: "TRNG",
                ir: &gdtrngb48807ab::REGISTERS,
            },
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
                kind: "gduart38ecaf091",
                version: "v1",
                block: "UART3",
                ir: &gduart38ecaf091::REGISTERS,
            },
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
                kind: "gduart38ecaf091",
                version: "v1",
                block: "UART3",
                ir: &gduart38ecaf091::REGISTERS,
            },
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
                kind: "gduart38ecaf091",
                version: "v1",
                block: "UART3",
                ir: &gduart38ecaf091::REGISTERS,
            },
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
                kind: "gduart38ecaf091",
                version: "v1",
                block: "UART3",
                ir: &gduart38ecaf091::REGISTERS,
            },
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
                kind: "gdusart06fc75967",
                version: "v1",
                block: "USART0",
                ir: &gdusart06fc75967::REGISTERS,
            },
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
                kind: "gdusart06fc75967",
                version: "v1",
                block: "USART0",
                ir: &gdusart06fc75967::REGISTERS,
            },
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
                kind: "gdusart06fc75967",
                version: "v1",
                block: "USART0",
                ir: &gdusart06fc75967::REGISTERS,
            },
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
                kind: "gdusart06fc75967",
                version: "v1",
                block: "USART0",
                ir: &gdusart06fc75967::REGISTERS,
            },
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
        name: "RTC_TAMPER",
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
        name: "EXTI_LINE9_5",
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
        name: "OTG_FS_WKUP",
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
        name: "OTG_FS",
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
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
                pub(crate) static PINS: &[Pin] = &[];
            #[path="../registers/gdadc0644c59d8_v1.rs"] pub mod gdadc0644c59d8;
#[path="../registers/gdadccommon6f53c1c8_v1.rs"] pub mod gdadccommon6f53c1c8;
#[path="../registers/gdcan06b36baa3_v1.rs"] pub mod gdcan06b36baa3;
#[path="../registers/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../registers/gdctc47444a2c_v1.rs"] pub mod gdctc47444a2c;
#[path="../registers/gddac4f1e7381_v1.rs"] pub mod gddac4f1e7381;
#[path="../registers/gddbg50e0203e_v1.rs"] pub mod gddbg50e0203e;
#[path="../registers/gddci704bb188_v1.rs"] pub mod gddci704bb188;
#[path="../registers/gddma024ec4b91_v1.rs"] pub mod gddma024ec4b91;
#[path="../registers/gdenetdma7fbba2f4_v1.rs"] pub mod gdenetdma7fbba2f4;
#[path="../registers/gdenetmac93552dd1_v1.rs"] pub mod gdenetmac93552dd1;
#[path="../registers/gdenetmacfcth8ada9e21_v1.rs"] pub mod gdenetmacfcth8ada9e21;
#[path="../registers/gdenetmsc10390666_v1.rs"] pub mod gdenetmsc10390666;
#[path="../registers/gdenetptp5c8a2d48_v1.rs"] pub mod gdenetptp5c8a2d48;
#[path="../registers/gdexmc55cf46b0_v1.rs"] pub mod gdexmc55cf46b0;
#[path="../registers/gdexti2861ec2a_v1.rs"] pub mod gdexti2861ec2a;
#[path="../registers/gdfmcf2343cdc_v1.rs"] pub mod gdfmcf2343cdc;
#[path="../registers/gdfsdeviceb377b28b_v1.rs"] pub mod gdfsdeviceb377b28b;
#[path="../registers/gdfsglobale74e6f0e_v1.rs"] pub mod gdfsglobale74e6f0e;
#[path="../registers/gdfshost44621b1c_v1.rs"] pub mod gdfshost44621b1c;
#[path="../registers/gdfspwrclk87dcd48b_v1.rs"] pub mod gdfspwrclk87dcd48b;
#[path="../registers/gdfwdgtb5a65d35_v1.rs"] pub mod gdfwdgtb5a65d35;
#[path="../registers/gdgpioa54e55541_v1.rs"] pub mod gdgpioa54e55541;
#[path="../registers/gdgpiob0a8ce2af_v1.rs"] pub mod gdgpiob0a8ce2af;
#[path="../registers/gdgpiocc25656a9_v1.rs"] pub mod gdgpiocc25656a9;
#[path="../registers/gdhsdevicec9d69f15_v1.rs"] pub mod gdhsdevicec9d69f15;
#[path="../registers/gdhsglobalc406147a_v1.rs"] pub mod gdhsglobalc406147a;
#[path="../registers/gdhshostc2377b4a_v1.rs"] pub mod gdhshostc2377b4a;
#[path="../registers/gdhspwrclk9376d26f_v1.rs"] pub mod gdhspwrclk9376d26f;
#[path="../registers/gdi2c0116537ab_v1.rs"] pub mod gdi2c0116537ab;
#[path="../registers/gdipae676fed9_v1.rs"] pub mod gdipae676fed9;
#[path="../registers/gdiref361590d6_v1.rs"] pub mod gdiref361590d6;
#[path="../registers/gdpmu52565809_v1.rs"] pub mod gdpmu52565809;
#[path="../registers/gdrcud76b9bb4_v1.rs"] pub mod gdrcud76b9bb4;
#[path="../registers/gdrtc34bd68c7_v1.rs"] pub mod gdrtc34bd68c7;
#[path="../registers/gdsdio6b548c0d_v1.rs"] pub mod gdsdio6b548c0d;
#[path="../registers/gdspi0a39abaa4_v1.rs"] pub mod gdspi0a39abaa4;
#[path="../registers/gdspi528277832_v1.rs"] pub mod gdspi528277832;
#[path="../registers/gdsyscfg417a49de_v1.rs"] pub mod gdsyscfg417a49de;
#[path="../registers/gdtimer0ac749699_v1.rs"] pub mod gdtimer0ac749699;
#[path="../registers/gdtimer1f33d033d_v1.rs"] pub mod gdtimer1f33d033d;
#[path="../registers/gdtimer519fda6d7_v1.rs"] pub mod gdtimer519fda6d7;
#[path="../registers/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../registers/gdtimer9b6e04d24_v1.rs"] pub mod gdtimer9b6e04d24;
#[path="../registers/gdtli3a8126bb_v1.rs"] pub mod gdtli3a8126bb;
#[path="../registers/gdtrngb48807ab_v1.rs"] pub mod gdtrngb48807ab;
#[path="../registers/gduart38ecaf091_v1.rs"] pub mod gduart38ecaf091;
#[path="../registers/gdusart06fc75967_v1.rs"] pub mod gdusart06fc75967;
#[path="../registers/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
