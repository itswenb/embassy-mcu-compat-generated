include!("../metadata_0035.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32H757ZG",
                family: "GD32",
                line: "GD32H73x_75x",
                memory: &[&[
    MemoryRegion {
        name: "ITCMRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x0,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "IROM1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 1048576,
        settings: None,
    },
    MemoryRegion {
        name: "DTCMRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 131072,
        settings: None,
    },
    MemoryRegion {
        name: "AXISRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x24000000,
        size: 851968,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM0",
        kind: MemoryRegionKind::Ram,
        address: 0x30000000,
        size: 16384,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x30004000,
        size: 16384,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };