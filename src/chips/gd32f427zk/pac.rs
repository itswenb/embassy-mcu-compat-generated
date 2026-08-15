#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - TAMPER_STAMP"]
    TAMPER_STAMP = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
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
    #[doc = "11 - DMA1_CHANNEL0"]
    DMA1_CHANNEL0 = 11,
    #[doc = "12 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 12,
    #[doc = "13 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 13,
    #[doc = "14 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 14,
    #[doc = "15 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 15,
    #[doc = "16 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 16,
    #[doc = "17 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
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
    #[doc = "47 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 47,
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
    #[doc = "54 - TIM6_DAC1"]
    TIM6_DAC1 = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_CHANNEL0"]
    DMA2_CHANNEL0 = 56,
    #[doc = "57 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 57,
    #[doc = "58 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 58,
    #[doc = "59 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 59,
    #[doc = "60 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 60,
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
    #[doc = "68 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 68,
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
    #[doc = "74 - USBHS_EP1_OUT"]
    USBHS_EP1_OUT = 74,
    #[doc = "75 - USBHS_EP1_IN"]
    USBHS_EP1_IN = 75,
    #[doc = "76 - USBHS_WKUP"]
    USBHS_WKUP = 76,
    #[doc = "77 - USBHS"]
    USBHS = 77,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "80 - RNG"]
    RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
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
        fn TAMPER_STAMP();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC_CRS();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL0();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn ADC();
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
        fn DMA1_CHANNEL7();
        fn FMC();
        fn SDIO();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC1();
        fn TIM7();
        fn DMA2_CHANNEL0();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn ENET();
        fn ENET_WKUP();
        fn CAN2_TX();
        fn CAN2_RX0();
        fn CAN2_RX1();
        fn CAN2_SCE();
        fn USBFS();
        fn DMA2_CHANNEL5();
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn USBHS_EP1_OUT();
        fn USBHS_EP1_IN();
        fn USBHS_WKUP();
        fn USBHS();
        fn DCMI();
        fn RNG();
        fn FPU();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 82] = [
        Vector { _handler: WWDG },
        Vector { _handler: LVD },
        Vector { _handler: TAMPER_STAMP },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC_CRS },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_CHANNEL0,
        },
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
        Vector { _handler: ADC },
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
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: FMC },
        Vector { _handler: SDIO },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: TIM6_DAC1 },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_CHANNEL0,
        },
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
        Vector { _handler: ENET },
        Vector { _handler: ENET_WKUP },
        Vector { _handler: CAN2_TX },
        Vector { _handler: CAN2_RX0 },
        Vector { _handler: CAN2_RX1 },
        Vector { _handler: CAN2_SCE },
        Vector { _handler: USBFS },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector {
            _handler: USBHS_EP1_OUT,
        },
        Vector { _handler: USBHS_EP1_IN },
        Vector { _handler: USBHS_WKUP },
        Vector { _handler: USBHS },
        Vector { _handler: DCMI },
        Vector { _reserved: 0 },
        Vector { _handler: RNG },
        Vector { _handler: FPU },
    ];
}
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: gdtimer519fda6d7::Timer5 = unsafe { gdtimer519fda6d7::Timer5::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::TimBasic = unsafe { timer::TimBasic::from_ptr(0x4000_1400usize as _) };
pub const TIM12: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4000_1800usize as _) };
pub const TIM13: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_1c00usize as _) };
pub const TIM14: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4000_2000usize as _) };
pub const RTC: gdrtc34bd68c7::Rtc = unsafe { gdrtc34bd68c7::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const I2S1_ADD: gdspi0a39abaa4::Spi0 = unsafe { gdspi0a39abaa4::Spi0::from_ptr(0x4000_3400usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const I2S2_ADD: gdspi0a39abaa4::Spi0 = unsafe { gdspi0a39abaa4::Spi0::from_ptr(0x4000_4000usize as _) };
pub const USART2: gdusart06fc75967::Usart0 = unsafe { gdusart06fc75967::Usart0::from_ptr(0x4000_4400usize as _) };
pub const USART3: gdusart06fc75967::Usart0 = unsafe { gdusart06fc75967::Usart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: *mut () = 0x4000_4c00usize as _;
pub const UART5: *mut () = 0x4000_5000usize as _;
pub const I2C1: gdi2c0116537ab::I2c0 = unsafe { gdi2c0116537ab::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c0116537ab::I2c0 = unsafe { gdi2c0116537ab::I2c0::from_ptr(0x4000_5800usize as _) };
pub const I2C3: gdi2c0116537ab::I2c0 = unsafe { gdi2c0116537ab::I2c0::from_ptr(0x4000_5c00usize as _) };
pub const CAN1: gdcan06b36baa3::Can0 = unsafe { gdcan06b36baa3::Can0::from_ptr(0x4000_6400usize as _) };
pub const CAN2: gdcan06b36baa3::Can0 = unsafe { gdcan06b36baa3::Can0::from_ptr(0x4000_6800usize as _) };
pub const CRS: gdctc47444a2c::Ctc = unsafe { gdctc47444a2c::Ctc::from_ptr(0x4000_6c00usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const UART7: gduart38ecaf091::Uart3 = unsafe { gduart38ecaf091::Uart3::from_ptr(0x4000_7800usize as _) };
pub const UART8: gduart38ecaf091::Uart3 = unsafe { gduart38ecaf091::Uart3::from_ptr(0x4000_7c00usize as _) };
pub const IREF: gdiref361590d6::Iref = unsafe { gdiref361590d6::Iref::from_ptr(0x4000_c400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0000usize as _) };
pub const TIM8: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0400usize as _) };
pub const USART1: gdusart06fc75967::Usart0 = unsafe { gdusart06fc75967::Usart0::from_ptr(0x4001_1000usize as _) };
pub const USART6: gdusart06fc75967::Usart0 = unsafe { gdusart06fc75967::Usart0::from_ptr(0x4001_1400usize as _) };
pub const ADC1: gdadc0644c59d8::Adc0 = unsafe { gdadc0644c59d8::Adc0::from_ptr(0x4001_2000usize as _) };
pub const ADC2: gdadc0644c59d8::Adc0 = unsafe { gdadc0644c59d8::Adc0::from_ptr(0x4001_2100usize as _) };
pub const ADC3: gdadc0644c59d8::Adc0 = unsafe { gdadc0644c59d8::Adc0::from_ptr(0x4001_2200usize as _) };
pub const ADC_COMMON: gdadccommon6f53c1c8::AdcCommon =
    unsafe { gdadccommon6f53c1c8::AdcCommon::from_ptr(0x4001_2300usize as _) };
pub const SDIO: sdmmc::Sdmmc = unsafe { sdmmc::Sdmmc::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const SPI4: gdspi0a39abaa4::Spi0 = unsafe { gdspi0a39abaa4::Spi0::from_ptr(0x4001_3400usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_3800usize as _) };
pub const EXTI: gdexti2861ec2a::Exti = unsafe { gdexti2861ec2a::Exti::from_ptr(0x4001_3c00usize as _) };
pub const TIM9: timer::Tim2ch = unsafe { timer::Tim2ch::from_ptr(0x4001_4000usize as _) };
pub const TIM10: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4400usize as _) };
pub const TIM11: timer::Tim1ch = unsafe { timer::Tim1ch::from_ptr(0x4001_4800usize as _) };
pub const SPI5: gdspi0a39abaa4::Spi0 = unsafe { gdspi0a39abaa4::Spi0::from_ptr(0x4001_5000usize as _) };
pub const SPI6: gdspi528277832::Spi5 = unsafe { gdspi528277832::Spi5::from_ptr(0x4001_5400usize as _) };
pub const LTDC: gdtli3a8126bb::Tli = unsafe { gdtli3a8126bb::Tli::from_ptr(0x4001_6800usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1000usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1400usize as _) };
pub const GPIOG: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_1c00usize as _) };
pub const GPIOI: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_3800usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_3c00usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_6400usize as _) };
pub const ENET_MAC: gdenetmac93552dd1::EnetMac = unsafe { gdenetmac93552dd1::EnetMac::from_ptr(0x4002_8000usize as _) };
pub const ENET_MSC: gdenetmsc10390666::EnetMsc = unsafe { gdenetmsc10390666::EnetMsc::from_ptr(0x4002_8100usize as _) };
pub const ENET_PTP: gdenetptp5c8a2d48::EnetPtp = unsafe { gdenetptp5c8a2d48::EnetPtp::from_ptr(0x4002_8700usize as _) };
pub const ENET_DMA: gdenetdma7fbba2f4::EnetDma = unsafe { gdenetdma7fbba2f4::EnetDma::from_ptr(0x4002_9000usize as _) };
pub const ENET_MAC_FCTH: gdenetmacfcth8ada9e21::EnetMacFcth =
    unsafe { gdenetmacfcth8ada9e21::EnetMacFcth::from_ptr(0x4002_9080usize as _) };
