include!("../metadata_0044.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32VW553KI",
                family: "GD32",
                line: "GD32VW55x",
                memory: &[&[
    MemoryRegion {
        name: "FLASH_0",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 2097152,
        settings: None,
    },
    MemoryRegion {
        name: "ram",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 294912,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };