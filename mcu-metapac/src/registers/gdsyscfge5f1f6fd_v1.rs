
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "System and memory architectur",
            ),
            items: &[
                BlockItem {
                    name: "pmcfg",
                    description: Some(
                        "Peripheral mode configuration register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pmcfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss0",
                    description: Some(
                        "EXTI sources selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss1",
                    description: Some(
                        "EXTI sources selection register 1",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss2",
                    description: Some(
                        "EXTI sources selection register 2",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "extiss3",
                    description: Some(
                        "EXTI sources selection register 3",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Extiss3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lkctl",
                    description: Some(
                        "Lockup control register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lkctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpsctl",
                    description: Some(
                        "I/O compensation control register",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpsctl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpscccfg",
                    description: Some(
                        "I/O compensation cell code configuration register",
                    ),
                    array: None,
                    byte_offset: 0x28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpscccfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel0",
                    description: Some(
                        "Timer input selection register 0",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel1",
                    description: Some(
                        "Timer input selection register 1",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel2",
                    description: Some(
                        "Timer input selection register 2",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel3",
                    description: Some(
                        "Timer input selection register 3",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel4",
                    description: Some(
                        "Timer input selection register 4",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel5",
                    description: Some(
                        "Timer input selection register 5",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timercisel6",
                    description: Some(
                        "Timer input selection register 6",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timercisel6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpuicac",
                    description: Some(
                        "CPU ICACHE error status registe",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpuicac",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpudcac",
                    description: Some(
                        "CPU DCACHE error status register",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpudcac",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fpuinten",
                    description: Some(
                        "FPU interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Fpuinten",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramcfg0",
                    description: Some(
                        "SYSCFG SRAM configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sramcfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sramcfg1",
                    description: Some(
                        "SYSCFG SRAM configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sramcfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg0",
                    description: Some(
                        "TIMER0 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg1",
                    description: Some(
                        "TIMER0 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer0cfg2",
                    description: Some(
                        "TIMER0 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x108,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer0cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1cfg0",
                    description: Some(
                        "TIMER1 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1cfg1",
                    description: Some(
                        "TIMER1 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer1cfg2",
                    description: Some(
                        "TIMER1 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x114,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer1cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2cfg0",
                    description: Some(
                        "TIMER2 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2cfg1",
                    description: Some(
                        "TIMER2 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x11c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer2cfg2",
                    description: Some(
                        "TIMER2 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer2cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3cfg0",
                    description: Some(
                        "TIMER3 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x124,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3cfg1",
                    description: Some(
                        "TIMER3 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer3cfg2",
                    description: Some(
                        "TIMER3 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x12c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer3cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4cfg0",
                    description: Some(
                        "TIMER4 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x130,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4cfg1",
                    description: Some(
                        "TIMER4 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x134,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer4cfg2",
                    description: Some(
                        "TIMER4 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer4cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7cfg0",
                    description: Some(
                        "TIMER7 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7cfg1",
                    description: Some(
                        "TIMER7 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer7cfg2",
                    description: Some(
                        "TIMER7 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer7cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg0",
                    description: Some(
                        "TIMER14 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg1",
                    description: Some(
                        "TIMER14 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer14cfg2",
                    description: Some(
                        "TIMER14 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer14cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer22cfg0",
                    description: Some(
                        "TIMER22 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer22cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer22cfg1",
                    description: Some(
                        "TIMER22 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer22cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer22cfg2",
                    description: Some(
                        "TIMER22 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer22cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer23cfg0",
                    description: Some(
                        "TIMER23 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer23cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer23cfg1",
                    description: Some(
                        "TIMER23 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer23cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer23cfg2",
                    description: Some(
                        "TIMER23 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer23cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer30cfg0",
                    description: Some(
                        "TIMER30 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer30cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer30cfg1",
                    description: Some(
                        "TIMER30 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x170,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer30cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer30cfg2",
                    description: Some(
                        "TIMER30 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x174,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer30cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer31cfg0",
                    description: Some(
                        "TIMER31 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x178,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer31cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer31cfg1",
                    description: Some(
                        "TIMER31 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x17c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer31cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer31cfg2",
                    description: Some(
                        "TIMER31 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer31cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer40cfg0",
                    description: Some(
                        "TIMER40 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer40cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer40cfg1",
                    description: Some(
                        "TIMER40 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer40cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer40cfg2",
                    description: Some(
                        "TIMER40 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x18c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer40cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer41cfg0",
                    description: Some(
                        "TIMER41 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x190,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer41cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer41cfg1",
                    description: Some(
                        "TIMER41 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x194,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer41cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer41cfg2",
                    description: Some(
                        "TIMER41 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x198,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer41cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer42cfg0",
                    description: Some(
                        "TIMER42 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x19c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer42cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer42cfg1",
                    description: Some(
                        "TIMER42 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x1a0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer42cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer42cfg2",
                    description: Some(
                        "TIMER42 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x1a4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer42cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer43cfg0",
                    description: Some(
                        "TIMER43 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x1a8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer43cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer43cfg1",
                    description: Some(
                        "TIMER43 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x1ac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer43cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer43cfg2",
                    description: Some(
                        "TIMER43 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x1b0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer43cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer44cfg0",
                    description: Some(
                        "TIMER44 configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x1b4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer44cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer44cfg1",
                    description: Some(
                        "TIMER44 configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x1b8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer44cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "timer44cfg2",
                    description: Some(
                        "TIMER44 configuration register 2",
                    ),
                    array: None,
                    byte_offset: 0x1bc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Timer44cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usercfg0",
                    description: Some(
                        "User configuration register 0",
                    ),
                    array: None,
                    byte_offset: 0x300,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Usercfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "usercfg1",
                    description: Some(
                        "User configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0x304,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Usercfg1",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cpscccfg",
            extends: None,
            description: Some(
                "I/O compensation cell code configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ncpscc",
                    description: Some(
                        "NMOS compensation cell code",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pcpscc",
                    description: Some(
                        "PMOS compensation cell code",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpsctl",
            extends: None,
            description: Some(
                "I/O compensation control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cps_en",
                    description: Some(
                        "I/O compensation cell enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cps_rdy",
                    description: Some(
                        "Compensation cell ready flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iospdop",
                    description: Some(
                        "I/O speed optimization, High-speed at low-voltage",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iolv",
                    description: Some(
                        "I/O in low voltage state",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpudcac",
            extends: None,
            description: Some(
                "CPU DCACHE error status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpu_dcerr",
                    description: Some(
                        "The DCACHE error bank information",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpu_dcdet",
                    description: Some(
                        "The DCACHE error detection information",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpuicac",
            extends: None,
            description: Some(
                "CPU ICACHE error status registe",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cpu_icerr",
                    description: Some(
                        "The ICACHE error bank information",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 22,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpu_icdet",
                    description: Some(
                        "The ICACHE error detection information",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss0",
            extends: None,
            description: Some(
                "EXTI sources selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti0_ss",
                    description: Some(
                        "EXTI 0 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti1_ss",
                    description: Some(
                        "EXTI 1 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti2_ss",
                    description: Some(
                        "EXTI 2 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti3_ss",
                    description: Some(
                        "EXTI 3 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss1",
            extends: None,
            description: Some(
                "EXTI sources selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti4_ss",
                    description: Some(
                        "EXTI 4 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti5_ss",
                    description: Some(
                        "EXTI 5 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti6_ss",
                    description: Some(
                        "EXTI 6 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti7_ss",
                    description: Some(
                        "EXTI 7 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss2",
            extends: None,
            description: Some(
                "EXTI sources selection register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti8_ss",
                    description: Some(
                        "EXTI 8 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti9_ss",
                    description: Some(
                        "EXTI 9 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti10_ss",
                    description: Some(
                        "EXTI 10 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti11_ss",
                    description: Some(
                        "EXTI 11 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Extiss3",
            extends: None,
            description: Some(
                "EXTI sources selection register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti12_ss",
                    description: Some(
                        "EXTI 12 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti13_ss",
                    description: Some(
                        "EXTI 13 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti14_ss",
                    description: Some(
                        "EXTI 14 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "exti15_ss",
                    description: Some(
                        "EXTI 15 sources selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fpuinten",
            extends: None,
            description: Some(
                "FPU interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iopiz",
                    description: Some(
                        "Invalid operation interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dzie",
                    description: Some(
                        "Divide by 0 interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ufie",
                    description: Some(
                        "Underflow interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ovfie",
                    description: Some(
                        "Overflow interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idie",
                    description: Some(
                        "Input denormal interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ixie",
                    description: Some(
                        "Inexact interrupt enable bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lkctl",
            extends: None,
            description: Some(
                "Lockup control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lvd_lock",
                    description: Some(
                        "Low voltage detector lockup bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cpu_lock",
                    description: Some(
                        "CPU lockup bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpram_lock",
                    description: Some(
                        "Region 2 backup SRAM ECC double error lockup bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1_lock",
                    description: Some(
                        "Region 1 SRAM1 ECC double error lockup bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram0_lock",
                    description: Some(
                        "Region 1 SRAM0 ECC double error lockup bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcm_lock",
                    description: Some(
                        "Region 0 DTCM ECC double error lock bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "itcm_lock",
                    description: Some(
                        "Region 0 ITCM-RAM ECC double error lock bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axiram_lock",
                    description: Some(
                        "Region 0 AXI-SRAM ECC double error lock bit",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pmcfg",
            extends: None,
            description: Some(
                "Peripheral mode configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c0fmpen",
                    description: Some(
                        "I2C0 Fm+",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1fmpen",
                    description: Some(
                        "I2C1 Fm+",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2fmpen",
                    description: Some(
                        "I2C2 Fm+",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3fmpen",
                    description: Some(
                        "I2C3 Fm+",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb6fmpen",
                    description: Some(
                        "PB6 pin Fm+ mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb7fmpen",
                    description: Some(
                        "PB7 pin Fm+ mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb8fmpen",
                    description: Some(
                        "PB8 pin Fm+ mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pb9fmpen",
                    description: Some(
                        "PB9 pin Fm+ mode enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet1_phy_sel",
                    description: Some(
                        "Ethernet1 PHY interface selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "enet0_phy_sel",
                    description: Some(
                        "Ethernet0 PHY interface selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa0swon",
                    description: Some(
                        "PA0 switch open",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa1swon",
                    description: Some(
                        "PA1 switch open",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pc2swon",
                    description: Some(
                        "PC2 switch open",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pc3swon",
                    description: Some(
                        "PC3 switch open",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sramcfg0",
            extends: None,
            description: Some(
                "SYSCFG SRAM configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "secure_sram_size",
                    description: Some(
                        "These bits indicate the size of secure sram",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sramcfg1",
            extends: None,
            description: Some(
                "SYSCFG SRAM configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tcm_waitstate",
                    description: Some(
                        "TCM wait state configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg0",
            extends: None,
            description: Some(
                "TIMER0 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg1",
            extends: None,
            description: Some(
                "TIMER0 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer0cfg2",
            extends: None,
            description: Some(
                "TIMER0 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg0",
            extends: None,
            description: Some(
                "TIMER14 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg1",
            extends: None,
            description: Some(
                "TIMER14 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer14cfg2",
            extends: None,
            description: Some(
                "TIMER14 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer1cfg0",
            extends: None,
            description: Some(
                "TIMER1 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer1cfg1",
            extends: None,
            description: Some(
                "TIMER1 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer1cfg2",
            extends: None,
            description: Some(
                "TIMER1 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer22cfg0",
            extends: None,
            description: Some(
                "TIMER22 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer22cfg1",
            extends: None,
            description: Some(
                "TIMER22 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer22cfg2",
            extends: None,
            description: Some(
                "TIMER22 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer23cfg0",
            extends: None,
            description: Some(
                "TIMER23 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer23cfg1",
            extends: None,
            description: Some(
                "TIMER23 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer23cfg2",
            extends: None,
            description: Some(
                "TIMER23 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer2cfg0",
            extends: None,
            description: Some(
                "TIMER2 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer2cfg1",
            extends: None,
            description: Some(
                "TIMER2 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer2cfg2",
            extends: None,
            description: Some(
                "TIMER2 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer30cfg0",
            extends: None,
            description: Some(
                "TIMER30 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer30cfg1",
            extends: None,
            description: Some(
                "TIMER30 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer30cfg2",
            extends: None,
            description: Some(
                "TIMER30 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer31cfg0",
            extends: None,
            description: Some(
                "TIMER31 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer31cfg1",
            extends: None,
            description: Some(
                "TIMER31 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer31cfg2",
            extends: None,
            description: Some(
                "TIMER31 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer3cfg0",
            extends: None,
            description: Some(
                "TIMER3 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer3cfg1",
            extends: None,
            description: Some(
                "TIMER3 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer3cfg2",
            extends: None,
            description: Some(
                "TIMER3 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer40cfg0",
            extends: None,
            description: Some(
                "TIMER40 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer40cfg1",
            extends: None,
            description: Some(
                "TIMER40 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer40cfg2",
            extends: None,
            description: Some(
                "TIMER40 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer41cfg0",
            extends: None,
            description: Some(
                "TIMER41 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer41cfg1",
            extends: None,
            description: Some(
                "TIMER41 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer41cfg2",
            extends: None,
            description: Some(
                "TIMER41 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer42cfg0",
            extends: None,
            description: Some(
                "TIMER42 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer42cfg1",
            extends: None,
            description: Some(
                "TIMER42 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer42cfg2",
            extends: None,
            description: Some(
                "TIMER42 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer43cfg0",
            extends: None,
            description: Some(
                "TIMER43 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer43cfg1",
            extends: None,
            description: Some(
                "TIMER43 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer43cfg2",
            extends: None,
            description: Some(
                "TIMER43 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer44cfg0",
            extends: None,
            description: Some(
                "TIMER44 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer44cfg1",
            extends: None,
            description: Some(
                "TIMER44 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer44cfg2",
            extends: None,
            description: Some(
                "TIMER44 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer4cfg0",
            extends: None,
            description: Some(
                "TIMER4 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer4cfg1",
            extends: None,
            description: Some(
                "TIMER4 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer4cfg2",
            extends: None,
            description: Some(
                "TIMER4 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer7cfg0",
            extends: None,
            description: Some(
                "TIMER7 configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg0",
                    description: Some(
                        "Quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg1",
                    description: Some(
                        "Quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg2",
                    description: Some(
                        "Quadrature decoder mode 2 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg3",
                    description: Some(
                        "Restart mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg4",
                    description: Some(
                        "Pause mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg5",
                    description: Some(
                        "Event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer7cfg1",
            extends: None,
            description: Some(
                "TIMER7 configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg6",
                    description: Some(
                        "External clock mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg7",
                    description: Some(
                        "Restart or event mode configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg8",
                    description: Some(
                        "Non-quadrature decoder mode 0 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscfg9",
                    description: Some(
                        "Non-quadrature decoder mode 1 configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timer7cfg2",
            extends: None,
            description: Some(
                "TIMER7 configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tscfg15",
                    description: Some(
                        "Internal trigger input source configuration",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel0",
            extends: None,
            description: Some(
                "Timer input selection register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer7_ci0_sel",
                    description: Some(
                        "Selects TIMER7_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci1_sel",
                    description: Some(
                        "Selects TIMER7_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci2_sel",
                    description: Some(
                        "Selects TIMER7_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer7_ci3_sel",
                    description: Some(
                        "Selects TIMER7_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci0_sel",
                    description: Some(
                        "Selects TIMER0_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci1_sel",
                    description: Some(
                        "Selects TIMER0_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci2_sel",
                    description: Some(
                        "Selects TIMER0_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer0_ci3_sel",
                    description: Some(
                        "Selects TIMER0_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel1",
            extends: None,
            description: Some(
                "Timer input selection register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer2_ci0_sel",
                    description: Some(
                        "TIMER2_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci1_sel",
                    description: Some(
                        "TIMER2_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci2_sel",
                    description: Some(
                        "TIMER2_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer2_ci3_sel",
                    description: Some(
                        "TIMER2_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci0_sel",
                    description: Some(
                        "TIMER1_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci1_sel",
                    description: Some(
                        "TIMER1_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci2_sel",
                    description: Some(
                        "TIMER1_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer1_ci3_sel",
                    description: Some(
                        "TIMER1_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel2",
            extends: None,
            description: Some(
                "Timer input selection register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer4_ci0_sel",
                    description: Some(
                        "TIMER4_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci1_sel",
                    description: Some(
                        "TIMER4_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci2_sel",
                    description: Some(
                        "TIMER4_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer4_ci3_sel",
                    description: Some(
                        "TIMER4_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci0_sel",
                    description: Some(
                        "TIMER3_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci1_sel",
                    description: Some(
                        "TIMER3_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci2_sel",
                    description: Some(
                        "TIMER3_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer3_ci3_sel",
                    description: Some(
                        "TIMER3_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel3",
            extends: None,
            description: Some(
                "Timer input selection register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer23_ci0_sel",
                    description: Some(
                        "TIMER23_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer23_ci1_sel",
                    description: Some(
                        "TIMER23_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer23_ci2_sel",
                    description: Some(
                        "TIMER23_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer23_ci3_sel",
                    description: Some(
                        "TIMER23_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer22_ci0_sel",
                    description: Some(
                        "TIMER22_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer22_ci1_sel",
                    description: Some(
                        "TIMER22_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer22_ci2_sel",
                    description: Some(
                        "TIMER22_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer22_ci3_sel",
                    description: Some(
                        "TIMER22_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel4",
            extends: None,
            description: Some(
                "Timer input selection register 4",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer31_ci0_sel",
                    description: Some(
                        "TIMER31_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer31_ci1_sel",
                    description: Some(
                        "TIMER31_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer31_ci2_sel",
                    description: Some(
                        "TIMER31_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer31_ci3_sel",
                    description: Some(
                        "TIMER31_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer30_ci0_sel",
                    description: Some(
                        "TIMER30_CI0 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer30_ci1_sel",
                    description: Some(
                        "TIMER30_CI1 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer30_ci2_sel",
                    description: Some(
                        "TIMER30_CI2 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer30_ci3_sel",
                    description: Some(
                        "TIMER30_CI3 input selection",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel5",
            extends: None,
            description: Some(
                "Timer input selection register 5",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer14_ci0_sel",
                    description: Some(
                        "Selects TIMER14_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer14_ci1_sel",
                    description: Some(
                        "Selects TIMER14_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer40_ci0_sel",
                    description: Some(
                        "Selects TIMER40_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer40_ci1_sel",
                    description: Some(
                        "Selects TIMER40_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer41_ci0_sel",
                    description: Some(
                        "Selects TIMER41_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer41_ci1_sel",
                    description: Some(
                        "Selects TIMER41_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer42_ci0_sel",
                    description: Some(
                        "Selects TIMER42_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer42_ci1_sel",
                    description: Some(
                        "Selects TIMER42_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Timercisel6",
            extends: None,
            description: Some(
                "Timer input selection register 6",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "timer15_ci0_sel",
                    description: Some(
                        "Selects TIMER15_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer16_ci0_sel",
                    description: Some(
                        "Selects TIMER16_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer43_ci0_sel",
                    description: Some(
                        "Selects TIMER43_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer43_ci1_sel",
                    description: Some(
                        "Selects TIMER43_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer44_ci0_sel",
                    description: Some(
                        "Selects TIMER44_CI0 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "timer44_ci1_sel",
                    description: Some(
                        "Selects TIMER44_CI1 input",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Usercfg0",
            extends: None,
            description: Some(
                "User configuration register 0",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "borlev",
                    description: Some(
                        "BOR Brownout reset threshold level",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boot_mode",
                    description: Some(
                        "Boot mode",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Usercfg1",
            extends: None,
            description: Some(
                "User configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ana_version",
                    description: Some(
                        "The analog version signal",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                