pub const IPA: gdipae676fed9::Ipa = unsafe { gdipae676fed9::Ipa::from_ptr(0x4002_b000usize as _) };
pub const HS_GLOBAL: gdhsglobalc406147a::HsGlobal =
    unsafe { gdhsglobalc406147a::HsGlobal::from_ptr(0x4004_0000usize as _) };
pub const HS_HOST: gdhshostc2377b4a::HsHost = unsafe { gdhshostc2377b4a::HsHost::from_ptr(0x4004_0400usize as _) };
pub const HS_DEVICE: gdhsdevicec9d69f15::HsDevice =
    unsafe { gdhsdevicec9d69f15::HsDevice::from_ptr(0x4004_0800usize as _) };
pub const HS_PWRCLK: gdhspwrclk9376d26f::HsPwrclk =
    unsafe { gdhspwrclk9376d26f::HsPwrclk::from_ptr(0x4004_0e00usize as _) };
pub const FS_GLOBAL: gdfsglobale74e6f0e::FsGlobal =
    unsafe { gdfsglobale74e6f0e::FsGlobal::from_ptr(0x5000_0000usize as _) };
pub const FS_HOST: gdfshost44621b1c::FsHost = unsafe { gdfshost44621b1c::FsHost::from_ptr(0x5000_0400usize as _) };
pub const FS_DEVICE: gdfsdeviceb377b28b::FsDevice =
    unsafe { gdfsdeviceb377b28b::FsDevice::from_ptr(0x5000_0800usize as _) };
