

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - RTC_TAMPER_TIMESTAMP"]
RTC_TAMPER_TIMESTAMP = 1 , # [doc = "2 - RTC_WAKE"]
RTC_WAKE = 2 , # [doc = "3 - FMC_GLOBAL"]
FMC_GLOBAL = 3 , # [doc = "5 - EXTI0"]
EXTI0 = 5 , # [doc = "6 - EXTI01"]
EXTI01 = 6 , # [doc = "7 - EXTI2"]
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
EXTI5_9 = 24 , # [doc = "25 - TIMER0_TRIG_UP_BREAK"]
TIMER0_TRIG_UP_BREAK = 25 , # [doc = "26 - TIMER0_CAP"]
TIMER0_CAP = 26 , # [doc = "27 - TIMER2"]
TIMER2 = 27 , # [doc = "28 - TIMER13"]
TIMER13 = 28 , # [doc = "29 - TIMER15"]
TIMER15 = 29 , # [doc = "30 - TIMER16"]
TIMER16 = 30 , # [doc = "31 - EXTI10_15"]
EXTI10_15 = 31 , # [doc = "33 - DMA_MUX"]
DMA_MUX = 33 , # [doc = "34 - CMP0"]
CMP0 = 34 , # [doc = "35 - CMP1"]
CMP1 = 35 , # [doc = "36 - I2C0_WAKE"]
I2C0_WAKE = 36 , # [doc = "37 - I2C1_WAKE"]
I2C1_WAKE = 37 , # [doc = "38 - USART0_WAKE"]
USART0_WAKE = 38 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn RTC_TAMPER_TIMESTAMP () ; fn RTC_WAKE () ; fn FMC_GLOBAL () ; fn EXTI0 () ; fn EXTI01 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1 () ; fn DMA_CHANNEL2 () ; fn ADC () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn RTC_ALARM () ; fn EXTI5_9 () ; fn TIMER0_TRIG_UP_BREAK () ; fn TIMER0_CAP () ; fn TIMER2 () ; fn TIMER13 () ; fn TIMER15 () ; fn TIMER16 () ; fn EXTI10_15 () ; fn DMA_MUX () ; fn CMP0 () ; fn CMP1 () ; fn I2C0_WAKE () ; fn I2C1_WAKE () ; fn USART0_WAKE () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 39]
= [Vector { _handler : WWDGT } , Vector { _handler : RTC_TAMPER_TIMESTAMP } , Vector { _handler : RTC_WAKE } , Vector { _handler : FMC_GLOBAL } , Vector { _reserved : 0 } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI01 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1 } , Vector { _handler : DMA_CHANNEL2 } , Vector { _handler : ADC } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : RTC_ALARM } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_TRIG_UP_BREAK } , Vector { _handler : TIMER0_CAP } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : EXTI10_15 } , Vector { _reserved : 0 } , Vector { _handler : DMA_MUX } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : I2C0_WAKE } , Vector { _handler : I2C1_WAKE } , Vector { _handler : USART0_WAKE } ,]
; } pub const TIMER2 : gdtimer270dbabdc :: Timer2 = unsafe { gdtimer270dbabdc :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER13 : gdtimer13b6ee86d8 :: Timer13 = unsafe { gdtimer13b6ee86d8 :: Timer13 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtcadbe90b4 :: Rtc = unsafe { gdrtcadbe90b4 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgtdd622579 :: Wwdgt = unsafe { gdwwdgtdd622579 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt5932fb56 :: Fwdgt = unsafe { gdfwdgt5932fb56 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi187e06d26 :: Spi1 = unsafe { gdspi187e06d26 :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart13d9e0c6f :: Usart1 = unsafe { gdusart13d9e0c6f :: Usart1 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart13d9e0c6f :: Usart1 = unsafe { gdusart13d9e0c6f :: Usart1 :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu5c455f73 :: Pmu = unsafe { gdpmu5c455f73 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const SYSCFG : gdsyscfg4a40a7d3 :: Syscfg = unsafe { gdsyscfg4a40a7d3 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti0b771307 :: Exti = unsafe { gdexti0b771307 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadcf283e75f :: Adc = unsafe { gdadcf283e75f :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer0533ef489 :: Timer0 = unsafe { gdtimer0533ef489 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi058ca27a4 :: Spi0 = unsafe { gdspi058ca27a4 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart0a0301eea :: Usart0 = unsafe { gdusart0a0301eea :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER15 : gdtimer15a386f15f :: Timer15 = unsafe { gdtimer15a386f15f :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer15a386f15f :: Timer15 = unsafe { gdtimer15a386f15f :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const DBG : gddbg63bf1c6a :: Dbg = unsafe { gddbg63bf1c6a :: Dbg :: from_ptr (0x4001_5800usize as _) } ; pub const CMP : gdcmp90f31e19 :: Cmp = unsafe { gdcmp90f31e19 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const DMA : gddma4ef405a0 :: Dma = unsafe { gddma4ef405a0 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMAMUX : gddmamux6e5e79f5 :: Dmamux = unsafe { gddmamux6e5e79f5 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcu53e64d61 :: Rcu = unsafe { gdrcu53e64d61 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcfda27991 :: Fmc = unsafe { gdfmcfda27991 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrcc5aea4f6 :: Crc = unsafe { gdcrcc5aea4f6 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpioaf444c8f9 :: Gpioa = unsafe { gdgpioaf444c8f9 :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob6efbc75f :: Gpiob = unsafe { gdgpiob6efbc75f :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpiob6efbc75f :: Gpiob = unsafe { gdgpiob6efbc75f :: Gpiob :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiob6efbc75f :: Gpiob = unsafe { gdgpiob6efbc75f :: Gpiob :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiob6efbc75f :: Gpiob = unsafe { gdgpiob6efbc75f :: Gpiob :: from_ptr (0x4800_1400usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcf283e75f_v1.rs"] pub mod gdadcf283e75f;
#[path="../../peripherals/gdcmp90f31e19_v1.rs"] pub mod gdcmp90f31e19;
#[path="../../peripherals/gdcrcc5aea4f6_v1.rs"] pub mod gdcrcc5aea4f6;
#[path="../../peripherals/gddbg63bf1c6a_v1.rs"] pub mod gddbg63bf1c6a;
#[path="../../peripherals/gddma4ef405a0_v1.rs"] pub mod gddma4ef405a0;
#[path="../../peripherals/gddmamux6e5e79f5_v1.rs"] pub mod gddmamux6e5e79f5;
#[path="../../peripherals/gdexti0b771307_v1.rs"] pub mod gdexti0b771307;
#[path="../../peripherals/gdfmcfda27991_v1.rs"] pub mod gdfmcfda27991;
#[path="../../peripherals/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../../peripherals/gdgpioaf444c8f9_v1.rs"] pub mod gdgpioaf444c8f9;
#[path="../../peripherals/gdgpiob6efbc75f_v1.rs"] pub mod gdgpiob6efbc75f;
#[path="../../peripherals/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../../peripherals/gdpmu5c455f73_v1.rs"] pub mod gdpmu5c455f73;
#[path="../../peripherals/gdrcu53e64d61_v1.rs"] pub mod gdrcu53e64d61;
#[path="../../peripherals/gdrtcadbe90b4_v1.rs"] pub mod gdrtcadbe90b4;
#[path="../../peripherals/gdspi058ca27a4_v1.rs"] pub mod gdspi058ca27a4;
#[path="../../peripherals/gdspi187e06d26_v1.rs"] pub mod gdspi187e06d26;
#[path="../../peripherals/gdsyscfg4a40a7d3_v1.rs"] pub mod gdsyscfg4a40a7d3;
#[path="../../peripherals/gdtimer0533ef489_v1.rs"] pub mod gdtimer0533ef489;
#[path="../../peripherals/gdtimer13b6ee86d8_v1.rs"] pub mod gdtimer13b6ee86d8;
#[path="../../peripherals/gdtimer15a386f15f_v1.rs"] pub mod gdtimer15a386f15f;
#[path="../../peripherals/gdtimer270dbabdc_v1.rs"] pub mod gdtimer270dbabdc;
#[path="../../peripherals/gdusart0a0301eea_v1.rs"] pub mod gdusart0a0301eea;
#[path="../../peripherals/gdusart13d9e0c6f_v1.rs"] pub mod gdusart13d9e0c6f;
#[path="../../peripherals/gdwwdgtdd622579_v1.rs"] pub mod gdwwdgtdd622579;
