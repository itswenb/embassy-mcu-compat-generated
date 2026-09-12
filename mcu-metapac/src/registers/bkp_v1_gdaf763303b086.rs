
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Bkp",
        extends: None,
        description: None,
        items: &[
            BlockItem {
                name: "rtccr",
                description: Some("RTC clock calibration register"),
                array: None,
                byte_offset: 0x2c,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Rtccr"),
                }),
            },
            BlockItem {
                name: "cr",
                description: Some("Control register"),
                array: None,
                byte_offset: 0x30,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr"),
                }),
            },
            BlockItem {
                name: "csr",
                description: Some("Control/status register"),
                array: None,
                byte_offset: 0x34,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some("Control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tpe",
                    description: Some("Tamper pin enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpal",
                    description: Some("Tamper pin active level"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Tpal"),
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some("Control/status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cte",
                    description: Some("Clear Tamper event"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cti",
                    description: Some("Clear Tamper Interrupt"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 1 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tpie",
                    description: Some("Tamper Pin interrupt enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 2 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tef",
                    description: Some("Tamper Event Flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tif",
                    description: Some("Tamper Interrupt Flag"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rtccr",
            extends: None,
            description: Some("RTC clock calibration register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cal",
                    description: Some("Calibration value"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cco",
                    description: Some("Calibration Clock Output"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asoe",
                    description: Some("Alarm or second output enable"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "asos",
                    description: Some("Alarm or second output selection"),
                    bit_offset: BitOffset::Regular(RegularBitOffset { offset: 9 }),
                    bit_size: 1,
                    array: None,
                    enumm: Some("Asos"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Asos",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Alarm",
                    description: Some("RTC Alarm pulse output selected"),
                    value: 0,
                },
                EnumVariant {
                    name: "Second",
                    description: Some("RTC Second pulse output selected"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Tpal",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "High",
                    description: Some(
                        "A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Low",
                    description: Some(
                        "A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