pub const FS_PWRCLK: gdfspwrclk87dcd48b::FsPwrclk =
    unsafe { gdfspwrclk87dcd48b::FsPwrclk::from_ptr(0x5000_0e00usize as _) };
pub const DCMI: dcmi::Dcmi = unsafe { dcmi::Dcmi::from_ptr(0x5005_0000usize as _) };
pub const RNG: gdtrngb48807ab::Trng = unsafe { gdtrngb48807ab::Trng::from_ptr(0x5006_0800usize as _) };
pub const FMC: gdexmc55cf46b0::Exmc = unsafe { gdexmc55cf46b0::Exmc::from_ptr(0xa000_0000usize as _) };
pub const DBGMCU: gddbg50e0203e::Dbg = unsafe { gddbg50e0203e::Dbg::from_ptr(0xe004_2000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v2.rs"]
pub mod dac;
#[path = "../../peripherals/dcmi_v1.rs"]
pub mod dcmi;
#[path = "../../peripherals/dma_v2.rs"]
pub mod dma;
#[path = "../../peripherals/flash_f2.rs"]
pub mod flash;
#[path = "../../peripherals/gdadc0644c59d8_v1.rs"]
pub mod gdadc0644c59d8;
#[path = "../../peripherals/gdadccommon6f53c1c8_v1.rs"]
pub mod gdadccommon6f53c1c8;
#[path = "../../peripherals/gdcan06b36baa3_v1.rs"]
pub mod gdcan06b36baa3;
#[path = "../../peripherals/gdctc47444a2c_v1.rs"]
pub mod gdctc47444a2c;
#[path = "../../peripherals/gddbg50e0203e_v1.rs"]
pub mod gddbg50e0203e;
#[path = "../../peripherals/gdenetdma7fbba2f4_v1.rs"]
pub mod gdenetdma7fbba2f4;
#[path = "../../peripherals/gdenetmac93552dd1_v1.rs"]
pub mod gdenetmac93552dd1;
#[path = "../../peripherals/gdenetmacfcth8ada9e21_v1.rs"]
pub mod gdenetmacfcth8ada9e21;
#[path = "../../peripherals/gdenetmsc10390666_v1.rs"]
pub mod gdenetmsc10390666;
#[path = "../../peripherals/gdenetptp5c8a2d48_v1.rs"]
pub mod gdenetptp5c8a2d48;
#[path = "../../peripherals/gdexmc55cf46b0_v1.rs"]
pub mod gdexmc55cf46b0;
#[path = "../../peripherals/gdexti2861ec2a_v1.rs"]
pub mod gdexti2861ec2a;
#[path = "../../peripherals/gdfsdeviceb377b28b_v1.rs"]
pub mod gdfsdeviceb377b28b;
#[path = "../../peripherals/gdfsglobale74e6f0e_v1.rs"]
pub mod gdfsglobale74e6f0e;
#[path = "../../peripherals/gdfshost44621b1c_v1.rs"]
pub mod gdfshost44621b1c;
#[path = "../../peripherals/gdfspwrclk87dcd48b_v1.rs"]
pub mod gdfspwrclk87dcd48b;
#[path = "../../peripherals/gdhsdevicec9d69f15_v1.rs"]
pub mod gdhsdevicec9d69f15;
#[path = "../../peripherals/gdhsglobalc406147a_v1.rs"]
pub mod gdhsglobalc406147a;
#[path = "../../peripherals/gdhshostc2377b4a_v1.rs"]
pub mod gdhshostc2377b4a;
#[path = "../../peripherals/gdhspwrclk9376d26f_v1.rs"]
pub mod gdhspwrclk9376d26f;
#[path = "../../peripherals/gdi2c0116537ab_v1.rs"]
pub mod gdi2c0116537ab;
#[path = "../../peripherals/gdipae676fed9_v1.rs"]
pub mod gdipae676fed9;
#[path = "../../peripherals/gdiref361590d6_v1.rs"]
pub mod gdiref361590d6;
#[path = "../../peripherals/gdrtc34bd68c7_v1.rs"]
pub mod gdrtc34bd68c7;
#[path = "../../peripherals/gdspi0a39abaa4_v1.rs"]
pub mod gdspi0a39abaa4;
#[path = "../../peripherals/gdspi528277832_v1.rs"]
pub mod gdspi528277832;
#[path = "../../peripherals/gdtimer519fda6d7_v1.rs"]
pub mod gdtimer519fda6d7;
#[path = "../../peripherals/gdtli3a8126bb_v1.rs"]
pub mod gdtli3a8126bb;
#[path = "../../peripherals/gdtrngb48807ab_v1.rs"]
pub mod gdtrngb48807ab;
#[path = "../../peripherals/gduart38ecaf091_v1.rs"]
pub mod gduart38ecaf091;
#[path = "../../peripherals/gdusart06fc75967_v1.rs"]
pub mod gdusart06fc75967;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_f2.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f2.rs"]
pub mod rcc;
#[path = "../../peripherals/sdmmc_v1.rs"]
pub mod sdmmc;
#[path = "../../peripherals/spi_v2_i2s.rs"]
pub mod spi;
#[path = "../../peripherals/syscfg_f2.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
