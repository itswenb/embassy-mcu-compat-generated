include!("../metadata_0047.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32M531RC",
                family: "GD32",
                line: "GD32M53x",
                memory: &[&[
    MemoryRegion {
        name: "IROM1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 262144,
        settings: None,
    },
    MemoryRegion {
        name: "IROM2",
        kind: MemoryRegionKind::Flash,
        address: 0x8800000,
        size: 65536,
        settings: None,
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x20000000,
        size: 32768,
        settings: None,
    },
]],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };