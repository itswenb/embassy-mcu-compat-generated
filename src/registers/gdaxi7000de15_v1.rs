
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Axi",
            extends: None,
            description: Some(
                "System and memory architectur",
            ),
            items: &[
                BlockItem {
                    name: "periph_id4",
                    description: Some(
                        "AXI peripheral ID4 register",
                    ),
                    array: None,
                    byte_offset: 0x1fd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PeriphId4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "periph_id0",
                    description: Some(
                        "AXI peripheral ID0 register",
                    ),
                    array: None,
                    byte_offset: 0x1fe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PeriphId0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "periph_id1",
                    description: Some(
                        "AXI peripheral ID1 register",
                    ),
                    array: None,
                    byte_offset: 0x1fe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PeriphId1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "periph_id2",
                    description: Some(
                        "AXI peripheral ID2 register",
                    ),
                    array: None,
                    byte_offset: 0x1fe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PeriphId2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "periph_id3",
                    description: Some(
                        "AXI peripheral ID3 register",
                    ),
                    array: None,
                    byte_offset: 0x1fec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PeriphId3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_id0",
                    description: Some(
                        "AXI componet ID0 register",
                    ),
                    array: None,
                    byte_offset: 0x1ff0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CompId0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_id1",
                    description: Some(
                        "AXI componet ID1 register",
                    ),
                    array: None,
                    byte_offset: 0x1ff4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CompId1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_id2",
                    description: Some(
                        "AXI componet ID2 register",
                    ),
                    array: None,
                    byte_offset: 0x1ff8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CompId2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "comp_id3",
                    description: Some(
                        "AXI componet ID3 register",
                    ),
                    array: None,
                    byte_offset: 0x1ffc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "CompId3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp0bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 0 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x2008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp0bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp0bm_ctl",
                    description: Some(
                        "AXI Master Port 0 bus matrix functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x2024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp0bmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp0_lb_ctl",
                    description: Some(
                        "AXI Master Port 0 long burst functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x202c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp0LbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp0_iss_ctl",
                    description: Some(
                        "AXI Master Port 0 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x2108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp0IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp1bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 1 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x3008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp1bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp1bm_ctl",
                    description: Some(
                        "AXI Master Port 1 bus matrix functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x3024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp1bmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp1_lb_ctl",
                    description: Some(
                        "AXI Master Port 1 long burst functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x302c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp1LbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp1_iss_ctl",
                    description: Some(
                        "AXI Master Port 1 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x3108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp1IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp2bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 2 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x4008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp2bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp3bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 3 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x5008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp3bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp4bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 4 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x6008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp4bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp5bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 5 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x7008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp5bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp6bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 6 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x8008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp6bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp6bm_ctl",
                    description: Some(
                        "AXI Master Port 6 bus matrix functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x8024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp6bmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp6_iss_ctl",
                    description: Some(
                        "AXI Master Port 6 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x8108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp6IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp7bm_iss_ctl",
                    description: Some(
                        "AXI Master Port 7 bus matrix issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x9008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp7bmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp7bm_ctl",
                    description: Some(
                        "AXI Master Port 7 bus matrix functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x9024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp7bmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mp7_iss_ctl",
                    description: Some(
                        "AXI Master Port 7 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x9108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mp7IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp0_ctl",
                    description: Some(
                        "AXI Slave Port 0 functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x42024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp0Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp0_ahbiss_ctl",
                    description: Some(
                        "AXI Slave Port 0 AHB issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x42028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp0AhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp0_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 0 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x42100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp0RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp0_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 0 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x42104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp0WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp0_iss_ctl",
                    description: Some(
                        "AXI Slave Port 0 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x42108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp0IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp1_ctl",
                    description: Some(
                        "AXI Slave Port 1 functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x43024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp1Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp1_ahbiss_ctl",
                    description: Some(
                        "AXI Slave Port 1 AHB issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x43028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp1AhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp1_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 1 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x43100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp1RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp1_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 1 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x43104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp1WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp1_iss_ctl",
                    description: Some(
                        "AXI Slave Port 1 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x43108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp1IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp2_ctl",
                    description: Some(
                        "AXI Slave Port 2 functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x44024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp2Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp2_ahbiss_ctl",
                    description: Some(
                        "AXI Slave Port 2 AHB issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x44028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp2AhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp2_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 2 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x44100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp2RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp2_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 2 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x44104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp2WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp2_iss_ctl",
                    description: Some(
                        "AXI Slave Port 2 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x44108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp2IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp3_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 3 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x45100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp3RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp3_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 3 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x45104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp3WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp3_iss_ctl",
                    description: Some(
                        "AXI Slave Port 3 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x45108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp3IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp4_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 4 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x46100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp4RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp4_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 4 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x46104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp4WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp4_iss_ctl",
                    description: Some(
                        "AXI Slave Port 4 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x46108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp4IssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp5_rdqos_ctl",
                    description: Some(
                        "AXI Slave Port 5 read QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x47100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp5RdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp5_wrqos_ctl",
                    description: Some(
                        "AXI Slave Port 5 write QOS control regist",
                    ),
                    array: None,
                    byte_offset: 0x47104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp5WrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sp5_iss_ctl",
                    description: Some(
                        "AXI Slave Port 5 issuing functionality control regist",
                    ),
                    array: None,
                    byte_offset: 0x47108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sp5IssCtl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CompId0",
            extends: None,
            description: Some(
                "AXI componet ID0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamb",
                    description: Some(
                        "Preamble bits",
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
            ],
        },
        FieldSet {
            name: "CompId1",
            extends: None,
            description: Some(
                "AXI componet ID1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "Preamble bits",
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
                    name: "class",
                    description: Some(
                        "Component class",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CompId2",
            extends: None,
            description: Some(
                "AXI componet ID2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamb",
                    description: Some(
                        "Preamble bits",
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
            ],
        },
        FieldSet {
            name: "CompId3",
            extends: None,
            description: Some(
                "AXI componet ID3 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "preamb",
                    description: Some(
                        "Preamble bits",
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
            ],
        },
        FieldSet {
            name: "Mp0IssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 0 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override AMIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override AMIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp0LbCtl",
            extends: None,
            description: Some(
                "AXI Master Port 0 long burst functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lben",
                    description: Some(
                        "Control long burst function",
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
            ],
        },
        FieldSet {
            name: "Mp0bmCtl",
            extends: None,
            description: Some(
                "AXI Master Port 0 bus matrix functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpdis",
                    description: Some(
                        "Beats packing function disable configure",
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
            ],
        },
        FieldSet {
            name: "Mp0bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 0 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp1IssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 1 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override AMIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override AMIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp1LbCtl",
            extends: None,
            description: Some(
                "AXI Master Port 1 long burst functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lben",
                    description: Some(
                        "Control long burst function",
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
            ],
        },
        FieldSet {
            name: "Mp1bmCtl",
            extends: None,
            description: Some(
                "AXI Master Port 1 bus matrix functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpdis",
                    description: Some(
                        "Beats packing function disable configure",
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
            ],
        },
        FieldSet {
            name: "Mp1bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 1 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp2bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 2 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp3bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 3 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp4bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 4 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp5bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 5 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp6IssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 6 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override AMIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override AMIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp6bmCtl",
            extends: None,
            description: Some(
                "AXI Master Port 6 bus matrix functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpdis",
                    description: Some(
                        "Beats packing function disable configure",
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
            ],
        },
        FieldSet {
            name: "Mp6bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 6 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp7IssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 7 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override AMIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override AMIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Mp7bmCtl",
            extends: None,
            description: Some(
                "AXI Master Port 7 bus matrix functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bpdis",
                    description: Some(
                        "Beats packing function disable configure",
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
            ],
        },
        FieldSet {
            name: "Mp7bmIssCtl",
            extends: None,
            description: Some(
                "AXI Master Port 7 bus matrix issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override target read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override target write issuing function",
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
            ],
        },
        FieldSet {
            name: "PeriphId0",
            extends: None,
            description: Some(
                "AXI peripheral ID0 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "Part number[7:0]",
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
            ],
        },
        FieldSet {
            name: "PeriphId1",
            extends: None,
            description: Some(
                "AXI peripheral ID1 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "partnum",
                    description: Some(
                        "Part number[11:8]",
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
                    name: "jep106id",
                    description: Some(
                        "JEP106 Identity[3:0]",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PeriphId2",
            extends: None,
            description: Some(
                "AXI peripheral ID2 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106id",
                    description: Some(
                        "Part number[11:8]",
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
                    name: "jep106cf",
                    description: Some(
                        "JEP106 code flag",
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
                    name: "partrev",
                    description: Some(
                        "Part revision",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PeriphId3",
            extends: None,
            description: Some(
                "AXI peripheral ID3 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "custmod",
                    description: Some(
                        "Customer modification",
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
                    name: "custrev",
                    description: Some(
                        "Customer version",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PeriphId4",
            extends: None,
            description: Some(
                "AXI peripheral ID4 register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jep106ccode",
                    description: Some(
                        "JEP106 continuation code",
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
                    name: "cnt4kb",
                    description: Some(
                        "4KB count",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sp0AhbissCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 0 AHB issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite write transaction to single beat AXI transaction function",
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
                    name: "rd_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite read transaction to single beat AXI transaction function",
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
            ],
        },
        FieldSet {
            name: "Sp0Ctl",
            extends: None,
            description: Some(
                "AXI Slave Port 0 functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "transalt",
                    description: Some(
                        "Transaction alteration configure",
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
            ],
        },
        FieldSet {
            name: "Sp0IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 0 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp0RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 0 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp0WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 0 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp1AhbissCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 1 AHB issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite write transaction to single beat AXI transaction function",
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
                    name: "rd_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite read transaction to single beat AXI transaction function",
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
            ],
        },
        FieldSet {
            name: "Sp1Ctl",
            extends: None,
            description: Some(
                "AXI Slave Port 1 functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "transalt",
                    description: Some(
                        "Transaction alteration configure",
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
            ],
        },
        FieldSet {
            name: "Sp1IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 1 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp1RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 1 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp1WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 1 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp2AhbissCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 2 AHB issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wr_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite write transaction to single beat AXI transaction function",
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
                    name: "rd_ahb_issov",
                    description: Some(
                        "Converts AHB-Lite read transaction to single beat AXI transaction function",
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
            ],
        },
        FieldSet {
            name: "Sp2Ctl",
            extends: None,
            description: Some(
                "AXI Slave Port 2 functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "transalt",
                    description: Some(
                        "Transaction alteration configure",
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
            ],
        },
        FieldSet {
            name: "Sp2IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 2 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp2RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 2 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp2WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 2 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp3IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 3 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp3RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 3 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp3WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 3 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp4IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 4 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp4RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 4 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp4WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 4 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp5IssCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 5 issuing functionality control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rd_issov",
                    description: Some(
                        "Override ASIB read issuing function",
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
                    name: "wr_issov",
                    description: Some(
                        "Override ASIB write issuing function",
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
            ],
        },
        FieldSet {
            name: "Sp5RdqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 5 read QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdqos",
                    description: Some(
                        "Read channel QoS configure",
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
            ],
        },
        FieldSet {
            name: "Sp5WrqosCtl",
            extends: None,
            description: Some(
                "AXI Slave Port 5 write QOS control regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrqos",
                    description: Some(
                        "Write channel QoS configure",
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
            ],
        },
    ],
    enums: &[],
};
