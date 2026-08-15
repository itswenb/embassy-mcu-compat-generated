
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "UsbfsGlobal",
            extends: None,
            description: Some(
                "USB full speed global registers",
            ),
            items: &[
                BlockItem {
                    name: "gahbcs",
                    description: Some(
                        "Global AHB control and status register (USBFS_GAHBCS)",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gahbcs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gusbcs",
                    description: Some(
                        "Global USB control and status register (OTG_FS_GUSBCSR)",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gusbcs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstctl",
                    description: Some(
                        "Global reset control register (USBFS_GRSTCTL)",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grstctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gintf",
                    description: Some(
                        "Global interrupt flag register (USBFS_GINTF)",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gintf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ginten",
                    description: Some(
                        "Global interrupt enable register (USBFS_GINTEN)",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ginten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstatr_device",
                    description: Some(
                        "Global Receive status read(Device mode)",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "GrstatrDevice",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstatr_host",
                    description: Some(
                        "Global Receive status read(Host mode)",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "GrstatrHost",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstatp_device",
                    description: Some(
                        "Global Receive status pop(Device mode)",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "GrstatpDevice",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grstatp_host",
                    description: Some(
                        "Global Receive status pop(Host mode)",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "GrstatpHost",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "grflen",
                    description: Some(
                        "Global Receive FIFO size register (USBFS_GRFLEN)",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Grflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep0tflen",
                    description: Some(
                        "Device IN endpoint 0 transmit FIFO length (Device mode)",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep0tflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hnptflen",
                    description: Some(
                        "Host non-periodic transmit FIFO length register (Host mode)",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hnptflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hnptfqstat",
                    description: Some(
                        "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Hnptfqstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gccfg",
                    description: Some(
                        "Global core configuration register (USBFS_GCCFG)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gccfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cid",
                    description: Some(
                        "core ID register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cid",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hptflen",
                    description: Some(
                        "Host periodic transmit FIFO length register (HPTFLEN)",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Hptflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep1tflen",
                    description: Some(
                        "device IN endpoint transmit FIFO size register (DIEP1TFLEN)",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep1tflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep2tflen",
                    description: Some(
                        "device IN endpoint transmit FIFO size register (DIEP2TFLEN)",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep2tflen",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "diep3tflen",
                    description: Some(
                        "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Diep3tflen",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cid",
            extends: None,
            description: Some(
                "core ID register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cid",
                    description: Some(
                        "Core ID",
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
            name: "Diep0tflen",
            extends: None,
            description: Some(
                "Device IN endpoint 0 transmit FIFO length (Device mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iep0txfd",
                    description: Some(
                        "in endpoint 0 Tx FIFO depth",
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
                Field {
                    name: "iep0txrsar",
                    description: Some(
                        "in endpoint 0 Tx RAM start address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep1tflen",
            extends: None,
            description: Some(
                "device IN endpoint transmit FIFO size register (DIEP1TFLEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptxrsar",
                    description: Some(
                        "IN endpoint FIFO transmit RAM start address",
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
                Field {
                    name: "ieptxfd",
                    description: Some(
                        "IN endpoint TxFIFO depth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep2tflen",
            extends: None,
            description: Some(
                "device IN endpoint transmit FIFO size register (DIEP2TFLEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptxrsar",
                    description: Some(
                        "IN endpoint FIFO transmit RAM start address",
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
                Field {
                    name: "ieptxfd",
                    description: Some(
                        "IN endpoint TxFIFO depth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Diep3tflen",
            extends: None,
            description: Some(
                "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ieptxrsar",
                    description: Some(
                        "IN endpoint FIFO4 transmit RAM start address",
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
                Field {
                    name: "ieptxfd",
                    description: Some(
                        "IN endpoint TxFIFO depth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gahbcs",
            extends: None,
            description: Some(
                "Global AHB control and status register (USBFS_GAHBCS)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ginten",
                    description: Some(
                        "Global interrupt enable",
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
                    name: "txfth",
                    description: Some(
                        "Tx FIFO threshold",
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
                    name: "ptxfth",
                    description: Some(
                        "Periodic Tx FIFO threshold",
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
            ],
        },
        FieldSet {
            name: "Gccfg",
            extends: None,
            description: Some(
                "Global core configuration register (USBFS_GCCFG)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwron",
                    description: Some(
                        "Power on",
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
                    name: "sofoen",
                    description: Some(
                        "SOF output enable",
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
            ],
        },
        FieldSet {
            name: "Ginten",
            extends: None,
            description: Some(
                "Global interrupt enable register (USBFS_GINTEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfie",
                    description: Some(
                        "Mode fault interrupt enable",
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
                    name: "sofie",
                    description: Some(
                        "Start of frame interrupt enable",
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
                    name: "rxfneie",
                    description: Some(
                        "Receive FIFO non-empty interrupt enable",
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
                    name: "nptxfeie",
                    description: Some(
                        "Non-periodic TxFIFO empty interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gnpinakie",
                    description: Some(
                        "Global non-periodic IN NAK effective interrupt enable",
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
                    name: "gonakie",
                    description: Some(
                        "Global OUT NAK effective interrupt enable",
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
                    name: "espie",
                    description: Some(
                        "Early suspend interrupt enable",
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
                    name: "spie",
                    description: Some(
                        "USB suspend interrupt enable",
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
                Field {
                    name: "rstie",
                    description: Some(
                        "USB reset interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enumfie",
                    description: Some(
                        "Enumeration finish interrupt enable",
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
                Field {
                    name: "isoopdie",
                    description: Some(
                        "Isochronous OUT packet dropped interrupt enable",
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
                Field {
                    name: "eopfie",
                    description: Some(
                        "End of periodic frame interrupt enable",
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
                    name: "iepie",
                    description: Some(
                        "IN endpoints interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oepie",
                    description: Some(
                        "OUT endpoints interrupt enable",
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
                    name: "isoincie",
                    description: Some(
                        "isochronous IN transfer not complete interrupt enable",
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
                    name: "pxncie_isooncie",
                    description: Some(
                        "periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)",
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
                    name: "hpie",
                    description: Some(
                        "Host port interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hcie",
                    description: Some(
                        "Host channels interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptxfeie",
                    description: Some(
                        "Periodic TxFIFO empty interrupt enable",
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
                    name: "discie",
                    description: Some(
                        "Disconnect interrupt enable",
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
                    name: "wkupie",
                    description: Some(
                        "Wakeup interrupt enable",
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
            name: "Gintf",
            extends: None,
            description: Some(
                "Global interrupt flag register (USBFS_GINTF)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "copm",
                    description: Some(
                        "Current operation mode",
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
                    name: "mfif",
                    description: Some(
                        "Mode fault interrupt flag",
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
                    name: "sof",
                    description: Some(
                        "Start of frame",
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
                    name: "rxfneif",
                    description: Some(
                        "RxFIFO non-empty interrupt flag",
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
                    name: "nptxfeif",
                    description: Some(
                        "Non-periodic TxFIFO empty interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gnpinak",
                    description: Some(
                        "Global Non-Periodic IN NAK effective",
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
                    name: "gonak",
                    description: Some(
                        "Global OUT NAK effective",
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
                    name: "esp",
                    description: Some(
                        "Early suspend",
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
                    name: "sp",
                    description: Some(
                        "USB suspend",
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
                Field {
                    name: "rst",
                    description: Some(
                        "USB reset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enumf",
                    description: Some(
                        "Enumeration finished",
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
                Field {
                    name: "isoopdif",
                    description: Some(
                        "Isochronous OUT packet dropped interrupt",
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
                Field {
                    name: "eopfif",
                    description: Some(
                        "End of periodic frame interrupt flag",
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
                    name: "iepif",
                    description: Some(
                        "IN endpoint interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oepif",
                    description: Some(
                        "OUT endpoint interrupt flag",
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
                    name: "isoincif",
                    description: Some(
                        "Isochronous IN transfer Not Complete Interrupt Flag",
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
                    name: "pxncif_isooncif",
                    description: Some(
                        "periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)",
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
                    name: "hpif",
                    description: Some(
                        "Host port interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hcif",
                    description: Some(
                        "Host channels interrupt flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ptxfeif",
                    description: Some(
                        "Periodic TxFIFO empty interrupt flag",
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
                    name: "idpsc",
                    description: Some(
                        "ID pin status change",
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
                    name: "discif",
                    description: Some(
                        "Disconnect interrupt flag",
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
                    name: "wkupif",
                    description: Some(
                        "Wakeup interrupt flag",
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
            name: "Grflen",
            extends: None,
            description: Some(
                "Global Receive FIFO size register (USBFS_GRFLEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rxfd",
                    description: Some(
                        "Rx FIFO depth",
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
            name: "GrstatpDevice",
            extends: None,
            description: Some(
                "Global Receive status pop(Device mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epnum",
                    description: Some(
                        "Endpoint number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcount",
                    description: Some(
                        "Byte count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpckst",
                    description: Some(
                        "Recieve packet status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GrstatpHost",
            extends: None,
            description: Some(
                "Global Receive status pop(Host mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnum",
                    description: Some(
                        "Channel number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcount",
                    description: Some(
                        "Byte count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpckst",
                    description: Some(
                        "Reivece packet status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GrstatrDevice",
            extends: None,
            description: Some(
                "Global Receive status read(Device mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "epnum",
                    description: Some(
                        "Endpoint number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcount",
                    description: Some(
                        "Byte count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpckst",
                    description: Some(
                        "Recieve packet status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GrstatrHost",
            extends: None,
            description: Some(
                "Global Receive status read(Host mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnum",
                    description: Some(
                        "Channel number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcount",
                    description: Some(
                        "Byte count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpid",
                    description: Some(
                        "Data PID",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rpckst",
                    description: Some(
                        "Reivece packet status",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Grstctl",
            extends: None,
            description: Some(
                "Global reset control register (USBFS_GRSTCTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "csrst",
                    description: Some(
                        "Core soft reset",
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
                    name: "hcsrst",
                    description: Some(
                        "HCLK soft reset",
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
                    name: "hfcrst",
                    description: Some(
                        "Host frame counter reset",
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
                    name: "rxff",
                    description: Some(
                        "RxFIFO flush",
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
                    name: "txff",
                    description: Some(
                        "TxFIFO flush",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
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
                            offset: 6,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gusbcs",
            extends: None,
            description: Some(
                "Global USB control and status register (OTG_FS_GUSBCSR)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "toc",
                    description: Some(
                        "Timeout calibration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "utt",
                    description: Some(
                        "USB turnaround time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fhm",
                    description: Some(
                        "Force host mode",
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
                    name: "fdm",
                    description: Some(
                        "Force device mode",
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
            ],
        },
        FieldSet {
            name: "Hnptflen",
            extends: None,
            description: Some(
                "Host non-periodic transmit FIFO length register (Host mode)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hnptxrsar",
                    description: Some(
                        "host non-periodic transmit Tx RAM start address",
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
                Field {
                    name: "hnptxfd",
                    description: Some(
                        "host non-periodic TxFIFO depth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hnptfqstat",
            extends: None,
            description: Some(
                "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nptxfs",
                    description: Some(
                        "Non-periodic TxFIFO space",
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
                Field {
                    name: "nptxrqs",
                    description: Some(
                        "Non-periodic transmit request queue space",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nptxrqtop",
                    description: Some(
                        "Top of the non-periodic transmit request queue",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Hptflen",
            extends: None,
            description: Some(
                "Host periodic transmit FIFO length register (HPTFLEN)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hptxfsar",
                    description: Some(
                        "Host periodic TxFIFO start address",
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
                Field {
                    name: "hptxfd",
                    description: Some(
                        "Host periodic TxFIFO depth",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                