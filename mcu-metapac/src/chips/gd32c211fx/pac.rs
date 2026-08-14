




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - TIMESTAMP"]
TIMESTAMP = 1 , # [doc = "3 - FMC"]
FMC = 3 , # [doc = "4 - RCU"]
RCU = 4 , # [doc = "5 - EXTI0"]
EXTI0 = 5 , # [doc = "6 - EXTI1"]
EXTI1 = 6 , # [doc = "7 - EXTI2"]
EXTI2 = 7 , # [doc = "8 - EXTI3"]
EXTI3 = 8 , # [doc = "9 - EXTI4"]
EXTI4 = 9 , # [doc = "10 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 10 , # [doc = "11 - DMA_CHANNEL1"]
DMA_CHANNEL1 = 11 , # [doc = "12 - DMA_CHANNEL2"]
DMA_CHANNEL2 = 12 , # [doc = "13 - ADC"]
ADC = 13 , # [doc = "14 - USART0"]
USART0 = 14 , # [doc = "15 - USART1"]
USART1 = 15 , # [doc = "16 - USART2"]
USART2 = 16 , # [doc = "17 - I2C0_EV"]
I2C0_EV = 17 , # [doc = "18 - I2C0_ER"]
I2C0_ER = 18 , # [doc = "19 - I2C1_EV"]
I2C1_EV = 19 , # [doc = "20 - I2C1_ER"]
I2C1_ER = 20 , # [doc = "21 - SPI0"]
SPI0 = 21 , # [doc = "22 - SPI1"]
SPI1 = 22 , # [doc = "23 - RTC_ALARM"]
RTC_ALARM = 23 , # [doc = "24 - EXTI5_9"]
EXTI5_9 = 24 , # [doc = "25 - TIMER0_TRG_CMT_UP_BRK"]
TIMER0_TRG_CMT_UP_BRK = 25 , # [doc = "26 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 26 , # [doc = "27 - TIMER2"]
TIMER2 = 27 , # [doc = "28 - TIMER13"]
TIMER13 = 28 , # [doc = "29 - TIMER15"]
TIMER15 = 29 , # [doc = "30 - TIMER16"]
TIMER16 = 30 , # [doc = "31 - EXTI10_15"]
EXTI10_15 = 31 , # [doc = "33 - DMAMUX"]
DMAMUX = 33 , # [doc = "34 - CMP0"]
CMP0 = 34 , # [doc = "35 - CMP1"]
CMP1 = 35 , # [doc = "36 - I2C0_WKUP"]
I2C0_WKUP = 36 , # [doc = "37 - I2C1_WKUP"]
I2C1_WKUP = 37 , # [doc = "38 - USART0_WKUP"]
USART0_WKUP = 38 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn TIMESTAMP () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1 () ; fn DMA_CHANNEL2 () ; fn ADC () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn RTC_ALARM () ; fn EXTI5_9 () ; fn TIMER0_TRG_CMT_UP_BRK () ; fn TIMER0_CHANNEL () ; fn TIMER2 () ; fn TIMER13 () ; fn TIMER15 () ; fn TIMER16 () ; fn EXTI10_15 () ; fn DMAMUX () ; fn CMP0 () ; fn CMP1 () ; fn I2C0_WKUP () ; fn I2C1_WKUP () ; fn USART0_WKUP () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 39]
= [Vector { _handler : WWDGT } , Vector { _handler : TIMESTAMP } , Vector { _reserved : 0 } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1 } , Vector { _handler : DMA_CHANNEL2 } , Vector { _handler : ADC } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : RTC_ALARM } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_TRG_CMT_UP_BRK } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : EXTI10_15 } , Vector { _reserved : 0 } , Vector { _handler : DMAMUX } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : I2C0_WKUP } , Vector { _handler : I2C1_WKUP } , Vector { _handler : USART0_WKUP } ,]
; } pub const TIMER2 : gdtimer46598974 :: Timer = unsafe { gdtimer46598974 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER13 : gdtimer46598974 :: Timer = unsafe { gdtimer46598974 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtce6f00754 :: Rtc = unsafe { gdrtce6f00754 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgtf694703e :: Wwdgt = unsafe { gdwwdgtf694703e :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi3e72f252 :: Spi = unsafe { gdspi3e72f252 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart34eadece :: Usart = unsafe { gdusart34eadece :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart34eadece :: Usart = unsafe { gdusart34eadece :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu4410be00 :: Pmu = unsafe { gdpmu4410be00 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const SYSCFG : gdsyscfgf7d28bd5 :: Syscfg = unsafe { gdsyscfgf7d28bd5 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextid7fe0966 :: Exti = unsafe { gdextid7fe0966 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadca6e20c2c :: Adc = unsafe { gdadca6e20c2c :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer46598974 :: Timer = unsafe { gdtimer46598974 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi3e72f252 :: Spi = unsafe { gdspi3e72f252 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart34eadece :: Usart = unsafe { gdusart34eadece :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER15 : gdtimer46598974 :: Timer = unsafe { gdtimer46598974 :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer46598974 :: Timer = unsafe { gdtimer46598974 :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const DBG : gddbgc0a1e7bf :: Dbg = unsafe { gddbgc0a1e7bf :: Dbg :: from_ptr (0x4001_5800usize as _) } ; pub const CMP : gdcmp13366a93 :: Cmp = unsafe { gdcmp13366a93 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const DMA : gddma35e38e2e :: Dma = unsafe { gddma35e38e2e :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA_CHXCTL_BASE : gddmachxctlbase70e585cf :: DmaChxctlBase = unsafe { gddmachxctlbase70e585cf :: DmaChxctlBase :: from_ptr (0x4002_0008usize as _) } ; pub const DMA_CHXCNT_BASE : gddmachxcntbase65c5fd05 :: DmaChxcntBase = unsafe { gddmachxcntbase65c5fd05 :: DmaChxcntBase :: from_ptr (0x4002_000cusize as _) } ; pub const DMA_CHXPADDR_BASE : gddmachxpaddrbasea8e95eb7 :: DmaChxpaddrBase = unsafe { gddmachxpaddrbasea8e95eb7 :: DmaChxpaddrBase :: from_ptr (0x4002_0010usize as _) } ; pub const DMA_CHXMADDR_BASE : gddmachxmaddrbase5e865b4b :: DmaChxmaddrBase = unsafe { gddmachxmaddrbase5e865b4b :: DmaChxmaddrBase :: from_ptr (0x4002_0014usize as _) } ; pub const DMAMUX : gddmamux3017f39e :: Dmamux = unsafe { gddmamux3017f39e :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbasebc255481 :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbasebc255481 :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcuffcd57c8 :: Rcu = unsafe { gdrcuffcd57c8 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmccf889ee9 :: Fmc = unsafe { gdfmccf889ee9 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpiod3b44485 :: Gpio = unsafe { gdgpiod3b44485 :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiod3b44485 :: Gpio = unsafe { gdgpiod3b44485 :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpiod3b44485 :: Gpio = unsafe { gdgpiod3b44485 :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiod3b44485 :: Gpio = unsafe { gdgpiod3b44485 :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiod3b44485 :: Gpio = unsafe { gdgpiod3b44485 :: Gpio :: from_ptr (0x4800_1400usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadca6e20c2c_v1.rs"] pub mod gdadca6e20c2c;
#[path="../../peripherals/gdcmp13366a93_v1.rs"] pub mod gdcmp13366a93;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gddbgc0a1e7bf_v1.rs"] pub mod gddbgc0a1e7bf;
#[path="../../peripherals/gddma35e38e2e_v1.rs"] pub mod gddma35e38e2e;
#[path="../../peripherals/gddmachxcntbase65c5fd05_v1.rs"] pub mod gddmachxcntbase65c5fd05;
#[path="../../peripherals/gddmachxctlbase70e585cf_v1.rs"] pub mod gddmachxctlbase70e585cf;
#[path="../../peripherals/gddmachxmaddrbase5e865b4b_v1.rs"] pub mod gddmachxmaddrbase5e865b4b;
#[path="../../peripherals/gddmachxpaddrbasea8e95eb7_v1.rs"] pub mod gddmachxpaddrbasea8e95eb7;
#[path="../../peripherals/gddmamux3017f39e_v1.rs"] pub mod gddmamux3017f39e;
#[path="../../peripherals/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../../peripherals/gddmamuxrmchxcfgbasebc255481_v1.rs"] pub mod gddmamuxrmchxcfgbasebc255481;
#[path="../../peripherals/gdextid7fe0966_v1.rs"] pub mod gdextid7fe0966;
#[path="../../peripherals/gdfmccf889ee9_v1.rs"] pub mod gdfmccf889ee9;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpiod3b44485_v1.rs"] pub mod gdgpiod3b44485;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdpmu4410be00_v1.rs"] pub mod gdpmu4410be00;
#[path="../../peripherals/gdrcuffcd57c8_v1.rs"] pub mod gdrcuffcd57c8;
#[path="../../peripherals/gdrtce6f00754_v1.rs"] pub mod gdrtce6f00754;
#[path="../../peripherals/gdspi3e72f252_v1.rs"] pub mod gdspi3e72f252;
#[path="../../peripherals/gdsyscfgf7d28bd5_v1.rs"] pub mod gdsyscfgf7d28bd5;
#[path="../../peripherals/gdtimer46598974_v1.rs"] pub mod gdtimer46598974;
#[path="../../peripherals/gdusart34eadece_v1.rs"] pub mod gdusart34eadece;
#[path="../../peripherals/gdwwdgtf694703e_v1.rs"] pub mod gdwwdgtf694703e;
