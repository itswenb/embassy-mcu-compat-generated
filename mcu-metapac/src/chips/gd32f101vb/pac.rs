

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU"]
RCU = 5 , # [doc = "6 - EXTI_LINE0"]
EXTI_LINE0 = 6 , # [doc = "7 - EXTI_LINE1"]
EXTI_LINE1 = 7 , # [doc = "8 - EXTI_LINE2"]
EXTI_LINE2 = 8 , # [doc = "9 - EXTI_LINE3"]
EXTI_LINE3 = 9 , # [doc = "10 - EXTI_LINE4"]
EXTI_LINE4 = 10 , # [doc = "11 - DMA0_CHANNEL0"]
DMA0_CHANNEL0 = 11 , # [doc = "12 - DMA0_CHANNEL1"]
DMA0_CHANNEL1 = 12 , # [doc = "13 - DMA0_CHANNEL2"]
DMA0_CHANNEL2 = 13 , # [doc = "14 - DMA0_CHANNEL3"]
DMA0_CHANNEL3 = 14 , # [doc = "15 - DMA0_CHANNEL4"]
DMA0_CHANNEL4 = 15 , # [doc = "16 - DMA0_CHANNEL5"]
DMA0_CHANNEL5 = 16 , # [doc = "17 - DMA0_CHANNEL6"]
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "19 - USBD_HP_CAN0_TX"]
USBD_HP_CAN0_TX = 19 , # [doc = "20 - USBD_LP_CAN0_RX0"]
USBD_LP_CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER2"]
TIMER2 = 29 , # [doc = "30 - TIMER3"]
TIMER3 = 30 , # [doc = "31 - I2C0_EV"]
I2C0_EV = 31 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "33 - I2C1_EV"]
I2C1_EV = 33 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - SPI0"]
SPI0 = 35 , # [doc = "36 - SPI1"]
SPI1 = 36 , # [doc = "37 - USART0"]
USART0 = 37 , # [doc = "38 - USART1"]
USART1 = 38 , # [doc = "39 - USART2"]
USART2 = 39 , # [doc = "40 - EXTI_LINE15_10"]
EXTI_LINE15_10 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "48 - EXMC"]
EXMC = 48 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn USBD_HP_CAN0_TX () ; fn USBD_LP_CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn EXMC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 49]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : USBD_HP_CAN0_TX } , Vector { _handler : USBD_LP_CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXMC } ,]
; } pub const TIMER1 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0800usize as _) } ; pub const RTC : gdrtcb40ef71d :: Rtc = unsafe { gdrtcb40ef71d :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtdc3d0d7a :: Fwdgt = unsafe { gdfwdgtdc3d0d7a :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi092308ad1 :: Spi0 = unsafe { gdspi092308ad1 :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart08d85785f :: Usart0 = unsafe { gdusart08d85785f :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart08d85785f :: Usart0 = unsafe { gdusart08d85785f :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c08f648655 :: I2c0 = unsafe { gdi2c08f648655 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c08f648655 :: I2c0 = unsafe { gdi2c08f648655 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const USBD : gdusbd3a06bc1e :: Usbd = unsafe { gdusbd3a06bc1e :: Usbd :: from_ptr (0x4000_5c00usize as _) } ; pub const CAN : gdcand17d981d :: Can = unsafe { gdcand17d981d :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const BKP : gdbkpd7dc7210 :: Bkp = unsafe { gdbkpd7dc7210 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu0a98243f :: Pmu = unsafe { gdpmu0a98243f :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac4621750f :: Dac = unsafe { gddac4621750f :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const AFIO : gdafio15610405 :: Afio = unsafe { gdafio15610405 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti11a1be47 :: Exti = unsafe { gdexti11a1be47 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadc0dda18ebe :: Adc0 = unsafe { gdadc0dda18ebe :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc134a2b2fe :: Adc1 = unsafe { gdadc134a2b2fe :: Adc1 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0e084a927 :: Timer0 = unsafe { gdtimer0e084a927 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi092308ad1 :: Spi0 = unsafe { gdspi092308ad1 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart08d85785f :: Usart0 = unsafe { gdusart08d85785f :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const DMA0 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const RCU : gdrcud84cce9d :: Rcu = unsafe { gdrcud84cce9d :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcd9f4c928 :: Fmc = unsafe { gdfmcd9f4c928 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc66a4f78d :: Crc = unsafe { gdcrc66a4f78d :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const EXMC : gdexmc61eab9d1 :: Exmc = unsafe { gdexmc61eab9d1 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg40666257 :: Dbg = unsafe { gddbg40666257 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc0dda18ebe_v1.rs"] pub mod gdadc0dda18ebe;
#[path="../../peripherals/gdadc134a2b2fe_v1.rs"] pub mod gdadc134a2b2fe;
#[path="../../peripherals/gdafio15610405_v1.rs"] pub mod gdafio15610405;
#[path="../../peripherals/gdbkpd7dc7210_v1.rs"] pub mod gdbkpd7dc7210;
#[path="../../peripherals/gdcand17d981d_v1.rs"] pub mod gdcand17d981d;
#[path="../../peripherals/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../../peripherals/gddac4621750f_v1.rs"] pub mod gddac4621750f;
#[path="../../peripherals/gddbg40666257_v1.rs"] pub mod gddbg40666257;
#[path="../../peripherals/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../../peripherals/gdexmc61eab9d1_v1.rs"] pub mod gdexmc61eab9d1;
#[path="../../peripherals/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../../peripherals/gdfmcd9f4c928_v1.rs"] pub mod gdfmcd9f4c928;
#[path="../../peripherals/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../../peripherals/gdgpioa979b0f67_v1.rs"] pub mod gdgpioa979b0f67;
#[path="../../peripherals/gdi2c08f648655_v1.rs"] pub mod gdi2c08f648655;
#[path="../../peripherals/gdpmu0a98243f_v1.rs"] pub mod gdpmu0a98243f;
#[path="../../peripherals/gdrcud84cce9d_v1.rs"] pub mod gdrcud84cce9d;
#[path="../../peripherals/gdrtcb40ef71d_v1.rs"] pub mod gdrtcb40ef71d;
#[path="../../peripherals/gdspi092308ad1_v1.rs"] pub mod gdspi092308ad1;
#[path="../../peripherals/gdtimer0e084a927_v1.rs"] pub mod gdtimer0e084a927;
#[path="../../peripherals/gdtimer1974d22f3_v1.rs"] pub mod gdtimer1974d22f3;
#[path="../../peripherals/gdusart08d85785f_v1.rs"] pub mod gdusart08d85785f;
#[path="../../peripherals/gdusbd3a06bc1e_v1.rs"] pub mod gdusbd3a06bc1e;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
