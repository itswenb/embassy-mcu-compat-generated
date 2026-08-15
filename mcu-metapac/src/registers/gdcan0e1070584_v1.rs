
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Can0",
            extends: None,
            description: Some(
                "Controller area network",
            ),
            items: &[
                BlockItem {
                    name: "ctl0",
                    description: Some(
                        "Control register 0",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl1",
                    description: Some(
                        "Control register 1",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer",
                    description: Some(
                        "Timer register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rmpubf",
                    description: Some(
                        "Receive mailbox public filter register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rmpubf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "err0",
                    description: Some(
                        "Error register 0",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Err0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "err1",
                    description: Some(
                        "Error register 1",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Err1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "inten",
                    description: Some(
                        "Interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Inten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat",
                    description: Some(
                        "Status register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctl2",
                    description: Some(
                        "Control register 2",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crcc",
                    description: Some(
                        "CRC for classical frame register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crcc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifopubf",
                    description: Some(
                        "Receive FIFO public filter register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifopubf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifoifmn",
                    description: Some(
                        "Receive FIFO identifier filter matching number register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifoifmn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bt",
                    description: Some(
                        "Bit timing register",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf0",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x880,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf1",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x884,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf2",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x888,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf3",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x88c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf4",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x890,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf5",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x894,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf6",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x898,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf7",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x89c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf8",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf9",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf10",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf11",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf12",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf13",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf14",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf15",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf15",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf16",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8c0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf17",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8c4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf17",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf18",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8c8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf18",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf19",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8cc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf20",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8d0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf20",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf21",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8d4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf22",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8d8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf23",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8dc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf24",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8e0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf24",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf25",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8e4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf26",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8e8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf26",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf27",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8ec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf28",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8f0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf28",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf29",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8f4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf29",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf30",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8f8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf30",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rfifompf31",
                    description: Some(
                        "Receive FIFO/mailbox private filter x register",
                    ),
                    array: None,
                    byte_offset: 0x8fc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rfifompf31",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_ctl0",
                    description: Some(
                        "CAN_sleep mode control register 0",
                    ),
                    array: None,
                    byte_offset: 0xb00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpCtl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_to",
                    description: Some(
                        "CAN_sleep mode timeout register",
                    ),
                    array: None,
                    byte_offset: 0xb04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpTo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_stat",
                    description: Some(
                        "CAN_sleep mode status register",
                    ),
                    array: None,
                    byte_offset: 0xb08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpStat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_eid0",
                    description: Some(
                        "CAN_sleep mode expected identifier 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb0c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpEid0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_edlc",
                    description: Some(
                        "CAN_sleep mode expected DLC register",
                    ),
                    array: None,
                    byte_offset: 0xb10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpEdlc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_edl0",
                    description: Some(
                        "CAN_sleep mode expected data low 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpEdl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_edl1",
                    description: Some(
                        "CAN_sleep mode expected data low 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpEdl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ifeid1",
                    description: Some(
                        "CAN_sleep mode identifier filter / expected identifier 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ifeid1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_df0edh0",
                    description: Some(
                        "CAN_sleep mode data 0 filter / expected data high 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpDf0edh0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_df1edh1",
                    description: Some(
                        "CAN_sleep mode data 1 filter / expected data high 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpDf1edh1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm0cs",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x control status information register",
                    ),
                    array: None,
                    byte_offset: 0xb40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm0cs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_rwm0i",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x identifier register",
                    ),
                    array: None,
                    byte_offset: 0xb44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpRwm0i",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm0d0",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm0d0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm0d1",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm0d1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm1cs",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x control status information register",
                    ),
                    array: None,
                    byte_offset: 0xb50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm1cs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_rwm1i",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x identifier register",
                    ),
                    array: None,
                    byte_offset: 0xb54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpRwm1i",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm1d0",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm1d0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm1d1",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm1d1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm2cs",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x control status information register",
                    ),
                    array: None,
                    byte_offset: 0xb60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm2cs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_rwm2i",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x identifier register",
                    ),
                    array: None,
                    byte_offset: 0xb64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpRwm2i",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm2d0",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm2d0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm2d1",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm2d1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm3cs",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x control status information register",
                    ),
                    array: None,
                    byte_offset: 0xb70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm3cs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_rwm3i",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x identifier register",
                    ),
                    array: None,
                    byte_offset: 0xb74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SlpRwm3i",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm3d0",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 0 register",
                    ),
                    array: None,
                    byte_offset: 0xb78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm3d0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rwm3d1",
                    description: Some(
                        "CAN_sleep mode received wakeup mailbox x data 1 register",
                    ),
                    array: None,
                    byte_offset: 0xb7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Rwm3d1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fdctl",
                    description: Some(
                        "FD control register",
                    ),
                    array: None,
                    byte_offset: 0xc00,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fdctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fdbt",
                    description: Some(
                        "FD bit timing register",
                    ),
                    array: None,
                    byte_offset: 0xc04,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fdbt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crccfd",
                    description: Some(
                        "CRC for classical and FD frame register",
                    ),
                    array: None,
                    byte_offset: 0xc08,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crccfd",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Bt",
            extends: None,
            description: Some(
                "Bit timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pbs2",
                    description: Some(
                        "Phase buffer segment 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pbs1",
                    description: Some(
                        "Phase buffer segment 1",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pts",
                    description: Some(
                        "Propagation time segment",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sjw",
                    description: Some(
                        "Resynchronization jump width",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "baudpsc",
                    description: Some(
                        "Baud rate prescaler",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crcc",
            extends: None,
            description: Some(
                "CRC for classical frame register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crctc",
                    description: Some(
                        "Transmitted CRC value for classical frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "antm",
                    description: Some(
                        "Associated number of mailbox for transmitting the CRCTC[14:0] value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crccfd",
            extends: None,
            description: Some(
                "CRC for classical and FD frame register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crctci",
                    description: Some(
                        "Transmitted CRC value for classical and ISO / non-ISO FD frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 21,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "antm",
                    description: Some(
                        "Associated number of mailbox for transmitting the CRCTCI[20:0] value",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ctl0",
            extends: None,
            description: Some(
                "Control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msz",
                    description: Some(
                        "Memory size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fs",
                    description: Some(
                        "Format selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fden",
                    description: Some(
                        "CAN FD operation enable",
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
                    name: "mst",
                    description: Some(
                        "Mailbox stop transmission",
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
                    name: "laprioen",
                    description: Some(
                        "Local arbitration priority enable",
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
                    name: "pnmod",
                    description: Some(
                        "Pretended Networking mode selection",
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
                    name: "dmaen",
                    description: Some(
                        "DMA enable",
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
                    name: "rpfqen",
                    description: Some(
                        "Rx private filters enable Rx mailbox queue enable",
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
                    name: "srdis",
                    description: Some(
                        "Self reception disable",
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
                    name: "pns",
                    description: Some(
                        "Pretended Networking state",
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
                    name: "pnen",
                    description: Some(
                        "Pretended Networking mode enable",
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
                    name: "lps",
                    description: Some(
                        "Low power state",
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
                    name: "werren",
                    description: Some(
                        "Error warning enable",
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
                    name: "slps",
                    description: Some(
                        "This bit is only valid when CAN_sleep mode is enabled",
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
                Field {
                    name: "inas",
                    description: Some(
                        "Inactive mode state",
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
                    name: "swrst",
                    description: Some(
                        "Software reset",
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
                    name: "nrdy",
                    description: Some(
                        "Not ready",
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
                    name: "halt",
                    description: Some(
                        "Halt CAN",
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
                    name: "rfen",
                    description: Some(
                        "Rx FIFO enable",
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
                    name: "inamod",
                    description: Some(
                        "Inactive mode enable",
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
                    name: "candis",
                    description: Some(
                        "CAN disable",
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
            name: "Ctl1",
            extends: None,
            description: Some(
                "Control register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmod",
                    description: Some(
                        "Monitor mode",
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
                    name: "mto",
                    description: Some(
                        "Mailbox transmission order",
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
                    name: "tsync",
                    description: Some(
                        "Time synchronization enable",
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
                    name: "abordis",
                    description: Some(
                        "Automatic Bus off recovery not enable",
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
                    name: "bspmod",
                    description: Some(
                        "Bit sampling mode",
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
                    name: "rwerrie",
                    description: Some(
                        "Rx error warning interrupt enable",
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
                    name: "twerrie",
                    description: Some(
                        "Tx error warning interrupt enable",
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
                    name: "lscmod",
                    description: Some(
                        "Loopback and silent communication mode",
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
                    name: "errsie",
                    description: Some(
                        "Error summary interrupt enable",
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
                    name: "boie",
                    description: Some(
                        "Bus off interrupt enable",
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
            ],
        },
        FieldSet {
            name: "Ctl2",
            extends: None,
            description: Some(
                "Control register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "werrie",
                    description: Some(
                        "Write error interrupt enable",
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
                    name: "efdis",
                    description: Some(
                        "Edge filtering disable",
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
                    name: "iso",
                    description: Some(
                        "ISO CAN FD",
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
                    name: "preen",
                    description: Some(
                        "Protocol exception detection enable by CAN standard",
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
                    name: "itsrc",
                    description: Some(
                        "Internal counter source",
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
                    name: "idertr_rmf",
                    description: Some(
                        "IDE and RTR field filter type for Rx mailbox reception",
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
                    name: "rrfrms",
                    description: Some(
                        "Remote request frame is stored",
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
                    name: "rfo",
                    description: Some(
                        "Receive filter order",
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
                    name: "asd",
                    description: Some(
                        "Arbitration start delay",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rffn",
                    description: Some(
                        "Rx FIFO filter number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borie",
                    description: Some(
                        "Bus off recovery interrupt enable",
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
                    name: "errfsie",
                    description: Some(
                        "Error summary interrupt enable bit for data phase of FD frames",
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
            name: "Err0",
            extends: None,
            description: Some(
                "Error register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tecnt",
                    description: Some(
                        "Transmit error count defined by the CAN standard",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "recnt",
                    description: Some(
                        "Receive error count defined by the CAN standard",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tefcnt",
                    description: Some(
                        "Transmit error count for the data phase of FD frames",
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
                    name: "refcnt",
                    description: Some(
                        "Receive error counter for data phase of FD frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Err1",
            extends: None,
            description: Some(
                "Error register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "werr",
                    description: Some(
                        "Write error",
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
                    name: "errsf",
                    description: Some(
                        "Error summary flag",
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
                    name: "bof",
                    description: Some(
                        "Bus off flag",
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
                    name: "rs",
                    description: Some(
                        "Receiving state",
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
                    name: "errsi",
                    description: Some(
                        "Error state indicator",
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
                    name: "ts",
                    description: Some(
                        "Transmitting state",
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
                    name: "idlef",
                    description: Some(
                        "IDLE flag",
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
                    name: "rwerrf",
                    description: Some(
                        "Rx error warning flag",
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
                    name: "twerrf",
                    description: Some(
                        "Tx error warning flag",
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
                    name: "stferr",
                    description: Some(
                        "Stuff error",
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
                    name: "fmerr",
                    description: Some(
                        "Form error",
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
                    name: "crcerr",
                    description: Some(
                        "CRC error",
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
                    name: "ackerr",
                    description: Some(
                        "ACK error",
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
                    name: "bderr",
                    description: Some(
                        "Bit dominant error for all format frames",
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
                    name: "brerr",
                    description: Some(
                        "Bit recessive error for all format frames",
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
                    name: "rwerrif",
                    description: Some(
                        "Rx error warning interrupt flag",
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
                    name: "twerrif",
                    description: Some(
                        "Tx error warning interrupt flag",
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
                    name: "syn",
                    description: Some(
                        "Synchronization flag",
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
                    name: "borf",
                    description: Some(
                        "Bus off recovery flag",
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
                    name: "errfsf",
                    description: Some(
                        "Error summary flag for data phase of FD frames",
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
                    name: "errovr",
                    description: Some(
                        "Error overrun",
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
                    name: "stfferr",
                    description: Some(
                        "Form error in data phase of FD frames",
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
                    name: "fmferr",
                    description: Some(
                        "Form error in data phase of FD frames",
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
                    name: "crcferr",
                    description: Some(
                        "CRC error in data phase of FD frames",
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
                    name: "bdferr",
                    description: Some(
                        "Bit dominant error in data phase of FD frames",
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
                    name: "brferr",
                    description: Some(
                        "Bit recessive error in data phase of FD frames",
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
            name: "Fdbt",
            extends: None,
            description: Some(
                "FD bit timing register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpbs2",
                    description: Some(
                        "Phase buffer segment 2 for data bit time",
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
                    name: "dpbs1",
                    description: Some(
                        "Phase buffer segment 1 for data bit time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpts",
                    description: Some(
                        "Propagation time segment for data bit time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsjw",
                    description: Some(
                        "Resynchronization jump width for data bit time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dbaudpsc",
                    description: Some(
                        "Baud rate prescaler for data bit time",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fdctl",
            extends: None,
            description: Some(
                "FD control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tdcv",
                    description: Some(
                        "Transmitter delay compensation value",
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
                    name: "tdco",
                    description: Some(
                        "Transmitter delay compensation offset",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tdcs",
                    description: Some(
                        "Transmitter delay compensation status",
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
                    name: "tdcen",
                    description: Some(
                        "Transmitter delay compensation enable",
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
                    name: "mdsz",
                    description: Some(
                        "Mailbox data size",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "brsen",
                    description: Some(
                        "Bit rate of data switch enable",
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
            name: "Ifeid1",
            extends: None,
            description: Some(
                "CAN_sleep mode identifier filter / expected identifier 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idfd_eht",
                    description: Some(
                        "ID filter data / ID expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtrfd",
                    description: Some(
                        "RTR filter data in CAN_sleep mode",
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
                    name: "idefd",
                    description: Some(
                        "IDE filter data in CAN_sleep mode",
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
            name: "Inten",
            extends: None,
            description: Some(
                "Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "miex",
                    description: Some(
                        "Message transmission and reception interrupt enable",
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
            name: "Rfifoifmn",
            extends: None,
            description: Some(
                "Receive FIFO identifier filter matching number register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "idfmn",
                    description: Some(
                        "Identifier filter matching number",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rfifompf0",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf1",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf10",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf11",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf12",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf13",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf14",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf15",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf16",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf17",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf18",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf19",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf2",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf20",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf21",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf22",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf23",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf24",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf25",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf26",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf27",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf28",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf29",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf3",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf30",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf31",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf4",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf5",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf6",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf7",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf8",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifompf9",
            extends: None,
            description: Some(
                "Receive FIFO/mailbox private filter x register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmfdx",
                    description: Some(
                        "FIFO/mailbox filter data",
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
            name: "Rfifopubf",
            extends: None,
            description: Some(
                "Receive FIFO public filter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ffdx",
                    description: Some(
                        "Rx FIFO filter data",
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
            name: "Rmpubf",
            extends: None,
            description: Some(
                "Receive mailbox public filter register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mfdx",
                    description: Some(
                        "Mailbox filter data",
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
            name: "Rwm0cs",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x control status information register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdlc",
                    description: Some(
                        "Received DLC bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrtr",
                    description: Some(
                        "Received RTR bit",
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
                    name: "ride",
                    description: Some(
                        "Received IDE bit",
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
                    name: "rsrr",
                    description: Some(
                        "Received SRR bit",
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
            ],
        },
        FieldSet {
            name: "Rwm0d0",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb3",
                    description: Some(
                        "Received data byte 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb2",
                    description: Some(
                        "Received data byte 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb1",
                    description: Some(
                        "Received data byte 1",
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
                    name: "rdb0",
                    description: Some(
                        "Received data byte 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm0d1",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb7",
                    description: Some(
                        "Received data byte 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb6",
                    description: Some(
                        "Received data byte 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb5",
                    description: Some(
                        "Received data byte 5",
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
                    name: "rdb4",
                    description: Some(
                        "Received data byte 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm1cs",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x control status information register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdlc",
                    description: Some(
                        "Received DLC bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrtr",
                    description: Some(
                        "Received RTR bit",
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
                    name: "ride",
                    description: Some(
                        "Received IDE bit",
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
                    name: "rsrr",
                    description: Some(
                        "Received SRR bit",
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
            ],
        },
        FieldSet {
            name: "Rwm1d0",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb3",
                    description: Some(
                        "Received data byte 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb2",
                    description: Some(
                        "Received data byte 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb1",
                    description: Some(
                        "Received data byte 1",
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
                    name: "rdb0",
                    description: Some(
                        "Received data byte 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm1d1",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb7",
                    description: Some(
                        "Received data byte 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb6",
                    description: Some(
                        "Received data byte 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb5",
                    description: Some(
                        "Received data byte 5",
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
                    name: "rdb4",
                    description: Some(
                        "Received data byte 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm2cs",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x control status information register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdlc",
                    description: Some(
                        "Received DLC bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrtr",
                    description: Some(
                        "Received RTR bit",
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
                    name: "ride",
                    description: Some(
                        "Received IDE bit",
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
                    name: "rsrr",
                    description: Some(
                        "Received SRR bit",
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
            ],
        },
        FieldSet {
            name: "Rwm2d0",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb3",
                    description: Some(
                        "Received data byte 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb2",
                    description: Some(
                        "Received data byte 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb1",
                    description: Some(
                        "Received data byte 1",
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
                    name: "rdb0",
                    description: Some(
                        "Received data byte 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm2d1",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb7",
                    description: Some(
                        "Received data byte 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb6",
                    description: Some(
                        "Received data byte 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb5",
                    description: Some(
                        "Received data byte 5",
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
                    name: "rdb4",
                    description: Some(
                        "Received data byte 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm3cs",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x control status information register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdlc",
                    description: Some(
                        "Received DLC bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrtr",
                    description: Some(
                        "Received RTR bit",
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
                    name: "ride",
                    description: Some(
                        "Received IDE bit",
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
                    name: "rsrr",
                    description: Some(
                        "Received SRR bit",
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
            ],
        },
        FieldSet {
            name: "Rwm3d0",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb3",
                    description: Some(
                        "Received data byte 3",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb2",
                    description: Some(
                        "Received data byte 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb1",
                    description: Some(
                        "Received data byte 1",
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
                    name: "rdb0",
                    description: Some(
                        "Received data byte 0",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rwm3d1",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x data 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdb7",
                    description: Some(
                        "Received data byte 7",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb6",
                    description: Some(
                        "Received data byte 6",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdb5",
                    description: Some(
                        "Received data byte 5",
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
                    name: "rdb4",
                    description: Some(
                        "Received data byte 4",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpCtl0",
            extends: None,
            description: Some(
                "CAN_sleep mode control register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fft",
                    description: Some(
                        "Frame filtering type in CAN_sleep mode",
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
                    name: "idft",
                    description: Some(
                        "ID field filtering type in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dataft",
                    description: Some(
                        "DATA field filtering type in CAN_sleep mode",
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
                    name: "nmm",
                    description: Some(
                        "Number of messages matching times",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wmie",
                    description: Some(
                        "Wakeup match interrupt enable",
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
                    name: "wtoie",
                    description: Some(
                        "Wakeup timeout interrupt enable",
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
            name: "SlpDf0edh0",
            extends: None,
            description: Some(
                "CAN_sleep mode data 0 filter / expected data high 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "db3fd_eht",
                    description: Some(
                        "Data byte 3 filter data / Data byte 3 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db2fd_eht",
                    description: Some(
                        "Data byte 3 filter data / Data byte 3 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db1fd_eht",
                    description: Some(
                        "Data byte 1 filter data / Data byte 1 expected high threshold in CAN_sleep mode",
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
                    name: "db0fd_eht",
                    description: Some(
                        "Data byte 0 filter data / Data byte 0 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpDf1edh1",
            extends: None,
            description: Some(
                "CAN_sleep mode data 1 filter / expected data high 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "db7fd_eht",
                    description: Some(
                        "Data byte 7 filter data / Data byte 7 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db6fd_eht",
                    description: Some(
                        "Data byte 6 filter data / Data byte 6 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db5fd_eht",
                    description: Some(
                        "Data byte 5 filter data / Data byte 5 expected high threshold in CAN_sleep mode",
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
                    name: "db4fd_eht",
                    description: Some(
                        "Data byte 4 filter data / Data byte 4 expected high threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpEdl0",
            extends: None,
            description: Some(
                "CAN_sleep mode expected data low 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "db3elt",
                    description: Some(
                        "Data byte 3 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db2elt",
                    description: Some(
                        "Data byte 2 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db1elt",
                    description: Some(
                        "Data byte 1 expected low threshold in CAN_sleep mode",
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
                    name: "db0elt",
                    description: Some(
                        "Data byte 0 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpEdl1",
            extends: None,
            description: Some(
                "CAN_sleep mode expected data low 1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "db7elt",
                    description: Some(
                        "Data byte 7 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db6elt",
                    description: Some(
                        "Data byte 6 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "db5elt",
                    description: Some(
                        "Data byte 5 expected low threshold in CAN_sleep mode",
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
                    name: "db4elt",
                    description: Some(
                        "Data byte 4 expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpEdlc",
            extends: None,
            description: Some(
                "CAN_sleep mode expected DLC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dlceht",
                    description: Some(
                        "DLC expected high threshold in CAN_sleep mode",
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
                    name: "dlcelt",
                    description: Some(
                        "DLC expected low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpEid0",
            extends: None,
            description: Some(
                "CAN_sleep mode expected identifier 0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "eidf_elt",
                    description: Some(
                        "Expected ID field / expected ID low threshold in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ertr",
                    description: Some(
                        "Expected RTR in CAN_sleep mode",
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
                    name: "eide",
                    description: Some(
                        "Expected IDE in CAN_sleep mode",
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
            name: "SlpRwm0i",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rid",
                    description: Some(
                        "Received ID bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpRwm1i",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rid",
                    description: Some(
                        "Received ID bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpRwm2i",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rid",
                    description: Some(
                        "Received ID bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpRwm3i",
            extends: None,
            description: Some(
                "CAN_sleep mode received wakeup mailbox x identifier register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rid",
                    description: Some(
                        "Received ID bits",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 29,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpStat",
            extends: None,
            description: Some(
                "CAN_sleep mode status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mmcnts",
                    description: Some(
                        "Matching message counter state",
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
                    name: "mmcnt",
                    description: Some(
                        "Matching message counter in CAN_sleep mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wms",
                    description: Some(
                        "Wakeup match flag status",
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
                    name: "wtos",
                    description: Some(
                        "Wakeup timeout flag status",
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
            name: "SlpTo",
            extends: None,
            description: Some(
                "CAN_sleep mode timeout register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wto",
                    description: Some(
                        "Wakeup timeout",
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
            name: "Stat",
            extends: None,
            description: Some(
                "Status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ms0_rfc",
                    description: Some(
                        "Mailbox 3 state / Clear Rx FIFO bit",
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
                    name: "ms1_res",
                    description: Some(
                        "Mailbox 1 state / Reserved",
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
                    name: "ms2_res",
                    description: Some(
                        "Mailbox 2 state / Reserved",
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
                    name: "ms3_res",
                    description: Some(
                        "Mailbox 3 state / Reserved",
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
                    name: "ms4_res",
                    description: Some(
                        "Mailbox 4 state / Reserved",
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
                    name: "ms5_rfne",
                    description: Some(
                        "Mailbox 5 state / Rx FIFO not empty",
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
                    name: "ms6_rfw",
                    description: Some(
                        "Mailbox 6 state / Rx FIFO warning",
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
                    name: "ms7_rfo",
                    description: Some(
                        "Mailbox 7 state / Rx FIFO overflow",
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
                    name: "msx",
                    description: Some(
                        "Mailbox x state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer",
            extends: None,
            description: Some(
                "Timer register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt",
                    description: Some(
                        "Counter value",
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
    ],
    enums: &[],
};
                