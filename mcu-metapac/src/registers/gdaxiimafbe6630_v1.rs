
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Axiim",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "axi_periph_id4",
                    description: None,
                    array: None,
                    byte_offset: 0x1fd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiPeriphId4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_periph_id0",
                    description: None,
                    array: None,
                    byte_offset: 0x1fe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiPeriphId0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_periph_id1",
                    description: None,
                    array: None,
                    byte_offset: 0x1fe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiPeriphId1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_periph_id2",
                    description: None,
                    array: None,
                    byte_offset: 0x1fe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiPeriphId2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_periph_id3",
                    description: None,
                    array: None,
                    byte_offset: 0x1fec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiPeriphId3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_comp_id0",
                    description: None,
                    array: None,
                    byte_offset: 0x1ff0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiCompId0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_comp_id1",
                    description: None,
                    array: None,
                    byte_offset: 0x1ff4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiCompId1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_comp_id2",
                    description: None,
                    array: None,
                    byte_offset: 0x1ff8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiCompId2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_comp_id3",
                    description: None,
                    array: None,
                    byte_offset: 0x1ffc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiCompId3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp0bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x2008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x2008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp0bm_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x2024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x2024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp0_lb_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x202c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x202c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp0_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x2108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x2108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp1bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x3008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x3008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp1bm_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x3024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x3024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp1_lb_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x302c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x302c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp1_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x3108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x3108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp2bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x4008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x4008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x4024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x402c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x4108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp3bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x5008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x5008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x5024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x502c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x5108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp4bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x6008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x6008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x6024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x602c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x6108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp5bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x7008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x7008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x7024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x702c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x7108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp6bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x8008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_6",
                    description: None,
                    array: None,
                    byte_offset: 0x8008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp6bm_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x8024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_6",
                    description: None,
                    array: None,
                    byte_offset: 0x8024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_6",
                    description: None,
                    array: None,
                    byte_offset: 0x802c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp6_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x8108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_6",
                    description: None,
                    array: None,
                    byte_offset: 0x8108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp7bm_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x9008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_iss_ctl_7",
                    description: None,
                    array: None,
                    byte_offset: 0x9008,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp7bm_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x9024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpxbm_ctl_7",
                    description: None,
                    array: None,
                    byte_offset: 0x9024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxbmCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_lb_ctl_7",
                    description: None,
                    array: None,
                    byte_offset: 0x902c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxLbCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mp7_iss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x9108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_mpx_iss_ctl_7",
                    description: None,
                    array: None,
                    byte_offset: 0x9108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiMpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_sp0_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x42024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x42024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_sp0_ahbiss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x42028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x42028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x42100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x42104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x42108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x43024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x43028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x43100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x43104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x43108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_sp2_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x44024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_sp2_ahbiss_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x44028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x45024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x45028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x45100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x45104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x45108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x46024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x46028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x46100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x46104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x46108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x47024,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_ahbiss_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x47028,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxAhbissCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_rdqos_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x47100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxRdqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_wrqos_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x47104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxWrqosCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "axi_spx_iss_ctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0x47108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AxiSpxIssCtl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AxiCompId0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_comp_id0_preamb",
                    description: None,
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
            name: "AxiCompId1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_comp_id1_preamb",
                    description: None,
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
                    name: "axi_comp_id1_class",
                    description: None,
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
            name: "AxiCompId2",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_comp_id2_preamb",
                    description: None,
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
            name: "AxiCompId3",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_comp_id3_preamb",
                    description: None,
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
            name: "AxiMpxIssCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_mpx_iss_ctl_rd_issov",
                    description: None,
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
                    name: "axi_mpx_iss_ctl_wr_issov",
                    description: None,
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
            name: "AxiMpxLbCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_mpx_lb_ctl_lben",
                    description: None,
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
            name: "AxiMpxbmCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_mpxbm_ctl_bpdis",
                    description: None,
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
            name: "AxiMpxbmIssCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_mpxbm_iss_ctl_rd_issov",
                    description: None,
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
                    name: "axi_mpxbm_iss_ctl_wr_issov",
                    description: None,
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
            name: "AxiPeriphId0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_periph_id0_partnum",
                    description: None,
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
            name: "AxiPeriphId1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_periph_id1_partnum",
                    description: None,
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
                    name: "axi_periph_id1_jep106id",
                    description: None,
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
            name: "AxiPeriphId2",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_periph_id2_jep106id",
                    description: None,
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
                    name: "axi_periph_id2_jep106cf",
                    description: None,
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
                    name: "axi_periph_id2_partrev",
                    description: None,
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
            name: "AxiPeriphId3",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_periph_id3_custmod",
                    description: None,
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
                    name: "axi_periph_id3_custrev",
                    description: None,
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
            name: "AxiPeriphId4",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_periph_id4_jep106ccode",
                    description: None,
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
                    name: "axi_periph_id4_4kbcnt",
                    description: None,
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
            name: "AxiSpxAhbissCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_spx_ahbiss_ctl_wr_ahb_issov",
                    description: None,
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
                    name: "axi_spx_ahbiss_ctl_rd_ahb_issov",
                    description: None,
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
            name: "AxiSpxCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_spx_ctl_transalt",
                    description: None,
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
            name: "AxiSpxIssCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_spx_iss_ctl_rd_issov",
                    description: None,
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
                    name: "axi_spx_iss_ctl_wr_issov",
                    description: None,
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
            name: "AxiSpxRdqosCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_spx_rdqos_ctl_rdqos",
                    description: None,
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
            name: "AxiSpxWrqosCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "axi_spx_wrqos_ctl_wrqos",
                    description: None,
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
                