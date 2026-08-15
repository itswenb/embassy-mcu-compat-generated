
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cau",
            extends: None,
            description: Some(
                "Cryptographic Acceleration Unit",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "Control register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stat0",
                    description: Some(
                        "Status register 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "di",
                    description: Some(
                        "Data input register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Di",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "do_",
                    description: Some(
                        "Data output register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Do",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dmaen",
                    description: Some(
                        "DMA enable register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dmaen",
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
                    byte_offset: 0x14,
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
                    name: "stat1",
                    description: Some(
                        "Status register 1",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Stat1",
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
                    byte_offset: 0x1c,
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
                    name: "key0h",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key0h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key0l",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key0l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key1h",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key1h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key1l",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key1l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key2h",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key2h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key2l",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key2l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key3h",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key3h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "key3l",
                    description: Some(
                        "CAU key register",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Key3l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iv0h",
                    description: Some(
                        "CAU initialization register",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iv0h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iv0l",
                    description: Some(
                        "CAU initialization register",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iv0l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iv1h",
                    description: Some(
                        "CAU initialization register",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iv1h",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iv1l",
                    description: Some(
                        "CAU initialization register",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Iv1l",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs0",
                    description: Some(
                        "GCM or CCM mode context switch register 0",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs1",
                    description: Some(
                        "GCM or CCM mode context switch register 1",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs2",
                    description: Some(
                        "GCM or CCM mode context switch register 2",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs3",
                    description: Some(
                        "GCM or CCM mode context switch register 3",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs4",
                    description: Some(
                        "GCM or CCM mode context switch register 4",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs5",
                    description: Some(
                        "GCM or CCM mode context switch register 5",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs6",
                    description: Some(
                        "GCM or CCM mode context switch register 6",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmccmctxs7",
                    description: Some(
                        "GCM or CCM mode context switch register 7",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmccmctxs7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs0",
                    description: Some(
                        "GCM mode context switch register 0",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs1",
                    description: Some(
                        "GCM mode context switch register 1",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs2",
                    description: Some(
                        "GCM mode context switch register 2",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs3",
                    description: Some(
                        "GCM mode context switch register 3",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs4",
                    description: Some(
                        "GCM mode context switch register 4",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs5",
                    description: Some(
                        "GCM mode context switch register 5",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs6",
                    description: Some(
                        "GCM mode context switch register 6",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcmctxs7",
                    description: Some(
                        "GCM mode context switch register 7",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcmctxs7",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl",
            extends: None,
            description: Some(
                "Control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "caudir",
                    description: Some(
                        "CAU direction",
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
                    name: "algm",
                    description: Some(
                        "Encryption/decryption algorithm mode bit 0 to bit 2",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "datam",
                    description: Some(
                        "Data swapping type mode configuration",
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
                    name: "keym",
                    description: Some(
                        "AES key size mode configuration",
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
                    name: "fflush",
                    description: Some(
                        "Flush FIFO",
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
                    name: "cauen",
                    description: Some(
                        "CAU Enable",
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
                    name: "gcm_ccmph",
                    description: Some(
                        "GCM CCM phase",
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
                    name: "algm_3",
                    description: Some(
                        "Encryption/decryption algorithm mode bit 3",
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
                    name: "nbpilb",
                    description: Some(
                        "Number of bytes padding in last block of payload",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Di",
            extends: None,
            description: Some(
                "Data input register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "di",
                    description: Some(
                        "Data input",
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
            name: "Dmaen",
            extends: None,
            description: Some(
                "DMA enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaien",
                    description: Some(
                        "DMA input enable",
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
                    name: "dmaoen",
                    description: Some(
                        "DMA output enable",
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
            name: "Do",
            extends: None,
            description: Some(
                "Data output register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "do_",
                    description: Some(
                        "Data output",
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
            name: "Gcmccmctxs0",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx0",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs1",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx1",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs2",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx2",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs3",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx3",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs4",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx4",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs5",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx5",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs6",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx6",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmccmctxs7",
            extends: None,
            description: Some(
                "GCM or CCM mode context switch register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx7",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs0",
            extends: None,
            description: Some(
                "GCM mode context switch register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx0",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs1",
            extends: None,
            description: Some(
                "GCM mode context switch register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx1",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs2",
            extends: None,
            description: Some(
                "GCM mode context switch register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx2",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs3",
            extends: None,
            description: Some(
                "GCM mode context switch register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx3",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs4",
            extends: None,
            description: Some(
                "GCM mode context switch register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx4",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs5",
            extends: None,
            description: Some(
                "GCM mode context switch register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx5",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs6",
            extends: None,
            description: Some(
                "GCM mode context switch register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx6",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Gcmctxs7",
            extends: None,
            description: Some(
                "GCM mode context switch register 7",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ctx7",
                    description: Some(
                        "The internal status of the CAU core",
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
            name: "Inten",
            extends: None,
            description: Some(
                "Interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iinten",
                    description: Some(
                        "IN FIFO interrupt enable",
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
                    name: "ointen",
                    description: Some(
                        "OUT FIFO interrupt enable",
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
            name: "Intf",
            extends: None,
            description: Some(
                "Interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iintf",
                    description: Some(
                        "IN FIFO enabled interrupt flag",
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
                    name: "ointf",
                    description: Some(
                        "OUT FIFO enabled interrupt flag",
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
            name: "Iv0h",
            extends: None,
            description: Some(
                "CAU initialization register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iv0h",
                    description: Some(
                        "The initialization vector for DES,TDES,AES",
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
            name: "Iv0l",
            extends: None,
            description: Some(
                "CAU initialization register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iv0l",
                    description: Some(
                        "The initialization vector for DES,TDES,AES",
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
            name: "Iv1h",
            extends: None,
            description: Some(
                "CAU initialization register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iv1h",
                    description: Some(
                        "The initialization vector for DES,TDES,AES",
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
            name: "Iv1l",
            extends: None,
            description: Some(
                "CAU initialization register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iv1l",
                    description: Some(
                        "The initialization vector for DES,TDES,AES",
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
            name: "Key0h",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0h",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key0l",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key0l",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key1h",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key1h",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key1l",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key1l",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key2h",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key2h",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key2l",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key2l",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key3h",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key3h",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Key3l",
            extends: None,
            description: Some(
                "CAU key register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "key3l",
                    description: Some(
                        "Key for DES,TDES,AES",
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
            name: "Stat0",
            extends: None,
            description: Some(
                "Status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iem",
                    description: Some(
                        "Input FIFO is empty",
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
                    name: "inf",
                    description: Some(
                        "Input FIFO is not full",
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
                    name: "one",
                    description: Some(
                        "Output FIFO is not empty",
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
                    name: "ofu",
                    description: Some(
                        "Output FIFO is full",
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
                    name: "busy",
                    description: Some(
                        "Busy bit",
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
            ],
        },
        FieldSet {
            name: "Stat1",
            extends: None,
            description: Some(
                "Status register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ista",
                    description: Some(
                        "IN FIFO interrupt status",
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
                    name: "osta",
                    description: Some(
                        "OUT FIFO interrupt status",
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
    ],
    enums: &[],
};
                