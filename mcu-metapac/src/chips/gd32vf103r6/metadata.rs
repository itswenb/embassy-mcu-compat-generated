include!("../metadata_0043.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32VF103R6",
                family: "GD32",
                line: "GD32VF103",
                memory: &[&[
    MemoryRegion {
        name: "FLASH_0",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 32768,
        settings: None,
    },
    MemoryRegion {
        name: "ram",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 10240,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };