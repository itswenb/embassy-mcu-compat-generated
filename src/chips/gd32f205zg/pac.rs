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
    #[doc = "5 - RCC"]
    RCC = 5,
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
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
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
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
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
    #[doc = "47 - ADC3"]
    ADC3 = 47,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDIO"]
    SDIO = 49,
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
    #[doc = "69 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 69,
    #[doc = "70 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "79 - CAU"]
    CAU = 79,
    #[doc = "80 - HAU_RNG"]
    HAU_RNG = 80,
    #[doc = "82 - UART7"]
    UART7 = 82,
    #[doc = "83 - UART8"]
    UART8 = 83,
    #[doc = "88 - LTDC"]
    LTDC = 88,
    #[doc = "89 - LTDC_ER"]
    LTDC_ER = 89,
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
        fn RCC();
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
        fn EXTI9_5();
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
        fn EXTI15_10();
        fn RTC_ALARM();
        fn USBFS_WKUP();
        fn TIM8_BRK_TIM12();
        fn TIM8_UP_TIM13();
        fn TIM8_TRG_COM_TIM14();
        fn TIM8_CC();
        fn ADC3();
        fn FMC();
        fn SDIO();
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
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn DCMI();
        fn CAU();
        fn HAU_RNG();
        fn UART7();
        fn UART8();
        fn LTDC();
        fn LTDC_ER();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 90] = [
        Vector { _handler: WWDG },
        Vector { _handler: LVD },
        Vector { _handler: TAMPER },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
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
        Vector { _handler: EXTI9_5 },
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
        Vector { _handler: EXTI15_10 },
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
        Vector { _handler: ADC3 },
        Vector { _handler: FMC },
        Vector { _handler: SDIO },
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
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DCMI },
        Vector { _handler: CAU },
        Vector { _handler: HAU_RNG },
        Vector { _reserved: 0 },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
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
pub const USART2: gdusart0677bab67::Usart0 = unsafe { gdusart0677bab67::Usart0::from_ptr(0x4000_4400usize as _) };
pub const USART3: gdusart0677bab67::Usart0 = unsafe { gdusart0677bab67::Usart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: gduart36dbe0a8a::Uart3 = unsafe { gduart36dbe0a8a::Uart3::from_ptr(0x4000_4c00usize as _) };
pub const UART5: gduart36dbe0a8a::Uart3 = unsafe { gduart36dbe0a8a::Uart3::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const CAN1: gdcan01d9ed235::Can0 = unsafe { gdcan01d9ed235::Can0::from_ptr(0x4000_6400usize as _) };
pub const CAN2: gdcan01d9ed235::Can0 = unsafe { gdcan01d9ed235::Can0::from_ptr(0x4000_6800usize as _) };
pub const BKP: bkp::Bkp = unsafe { bkp::Bkp::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const UART7: gduart36dbe0a8a::Uart3 = unsafe { gduart36dbe0a8a::Uart3::from_ptr(0x4000_7800usize as _) };
pub const UART8: gduart36dbe0a8a::Uart3 = unsafe { gduart36dbe0a8a::Uart3::from_ptr(0x4000_7c00usize as _) };
pub const I2C3: gdi2c0700b93ad::I2c0 = unsafe { gdi2c0700b93ad::I2c0::from_ptr(0x4000_c000usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1800usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1c00usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_2000usize as _) };
pub const ADC1: gdadc059fb2391::Adc0 = unsafe { gdadc059fb2391::Adc0::from_ptr(0x4001_2400usize as _) };
pub const ADC2: gdadc059fb2391::Adc0 = unsafe { gdadc059fb2391::Adc0::from_ptr(0x4001_2800usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_3400usize as _) };
pub const USART1: gdusart0677bab67::Usart0 = unsafe { gdusart0677bab67::Usart0::from_ptr(0x4001_3800usize as _) };
pub const ADC3: gdadc059fb2391::Adc0 = unsafe { gdadc059fb2391::Adc0::from_ptr(0x4001_3c00usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_4c00usize as _) };
pub const TIM10: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_5000usize as _) };
pub const TIM11: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_5400usize as _) };
pub const LTDC: gdtli89ae65d5::Tli = unsafe { gdtli89ae65d5::Tli::from_ptr(0x4001_6800usize as _) };
pub const USART6: gdusart0677bab67::Usart0 = unsafe { gdusart0677bab67::Usart0::from_ptr(0x4001_7000usize as _) };
pub const GPIOH: gdgpioa979b0f67::Gpioa = unsafe { gdgpioa979b0f67::Gpioa::from_ptr(0x4001_7400usize as _) };
pub const GPIOI: gdgpioa979b0f67::Gpioa = unsafe { gdgpioa979b0f67::Gpioa::from_ptr(0x4001_7800usize as _) };
pub const SDIO: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4001_8000usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const ENET_MAC: gdenetmac09c072f4::EnetMac = unsafe { gdenetmac09c072f4::EnetMac::from_ptr(0x4002_8000usize as _) };
pub const ENET_MSC: gdenetmsc9217fdbd::EnetMsc = unsafe { gdenetmsc9217fdbd::EnetMsc::from_ptr(0x4002_8100usize as _) };
pub const ENET_PTP: gdenetptpf491bb9d::EnetPtp = unsafe { gdenetptpf491bb9d::EnetPtp::from_ptr(0x4002_8700usize as _) };
pub const ENET_DMA: gdenetdmacba9250b::EnetDma = unsafe { gdenetdmacba9250b::EnetDma::from_ptr(0x4002_9000usize as _) };
pub const USBFS_GLOBAL: gdusbfsglobal019fa48e::UsbfsGlobal =
    unsafe { gdusbfsglobal019fa48e::UsbfsGlobal::from_ptr(0x5000_0000usize as _) };
