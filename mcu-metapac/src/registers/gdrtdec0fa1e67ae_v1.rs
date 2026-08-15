
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtdec0",
            extends: None,
            description: Some(
                "Real-time decryption",
            ),
            items: &[
                BlockItem {
                    name: "are0_cfg",
                    description: Some(
                        "Area 0 configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_saddr",
                    description: Some(
                        "Area 0 start address regist",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_eaddr",
                    description: Some(
                        "Area 0 end address regist",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Eaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_nonce0",
                    description: Some(
                        "Area 0 nonce 0 register 0",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Nonce0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_nonce1",
                    description: Some(
                        "Area 0 nonce register 1",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Nonce1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_key0",
                    description: Some(
                        "Area 0 key register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_key1",
                    description: Some(
                        "Area 0 key register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Key1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_key2",
                    description: Some(
                        "Area 0 key register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Key2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are0_key3",
                    description: Some(
                        "Area 0 key register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are0Key3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_cfg",
                    description: Some(
                        "Area 1 configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_saddr",
                    description: Some(
                        "Area 1 start address regist",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_eaddr",
                    description: Some(
                        "Area 1 end address regist",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Eaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_nonce0",
                    description: Some(
                        "Area 1 nonce register 0",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Nonce0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_nonce1",
                    description: Some(
                        "Area 1 nonce register 1",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Nonce1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_key0",
                    description: Some(
                        "Area 1 key register",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_key1",
                    description: Some(
                        "Area 1 key register",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Key1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_key2",
                    description: Some(
                        "Area 1 key register",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Key2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are1_key3",
                    description: Some(
                        "Area 1 key register",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are1Key3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_cfg",
                    description: Some(
                        "Area 2 configuration regist",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_saddr",
                    description: Some(
                        "Area 2 start address regist",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_eaddr",
                    description: Some(
                        "Area 2 end address regist",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Eaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_nonce0",
                    description: Some(
                        "Area 2 nonce register 0",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Nonce0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_nonce1",
                    description: Some(
                        "Area 2 nonce register 1",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Nonce1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_key0",
                    description: Some(
                        "Area 2 key register",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_key1",
                    description: Some(
                        "Area 2 key register",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Key1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_key2",
                    description: Some(
                        "Area 2 key register",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Key2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are2_key3",
                    description: Some(
                        "Area 2 key register",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are2Key3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_cfg",
                    description: Some(
                        "Area 3 configuration regist",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Cfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_saddr",
                    description: Some(
                        "Area 3 start address regist",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Saddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_eaddr",
                    description: Some(
                        "Area 3 end address regist",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Eaddr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_nonce0",
                    description: Some(
                        "Area 3 nonce register 0",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Nonce0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_nonce1",
                    description: Some(
                        "Area 3 nonce register",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Nonce1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_key0",
                    description: Some(
                        "Area 3 key register",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Key0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_key1",
                    description: Some(
                        "Area 3 key register",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Key1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_key2",
                    description: Some(
                        "Area 3 key register",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Key2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "are3_key3",
                    description: Some(
                        "Area 3 key register",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Are3Key3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intf",
                    description: Some(
                        "Interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "intc",
                    description: Some(
                        "Interrupt flag clear register",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Intc",
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
                    byte_offset: 0x308,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Are0Cfg",
            extends: None,
            description: Some(
                "Area 0 configuration regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_en",
                    description: Some(
                        "Area real-time decryption enable bit",
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
                    name: "are_cfg_lk",
                    description: Some(
                        "Area configure lock bit",
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
                    name: "are_k_lk",
                    description: Some(
                        "Area key lock bit",
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
                    name: "mode",
                    description: Some(
                        "RTDEC mode bits",
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
                    name: "are_k_crc",
                    description: Some(
                        "8-bit CRC of area key bits",
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
                    name: "are_fmver",
                    description: Some(
                        "Area firmware version bits",
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
            name: "Are0Eaddr",
            extends: None,
            description: Some(
                "Area 0 end address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_eaddr",
                    description: Some(
                        "Area x end address bits",
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
            name: "Are0Key0",
            extends: None,
            description: Some(
                "Area 0 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key0",
                    description: Some(
                        "Area x key bits, ARE_KEY[31:0]",
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
            name: "Are0Key1",
            extends: None,
            description: Some(
                "Area 0 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key1",
                    description: Some(
                        "Area x key bits, ARE_KEY[63:32]",
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
            name: "Are0Key2",
            extends: None,
            description: Some(
                "Area 0 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key2",
                    description: Some(
                        "Area x key bits, ARE_KEY[95:64]",
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
            name: "Are0Key3",
            extends: None,
            description: Some(
                "Area 0 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key3",
                    description: Some(
                        "Area x key bits, ARE_KEY[127:96]",
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
            name: "Are0Nonce0",
            extends: None,
            description: Some(
                "Area 0 nonce 0 register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce0",
                    description: Some(
                        "Area 0 nonce address bits",
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
            name: "Are0Nonce1",
            extends: None,
            description: Some(
                "Area 0 nonce register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce1",
                    description: Some(
                        "Area 0 nonce 1 address bits",
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
            name: "Are0Saddr",
            extends: None,
            description: Some(
                "Area 0 start address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_saddr",
                    description: Some(
                        "Area x start address bits",
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
            name: "Are1Cfg",
            extends: None,
            description: Some(
                "Area 1 configuration regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_en",
                    description: Some(
                        "Area real-time decryption enable bit",
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
                    name: "are_cfg_lk",
                    description: Some(
                        "Area configure lock bit",
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
                    name: "are_k_lk",
                    description: Some(
                        "Area key lock bit",
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
                    name: "mode",
                    description: Some(
                        "RTDEC mode bits",
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
                    name: "are_k_crc",
                    description: Some(
                        "8-bit CRC of area key bits",
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
                    name: "are_fmver",
                    description: Some(
                        "Area firmware version bits",
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
            name: "Are1Eaddr",
            extends: None,
            description: Some(
                "Area 1 end address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_eaddr",
                    description: Some(
                        "Area x end address bits",
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
            name: "Are1Key0",
            extends: None,
            description: Some(
                "Area 1 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key0",
                    description: Some(
                        "Area x key bits, ARE_KEY[31:0]",
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
            name: "Are1Key1",
            extends: None,
            description: Some(
                "Area 1 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key1",
                    description: Some(
                        "Area x key bits, ARE_KEY[63:32]",
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
            name: "Are1Key2",
            extends: None,
            description: Some(
                "Area 1 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key2",
                    description: Some(
                        "Area x key bits, ARE_KEY[95:64]",
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
            name: "Are1Key3",
            extends: None,
            description: Some(
                "Area 1 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key3",
                    description: Some(
                        "Area x key bits, ARE_KEY[127:96]",
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
            name: "Are1Nonce0",
            extends: None,
            description: Some(
                "Area 1 nonce register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce0",
                    description: Some(
                        "Area 1 nonce 0 address bits",
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
            name: "Are1Nonce1",
            extends: None,
            description: Some(
                "Area 1 nonce register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce1",
                    description: Some(
                        "Area 1 nonce 1 address bits, ARE_RANDNUM[63:32]",
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
            name: "Are1Saddr",
            extends: None,
            description: Some(
                "Area 1 start address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_saddr",
                    description: Some(
                        "Area x start address bits",
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
            name: "Are2Cfg",
            extends: None,
            description: Some(
                "Area 2 configuration regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_en",
                    description: Some(
                        "Area real-time decryption enable bit",
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
                    name: "are_cfg_lk",
                    description: Some(
                        "Area configure lock bit",
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
                    name: "are_k_lk",
                    description: Some(
                        "Area key lock bit",
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
                    name: "mode",
                    description: Some(
                        "RTDEC mode bits",
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
                    name: "are_k_crc",
                    description: Some(
                        "8-bit CRC of area key bits",
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
                    name: "are_fmver",
                    description: Some(
                        "Area firmware version bits",
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
            name: "Are2Eaddr",
            extends: None,
            description: Some(
                "Area 2 end address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_eaddr",
                    description: Some(
                        "Area x end address bits",
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
            name: "Are2Key0",
            extends: None,
            description: Some(
                "Area 2 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key0",
                    description: Some(
                        "Area x key bits, ARE_KEY[31:0]",
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
            name: "Are2Key1",
            extends: None,
            description: Some(
                "Area 2 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key1",
                    description: Some(
                        "Area x key bits, ARE_KEY[63:32]",
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
            name: "Are2Key2",
            extends: None,
            description: Some(
                "Area 2 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key2",
                    description: Some(
                        "Area x key bits, ARE_KEY[95:64]",
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
            name: "Are2Key3",
            extends: None,
            description: Some(
                "Area 2 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key3",
                    description: Some(
                        "Area x key bits, ARE_KEY[127:96]",
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
            name: "Are2Nonce0",
            extends: None,
            description: Some(
                "Area 2 nonce register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce0",
                    description: Some(
                        "Area 2 nonce 0 address bits",
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
            name: "Are2Nonce1",
            extends: None,
            description: Some(
                "Area 2 nonce register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce1",
                    description: Some(
                        "Area 2 nonce 1 address bits, ARE_RANDNUM[63:32]",
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
            name: "Are2Saddr",
            extends: None,
            description: Some(
                "Area 2 start address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_saddr",
                    description: Some(
                        "Area x start address bits",
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
            name: "Are3Cfg",
            extends: None,
            description: Some(
                "Area 3 configuration regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_en",
                    description: Some(
                        "Area real-time decryption enable bit",
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
                    name: "are_cfg_lk",
                    description: Some(
                        "Area configure lock bit",
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
                    name: "are_k_lk",
                    description: Some(
                        "Area key lock bit",
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
                    name: "mode",
                    description: Some(
                        "RTDEC mode bits",
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
                    name: "are_k_crc",
                    description: Some(
                        "8-bit CRC of area key bits",
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
                    name: "are_fmver",
                    description: Some(
                        "Area firmware version bits",
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
            name: "Are3Eaddr",
            extends: None,
            description: Some(
                "Area 3 end address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_eaddr",
                    description: Some(
                        "Area x end address bits",
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
            name: "Are3Key0",
            extends: None,
            description: Some(
                "Area 3 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key0",
                    description: Some(
                        "Area x key bits, ARE_KEY[31:0]",
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
            name: "Are3Key1",
            extends: None,
            description: Some(
                "Area 3 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key1",
                    description: Some(
                        "Area x key bits, ARE_KEY[63:32]",
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
            name: "Are3Key2",
            extends: None,
            description: Some(
                "Area 3 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key2",
                    description: Some(
                        "Area x key bits, ARE_KEY[95:64]",
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
            name: "Are3Key3",
            extends: None,
            description: Some(
                "Area 3 key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_key3",
                    description: Some(
                        "Area x key bits, ARE_KEY[127:96]",
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
            name: "Are3Nonce0",
            extends: None,
            description: Some(
                "Area 3 nonce register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce0",
                    description: Some(
                        "Area 3 nonce 0 address bits",
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
            name: "Are3Nonce1",
            extends: None,
            description: Some(
                "Area 3 nonce register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_nonce1",
                    description: Some(
                        "Area 3 nonce 1 address bits, ARE_RANDNUM[63:32]",
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
            name: "Are3Saddr",
            extends: None,
            description: Some(
                "Area 3 start address regist",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "are_saddr",
                    description: Some(
                        "Area x start address bits",
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
            name: "Intc",
            extends: None,
            description: Some(
                "Interrupt flag clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secec",
                    description: Some(
                        "Security error interrupt flag",
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
                    name: "econec",
                    description: Some(
                        "Execute-only or execute-never error flag clear",
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
                    name: "kec",
                    description: Some(
                        "Key error flag clear",
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
                    name: "seceie",
                    description: Some(
                        "Enable bit for security error interrupt",
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
                    name: "econeie",
                    description: Some(
                        "Enable bit for execute-only or execute-never error interrupt",
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
                    name: "keie",
                    description: Some(
                        "Enable bit for key error interrupt",
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
            ],
        },
        FieldSet {
            name: "Intf",
            extends: None,
            description: Some(
                "Interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secef",
                    description: Some(
                        "Security error interrupt flag",
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
                    name: "econef",
                    description: Some(
                        "Execute-only or execute-never error interrupt flag",
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
                    name: "kef",
                    description: Some(
                        "Key error interrupt flag",
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
            ],
        },
    ],
    enums: &[],
};
                