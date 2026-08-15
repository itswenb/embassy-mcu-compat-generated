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
    #[doc = "5 - RCC_CTC"]
    RCC_CTC = 5,
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
    #[doc = "24 - TIM1_BRK"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1_UP"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 26,
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
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
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
    #[doc = "61 - ENET"]
    ENET = 61,
    #[doc = "62 - ENET_WKUP"]
    ENET_WKUP = 62,
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
        fn RCC_CTC();
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
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
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
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
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
        fn ENET();
        fn ENET_WKUP();
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
        Vector { _handler: RCC_CTC },
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
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
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
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
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
        Vector { _handler: ENET },
        Vector { _handler: ENET_WKUP },
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
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: gdusart08d85785f::Usart0 = unsafe { gdusart08d85785f::Usart0::from_ptr(0x4000_4400usize as _) };
pub const USART3: gdusart08d85785f::Usart0 = unsafe { gdusart08d85785f::Usart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: gduart35ffe463f::Uart3 = unsafe { gduart35ffe463f::Uart3::from_ptr(0x4000_4c00usize as _) };
pub const UART5: gduart35ffe463f::Uart3 = unsafe { gduart35ffe463f::Uart3::from_ptr(0x4000_5000usize as _) };
pub const I2C1: gdi2c08f648655::I2c0 = unsafe { gdi2c08f648655::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c08f648655::I2c0 = unsafe { gdi2c08f648655::I2c0::from_ptr(0x4000_5800usize as _) };
pub const CAN1: gdcan06b36baa3::Can0 = unsafe { gdcan06b36baa3::Can0::from_ptr(0x4000_6400usize as _) };
pub const CAN2: gdcan06b36baa3::Can0 = unsafe { gdcan06b36baa3::Can0::from_ptr(0x4000_6800usize as _) };
pub const BKP: bkp::Bkp = unsafe { bkp::Bkp::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: gdexti11a1be47::Exti = unsafe { gdexti11a1be47::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1800usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1c00usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_2000usize as _) };
pub const ADC1: gdadc0dda18ebe::Adc0 = unsafe { gdadc0dda18ebe::Adc0::from_ptr(0x4001_2400usize as _) };
pub const ADC2: gdadc134a2b2fe::Adc1 = unsafe { gdadc134a2b2fe::Adc1::from_ptr(0x4001_2800usize as _) };
pub const TIM1: gdtimer0e084a927::Timer0 = unsafe { gdtimer0e084a927::Timer0::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: gdtimer0e084a927::Timer0 = unsafe { gdtimer0e084a927::Timer0::from_ptr(0x4001_3400usize as _) };
pub const USART1: gdusart08d85785f::Usart0 = unsafe { gdusart08d85785f::Usart0::from_ptr(0x4001_3800usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const ENET_MAC: gdenetmac391eb49a::EnetMac = unsafe { gdenetmac391eb49a::EnetMac::from_ptr(0x4002_8000usize as _) };
pub const ENET_MSC: gdenetmsc9217fdbd::EnetMsc = unsafe { gdenetmsc9217fdbd::EnetMsc::from_ptr(0x4002_8100usize as _) };
pub const ENET_PTP: gdenetptpf491bb9d::EnetPtp = unsafe { gdenetptpf491bb9d::EnetPtp::from_ptr(0x4002_8700usize as _) };
pub const ENET_DMA: gdenetdmacda66e8d::EnetDma = unsafe { gdenetdmacda66e8d::EnetDma::from_ptr(0x4002_9000usize as _) };
pub const ENET_MAC_FCTH: gdenetmacfcth8ada9e21::EnetMacFcth =
    unsafe { gdenetmacfcth8ada9e21::EnetMacFcth::from_ptr(0x4002_9080usize as _) };
pub const USBFS_GLOBAL: gdusbfsglobal48a5dcd1::UsbfsGlobal =
    unsafe { gdusbfsglobal48a5dcd1::UsbfsGlobal::from_ptr(0x5000_0000usize as _) };
pub const USBFS_HOST: gdusbfshost6fa885e5::UsbfsHost =
    unsafe { gdusbfshost6fa885e5::UsbfsHost::from_ptr(0x5000_0400usize as _) };
pub const USBFS_DEVICE: gdusbfsdevicea4903788::UsbfsDevice =
    unsafe { gdusbfsdevicea4903788::UsbfsDevice::from_ptr(0x5000_0800usize as _) };
pub const USBFS_PWRCLK: gdusbfspwrclk2ac667f0::UsbfsPwrclk =
    unsafe { gdusbfspwrclk2ac667f0::UsbfsPwrclk::from_ptr(0x5000_0e00usize as _) };
pub const FMC: gdexmc61eab9d1::Exmc = unsafe { gdexmc61eab9d1::Exmc::from_ptr(0xa000_0000usize as _) };
pub const DBGMCU: gddbg40666257::Dbg = unsafe { gddbg40666257::Dbg::from_ptr(0xe004_2000usize as _) };
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
#[path = "../../peripherals/gdadc0dda18ebe_v1.rs"]
pub mod gdadc0dda18ebe;
#[path = "../../peripherals/gdadc134a2b2fe_v1.rs"]
pub mod gdadc134a2b2fe;
#[path = "../../peripherals/gdcan06b36baa3_v1.rs"]
pub mod gdcan06b36baa3;
#[path = "../../peripherals/gddbg40666257_v1.rs"]
pub mod gddbg40666257;
#[path = "../../peripherals/gdenetdmacda66e8d_v1.rs"]
pub mod gdenetdmacda66e8d;
#[path = "../../peripherals/gdenetmac391eb49a_v1.rs"]
pub mod gdenetmac391eb49a;
#[path = "../../peripherals/gdenetmacfcth8ada9e21_v1.rs"]
pub mod gdenetmacfcth8ada9e21;
#[path = "../../peripherals/gdenetmsc9217fdbd_v1.rs"]
pub mod gdenetmsc9217fdbd;
#[path = "../../peripherals/gdenetptpf491bb9d_v1.rs"]
pub mod gdenetptpf491bb9d;
#[path = "../../peripherals/gdexmc61eab9d1_v1.rs"]
pub mod gdexmc61eab9d1;
#[path = "../../peripherals/gdexti11a1be47_v1.rs"]
pub mod gdexti11a1be47;
#[path = "../../peripherals/gdi2c08f648655_v1.rs"]
pub mod gdi2c08f648655;
#[path = "../../peripherals/gdtimer0e084a927_v1.rs"]
pub mod gdtimer0e084a927;
#[path = "../../peripherals/gduart35ffe463f_v1.rs"]
pub mod gduart35ffe463f;
#[path = "../../peripherals/gdusart08d85785f_v1.rs"]
pub mod gdusart08d85785f;
#[path = "../../peripherals/gdusbfsdevicea4903788_v1.rs"]
pub mod gdusbfsdevicea4903788;
#[path = "../../peripherals/gdusbfsglobal48a5dcd1_v1.rs"]
pub mod gdusbfsglobal48a5dcd1;
#[path = "../../peripherals/gdusbfshost6fa885e5_v1.rs"]
pub mod gdusbfshost6fa885e5;
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
