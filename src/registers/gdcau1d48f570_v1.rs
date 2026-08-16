
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cau",
            extends: None,
            description: Some(
                "Cryptographic acceleration unit",
            ),
            items: &[
                BlockItem {
                    name: "ctl",
                    description: Some(
                        "CAU control register",
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
                        "CAU status register 0",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "CAU data input register",
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
                        "CAU data output register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "CAU DMA enable register",
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
                        "CAU interrupt enable register",
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
                        "CAU interrupt status flag register 1",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "CAU enable interrupt status flag register",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctl",
            extends: None,
            description: Some(
                "CAU control register",
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
                        "Encryption/decryption algorithm mode",
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
                        "FIFO flush",
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
                        "Cryptographic module enable",
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
            name: "Di",
            extends: None,
            description: Some(
                "CAU data input register",
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
                "CAU DMA enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmaien",
                    description: Some(
                        "In FIFO DMA enable",
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
                        "Out FIFO DMA enable",
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
                "CAU data output register",
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
            name: "Inten",
            extends: None,
            description: Some(
                "CAU interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iinten",
                    description: Some(
                        "In FIFO interrupt enable",
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
                        "Out FIFO interrupt enable",
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
                "CAU enable interrupt status flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iintf",
                    description: Some(
                        "In FIFO enabled interrupt flag",
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
                        "Out FIFO enabled interrupt flag",
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
                "CAU status register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iem",
                    description: Some(
                        "IN FIFO empty flag",
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
                        "IN FIFO not full flag",
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
                        "OUT FIFO not empty flag",
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
                        "OUT FIFO full flag",
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
                        "BUSY flag",
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
                "CAU interrupt status flag register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ista",
                    description: Some(
                        "In FIFO interrupt flag",
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
                        "Out FIFO interrupt flag",
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
                