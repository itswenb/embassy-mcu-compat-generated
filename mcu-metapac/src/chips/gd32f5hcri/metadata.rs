include!("../metadata_0038.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32F5HCRI",
                family: "GD32",
                line: "GD32W51x_F5HC",
                memory: &[&[
    MemoryRegion {
        name: "IROM1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 2097152,
        settings: None,
    },
    MemoryRegion {
        name: "IROM2",
        kind: MemoryRegionKind::Flash,
        address: 0xc000000,
        size: 2097152,
        settings: None,
    },
    MemoryRegion {
        name: "IRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 327680,
        settings: None,
    },
    MemoryRegion {
        name: "IRAM2",
        kind: MemoryRegionKind::Ram,
        address: 0x30000000,
        size: 327680,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };