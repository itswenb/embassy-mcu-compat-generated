
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EnetMac",
            extends: None,
            description: Some(
                "Ethernet: media access control",
            ),
            items: &[
                BlockItem {
                    name: "mac_cfg",
                    description: Some(
                        "Ethernet MAC configuration register (MAC_CFG)",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_frmf",
                    description: Some(
                        "Ethernet MAC frame filter register (MAC_FRMF)",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacFrmf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_hlh",
                    description: Some(
                        "Ethernet MAC hash list high register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacHlh",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_hll",
                    description: Some(
                        "Ethernet MAC hash list low register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacHll",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_phy_ctl",
                    description: Some(
                        "Ethernet MAC PHY control register (MAC_PHY_CTL)",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacPhyCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_phy_data",
                    description: Some(
                        "Ethernet MAC MII data register (MAC_PHY_DATA)",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacPhyData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_fctl",
                    description: Some(
                        "Ethernet MAC flow control register (MAC_FCTL)",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacFctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_vlt",
                    description: Some(
                        "Ethernet MAC VLAN tag register (MAC_VLT)",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacVlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_rwff",
                    description: Some(
                        "Ethernet MAC remote wakeup frame filter register (MAC_RWFF)",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "mac_wum",
                    description: Some(
                        "Ethernet MAC wakeup management register (MAC_WUM)",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacWum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_intf",
                    description: Some(
                        "Ethernet MAC interrupt flag register (MAC_INTF)",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "MacIntf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_intmsk",
                    description: Some(
                        "Ethernet MAC interrupt mask register (MAC_INTMSK)",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacIntmsk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr0h",
                    description: Some(
                        "Ethernet MAC address 0 high register (MAC_ADDR0H)",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr0h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr0l",
                    description: Some(
                        "Ethernet MAC address 0 low register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr0l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr1h",
                    description: Some(
                        "Ethernet MAC address 1 high register (MAC_ADDR1H)",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr1h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr1l",
                    description: Some(
                        "Ethernet MAC address1 low register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr1l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr2h",
                    description: Some(
                        "Ethernet MAC address 2 high register (MAC_ADDR2H)",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr2h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr2l",
                    description: Some(
                        "Ethernet MAC address 2 low register",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr2l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr3h",
                    description: Some(
                        "Ethernet MAC address 3 high register (MAC_ADDR3H)",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr3h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mac_addr3l",
                    description: Some(
                        "Ethernet MAC address 3 low register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "MacAddr3l",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "MacAddr0h",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 high register (MAC_ADDR0H)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr0h",
                    description: Some(
                        "MAC address0 high",
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
                    name: "mo",
                    description: Some(
                        "Always 1",
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
            name: "MacAddr0l",
            extends: None,
            description: Some(
                "Ethernet MAC address 0 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr0l",
                    description: Some(
                        "MAC address0 low",
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
            name: "MacAddr1h",
            extends: None,
            description: Some(
                "Ethernet MAC address 1 high register (MAC_ADDR1H)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr1h",
                    description: Some(
                        "MAC address1 high",
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
                    name: "mb",
                    description: Some(
                        "Mask byte",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saf",
                    description: Some(
                        "Source address filter",
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
                    name: "afe",
                    description: Some(
                        "Address filter enable",
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
            name: "MacAddr1l",
            extends: None,
            description: Some(
                "Ethernet MAC address1 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr1l",
                    description: Some(
                        "MAC address1 low",
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
            name: "MacAddr2h",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 high register (MAC_ADDR2H)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr2h",
                    description: Some(
                        "Ethernet MAC address 2 high register",
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
                    name: "mb",
                    description: Some(
                        "Mask byte",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saf",
                    description: Some(
                        "Source address filter",
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
                    name: "afe",
                    description: Some(
                        "Address filter enable",
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
            name: "MacAddr2l",
            extends: None,
            description: Some(
                "Ethernet MAC address 2 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr2l",
                    description: Some(
                        "MAC address2 low",
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
            name: "MacAddr3h",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 high register (MAC_ADDR3H)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr3h",
                    description: Some(
                        "MAC address3 high",
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
                    name: "mb",
                    description: Some(
                        "Mask byte",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saf",
                    description: Some(
                        "Source address filter",
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
                    name: "afe",
                    description: Some(
                        "Address filter enable",
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
            name: "MacAddr3l",
            extends: None,
            description: Some(
                "Ethernet MAC address 3 low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "addr3l",
                    description: Some(
                        "MAC address3 low",
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
            name: "MacCfg",
            extends: None,
            description: Some(
                "Ethernet MAC configuration register (MAC_CFG)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ren",
                    description: Some(
                        "Receiver enable",
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
                    name: "ten",
                    description: Some(
                        "Transmitter enable",
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
                    name: "dfc",
                    description: Some(
                        "Deferral check",
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
                    name: "bol",
                    description: Some(
                        "Back-off limit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apcd",
                    description: Some(
                        "Automatic pad/CRC drop",
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
                    name: "rtd",
                    description: Some(
                        "Retry disable",
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
                    name: "ipfco",
                    description: Some(
                        "IP frame checksum offload",
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
                    name: "dpm",
                    description: Some(
                        "Duplex mode",
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
                    name: "lbm",
                    description: Some(
                        "Loopback mode",
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
                    name: "rod",
                    description: Some(
                        "Receive own disable",
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
                    name: "spd",
                    description: Some(
                        "Fast Ethernet speed",
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
                    name: "csd",
                    description: Some(
                        "Carrier sense disable",
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
                    name: "igbs",
                    description: Some(
                        "Inter frame gap bit selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jbd",
                    description: Some(
                        "Jabber disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdd",
                    description: Some(
                        "Watchdog disable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacFctl",
            extends: None,
            description: Some(
                "Ethernet MAC flow control register (MAC_FCTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flcb_bkpa",
                    description: Some(
                        "Flow control busy/back pressure activate",
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
                    name: "tfcen",
                    description: Some(
                        "Transmit flow control enable",
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
                    name: "rfcen",
                    description: Some(
                        "Receive flow control enable",
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
                    name: "upfdt",
                    description: Some(
                        "Unicast pause frame detect",
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
                    name: "plts",
                    description: Some(
                        "Pause low threshold",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dzqp",
                    description: Some(
                        "Disable Zero-quanta pause",
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
                    name: "ptm",
                    description: Some(
                        "Pause time",
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
            name: "MacFrmf",
            extends: None,
            description: Some(
                "Ethernet MAC frame filter register (MAC_FRMF)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pm",
                    description: Some(
                        "Promiscuous mode",
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
                    name: "huf",
                    description: Some(
                        "Hash unicast filter",
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
                    name: "hmf",
                    description: Some(
                        "Hash multicast filter",
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
                    name: "daiflt",
                    description: Some(
                        "Destination address inverse filtering",
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
                    name: "mfd",
                    description: Some(
                        "multicast filter disable",
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
                    name: "bfrmd",
                    description: Some(
                        "Broadcast frames disable",
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
                    name: "pcfrm",
                    description: Some(
                        "Pass control frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saiflt",
                    description: Some(
                        "Source address inverse filtering",
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
                    name: "saflt",
                    description: Some(
                        "Source address filter",
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
                    name: "hpflt",
                    description: Some(
                        "Hash or perfect filter",
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
                    name: "far",
                    description: Some(
                        "Frames all receive",
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
            name: "MacHlh",
            extends: None,
            description: Some(
                "Ethernet MAC hash list high register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hlh",
                    description: Some(
                        "Hash list high",
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
            name: "MacHll",
            extends: None,
            description: Some(
                "Ethernet MAC hash list low register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hll",
                    description: Some(
                        "Hash list low",
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
            name: "MacIntf",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt flag register (MAC_INTF)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wum",
                    description: Some(
                        "WUM status",
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
                    name: "msc",
                    description: Some(
                        "MSC status",
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
                    name: "mscr",
                    description: Some(
                        "MSC receive status",
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
                    name: "msct",
                    description: Some(
                        "MSC transmit status",
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
                    name: "tmst",
                    description: Some(
                        "Time stamp trigger status",
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
            ],
        },
        FieldSet {
            name: "MacIntmsk",
            extends: None,
            description: Some(
                "Ethernet MAC interrupt mask register (MAC_INTMSK)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wumim",
                    description: Some(
                        "WUM interrupt mask",
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
                    name: "tmstim",
                    description: Some(
                        "Time stamp trigger interrupt mask",
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
            ],
        },
        FieldSet {
            name: "MacPhyCtl",
            extends: None,
            description: Some(
                "Ethernet MAC PHY control register (MAC_PHY_CTL)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb",
                    description: Some(
                        "PHY busy",
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
                    name: "pw",
                    description: Some(
                        "PHY write",
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
                    name: "clr",
                    description: Some(
                        "Clock range",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pr",
                    description: Some(
                        "PHY register",
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
                Field {
                    name: "pa",
                    description: Some(
                        "PHY address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "MacPhyData",
            extends: None,
            description: Some(
                "Ethernet MAC MII data register (MAC_PHY_DATA)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "PHY data",
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
            name: "MacVlt",
            extends: None,
            description: Some(
                "Ethernet MAC VLAN tag register (MAC_VLT)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlti",
                    description: Some(
                        "VLAN tag identifier (for receive frames)",
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
                    name: "vltc",
                    description: Some(
                        "12-bit VLAN tag comparison",
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
            ],
        },
        FieldSet {
            name: "MacWum",
            extends: None,
            description: Some(
                "Ethernet MAC wakeup management register (MAC_WUM)",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwd",
                    description: Some(
                        "Power down",
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
                    name: "mpen",
                    description: Some(
                        "Magic Packet enable",
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
                    name: "wfen",
                    description: Some(
                        "Wakeup frame enable",
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
                    name: "mpkr",
                    description: Some(
                        "Magic packet received",
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
                    name: "wufr",
                    description: Some(
                        "Wakeup frame received",
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
                    name: "gu",
                    description: Some(
                        "Global unicast",
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
                    name: "wuffrpr",
                    description: Some(
                        "Wakeup frame filter register pointer reset",
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
    ],
    enums: &[],
};
                