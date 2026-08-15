#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - TAMPER"]
    TAMPER = 2,
    #[doc = "3 - RTC"]
    RTC = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC_CRS"]
    RCC_CRS = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC1_2"]
    ADC1_2 = 18,
    #[doc = "19 - CAN1_TX"]
    CAN1_TX = 19,
    #[doc = "20 - CAN1_RX0"]
    CAN1_RX0 = 20,
    #[doc = "21 - CAN1_RX1"]
    CAN1_RX1 = 21,
    #[doc = "22 - CAN1_SCE"]
    CAN1_SCE = 22,
    #[doc = "23 - EXTI5_9"]
    EXTI5_9 = 23,
    #[doc = "24 - TIM1_BRK_TIM9"]
    TIM1_BRK_TIM9 = 24,
    #[doc = "25 - TIM1_UP_TIM10"]
    TIM1_UP_TIM10 = 25,
    #[doc = "26 - TIM1_TRG_COM_TIM11"]
    TIM1_TRG_COM_TIM11 = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI10_15"]
    EXTI10_15 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - USBFS_WKUP"]
    USBFS_WKUP = 42,
    #[doc = "43 - TIM8_BRK_TIM12"]
    TIM8_BRK_TIM12 = 43,
    #[doc = "44 - TIM8_UP_TIM13"]
    TIM8_UP_TIM13 = 44,
    #[doc = "45 - TIM8_TRG_COM_TIM14"]
    TIM8_TRG_COM_TIM14 = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6"]
    TIM6 = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 56,
    #[doc = "57 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 57,
    #[doc = "58 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 58,
    #[doc = "59 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 59,
    #[doc = "60 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 60,
    #[doc = "63 - CAN2_TX"]
    CAN2_TX = 63,
    #[doc = "64 - CAN2_RX0"]
    CAN2_RX0 = 64,
    #[doc = "65 - CAN2_RX1"]
    CAN2_RX1 = 65,
    #[doc = "66 - CAN2_SCE"]
    CAN2_SCE = 66,
    #[doc = "67 - USBFS"]
    USBFS = 67,
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
        fn TAMPER();
        fn RTC();
        fn FLASH();
        fn RCC_CRS();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1_2();
        fn CAN1_TX();
        fn CAN1_RX0();
        fn CAN1_RX1();
        fn CAN1_SCE();
        fn EXTI5_9();
        fn TIM1_BRK_TIM9();
        fn TIM1_UP_TIM10();
        fn TIM1_TRG_COM_TIM11();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI10_15();
        fn RTC_ALARM();
        fn USBFS_WKUP();
        fn TIM8_BRK_TIM12();
        fn TIM8_UP_TIM13();
        fn TIM8_TRG_COM_TIM14();
        fn TIM8_CC();
        fn FMC();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6();
        fn TIM7();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn DMA2_CHANNEL5();
        fn CAN2_TX();
        fn CAN2_RX0();
        fn CAN2_RX1();
        fn CAN2_SCE();
        fn USBFS();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 68] = [
        Vector { _handler: WWDG },
        Vector { _handler: LVD },
        Vector { _handler: TAMPER },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC_CRS },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: ADC1_2 },
        Vector { _handler: CAN1_TX },
        Vector { _handler: CAN1_RX0 },
        Vector { _handler: CAN1_RX1 },
        Vector { _handler: CAN1_SCE },
        Vector { _handler: EXTI5_9 },
        Vector {
            _handler: TIM1_BRK_TIM9,
        },
        Vector {
            _handler: TIM1_UP_TIM10,
        },
        Vector {
            _handler: TIM1_TRG_COM_TIM11,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: EXTI10_15 },
        Vector { _handler: RTC_ALARM },
        Vector { _handler: USBFS_WKUP },
        Vector {
            _handler: TIM8_BRK_TIM12,
        },
        Vector {
            _handler: TIM8_UP_TIM13,
        },
        Vector {
            _handler: TIM8_TRG_COM_TIM14,
        },
        Vector { _handler: TIM8_CC },
        Vector { _reserved: 0 },
        Vector { _handler: FMC },
        Vector { _reserved: 0 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_CHANNEL1,
        },
        Vector {
            _handler: DMA2_CHANNEL2,
        },
        Vector {
            _handler: DMA2_CHANNEL3,
        },
        Vector {
            _handler: DMA2_CHANNEL4,
        },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CAN2_TX },
        Vector { _handler: CAN2_RX0 },
        Vector { _handler: CAN2_RX1 },
        Vector { _handler: CAN2_SCE },
        Vector { _handler: USBFS },
    ];
}
pub const TIM2: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: gdusart05f10d25f::Usart0 = unsafe { gdusart05f10d25f::Usart0::from_ptr(0x4000_4400usize as _) };
pub const USART3: gdusart05f10d25f::Usart0 = unsafe { gdusart05f10d25f::Usart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: gduart384580f85::Uart3 = unsafe { gduart384580f85::Uart3::from_ptr(0x4000_4c00usize as _) };
pub const UART5: gduart384580f85::Uart3 = unsafe { gduart384580f85::Uart3::from_ptr(0x4000_5000usize as _) };
pub const I2C1: gdi2c0e704b87e::I2c0 = unsafe { gdi2c0e704b87e::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c0e704b87e::I2c0 = unsafe { gdi2c0e704b87e::I2c0::from_ptr(0x4000_5800usize as _) };
pub const CAN1: gdcan0b8705c1f::Can0 = unsafe { gdcan0b8705c1f::Can0::from_ptr(0x4000_6400usize as _) };
pub const CAN2: gdcan0b8705c1f::Can0 = unsafe { gdcan0b8705c1f::Can0::from_ptr(0x4000_6800usize as _) };
pub const BKP: bkp::Bkp = unsafe { bkp::Bkp::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const CRS: gdctcb7d69f86::Ctc = unsafe { gdctcb7d69f86::Ctc::from_ptr(0x4000_c800usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: gdexti9c0c2c74::Exti = unsafe { gdexti9c0c2c74::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1800usize as _) };
pub const ADC1: gdadc0742bf082::Adc0 = unsafe { gdadc0742bf082::Adc0::from_ptr(0x4001_2400usize as _) };
pub const ADC2: gdadc16b3105a2::Adc1 = unsafe { gdadc16b3105a2::Adc1::from_ptr(0x4001_2800usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400usize as _) };
pub const USART1: gdusart05f10d25f::Usart0 = unsafe { gdusart05f10d25f::Usart0::from_ptr(0x4001_3800usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_4c00usize as _) };
pub const TIM10: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_5000usize as _) };
pub const TIM11: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_5400usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const USBFS_GLOBAL: gdusbfsglobal36d63ef4::UsbfsGlobal =
    unsafe { gdusbfsglobal36d63ef4::UsbfsGlobal::from_ptr(0x5000_0000usize as _) };
pub const USBFS_HOST: gdusbfshost5f42a79e::UsbfsHost =
    unsafe { gdusbfshost5f42a79e::UsbfsHost::from_ptr(0x5000_0400usize as _) };
pub const USBFS_DEVICE: gdusbfsdevicea4903788::UsbfsDevice =
    unsafe { gdusbfsdevicea4903788::UsbfsDevice::from_ptr(0x5000_0800usize as _) };
pub const USBFS_PWRCLK: gdusbfspwrclk2ac667f0::UsbfsPwrclk =
    unsafe { gdusbfspwrclk2ac667f0::UsbfsPwrclk::from_ptr(0x5000_0e00usize as _) };
pub const FMC: gdexmc0c702b92::Exmc = unsafe { gdexmc0c702b92::Exmc::from_ptr(0xa000_0000usize as _) };
pub const DBGMCU: gddbgf31a76f7::Dbg = unsafe { gddbgf31a76f7::Dbg::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/afio_f1.rs"]
pub mod afio;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/bkp_v1.rs"]
pub mod bkp;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v1.rs"]
pub mod dac;
#[path = "../../peripherals/flash_f1.rs"]
pub mod flash;
#[path = "../../peripherals/gdadc0742bf082_v1.rs"]
pub mod gdadc0742bf082;
#[path = "../../peripherals/gdadc16b3105a2_v1.rs"]
pub mod gdadc16b3105a2;
#[path = "../../peripherals/gdcan0b8705c1f_v1.rs"]
pub mod gdcan0b8705c1f;
#[path = "../../peripherals/gdctcb7d69f86_v1.rs"]
pub mod gdctcb7d69f86;
#[path = "../../peripherals/gddbgf31a76f7_v1.rs"]
pub mod gddbgf31a76f7;
#[path = "../../peripherals/gdexmc0c702b92_v1.rs"]
pub mod gdexmc0c702b92;
#[path = "../../peripherals/gdexti9c0c2c74_v1.rs"]
pub mod gdexti9c0c2c74;
#[path = "../../peripherals/gdi2c0e704b87e_v1.rs"]
pub mod gdi2c0e704b87e;
#[path = "../../peripherals/gduart384580f85_v1.rs"]
pub mod gduart384580f85;
#[path = "../../peripherals/gdusart05f10d25f_v1.rs"]
pub mod gdusart05f10d25f;
#[path = "../../peripherals/gdusbfsdevicea4903788_v1.rs"]
pub mod gdusbfsdevicea4903788;
#[path = "../../peripherals/gdusbfsglobal36d63ef4_v1.rs"]
pub mod gdusbfsglobal36d63ef4;
#[path = "../../peripherals/gdusbfshost5f42a79e_v1.rs"]
pub mod gdusbfshost5f42a79e;
#[path = "../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"]
pub mod gdusbfspwrclk2ac667f0;
#[path = "../../peripherals/gpio_v1.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_f1.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f1.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v1.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v1_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
