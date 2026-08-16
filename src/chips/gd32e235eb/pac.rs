#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - FLASH"]
    FLASH = 3,
    #[doc = "4 - RCC"]
    RCC = 4,
    #[doc = "5 - EXTI0_1"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15"]
    EXTI4_15 = 7,
    #[doc = "9 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 9,
    #[doc = "10 - DMA1_CHANNEL2_3"]
    DMA1_CHANNEL2_3 = 10,
    #[doc = "11 - DMA1_CHANNEL4_5"]
    DMA1_CHANNEL4_5 = 11,
    #[doc = "12 - ADC1_CMP"]
    ADC1_CMP = 12,
    #[doc = "13 - TIM1_BRK_UP_TRG_COM"]
    TIM1_BRK_UP_TRG_COM = 13,
    #[doc = "14 - TIM1_CC"]
    TIM1_CC = 14,
    #[doc = "16 - TIM3"]
    TIM3 = 16,
    #[doc = "17 - TIM6"]
    TIM6 = 17,
    #[doc = "19 - TIM14"]
    TIM14 = 19,
    #[doc = "20 - TIM15"]
    TIM15 = 20,
    #[doc = "21 - TIM16"]
    TIM16 = 21,
    #[doc = "22 - TIM17"]
    TIM17 = 22,
    #[doc = "23 - I2C1_EV"]
    I2C1_EV = 23,
    #[doc = "24 - I2C2_EV"]
    I2C2_EV = 24,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "26 - SPI2"]
    SPI2 = 26,
    #[doc = "27 - USART1"]
    USART1 = 27,
    #[doc = "28 - USART2"]
    USART2 = 28,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn LVD();
        fn RTC();
        fn FLASH();
        fn RCC();
        fn EXTI0_1();
        fn EXTI2_3();
        fn EXTI4_15();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2_3();
        fn DMA1_CHANNEL4_5();
        fn ADC1_CMP();
        fn TIM1_BRK_UP_TRG_COM();
        fn TIM1_CC();
        fn TIM3();
        fn TIM6();
        fn TIM14();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn I2C1_EV();
        fn I2C2_EV();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn I2C1_ER();
        fn I2C2_ER();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 35] = [
        Vector { _handler: WWDG },
        Vector { _handler: LVD },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0_1 },
        Vector { _handler: EXTI2_3 },
        Vector { _handler: EXTI4_15 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2_3,
        },
        Vector {
            _handler: DMA1_CHANNEL4_5,
        },
        Vector { _handler: ADC1_CMP },
        Vector {
            _handler: TIM1_BRK_UP_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _reserved: 0 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM6 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM14 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C2_EV },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C1_ER },
        Vector { _reserved: 0 },
        Vector { _handler: I2C2_ER },
    ];
}
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const RTC: gdrtc335eb78b::Rtc = unsafe { gdrtc335eb78b::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: gdspi1de3ba8e1::Spi1 = unsafe { gdspi1de3ba8e1::Spi1::from_ptr(0x4000_3800usize as _) };
pub const USART2: gdusart0bd1afef3::Usart0 = unsafe { gdusart0bd1afef3::Usart0::from_ptr(0x4000_4400usize as _) };
pub const I2C1: gdi2c08b9ac71f::I2c0 = unsafe { gdi2c08b9ac71f::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c08b9ac71f::I2c0 = unsafe { gdi2c08b9ac71f::I2c0::from_ptr(0x4000_5800usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const CMP: gdcmpfe28bbad::Cmp = unsafe { gdcmpfe28bbad::Cmp::from_ptr(0x4001_001cusize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const ADC1: gdadc47519d73::Adc = unsafe { gdadc47519d73::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: gdspi0e52b5b99::Spi0 = unsafe { gdspi0e52b5b99::Spi0::from_ptr(0x4001_3000usize as _) };
pub const USART1: gdusart0bd1afef3::Usart0 = unsafe { gdusart0bd1afef3::Usart0::from_ptr(0x4001_3800usize as _) };
pub const TIM15: timer::Tim2chCmp = unsafe { timer::Tim2chCmp::from_ptr(0x4001_4000usize as _) };
pub const TIM16: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4400usize as _) };
pub const TIM17: timer::Tim1chCmp = unsafe { timer::Tim1chCmp::from_ptr(0x4001_4800usize as _) };
pub const DBGMCU: gddbgmcu751e9bc8::Dbgmcu = unsafe { gddbgmcu751e9bc8::Dbgmcu::from_ptr(0x4001_5800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: gdcrc8a4036fe::Crc = unsafe { gdcrc8a4036fe::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gdgpioc88586c6c::Gpioc = unsafe { gdgpioc88586c6c::Gpioc::from_ptr(0x4800_0800usize as _) };
pub const GPIOF: gdgpiof41ef0f55::Gpiof = unsafe { gdgpiof41ef0f55::Gpiof::from_ptr(0x4800_1400usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/exti_gd396c6dd7493e.rs"]
pub mod exti;
#[path = "../../peripherals/flash_f0.rs"]
pub mod flash;
#[path = "../../peripherals/gdadc47519d73_v1.rs"]
pub mod gdadc47519d73;
#[path = "../../peripherals/gdcmpfe28bbad_v1.rs"]
pub mod gdcmpfe28bbad;
#[path = "../../peripherals/gdcrc8a4036fe_v1.rs"]
pub mod gdcrc8a4036fe;
#[path = "../../peripherals/gddbgmcu751e9bc8_v1.rs"]
pub mod gddbgmcu751e9bc8;
#[path = "../../peripherals/gdgpioc88586c6c_v1.rs"]
pub mod gdgpioc88586c6c;
#[path = "../../peripherals/gdgpiof41ef0f55_v1.rs"]
pub mod gdgpiof41ef0f55;
#[path = "../../peripherals/gdi2c08b9ac71f_v1.rs"]
pub mod gdi2c08b9ac71f;
#[path = "../../peripherals/gdrtc335eb78b_v1.rs"]
pub mod gdrtc335eb78b;
#[path = "../../peripherals/gdspi0e52b5b99_v1.rs"]
pub mod gdspi0e52b5b99;
#[path = "../../peripherals/gdspi1de3ba8e1_v1.rs"]
pub mod gdspi1de3ba8e1;
#[path = "../../peripherals/gdusart0bd1afef3_v1.rs"]
pub mod gdusart0bd1afef3;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_f0x0.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f0v2.rs"]
pub mod rcc;
#[path = "../../peripherals/syscfg_f0.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
