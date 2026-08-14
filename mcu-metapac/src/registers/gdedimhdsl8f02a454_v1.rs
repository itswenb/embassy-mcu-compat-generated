
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "EdimHdsl",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "edim_hdsl_sys_ctrl",
                    description: None,
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSysCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_sync_ctrl",
                    description: None,
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSyncCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_master_qm",
                    description: None,
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMasterQm",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_event_h",
                    description: None,
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEventH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_event_l",
                    description: None,
                    array: None,
                    byte_offset: 0x5,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEventL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mask_h",
                    description: None,
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaskH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mask_l",
                    description: None,
                    array: None,
                    byte_offset: 0x7,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaskL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mask_sum",
                    description: None,
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaskSum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_edges",
                    description: None,
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEdges",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_delay",
                    description: None,
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslDelay",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_version",
                    description: None,
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVersion",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_id2",
                    description: None,
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncId2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_id1",
                    description: None,
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_id0",
                    description: None,
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pos4",
                    description: None,
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pos3",
                    description: None,
                    array: None,
                    byte_offset: 0x11,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pos2",
                    description: None,
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pos1",
                    description: None,
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pos0",
                    description: None,
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vel2",
                    description: None,
                    array: None,
                    byte_offset: 0x15,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vel1",
                    description: None,
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vel0",
                    description: None,
                    array: None,
                    byte_offset: 0x17,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_sum",
                    description: None,
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer0",
                    description: None,
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer1",
                    description: None,
                    array: None,
                    byte_offset: 0x21,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer2",
                    description: None,
                    array: None,
                    byte_offset: 0x22,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer3",
                    description: None,
                    array: None,
                    byte_offset: 0x23,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer4",
                    description: None,
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer5",
                    description: None,
                    array: None,
                    byte_offset: 0x25,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer6",
                    description: None,
                    array: None,
                    byte_offset: 0x26,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_buffer7",
                    description: None,
                    array: None,
                    byte_offset: 0x27,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_add_h",
                    description: None,
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcAddH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_add_l",
                    description: None,
                    array: None,
                    byte_offset: 0x29,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcAddL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_off_h",
                    description: None,
                    array: None,
                    byte_offset: 0x2a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcOffH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_off_l",
                    description: None,
                    array: None,
                    byte_offset: 0x2b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcOffL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_ctrl",
                    description: None,
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pipe_s",
                    description: None,
                    array: None,
                    byte_offset: 0x2d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPipeS",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pipe_d",
                    description: None,
                    array: None,
                    byte_offset: 0x2e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPipeD",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_pc_data",
                    description: None,
                    array: None,
                    byte_offset: 0x2f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPcData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_acc_err_cnt",
                    description: None,
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslAccErrCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_maxacc",
                    description: None,
                    array: None,
                    byte_offset: 0x39,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaxacc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_maxdev_h",
                    description: None,
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaxdevH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_maxdev_l",
                    description: None,
                    array: None,
                    byte_offset: 0x3b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaxdevL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_dummy",
                    description: None,
                    array: None,
                    byte_offset: 0x3f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslDummy",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_0",
                    description: None,
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_1",
                    description: None,
                    array: None,
                    byte_offset: 0x61,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_2",
                    description: None,
                    array: None,
                    byte_offset: 0x62,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_3",
                    description: None,
                    array: None,
                    byte_offset: 0x63,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_4",
                    description: None,
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_5",
                    description: None,
                    array: None,
                    byte_offset: 0x65,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_6",
                    description: None,
                    array: None,
                    byte_offset: 0x66,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mir_st_7",
                    description: None,
                    array: None,
                    byte_offset: 0x67,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMirSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_edges",
                    description: None,
                    array: None,
                    byte_offset: 0x209,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_delay",
                    description: None,
                    array: None,
                    byte_offset: 0x20a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_version",
                    description: None,
                    array: None,
                    byte_offset: 0x20b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_enc_id2",
                    description: None,
                    array: None,
                    byte_offset: 0x20d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_enc_id1",
                    description: None,
                    array: None,
                    byte_offset: 0x20e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_enc_id0",
                    description: None,
                    array: None,
                    byte_offset: 0x20f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos4",
                    description: None,
                    array: None,
                    byte_offset: 0x219,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos3",
                    description: None,
                    array: None,
                    byte_offset: 0x21a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos2",
                    description: None,
                    array: None,
                    byte_offset: 0x21b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVpos2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos1",
                    description: None,
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos0",
                    description: None,
                    array: None,
                    byte_offset: 0x21d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vposcrc_h",
                    description: None,
                    array: None,
                    byte_offset: 0x21e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVposcrcH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vposcrc_l",
                    description: None,
                    array: None,
                    byte_offset: 0x21f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVposcrcL",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_ctrl",
                    description: None,
                    array: None,
                    byte_offset: 0x235,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSafeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_safe_sum",
                    description: None,
                    array: None,
                    byte_offset: 0x236,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSafeSum",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_s_pc_data",
                    description: None,
                    array: None,
                    byte_offset: 0x237,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSPcData",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_event_s",
                    description: None,
                    array: None,
                    byte_offset: 0x23d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEventS",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mask_s",
                    description: None,
                    array: None,
                    byte_offset: 0x23e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMaskS",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_0",
                    description: None,
                    array: None,
                    byte_offset: 0x240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_1",
                    description: None,
                    array: None,
                    byte_offset: 0x241,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_2",
                    description: None,
                    array: None,
                    byte_offset: 0x242,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_3",
                    description: None,
                    array: None,
                    byte_offset: 0x243,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_4",
                    description: None,
                    array: None,
                    byte_offset: 0x244,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_5",
                    description: None,
                    array: None,
                    byte_offset: 0x245,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_6",
                    description: None,
                    array: None,
                    byte_offset: 0x246,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc_st_7",
                    description: None,
                    array: None,
                    byte_offset: 0x247,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEncSt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_srssi",
                    description: None,
                    array: None,
                    byte_offset: 0x27c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslSrssi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_mail",
                    description: None,
                    array: None,
                    byte_offset: 0x27e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslMail",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_ping",
                    description: None,
                    array: None,
                    byte_offset: 0x27f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslPing",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_version2",
                    description: None,
                    array: None,
                    byte_offset: 0x30b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVersion2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_enc2_id",
                    description: None,
                    array: None,
                    byte_offset: 0x30f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslEnc2Id",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_status2",
                    description: None,
                    array: None,
                    byte_offset: 0x318,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslStatus2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos24",
                    description: None,
                    array: None,
                    byte_offset: 0x319,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos23",
                    description: None,
                    array: None,
                    byte_offset: 0x31a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos22",
                    description: None,
                    array: None,
                    byte_offset: 0x31b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos21",
                    description: None,
                    array: None,
                    byte_offset: 0x31c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vpos20",
                    description: None,
                    array: None,
                    byte_offset: 0x31d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vposcrc2_h",
                    description: None,
                    array: None,
                    byte_offset: 0x31e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVposcrc2H",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_vposcrc2_l",
                    description: None,
                    array: None,
                    byte_offset: 0x31f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslVposcrc2L",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_dummy2",
                    description: None,
                    array: None,
                    byte_offset: 0x33f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "EdimHdslDummy2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_ctl",
                    description: None,
                    array: None,
                    byte_offset: 0x380,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslCtl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_online_status_d",
                    description: None,
                    array: None,
                    byte_offset: 0x384,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslOnlineStatusD",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_online_status_1",
                    description: None,
                    array: None,
                    byte_offset: 0x388,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslOnlineStatus1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_online_status_2",
                    description: None,
                    array: None,
                    byte_offset: 0x38c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslOnlineStatus2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_synlk",
                    description: None,
                    array: None,
                    byte_offset: 0x390,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslSynlk",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "edim_hdsl_inten",
                    description: None,
                    array: None,
                    byte_offset: 0x394,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "EdimHdslInten",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "EdimHdslAccErrCnt",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_acc_err_cnt_cnt",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslCtl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_ctl_endian",
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
                    name: "edim_hdsl_ctl_ssel1",
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
                Field {
                    name: "edim_hdsl_ctl_dsel",
                    description: None,
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
                    name: "edim_hdsl_ctl_ssel2",
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
            ],
        },
        FieldSet {
            name: "EdimHdslDelay",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_delay_cbldly",
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
                    name: "edim_hdsl_delay_rssi",
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
            name: "EdimHdslDummy",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_dummy_dummy",
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
            name: "EdimHdslDummy2",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_dummy2_dummy",
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
            name: "EdimHdslEdges",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_edges_edges",
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
            name: "EdimHdslEnc2Id",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_enc2_id_enc2id",
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
            name: "EdimHdslEncId2",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_enc_id2_encid",
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
                    name: "edim_hdsl_enc_id2_sci",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslEncSt",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_enc_st_encst",
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
            name: "EdimHdslEventH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_event_h_prst",
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
                    name: "edim_hdsl_event_h_dte",
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
                Field {
                    name: "edim_hdsl_event_h_pos",
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
                    name: "edim_hdsl_event_h_sum",
                    description: None,
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
                    name: "edim_hdsl_event_h_int",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslEventL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_event_l_frel",
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
                Field {
                    name: "edim_hdsl_event_l_qmlw",
                    description: None,
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
                    name: "edim_hdsl_event_l_ans",
                    description: None,
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
                    name: "edim_hdsl_event_l_min",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslEventS",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_event_s_fres",
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
                    name: "edim_hdsl_event_s_min",
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
                Field {
                    name: "edim_hdsl_event_s_prst",
                    description: None,
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
                    name: "edim_hdsl_event_s_qmlw",
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
                    name: "edim_hdsl_event_s_vpos",
                    description: None,
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
                    name: "edim_hdsl_event_s_sce",
                    description: None,
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
                    name: "edim_hdsl_event_s_ssum",
                    description: None,
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
                    name: "edim_hdsl_event_s_sint",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslInten",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_inten_intsrc",
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
                    name: "edim_hdsl_inten_postxie",
                    description: None,
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
                    name: "edim_hdsl_inten_intsie",
                    description: None,
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
            name: "EdimHdslMail",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mail_mail",
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
            name: "EdimHdslMaskH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mask_h_mprst",
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
                    name: "edim_hdsl_mask_h_mdte",
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
                Field {
                    name: "edim_hdsl_mask_h_mpos",
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
                    name: "edim_hdsl_mask_h_msum",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMaskL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mask_l_mfrel",
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
                Field {
                    name: "edim_hdsl_mask_l_mqmlw",
                    description: None,
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
                    name: "edim_hdsl_mask_l_mans",
                    description: None,
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
                    name: "edim_hdsl_mask_l_mmin",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMaskS",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mask_s_mfres",
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
                    name: "edim_hdsl_mask_s_mmin",
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
                Field {
                    name: "edim_hdsl_mask_s_mprst",
                    description: None,
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
                    name: "edim_hdsl_mask_s_mqmlw",
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
                    name: "edim_hdsl_mask_s_mvpos",
                    description: None,
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
                    name: "edim_hdsl_mask_s_msce",
                    description: None,
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
                    name: "edim_hdsl_mask_s_mssum",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMaskSum",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mask_sum_0",
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
                    name: "edim_hdsl_mask_sum_msum",
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
                Field {
                    name: "edim_hdsl_mask_sum_1",
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
                Field {
                    name: "edim_hdsl_mask_sum_2",
                    description: None,
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
                    name: "edim_hdsl_mask_sum_3",
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
                    name: "edim_hdsl_mask_sum_4",
                    description: None,
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
                    name: "edim_hdsl_mask_sum_5",
                    description: None,
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
                    name: "edim_hdsl_mask_sum_6",
                    description: None,
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
                    name: "edim_hdsl_mask_sum_7",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMasterQm",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_master_qm_qm",
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
                    name: "edim_hdsl_master_qm_link",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMaxacc",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_maxacc_mnt",
                    description: None,
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
                    name: "edim_hdsl_maxacc_res",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslMaxdevH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_maxdev_h_devh",
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
            name: "EdimHdslMaxdevL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_maxdev_l_devl",
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
            name: "EdimHdslMirSt",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mir_st_mirst",
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
            name: "EdimHdslMirSum",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_mir_sum_sum",
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
            name: "EdimHdslOnlineStatus1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_online_status_1_fres",
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
                    name: "edim_hdsl_online_status_1_qmlw",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_min",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_postx",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_prst",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_vpos",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_sce",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_ssum",
                    description: None,
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
                    name: "edim_hdsl_online_status_1_sint",
                    description: None,
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
            name: "EdimHdslOnlineStatus2",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_online_status_2_qmlw",
                    description: None,
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
                    name: "edim_hdsl_online_status_2_postx",
                    description: None,
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
                    name: "edim_hdsl_online_status_2_prst",
                    description: None,
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
                    name: "edim_hdsl_online_status_2_vpos2",
                    description: None,
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
                    name: "edim_hdsl_online_status_2_sce2",
                    description: None,
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
                    name: "edim_hdsl_online_status_2_ssum2",
                    description: None,
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
            name: "EdimHdslOnlineStatusD",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_online_status_d_frel",
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
                Field {
                    name: "edim_hdsl_online_status_d_qmlw",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_ans",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_min",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_postx",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_prst",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_dte",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_pos",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_sum",
                    description: None,
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
                    name: "edim_hdsl_online_status_d_int",
                    description: None,
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
            name: "EdimHdslPcAddH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_add_h_laddh",
                    description: None,
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
                    name: "edim_hdsl_pc_add_h_llen",
                    description: None,
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
                    name: "edim_hdsl_pc_add_h_lind",
                    description: None,
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
                    name: "edim_hdsl_pc_add_h_loff",
                    description: None,
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
                    name: "edim_hdsl_pc_add_h_lrw",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslPcAddL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_add_l_laddl",
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
            name: "EdimHdslPcCtrl",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_ctrl_lsta",
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
            name: "EdimHdslPcData",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_data_pcdata",
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
            name: "EdimHdslPcOffH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_off_h_loffaddh",
                    description: None,
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
                    name: "edim_hdsl_pc_off_h_lid",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslPcOffL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pc_off_l_loffaddl",
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
            name: "EdimHdslPing",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_ping_ping",
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
            name: "EdimHdslPipeD",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pipe_d_scdata",
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
            name: "EdimHdslPipeS",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_pipe_s_psci",
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
                    name: "edim_hdsl_pipe_s_perr",
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
                Field {
                    name: "edim_hdsl_pipe_s_pemp",
                    description: None,
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
                    name: "edim_hdsl_pipe_s_povr",
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
            ],
        },
        FieldSet {
            name: "EdimHdslSPcData",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_s_pc_data_spcdata",
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
            name: "EdimHdslSafeCtrl",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_safe_ctrl_mrst",
                    description: None,
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
                    name: "edim_hdsl_safe_ctrl_prst",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslSafeSum",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_safe_sum_ssum",
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
            name: "EdimHdslSrssi",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_srssi_srssi",
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
            ],
        },
        FieldSet {
            name: "EdimHdslStatus2",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_status2_fix2",
                    description: None,
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
                    name: "edim_hdsl_status2_err2",
                    description: None,
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
                    name: "edim_hdsl_status2_test2",
                    description: None,
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
                    name: "edim_hdsl_status2_tog2",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslSyncCtrl",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_sync_ctrl_es",
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
            name: "EdimHdslSynlk",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "edim_hdsl_synlk_synlk",
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
            name: "EdimHdslSysCtrl",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_sys_ctrl_oen",
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
                    name: "edim_hdsl_sys_ctrl_spol",
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
                Field {
                    name: "edim_hdsl_sys_ctrl_loop",
                    description: None,
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
                    name: "edim_hdsl_sys_ctrl_frst",
                    description: None,
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
                    name: "edim_hdsl_sys_ctrl_mrst",
                    description: None,
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
                    name: "edim_hdsl_sys_ctrl_prst",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslVersion",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_version_minor",
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
                    name: "edim_hdsl_version_major",
                    description: None,
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
                    name: "edim_hdsl_version_code",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslVersion2",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_version2_minor",
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
                    name: "edim_hdsl_version2_major",
                    description: None,
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
                    name: "edim_hdsl_version2_code",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "EdimHdslVpos2",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_vpos2_vpos2",
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
            name: "EdimHdslVposcrc2H",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_vposcrc2_h_vposcrc2h",
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
            name: "EdimHdslVposcrc2L",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_vposcrc2_l_vposcrc2l",
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
            name: "EdimHdslVposcrcH",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_vposcrc_h_vposcrch",
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
            name: "EdimHdslVposcrcL",
            extends: None,
            description: None,
            bit_size: 8,
            fields: &[
                Field {
                    name: "edim_hdsl_vposcrc_l_vposcrcl",
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
    ],
    enums: &[],
};
                