
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "HsDevice",
            extends: None,
            description: Some(
                "USB on the go high speed device",
            ),
            items: &[
                BlockItem {
                    name: "dcfg",
                    description: Some(
                        "device configuration register (DCFG)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dctl",
                    description: Some(
                        "device control register (DCTL)",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dstat",
                    description: Some(
                        "device status register (DSTAT)",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepinten",
                    description: Some(
                        "device IN endpoint common interrupt mask register (DIEPINTEN)",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepinten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doepinten",
                    description: Some(
                        "device OUT endpoint common interrupt enable register (DOEPINTEN)",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doepinten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daepint",
                    description: Some(
                        "device all endpoints interrupt register (DAEPINT)",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Daepint",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "daepinten",
                    description: Some(
                        "Device all endpoints interrupt enable register (DAEPINTEN)",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Daepinten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dvbusdt",
                    description: Some(
                        "device VBUS discharge time register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dvbusdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dvbuspt",
                    description: Some(
                        "device VBUS pulsing time register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dvbuspt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dthc",
                    description: Some(
                        "Device threshold control register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dthc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diepfeinten",
                    description: Some(
                        "device IN endpoint FIFO empty interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diepfeinten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dep1int",
                    description: Some(
                        "device endpoint 1 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dep1int",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dep1inten",
                    description: Some(
                        "device endpoint 1 interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dep1inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1inten",
                    description: Some(
                        "device IN endpoint 1 interrupt mask register (DIEP1INTEN)",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep1inten",
                    description: Some(
                        "device OUT endpoint common interrupt enable register (DOEP1INTEN)",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep1inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0ctl",
                    description: Some(
                        "Device IN endpoint 0 control register (USBHS_DIEP0CTL)",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0intf",
                    description: Some(
                        "Device IN endpoint-0 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0len",
                    description: Some(
                        "device IN endpoint-0 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0dmaaddr",
                    description: Some(
                        "device IN endpoint 0 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0tfstat",
                    description: Some(
                        "device IN endpoint 0 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1ctl",
                    description: Some(
                        "Device IN endpoint-x control register",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1intf",
                    description: Some(
                        "Device IN endpoint-1 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1len",
                    description: Some(
                        "device IN endpoint-1 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1dmaaddr",
                    description: Some(
                        "device IN endpoint 1 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1tfstat",
                    description: Some(
                        "device IN endpoint 1 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2ctl",
                    description: Some(
                        "device endpoint-2 control register",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2intf",
                    description: Some(
                        "Device IN endpoint-2 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2len",
                    description: Some(
                        "device IN endpoint-2 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2dmaaddr",
                    description: Some(
                        "device IN endpoint 2 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2tfstat",
                    description: Some(
                        "device IN endpoint 2 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3ctl",
                    description: Some(
                        "device endpoint-3 control register",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3intf",
                    description: Some(
                        "Device IN endpoint-3 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3len",
                    description: Some(
                        "device IN endpoint-3 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3dmaaddr",
                    description: Some(
                        "device IN endpoint 3 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3tfstat",
                    description: Some(
                        "device IN endpoint 3 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep4ctl",
                    description: Some(
                        "device endpoint-4 control register",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep4ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep4intf",
                    description: Some(
                        "Device IN endpoint-4 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep4intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep4len",
                    description: Some(
                        "device IN endpoint-4 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep4len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep4dmaaddr",
                    description: Some(
                        "device IN endpoint 4 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep4dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep4tfstat",
                    description: Some(
                        "device IN endpoint 4 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep4tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep5ctl",
                    description: Some(
                        "device endpoint-5 control register",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep5ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep5intf",
                    description: Some(
                        "Device IN endpoint-5 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep5intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep5len",
                    description: Some(
                        "device IN endpoint-5 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep5len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep5dmaaddr",
                    description: Some(
                        "device IN endpoint 5 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep5dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep5tfstat",
                    description: Some(
                        "device IN endpoint 5 transmit FIFO status register",
                    ),
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep5tfstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep0ctl",
                    description: Some(
                        "Device OUT endpoint-0 control register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep0ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep0intf",
                    description: Some(
                        "device out endpoint-0 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x308,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep0intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep0len",
                    description: Some(
                        "device OUT endpoint-0 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x310,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep0len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep0dmaaddr",
                    description: Some(
                        "device OUT endpoint 0 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x314,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep0dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep1ctl",
                    description: Some(
                        "Device OUT endpoint-1 control register",
                    ),
                    array: None,
                    byte_offset: 0x320,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep1ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep1intf",
                    description: Some(
                        "device out endpoint-1 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x328,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep1intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep1len",
                    description: Some(
                        "device OUT endpoint-1 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x330,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep1len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep1dmaaddr",
                    description: Some(
                        "device OUT endpoint 1 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x334,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep1dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep2ctl",
                    description: Some(
                        "Device OUT endpoint-2 control register",
                    ),
                    array: None,
                    byte_offset: 0x340,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep2ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep2intf",
                    description: Some(
                        "device out endpoint-2 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x348,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep2intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep2len",
                    description: Some(
                        "device OUT endpoint-2 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x350,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep2len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep2dmaaddr",
                    description: Some(
                        "device OUT endpoint 2 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x354,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep2dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep3ctl",
                    description: Some(
                        "Device OUT endpoint-3 control register",
                    ),
                    array: None,
                    byte_offset: 0x360,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep3ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep3intf",
                    description: Some(
                        "device out endpoint-3 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x368,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep3intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep3len",
                    description: Some(
                        "device OUT endpoint-3 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x370,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep3len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep3dmaaddr",
                    description: Some(
                        "device OUT endpoint 3 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x374,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep3dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep4ctl",
                    description: Some(
                        "Device OUT endpoint-4 control register",
                    ),
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep4ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep4intf",
                    description: Some(
                        "device out endpoint-4 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep4intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep4len",
                    description: Some(
                        "device OUT endpoint-4 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep4len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep4dmaaddr",
                    description: Some(
                        "device OUT endpoint 4 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep4dmaaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep5ctl",
                    description: Some(
                        "Device OUT endpoint-5 control register",
                    ),
                    array: None,
                    byte_offset: 0x3a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep5ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep5intf",
                    description: Some(
                        "device out endpoint-5 interrupt register",
                    ),
                    array: None,
                    byte_offset: 0x3a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep5intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep5len",
                    description: Some(
                        "device OUT endpoint-5 transfer length register",
                    ),
                    array: None,
                    byte_offset: 0x3b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep5len",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doep5dmaaddr",
                    description: Some(
                        "device OUT endpoint 5 DMA address register",
                    ),
                    array: None,
                    byte_offset: 0x3b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doep5dmaaddr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Daepint",
            extends: None,
            description: Some(
                "device all endpoints interrupt register (DAEPINT)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepitb",
                    description: Some(
                        "Device all IN endpoint interrupt bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oepitb",
                    description: Some(
                        "Device all OUT endpoint interrupt bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Daepinten",
            extends: None,
            description: Some(
                "Device all endpoints interrupt enable register (DAEPINTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iepie",
                    description: Some(
                        "IN endpoint interrupt enable bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oepie",
                    description: Some(
                        "OUT endpoint interrupt enable bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dcfg",
            extends: None,
            description: Some(
                "device configuration register (DCFG)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ds",
                    description: Some(
                        "Device speed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nzlsoh",
                    description: Some(
                        "Non-zero-length status OUT handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dar",
                    description: Some(
                        "Device address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eopft",
                    description: Some(
                        "end of periodic frame time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dctl",
            extends: None,
            description: Some(
                "device control register (DCTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rwkup",
                    description: Some(
                        "Remote wakeup signaling",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd",
                    description: Some(
                        "Soft disconnect",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gins",
                    description: Some(
                        "Global IN NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gons",
                    description: Some(
                        "Global OUT NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtest",
                    description: Some(
                        "Device Test control",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sginak",
                    description: Some(
                        "Set global IN NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cginak",
                    description: Some(
                        "Clear global IN NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sgonak",
                    description: Some(
                        "Set global OUT NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cgonak",
                    description: Some(
                        "Clear global OUT NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "poif",
                    description: Some(
                        "Power-on initialization finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dep1int",
            extends: None,
            description: Some(
                "device endpoint 1 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iep1int",
                    description: Some(
                        "IN endpoint 1 interrupt bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oep1int",
                    description: Some(
                        "OUT endpoint 1 interrupt bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dep1inten",
            extends: None,
            description: Some(
                "device endpoint 1 interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iep1inten",
                    description: Some(
                        "IN endpoint 1 interrupt enable bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oep1inten",
                    description: Some(
                        "OUT endpoint 1 interrupt enable bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep0ctl",
            extends: None,
            description: Some(
                "Device IN endpoint 0 control register (USBHS_DIEP0CTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "Maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "TxFIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep0dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 0 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep0intf",
            extends: None,
            description: Some(
                "Device IN endpoint-0 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep0len",
            extends: None,
            description: Some(
                "device IN endpoint-0 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep0tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 0 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1ctl",
            extends: None,
            description: Some(
                "Device IN endpoint-x control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "Tx FIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "Set DATA1 PID/Set odd frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 1 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1inten",
            extends: None,
            description: Some(
                "device IN endpoint 1 interrupt mask register (DIEP1INTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tfen",
                    description: Some(
                        "Transfer finished interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdisen",
                    description: Some(
                        "Endpoint disabled interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "citoen",
                    description: Some(
                        "Control IN timeout condition interrupt enable (Non-isochronous endpoints)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfuden",
                    description: Some(
                        "Endpoint Tx FIFO underrun interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepneen",
                    description: Some(
                        "IN endpoint NAK effective interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naken",
                    description: Some(
                        "NAK handshake sent by USBHS interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1intf",
            extends: None,
            description: Some(
                "Device IN endpoint-1 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1len",
            extends: None,
            description: Some(
                "device IN endpoint-1 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 1 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2ctl",
            extends: None,
            description: Some(
                "device endpoint-2 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "Tx FIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "Set DATA1 PID/Set odd frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 2 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2intf",
            extends: None,
            description: Some(
                "Device IN endpoint-2 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2len",
            extends: None,
            description: Some(
                "device IN endpoint-2 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 2 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3ctl",
            extends: None,
            description: Some(
                "device endpoint-3 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "Tx FIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "Set DATA1 PID/Set odd frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 3 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3intf",
            extends: None,
            description: Some(
                "Device IN endpoint-3 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3len",
            extends: None,
            description: Some(
                "device IN endpoint-3 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 3 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep4ctl",
            extends: None,
            description: Some(
                "device endpoint-4 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "Tx FIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "Set DATA1 PID/Set odd frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep4dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 4 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep4intf",
            extends: None,
            description: Some(
                "Device IN endpoint-4 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep4len",
            extends: None,
            description: Some(
                "device IN endpoint-4 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep4tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 4 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep5ctl",
            extends: None,
            description: Some(
                "device endpoint-5 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfnum",
                    description: Some(
                        "Tx FIFO number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVNFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "Set DATA1 PID/Set odd frame",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep5dmaaddr",
            extends: None,
            description: Some(
                "device IN endpoint 5 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep5intf",
            extends: None,
            description: Some(
                "Device IN endpoint-5 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cito",
                    description: Some(
                        "Control in timeout interrupt",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfud",
                    description: Some(
                        "Endpoint Tx FIFO underrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepne",
                    description: Some(
                        "IN endpoint NAK effective",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txfe",
                    description: Some(
                        "Transmit FIFO empty",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nak",
                    description: Some(
                        "NAK handshake sent by USBHS",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep5len",
            extends: None,
            description: Some(
                "device IN endpoint-5 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mcnt",
                    description: Some(
                        "Multi count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep5tfstat",
            extends: None,
            description: Some(
                "device IN endpoint 5 transmit FIFO status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptfs",
                    description: Some(
                        "IN endpoint TxFIFO space available",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diepfeinten",
            extends: None,
            description: Some(
                "device IN endpoint FIFO empty interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptxfeie",
                    description: Some(
                        "IN EP Tx FIFO empty interrupt enable bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diepinten",
            extends: None,
            description: Some(
                "device IN endpoint common interrupt mask register (DIEPINTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tfen",
                    description: Some(
                        "Transfer completed interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdisen",
                    description: Some(
                        "Endpoint disabled interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "citoen",
                    description: Some(
                        "Control IN timeout condition interrupt enable (Non-isochronous endpoints)",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptxfuden",
                    description: Some(
                        "Endpoint Tx FIFO underrun interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iepneen",
                    description: Some(
                        "IN endpoint NAK effective interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naken",
                    description: Some(
                        "NAK handshake sent by USBHS interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep0ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-0 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "Maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep0dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 0 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep0intf",
            extends: None,
            description: Some(
                "device out endpoint-0 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep0len",
            extends: None,
            description: Some(
                "device OUT endpoint-0 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt",
                    description: Some(
                        "SETUP packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep1ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-1 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVENFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "SD1PID/SODDFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep1dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 1 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep1inten",
            extends: None,
            description: Some(
                "device OUT endpoint common interrupt enable register (DOEP1INTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tfen",
                    description: Some(
                        "Transfer completed interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdisen",
                    description: Some(
                        "Endpoint disabled interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpfen",
                    description: Some(
                        "SETUP phase done interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovren",
                    description: Some(
                        "Endpoint Rx FIFO overrun interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstpen",
                    description: Some(
                        "Back-to-back SETUP packets ( Only for control OUT endpoint) interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyeten",
                    description: Some(
                        "NYET handshake is sent interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep1intf",
            extends: None,
            description: Some(
                "device out endpoint-1 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep1len",
            extends: None,
            description: Some(
                "device OUT endpoint-1 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt_rxdpid",
                    description: Some(
                        "SETUP packet count/Received data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep2ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-2 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVENFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "SD1PID/SODDFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep2dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 2 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep2intf",
            extends: None,
            description: Some(
                "device out endpoint-2 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep2len",
            extends: None,
            description: Some(
                "device OUT endpoint-2 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt_rxdpid",
                    description: Some(
                        "SETUP packet count/Received data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep3ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-3 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVENFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "SD1PID/SODDFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep3dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 3 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep3intf",
            extends: None,
            description: Some(
                "device out endpoint-3 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep3len",
            extends: None,
            description: Some(
                "device OUT endpoint-3 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt_rxdpid",
                    description: Some(
                        "SETUP packet count/Received data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep4ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-4 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVENFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "SD1PID/SODDFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep4dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 4 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep4intf",
            extends: None,
            description: Some(
                "device out endpoint-4 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep4len",
            extends: None,
            description: Some(
                "device OUT endpoint-4 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt_rxdpid",
                    description: Some(
                        "SETUP packet count/Received data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep5ctl",
            extends: None,
            description: Some(
                "Device OUT endpoint-5 control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mpl",
                    description: Some(
                        "maximum packet length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epact",
                    description: Some(
                        "Endpoint active",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eofrm_dpid",
                    description: Some(
                        "EOFRM/DPID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "naks",
                    description: Some(
                        "NAK status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eptype",
                    description: Some(
                        "Endpoint type",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snoop",
                    description: Some(
                        "Snoop mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stall",
                    description: Some(
                        "STALL handshake",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnak",
                    description: Some(
                        "Clear NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "snak",
                    description: Some(
                        "Set NAK",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd0pid_sevenfrm",
                    description: Some(
                        "SD0PID/SEVENFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sd1pid_soddfrm",
                    description: Some(
                        "SD1PID/SODDFRM",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epd",
                    description: Some(
                        "Endpoint disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epen",
                    description: Some(
                        "Endpoint enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep5dmaaddr",
            extends: None,
            description: Some(
                "device OUT endpoint 5 DMA address register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaaddr",
                    description: Some(
                        "DMA address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep5intf",
            extends: None,
            description: Some(
                "device out endpoint-5 interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tf",
                    description: Some(
                        "Transfer finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdis",
                    description: Some(
                        "Endpoint disabled",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpf",
                    description: Some(
                        "Setup phase finished",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovr",
                    description: Some(
                        "Endpoint Rx FIFO overrun",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstp",
                    description: Some(
                        "Back-to-back SETUP packets",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyet",
                    description: Some(
                        "NYET handshake is sent",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doep5len",
            extends: None,
            description: Some(
                "device OUT endpoint-5 transfer length register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tlen",
                    description: Some(
                        "Transfer length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 19,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcnt",
                    description: Some(
                        "Packet count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpcnt_rxdpid",
                    description: Some(
                        "SETUP packet count/Received data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doepinten",
            extends: None,
            description: Some(
                "device OUT endpoint common interrupt enable register (DOEPINTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tfen",
                    description: Some(
                        "Transfer completed interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "epdisen",
                    description: Some(
                        "Endpoint disabled interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stpfen",
                    description: Some(
                        "SETUP phase done interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eprxfovren",
                    description: Some(
                        "Endpoint Rx FIFO overrun interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btbstpen",
                    description: Some(
                        "Back-to-back SETUP packets ( Only for control OUT endpoint) interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nyeten",
                    description: Some(
                        "NYET handshake is sent interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dstat",
            extends: None,
            description: Some(
                "device status register (DSTAT)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spst",
                    description: Some(
                        "Suspend status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "es",
                    description: Some(
                        "Enumerated speed",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fnrsof",
                    description: Some(
                        "Frame number of the received SOF",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dthc",
            extends: None,
            description: Some(
                "Device threshold control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nisointven",
                    description: Some(
                        "Non-isochronous IN endpoint threshold value enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "isointven",
                    description: Some(
                        "Isochronous IN endpoint threshold value enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "txtvlen",
                    description: Some(
                        "Receive threshold value length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxtven",
                    description: Some(
                        "Receive threshold value enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rxtvlen",
                    description: Some(
                        "Receive threshold value length",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apen",
                    description: Some(
                        "Arbitrator parking enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dvbusdt",
            extends: None,
            description: Some(
                "device VBUS discharge time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dvbusdt",
                    description: Some(
                        "Device VBUS discharge time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dvbuspt",
            extends: None,
            description: Some(
                "device VBUS pulsing time register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dvbuspt",
                    description: Some(
                        "Device VBUS pulsing time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                