pub const USBFS_HOST: gdusbfshost5f42a79e::UsbfsHost =
    unsafe { gdusbfshost5f42a79e::UsbfsHost::from_ptr(0x5000_0400usize as _) };
pub const USBFS_DEVICE: gdusbfsdevicea4903788::UsbfsDevice =
    unsafe { gdusbfsdevicea4903788::UsbfsDevice::from_ptr(0x5000_0800usize as _) };
pub const USBFS_PWRCLK: gdusbfspwrclk2ac667f0::UsbfsPwrclk =
    unsafe { gdusbfspwrclk2ac667f0::UsbfsPwrclk::from_ptr(0x5000_0e00usize as _) };
pub const DCMI: gddcia70582ff::Dci = unsafe { gddcia70582ff::Dci::from_ptr(0x5005_0000usize as _) };
pub const CAU: gdcau1d48f570::Cau = unsafe { gdcau1d48f570::Cau::from_ptr(0x5006_0000usize as _) };
pub const HAU: gdhau67f15641::Hau = unsafe { gdhau67f15641::Hau::from_ptr(0x5006_0400usize as _) };
pub const RNG: gdtrngbf61c352::Trng = unsafe { gdtrngbf61c352::Trng::from_ptr(0x5006_0800usize as _) };
pub const FMC: gdexmcb6dcdf27::Exmc = unsafe { gdexmcb6dcdf27::Exmc::from_ptr(0xa000_0000usize as _) };
pub const DBGMCU: gddbgb21f1063::Dbg = unsafe { gddbgb21f1063::Dbg::from_ptr(0xe004_2000usize as _) };
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
#[path = "../../peripherals/exti_gdbeb972624ea3.rs"]
pub mod exti;
#[path = "../../peripherals/flash_f1.rs"]
pub mod flash;
#[path = "../../peripherals/gdadc059fb2391_v1.rs"]
pub mod gdadc059fb2391;
#[path = "../../peripherals/gdcan01d9ed235_v1.rs"]
pub mod gdcan01d9ed235;
#[path = "../../peripherals/gdcau1d48f570_v1.rs"]
pub mod gdcau1d48f570;
#[path = "../../peripherals/gddbgb21f1063_v1.rs"]
pub mod gddbgb21f1063;
#[path = "../../peripherals/gddcia70582ff_v1.rs"]
pub mod gddcia70582ff;
#[path = "../../peripherals/gdenetdmacba9250b_v1.rs"]
pub mod gdenetdmacba9250b;
#[path = "../../peripherals/gdenetmac09c072f4_v1.rs"]
pub mod gdenetmac09c072f4;
#[path = "../../peripherals/gdenetmsc9217fdbd_v1.rs"]
pub mod gdenetmsc9217fdbd;
#[path = "../../peripherals/gdenetptpf491bb9d_v1.rs"]
pub mod gdenetptpf491bb9d;
#[path = "../../peripherals/gdexmcb6dcdf27_v1.rs"]
pub mod gdexmcb6dcdf27;
#[path = "../../peripherals/gdgpioa979b0f67_v1.rs"]
pub mod gdgpioa979b0f67;
#[path = "../../peripherals/gdhau67f15641_v1.rs"]
pub mod gdhau67f15641;
#[path = "../../peripherals/gdi2c0700b93ad_v1.rs"]
pub mod gdi2c0700b93ad;
#[path = "../../peripherals/gdtli89ae65d5_v1.rs"]
pub mod gdtli89ae65d5;
#[path = "../../peripherals/gdtrngbf61c352_v1.rs"]
pub mod gdtrngbf61c352;
#[path = "../../peripherals/gduart36dbe0a8a_v1.rs"]
pub mod gduart36dbe0a8a;
#[path = "../../peripherals/gdusart0677bab67_v1.rs"]
pub mod gdusart0677bab67;
#[path = "../../peripherals/gdusbfsdevicea4903788_v1.rs"]
pub mod gdusbfsdevicea4903788;
#[path = "../../peripherals/gdusbfsglobal019fa48e_v1.rs"]
pub mod gdusbfsglobal019fa48e;
#[path = "../../peripherals/gdusbfshost5f42a79e_v1.rs"]
pub mod gdusbfshost5f42a79e;
#[path = "../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"]
pub mod gdusbfspwrclk2ac667f0;
#[path = "../../peripherals/gpio_v1.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v1_gd87a4c48e1698.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_f1.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f1.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_v1.rs"]
pub mod rtc;
#[path = "../../peripherals/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v1_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
