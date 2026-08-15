include!("../metadata_0045.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "GD32W515P0",
                family: "GD32",
                line: "GD32W51x",
                memory: &[],
                peripherals: PERIPHERALS,
                nvic_priority_bits: None,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
                pins: PINS,
            };