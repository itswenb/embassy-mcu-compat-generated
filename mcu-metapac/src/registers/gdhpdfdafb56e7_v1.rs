
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Hpdf",
            extends: None,
            description: None,
            items: &[
                BlockItem {
                    name: "hpdf_chxctl_0",
                    description: None,
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_0",
                    description: None,
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_0",
                    description: None,
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_0",
                    description: None,
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_0",
                    description: None,
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_0",
                    description: None,
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_1",
                    description: None,
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_1",
                    description: None,
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_1",
                    description: None,
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_1",
                    description: None,
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_1",
                    description: None,
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_1",
                    description: None,
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_2",
                    description: None,
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_2",
                    description: None,
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_2",
                    description: None,
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_2",
                    description: None,
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_2",
                    description: None,
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_2",
                    description: None,
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_3",
                    description: None,
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_3",
                    description: None,
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_3",
                    description: None,
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_3",
                    description: None,
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_3",
                    description: None,
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_3",
                    description: None,
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_4",
                    description: None,
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_4",
                    description: None,
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_4",
                    description: None,
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_4",
                    description: None,
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_4",
                    description: None,
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_4",
                    description: None,
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_5",
                    description: None,
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_5",
                    description: None,
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_5",
                    description: None,
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_5",
                    description: None,
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_5",
                    description: None,
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_5",
                    description: None,
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_6",
                    description: None,
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_6",
                    description: None,
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_6",
                    description: None,
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_6",
                    description: None,
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_6",
                    description: None,
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_6",
                    description: None,
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxctl_7",
                    description: None,
                    array: None,
                    byte_offset: 0xe0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg0_7",
                    description: None,
                    array: None,
                    byte_offset: 0xe4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxcfg1_7",
                    description: None,
                    array: None,
                    byte_offset: 0xe8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxtmfdt_7",
                    description: None,
                    array: None,
                    byte_offset: 0xec,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxtmfdt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxpdi_7",
                    description: None,
                    array: None,
                    byte_offset: 0xf0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxpdi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_chxps_7",
                    description: None,
                    array: None,
                    byte_offset: 0xf4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfChxps",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl0_0",
                    description: None,
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl1_0",
                    description: None,
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltystat_0",
                    description: None,
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltystat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyintc_0",
                    description: None,
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyintc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyicgs_0",
                    description: None,
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyicgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltysfcfg_0",
                    description: None,
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltysfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyidata_0",
                    description: None,
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyrdata_0",
                    description: None,
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyrdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmht_0",
                    description: None,
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmlt_0",
                    description: None,
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmstat_0",
                    description: None,
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmfc_0",
                    description: None,
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmax_0",
                    description: None,
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmin_0",
                    description: None,
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyct_0",
                    description: None,
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl0_1",
                    description: None,
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl1_1",
                    description: None,
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltystat_1",
                    description: None,
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltystat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyintc_1",
                    description: None,
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyintc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyicgs_1",
                    description: None,
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyicgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltysfcfg_1",
                    description: None,
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltysfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyidata_1",
                    description: None,
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyrdata_1",
                    description: None,
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyrdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmht_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmlt_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmstat_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmfc_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmax_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmin_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyct_1",
                    description: None,
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl0_2",
                    description: None,
                    array: None,
                    byte_offset: 0x200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl1_2",
                    description: None,
                    array: None,
                    byte_offset: 0x204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltystat_2",
                    description: None,
                    array: None,
                    byte_offset: 0x208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltystat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyintc_2",
                    description: None,
                    array: None,
                    byte_offset: 0x20c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyintc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyicgs_2",
                    description: None,
                    array: None,
                    byte_offset: 0x210,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyicgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltysfcfg_2",
                    description: None,
                    array: None,
                    byte_offset: 0x214,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltysfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyidata_2",
                    description: None,
                    array: None,
                    byte_offset: 0x218,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyrdata_2",
                    description: None,
                    array: None,
                    byte_offset: 0x21c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyrdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmht_2",
                    description: None,
                    array: None,
                    byte_offset: 0x220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmlt_2",
                    description: None,
                    array: None,
                    byte_offset: 0x224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmstat_2",
                    description: None,
                    array: None,
                    byte_offset: 0x228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmfc_2",
                    description: None,
                    array: None,
                    byte_offset: 0x22c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmax_2",
                    description: None,
                    array: None,
                    byte_offset: 0x230,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmin_2",
                    description: None,
                    array: None,
                    byte_offset: 0x234,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyct_2",
                    description: None,
                    array: None,
                    byte_offset: 0x238,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyct",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl0_3",
                    description: None,
                    array: None,
                    byte_offset: 0x280,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyctl1_3",
                    description: None,
                    array: None,
                    byte_offset: 0x284,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyctl1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltystat_3",
                    description: None,
                    array: None,
                    byte_offset: 0x288,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltystat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyintc_3",
                    description: None,
                    array: None,
                    byte_offset: 0x28c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyintc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyicgs_3",
                    description: None,
                    array: None,
                    byte_offset: 0x290,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyicgs",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltysfcfg_3",
                    description: None,
                    array: None,
                    byte_offset: 0x294,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltysfcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyidata_3",
                    description: None,
                    array: None,
                    byte_offset: 0x298,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyidata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyrdata_3",
                    description: None,
                    array: None,
                    byte_offset: 0x29c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyrdata",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmht_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmht",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmlt_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmlt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmstat_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmstat",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltytmfc_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltytmfc",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmax_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmax",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyemmin_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyemmin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hpdf_fltyct_3",
                    description: None,
                    array: None,
                    byte_offset: 0x2b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "HpdfFltyct",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "HpdfChxcfg0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxcfg0_dtrs",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_chxcfg0_caloff",
                    description: None,
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
            name: "HpdfChxcfg1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxcfg1_mmct",
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
                    name: "hpdf_chxcfg1_mmbsd",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_chxcfg1_tmfor",
                    description: None,
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
                    name: "hpdf_chxcfg1_tmsfo",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfChxctl",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxctl_sityp",
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
                    name: "hpdf_chxctl_spickss",
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
                    name: "hpdf_chxctl_mmen",
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
                    name: "hpdf_chxctl_cklen",
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
                    name: "hpdf_chxctl_chen",
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
                Field {
                    name: "hpdf_chxctl_chpinsel",
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
                    name: "hpdf_chxctl_cmsd",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_chxctl_dpm",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfChxpdi",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxpdi_datain0",
                    description: None,
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
                    name: "hpdf_chxpdi_datain1",
                    description: None,
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
            name: "HpdfChxps",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxps_plsk",
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
            ],
        },
        FieldSet {
            name: "HpdfChxtmfdt",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_chxtmfdt_tmdata",
                    description: None,
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
            name: "HpdfFltyct",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyct_ctcnt",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 28,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfFltyctl0",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyctl0_flten",
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
                    name: "hpdf_fltyctl0_sicc",
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
                    name: "hpdf_fltyctl0_icsyn",
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
                    name: "hpdf_fltyctl0_scmod",
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
                    name: "hpdf_fltyctl0_icdmaen",
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
                    name: "hpdf_fltyctl0_ictssel",
                    description: None,
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
                    name: "hpdf_fltyctl0_icteen",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_fltyctl0_srcs",
                    description: None,
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
                    name: "hpdf_fltyctl0_rccm",
                    description: None,
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
                    name: "hpdf_fltyctl0_rcsyn",
                    description: None,
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
                    name: "hpdf_fltyctl0_rcdmaen",
                    description: None,
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
                    name: "hpdf_fltyctl0_rcs",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_fltyctl0_fast",
                    description: None,
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
                    name: "hpdf_fltyctl0_tmfm",
                    description: None,
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
            name: "HpdfFltyctl1",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyctl1_iceie",
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
                    name: "hpdf_fltyctl1_rceie",
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
                    name: "hpdf_fltyctl1_icdoie",
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
                    name: "hpdf_fltyctl1_rcdoie",
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
                    name: "hpdf_fltyctl1_tmie",
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
                    name: "hpdf_fltyctl1_emcs",
                    description: None,
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
                    name: "hpdf_fltyctl1_tmchen",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfFltyemmax",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyemmax_maxdc",
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
                    name: "hpdf_fltyemmax_maxval",
                    description: None,
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
            name: "HpdfFltyemmin",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyemmin_mindc",
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
                    name: "hpdf_fltyemmin_minval",
                    description: None,
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
            name: "HpdfFltyicgs",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyicgs_icgsel",
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
            name: "HpdfFltyidata",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyidata_icch",
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
                    name: "hpdf_fltyidata_idtat",
                    description: None,
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
            name: "HpdfFltyintc",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyintc_icdofc",
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
                    name: "hpdf_fltyintc_rcdofc",
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
            name: "HpdfFltyrdata",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltyrdata_rcch",
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
                    name: "hpdf_fltyrdata_rchpdt",
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
                    name: "hpdf_fltyrdata_rdata",
                    description: None,
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
            name: "HpdfFltysfcfg",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltysfcfg_ior",
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
                    name: "hpdf_fltysfcfg_sfor",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hpdf_fltysfcfg_sfo",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfFltystat",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltystat_icef",
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
                    name: "hpdf_fltystat_rcef",
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
                    name: "hpdf_fltystat_icdof",
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
                    name: "hpdf_fltystat_rcdof",
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
                    name: "hpdf_fltystat_tmeof",
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
                    name: "hpdf_fltystat_icpf",
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
                    name: "hpdf_fltystat_rcpf",
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
            name: "HpdfFltytmfc",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltytmfc_ltfc",
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
                    name: "hpdf_fltytmfc_htfc",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "HpdfFltytmht",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltytmht_htbsd",
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
                    name: "hpdf_fltytmht_htval",
                    description: None,
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
            name: "HpdfFltytmlt",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltytmlt_ltbsd",
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
                    name: "hpdf_fltytmlt_ltval",
                    description: None,
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
            name: "HpdfFltytmstat",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpdf_fltytmstat_ltf",
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
                    name: "hpdf_fltytmstat_htf",
                    description: None,
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
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
                