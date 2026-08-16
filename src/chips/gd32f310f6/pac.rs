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
    #[doc = "4 - RCC_CRS"]
    RCC_CRS = 4,
    #[doc = "5 - EXTI0_1"]
    EXTI0_1 = 5,
    #[doc = "6 - EXTI2_3"]
    EXTI2_3 = 6,
    #[doc = "7 - EXTI4_15"]
    EXTI4_15 = 7,
    #[doc = "8 - TSI"]
    TSI = 8,
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
    #[doc = "15 - TIM2"]
    TIM2 = 15,
    #[doc = "16 - TIM3"]
    TIM3 = 16,
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
    #[doc = "48 - DMA1_CHANNEL6_7"]
    DMA1_CHANNEL6_7 = 48,
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
        fn RCC_CRS();
        fn EXTI0_1();
        fn EXTI2_3();
        fn EXTI4_15();
        fn TSI();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2_3();
        fn DMA1_CHANNEL4_5();
        fn ADC1_CMP();
        fn TIM1_BRK_UP_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
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
        fn DMA1_CHANNEL6_7();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 49] = [
        Vector { _handler: WWDG },
        Vector { _handler: LVD },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC_CRS },
        Vector { _handler: EXTI0_1 },
        Vector { _handler: EXTI2_3 },
        Vector { _handler: EXTI4_15 },
        Vector { _handler: TSI },
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
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _reserved: 0 },
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA1_CHANNEL6_7,
        },
    ];
}
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0400usize as _) };
pub const TIM6: gdtimer5183dba8f::Timer5 = unsafe { gdtimer5183dba8f::Timer5::from_ptr(0x4000_1000usize as _) };
pub const TIM14: gdtimer1309b6b8cc::Timer13 = unsafe { gdtimer1309b6b8cc::Timer13::from_ptr(0x4000_2000usize as _) };
pub const RTC: gdrtc7ef316ca::Rtc = unsafe { gdrtc7ef316ca::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const USART2: gdusart08bc22e17::Usart0 = unsafe { gdusart08bc22e17::Usart0::from_ptr(0x4000_4400usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: gddacc6b1bb98::Dac = unsafe { gddacc6b1bb98::Dac::from_ptr(0x4000_7400usize as _) };
pub const CEC: gdcec9fb29752::Cec = unsafe { gdcec9fb29752::Cec::from_ptr(0x4000_7800usize as _) };
pub const CRS: gdctc57a0fbe5::Ctc = unsafe { gdctc57a0fbe5::Ctc::from_ptr(0x4000_c800usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const CMP: gdcmp6176059a::Cmp = unsafe { gdcmp6176059a::Cmp::from_ptr(0x4001_001cusize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const ADC1: gdadcb7217899::Adc = unsafe { gdadcb7217899::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: gdtimer0d9a58b68::Timer0 = unsafe { gdtimer0d9a58b68::Timer0::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: gdusart08bc22e17::Usart0 = unsafe { gdusart08bc22e17::Usart0::from_ptr(0x4001_3800usize as _) };
pub const TIM15: gdtimer144dec44bb::Timer14 = unsafe { gdtimer144dec44bb::Timer14::from_ptr(0x4001_4000usize as _) };
pub const TIM16: gdtimer15dc6fd783::Timer15 = unsafe { gdtimer15dc6fd783::Timer15::from_ptr(0x4001_4400usize as _) };
pub const TIM17: gdtimer15dc6fd783::Timer15 = unsafe { gdtimer15dc6fd783::Timer15::from_ptr(0x4001_4800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const TSI: gdtsid83e70fb::Tsi = unsafe { gdtsid83e70fb::Tsi::from_ptr(0x4002_4000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gdgpioc47392aee::Gpioc = unsafe { gdgpioc47392aee::Gpioc::from_ptr(0x4800_0800usize as _) };
pub const GPIOD: gdgpiodc257f1c6::Gpiod = unsafe { gdgpiodc257f1c6::Gpiod::from_ptr(0x4800_0c00usize as _) };
pub const GPIOF: gdgpiof564f1005::Gpiof = unsafe { gdgpiof564f1005::Gpiof::from_ptr(0x4800_1400usize as _) };
pub const USBFS_GLOBAL: gdusbfsglobal1a7549aa::UsbfsGlobal =
    unsafe { gdusbfsglobal1a7549aa::UsbfsGlobal::from_ptr(0x5000_0000usize as _) };
pub const USBFS_HOST: gdusbfshost5f42a79e::UsbfsHost =
    unsafe { gdusbfshost5f42a79e::UsbfsHost::from_ptr(0x5000_0400usize as _) };
pub const USBFS_DEVICE: gdusbfsdevice6d1906cf::UsbfsDevice =
    unsafe { gdusbfsdevice6d1906cf::UsbfsDevice::from_ptr(0x5000_0800usize as _) };
pub const USBFS_PWRCLK: gdusbfspwrclk2ac667f0::UsbfsPwrclk =
    unsafe { gdusbfspwrclk2ac667f0::UsbfsPwrclk::from_ptr(0x5000_0e00usize as _) };
pub const DBGMCU: gddbg7f4c1511::Dbg = unsafe { gddbg7f4c1511::Dbg::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/exti_gd9943aed1590c.rs"]
pub mod exti;
#[path = "../../peripherals/flash_l1.rs"]
pub mod flash;
#[path = "../../peripherals/gdadcb7217899_v1.rs"]
pub mod gdadcb7217899;
#[path = "../../peripherals/gdcec9fb29752_v1.rs"]
pub mod gdcec9fb29752;
#[path = "../../peripherals/gdcmp6176059a_v1.rs"]
pub mod gdcmp6176059a;
#[path = "../../peripherals/gdctc57a0fbe5_v1.rs"]
pub mod gdctc57a0fbe5;
#[path = "../../peripherals/gddacc6b1bb98_v1.rs"]
pub mod gddacc6b1bb98;
#[path = "../../peripherals/gddbg7f4c1511_v1.rs"]
pub mod gddbg7f4c1511;
#[path = "../../peripherals/gdgpioc47392aee_v1.rs"]
pub mod gdgpioc47392aee;
#[path = "../../peripherals/gdgpiodc257f1c6_v1.rs"]
pub mod gdgpiodc257f1c6;
#[path = "../../peripherals/gdgpiof564f1005_v1.rs"]
pub mod gdgpiof564f1005;
#[path = "../../peripherals/gdrtc7ef316ca_v1.rs"]
pub mod gdrtc7ef316ca;
#[path = "../../peripherals/gdtimer0d9a58b68_v1.rs"]
pub mod gdtimer0d9a58b68;
#[path = "../../peripherals/gdtimer1309b6b8cc_v1.rs"]
pub mod gdtimer1309b6b8cc;
#[path = "../../peripherals/gdtimer144dec44bb_v1.rs"]
pub mod gdtimer144dec44bb;
#[path = "../../peripherals/gdtimer15dc6fd783_v1.rs"]
pub mod gdtimer15dc6fd783;
#[path = "../../peripherals/gdtimer5183dba8f_v1.rs"]
pub mod gdtimer5183dba8f;
#[path = "../../peripherals/gdtsid83e70fb_v1.rs"]
pub mod gdtsid83e70fb;
#[path = "../../peripherals/gdusart08bc22e17_v1.rs"]
pub mod gdusart08bc22e17;
#[path = "../../peripherals/gdusbfsdevice6d1906cf_v1.rs"]
pub mod gdusbfsdevice6d1906cf;
#[path = "../../peripherals/gdusbfsglobal1a7549aa_v1.rs"]
pub mod gdusbfsglobal1a7549aa;
#[path = "../../peripherals/gdusbfshost5f42a79e_v1.rs"]
pub mod gdusbfshost5f42a79e;
#[path = "../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"]
pub mod gdusbfspwrclk2ac667f0;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v1_gde9aa891ad291.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_l1.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_l1.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_v2_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_l1.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
