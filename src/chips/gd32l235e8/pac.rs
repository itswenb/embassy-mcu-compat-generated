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
    #[doc = "18 - ADC1"]
    ADC1 = 18,
    #[doc = "19 - USBD_HP_CAN_TX"]
    USBD_HP_CAN_TX = 19,
    #[doc = "20 - USBD_LP_CAN_RX0"]
    USBD_LP_CAN_RX0 = 20,
    #[doc = "21 - TIM2"]
    TIM2 = 21,
    #[doc = "22 - TIM3"]
    TIM3 = 22,
    #[doc = "23 - TIM9"]
    TIM9 = 23,
    #[doc = "24 - TIM12"]
    TIM12 = 24,
    #[doc = "25 - TIM6"]
    TIM6 = 25,
    #[doc = "26 - TIM7"]
    TIM7 = 26,
    #[doc = "27 - USART1"]
    USART1 = 27,
    #[doc = "28 - USART2"]
    USART2 = 28,
    #[doc = "29 - UART4"]
    UART4 = 29,
    #[doc = "30 - UART5"]
    UART5 = 30,
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
    #[doc = "37 - DAC1"]
    DAC1 = 37,
    #[doc = "39 - I2C3_EV"]
    I2C3_EV = 39,
    #[doc = "40 - I2C3_ER"]
    I2C3_ER = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "42 - USBD_WKUP"]
    USBD_WKUP = 42,
    #[doc = "43 - EXTI5_9"]
    EXTI5_9 = 43,
    #[doc = "44 - TIM1_TRG_COM_UP_BRK"]
    TIM1_TRG_COM_UP_BRK = 44,
    #[doc = "45 - TIM1_CC"]
    TIM1_CC = 45,
    #[doc = "46 - TIM15"]
    TIM15 = 46,
    #[doc = "47 - EXTI10_15"]
    EXTI10_15 = 47,
    #[doc = "48 - TIM41"]
    TIM41 = 48,
    #[doc = "49 - CAN_RX1"]
    CAN_RX1 = 49,
    #[doc = "50 - CAN_EWMC"]
    CAN_EWMC = 50,
    #[doc = "55 - DMAMUX1"]
    DMAMUX1 = 55,
    #[doc = "56 - CMP0"]
    CMP0 = 56,
    #[doc = "57 - CMP1"]
    CMP1 = 57,
    #[doc = "58 - I2C1_WKUP"]
    I2C1_WKUP = 58,
    #[doc = "59 - I2C3_WKUP"]
    I2C3_WKUP = 59,
    #[doc = "60 - USART1_WKUP"]
    USART1_WKUP = 60,
    #[doc = "61 - LPUART1"]
    LPUART1 = 61,
    #[doc = "62 - CAU"]
    CAU = 62,
    #[doc = "63 - RNG"]
    RNG = 63,
    #[doc = "64 - SLCD"]
    SLCD = 64,
    #[doc = "65 - USART2_WKUP"]
    USART2_WKUP = 65,
    #[doc = "66 - I2C2_WKUP"]
    I2C2_WKUP = 66,
    #[doc = "67 - LPUART1_WKUP"]
    LPUART1_WKUP = 67,
    #[doc = "68 - LPTIM1"]
    LPTIM1 = 68,
    #[doc = "69 - LPUART2_WKUP"]
    LPUART2_WKUP = 69,
    #[doc = "70 - LPTIM2"]
    LPTIM2 = 70,
    #[doc = "71 - LPUART2"]
    LPUART2 = 71,
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
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1();
        fn USBD_HP_CAN_TX();
        fn USBD_LP_CAN_RX0();
        fn TIM2();
        fn TIM3();
        fn TIM9();
        fn TIM12();
        fn TIM6();
        fn TIM7();
        fn USART1();
        fn USART2();
        fn UART4();
        fn UART5();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn DAC1();
        fn I2C3_EV();
        fn I2C3_ER();
        fn RTC_ALARM();
        fn USBD_WKUP();
        fn EXTI5_9();
        fn TIM1_TRG_COM_UP_BRK();
        fn TIM1_CC();
        fn TIM15();
        fn EXTI10_15();
        fn TIM41();
        fn CAN_RX1();
        fn CAN_EWMC();
        fn DMAMUX1();
        fn CMP0();
        fn CMP1();
        fn I2C1_WKUP();
        fn I2C3_WKUP();
        fn USART1_WKUP();
        fn LPUART1();
        fn CAU();
        fn RNG();
        fn SLCD();
        fn USART2_WKUP();
        fn I2C2_WKUP();
        fn LPUART1_WKUP();
        fn LPTIM1();
        fn LPUART2_WKUP();
        fn LPTIM2();
        fn LPUART2();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 72] = [
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
        Vector { _handler: ADC1 },
        Vector {
            _handler: USBD_HP_CAN_TX,
        },
        Vector {
            _handler: USBD_LP_CAN_RX0,
        },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM9 },
        Vector { _handler: TIM12 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: DAC1 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: RTC_ALARM },
        Vector { _handler: USBD_WKUP },
        Vector { _handler: EXTI5_9 },
        Vector {
            _handler: TIM1_TRG_COM_UP_BRK,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM15 },
        Vector { _handler: EXTI10_15 },
        Vector { _handler: TIM41 },
        Vector { _handler: CAN_RX1 },
        Vector { _handler: CAN_EWMC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMAMUX1 },
        Vector { _handler: CMP0 },
        Vector { _handler: CMP1 },
        Vector { _handler: I2C1_WKUP },
        Vector { _handler: I2C3_WKUP },
        Vector { _handler: USART1_WKUP },
        Vector { _handler: LPUART1 },
        Vector { _handler: CAU },
        Vector { _handler: RNG },
        Vector { _handler: SLCD },
        Vector { _handler: USART2_WKUP },
        Vector { _handler: I2C2_WKUP },
        Vector { _handler: LPUART1_WKUP },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPUART2_WKUP },
        Vector { _handler: LPTIM2 },
        Vector { _handler: LPUART2 },
    ];
}
pub const TIM2: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp16 = unsafe { timer::TimGp16::from_ptr(0x4000_0400usize as _) };
pub const TIM6: gdtimer5183dba8f::Timer5 = unsafe { gdtimer5183dba8f::Timer5::from_ptr(0x4000_1000usize as _) };
pub const TIM7: gdtimer6b465bb6b::Timer6 = unsafe { gdtimer6b465bb6b::Timer6::from_ptr(0x4000_1400usize as _) };
pub const TIM12: gdtimer8dfb2bdb4::Timer8 = unsafe { gdtimer8dfb2bdb4::Timer8::from_ptr(0x4000_1800usize as _) };
pub const SLCD: gdslcd6dc6af89::Slcd = unsafe { gdslcd6dc6af89::Slcd::from_ptr(0x4000_2400usize as _) };
pub const RTC: gdrtc30fffb52::Rtc = unsafe { gdrtc30fffb52::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: gdwwdgtdd622579::Wwdgt = unsafe { gdwwdgtdd622579::Wwdgt::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: gdspi19358bf74::Spi1 = unsafe { gdspi19358bf74::Spi1::from_ptr(0x4000_3800usize as _) };
pub const USART2: gdusart0184abb20::Usart0 = unsafe { gdusart0184abb20::Usart0::from_ptr(0x4000_4400usize as _) };
pub const LPUART2: gdlpuart058954338::Lpuart0 = unsafe { gdlpuart058954338::Lpuart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: gduart37add471e::Uart3 = unsafe { gduart37add471e::Uart3::from_ptr(0x4000_4c00usize as _) };
pub const UART5: gduart47d66af8a::Uart4 = unsafe { gduart47d66af8a::Uart4::from_ptr(0x4000_5000usize as _) };
pub const I2C1: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_5800usize as _) };
pub const USBD: gdusbd3c6a50b5::Usbd = unsafe { gdusbd3c6a50b5::Usbd::from_ptr(0x4000_5c00usize as _) };
pub const CAN: gdcanf0c54386::Can = unsafe { gdcanf0c54386::Can::from_ptr(0x4000_6400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC1: gddac7e57a629::Dac = unsafe { gddac7e57a629::Dac::from_ptr(0x4000_7400usize as _) };
pub const LPTIM2: gdlptimer0fade8b7a::Lptimer0 =
    unsafe { gdlptimer0fade8b7a::Lptimer0::from_ptr(0x4000_7c00usize as _) };
pub const LPUART1: gdlpuart058954338::Lpuart0 = unsafe { gdlpuart058954338::Lpuart0::from_ptr(0x4000_8000usize as _) };
pub const LPTIM1: gdlptimer0fade8b7a::Lptimer0 =
    unsafe { gdlptimer0fade8b7a::Lptimer0::from_ptr(0x4000_9400usize as _) };
pub const I2C3: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_c000usize as _) };
pub const CRS: gdctceaaaf458::Ctc = unsafe { gdctceaaaf458::Ctc::from_ptr(0x4000_c800usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4001_0000usize as _) };
pub const VREF: gdvreff6814bb8::Vref = unsafe { gdvreff6814bb8::Vref::from_ptr(0x4001_0030usize as _) };
pub const EXTI: gdexti30fc9668::Exti = unsafe { gdexti30fc9668::Exti::from_ptr(0x4001_0400usize as _) };
pub const ADC1: gdadca4c861d7::Adc = unsafe { gdadca4c861d7::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: gdspi0cf000376::Spi0 = unsafe { gdspi0cf000376::Spi0::from_ptr(0x4001_3000usize as _) };
pub const USART1: gdusart0184abb20::Usart0 = unsafe { gdusart0184abb20::Usart0::from_ptr(0x4001_3800usize as _) };
pub const TIM15: gdtimer14452fee2b::Timer14 = unsafe { gdtimer14452fee2b::Timer14::from_ptr(0x4001_4000usize as _) };
pub const TIM9: gdtimer8dfb2bdb4::Timer8 = unsafe { gdtimer8dfb2bdb4::Timer8::from_ptr(0x4001_4c00usize as _) };
pub const DBGMCU: gddbgmcu02036f49::Dbgmcu = unsafe { gddbgmcu02036f49::Dbgmcu::from_ptr(0x4001_5800usize as _) };
pub const CMP: gdcmpd90af10b::Cmp = unsafe { gdcmpd90af10b::Cmp::from_ptr(0x4001_7c00usize as _) };
pub const TIM41: gdtimer14452fee2b::Timer14 = unsafe { gdtimer14452fee2b::Timer14::from_ptr(0x4001_d000usize as _) };
pub const DMA1: bdma::Dma = unsafe { bdma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMAMUX1: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: gdcrc67d273cb::Crc = unsafe { gdcrc67d273cb::Crc::from_ptr(0x4002_3000usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4800_0400usize as _) };
pub const GPIOC: gdgpioc0fba06c4::Gpioc = unsafe { gdgpioc0fba06c4::Gpioc::from_ptr(0x4800_0800usize as _) };
pub const GPIOD: gdgpiod7229d923::Gpiod = unsafe { gdgpiod7229d923::Gpiod::from_ptr(0x4800_0c00usize as _) };
pub const GPIOF: gdgpiof7c6237df::Gpiof = unsafe { gdgpiof7c6237df::Gpiof::from_ptr(0x4800_1400usize as _) };
pub const CAU: gdcaue9e51f0c::Cau = unsafe { gdcaue9e51f0c::Cau::from_ptr(0x5006_0000usize as _) };
pub const RNG: gdtrngbf61c352::Trng = unsafe { gdtrngbf61c352::Trng::from_ptr(0x5006_0800usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/bdma_v1.rs"]
pub mod bdma;
#[path = "../../peripherals/dmamux_gd752f98b8d3cb.rs"]
pub mod dmamux;
#[path = "../../peripherals/flash_g0x1.rs"]
pub mod flash;
#[path = "../../peripherals/gdadca4c861d7_v1.rs"]
pub mod gdadca4c861d7;
#[path = "../../peripherals/gdcanf0c54386_v1.rs"]
pub mod gdcanf0c54386;
#[path = "../../peripherals/gdcaue9e51f0c_v1.rs"]
pub mod gdcaue9e51f0c;
#[path = "../../peripherals/gdcmpd90af10b_v1.rs"]
pub mod gdcmpd90af10b;
#[path = "../../peripherals/gdcrc67d273cb_v1.rs"]
pub mod gdcrc67d273cb;
#[path = "../../peripherals/gdctceaaaf458_v1.rs"]
pub mod gdctceaaaf458;
#[path = "../../peripherals/gddac7e57a629_v1.rs"]
pub mod gddac7e57a629;
#[path = "../../peripherals/gddbgmcu02036f49_v1.rs"]
pub mod gddbgmcu02036f49;
#[path = "../../peripherals/gdexti30fc9668_v1.rs"]
pub mod gdexti30fc9668;
#[path = "../../peripherals/gdgpioc0fba06c4_v1.rs"]
pub mod gdgpioc0fba06c4;
#[path = "../../peripherals/gdgpiod7229d923_v1.rs"]
pub mod gdgpiod7229d923;
#[path = "../../peripherals/gdgpiof7c6237df_v1.rs"]
pub mod gdgpiof7c6237df;
#[path = "../../peripherals/gdi2c0cd973dc4_v1.rs"]
pub mod gdi2c0cd973dc4;
#[path = "../../peripherals/gdlptimer0fade8b7a_v1.rs"]
pub mod gdlptimer0fade8b7a;
#[path = "../../peripherals/gdlpuart058954338_v1.rs"]
pub mod gdlpuart058954338;
#[path = "../../peripherals/gdrtc30fffb52_v1.rs"]
pub mod gdrtc30fffb52;
#[path = "../../peripherals/gdslcd6dc6af89_v1.rs"]
pub mod gdslcd6dc6af89;
#[path = "../../peripherals/gdspi0cf000376_v1.rs"]
pub mod gdspi0cf000376;
#[path = "../../peripherals/gdspi19358bf74_v1.rs"]
pub mod gdspi19358bf74;
#[path = "../../peripherals/gdtimer14452fee2b_v1.rs"]
pub mod gdtimer14452fee2b;
#[path = "../../peripherals/gdtimer5183dba8f_v1.rs"]
pub mod gdtimer5183dba8f;
#[path = "../../peripherals/gdtimer6b465bb6b_v1.rs"]
pub mod gdtimer6b465bb6b;
#[path = "../../peripherals/gdtimer8dfb2bdb4_v1.rs"]
pub mod gdtimer8dfb2bdb4;
#[path = "../../peripherals/gdtrngbf61c352_v1.rs"]
pub mod gdtrngbf61c352;
#[path = "../../peripherals/gduart37add471e_v1.rs"]
pub mod gduart37add471e;
#[path = "../../peripherals/gduart47d66af8a_v1.rs"]
pub mod gduart47d66af8a;
#[path = "../../peripherals/gdusart0184abb20_v1.rs"]
pub mod gdusart0184abb20;
#[path = "../../peripherals/gdusbd3c6a50b5_v1.rs"]
pub mod gdusbd3c6a50b5;
#[path = "../../peripherals/gdvreff6814bb8_v1.rs"]
pub mod gdvreff6814bb8;
#[path = "../../peripherals/gdwwdgtdd622579_v1.rs"]
pub mod gdwwdgtdd622579;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v2.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_g0.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_g0x1.rs"]
pub mod rcc;
#[path = "../../peripherals/syscfg_g0.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
