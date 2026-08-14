include!("../metadata_0021.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32F101ZD",
                family: "GD32",
                line: "GD32F10x",
                memory: &[&[
    MemoryRegion {
        name: "IROM1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 393216,
        settings: None,
    },
    MemoryRegion {
        name: "IRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 49152,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };