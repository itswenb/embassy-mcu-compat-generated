include!("../compat_metadata_0110.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "GD32H737ZM",
    family: "GD32",
    line: "GD32H73x_75x",
    memory: &[&[
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 3932160,
            settings: Some(FlashSettings {
                erase_size: 4096,
                write_size: 4,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 131072,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM3",
            kind: MemoryRegionKind::Ram,
            address: 0x24000000,
            size: 851968,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM4",
            kind: MemoryRegionKind::Ram,
            address: 0x30000000,
            size: 16384,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM5",
            kind: MemoryRegionKind::Ram,
            address: 0x30004000,
            size: 16384,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};
