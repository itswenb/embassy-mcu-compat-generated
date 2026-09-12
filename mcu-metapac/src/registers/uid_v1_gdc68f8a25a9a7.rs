
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "GdUidFe175d2bf68b",
        extends: None,
        description: Some("GD32 工厂写入的唯一设备标识"),
        items: &[BlockItem {
            name: "uid",
            description: None,
            array: Some(Array::Regular(RegularArray { len: 3, stride: 4 })),
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::Read,
                bit_size: 32,
                fieldset: None,
            }),
        }],
    }],
    fieldsets: &[],
    enums: &[],
};
