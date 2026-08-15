include!("../metadata_0021.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32F107ZF",
                family: "GD32",
                line: "GD32F10x",
                memory: &[&[
    MemoryRegion {
        name: "IROM1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 786432,
        settings: None,
    },
    MemoryRegion {
        name: "IRAM1",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 98304,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };