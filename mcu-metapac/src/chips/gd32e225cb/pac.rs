

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - RTC"]
RTC = 2 , # [doc = "3 - FMC"]
FMC = 3 , # [doc = "4 - RCU"]
RCU = 4 , # [doc = "5 - EXTI0_1"]
EXTI0_1 = 5 , # [doc = "6 - EXTI2_3"]
EXTI2_3 = 6 , # [doc = "7 - EXTI4_15"]
EXTI4_15 = 7 , # [doc = "9 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 9 , # [doc = "10 - DMA_CHANNEL1_2"]
DMA_CHANNEL1_2 = 10 , # [doc = "11 - DMA_CHANNEL3_4"]
DMA_CHANNEL3_4 = 11 , # [doc = "12 - ADC_CMP"]
ADC_CMP = 12 , # [doc = "13 - TIMER0_BRK_UP_TRG_COM"]
TIMER0_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIMER0_CC"]
TIMER0_CC = 14 , # [doc = "16 - TIMER2"]
TIMER2 = 16 , # [doc = "17 - TIMER5"]
TIMER5 = 17 , # [doc = "19 - TIMER13"]
TIMER13 = 19 , # [doc = "20 - TIMER14"]
TIMER14 = 20 , # [doc = "21 - TIMER15"]
TIMER15 = 21 , # [doc = "22 - TIMER16"]
TIMER16 = 22 , # [doc = "23 - I2C0_EV"]
I2C0_EV = 23 , # [doc = "24 - I2C1_EV"]
I2C1_EV = 24 , # [doc = "25 - SPI0"]
SPI0 = 25 , # [doc = "26 - SPI1"]
SPI1 = 26 , # [doc = "27 - USART0"]
USART0 = 27 , # [doc = "28 - USART1"]
USART1 = 28 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CC () ; fn TIMER2 () ; fn TIMER5 () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn I2C0_ER () ; fn I2C1_ER () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 35]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CC } , Vector { _reserved : 0 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER5 } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } ,]
; } pub const TIMER2 : gdtimer20cf13e9f :: Timer2 = unsafe { gdtimer20cf13e9f :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER13 : gdtimer1309b6b8cc :: Timer13 = unsafe { gdtimer1309b6b8cc :: Timer13 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc335eb78b :: Rtc = unsafe { gdrtc335eb78b :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt5932fb56 :: Fwdgt = unsafe { gdfwdgt5932fb56 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi1de3ba8e1 :: Spi1 = unsafe { gdspi1de3ba8e1 :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart0bd1afef3 :: Usart0 = unsafe { gdusart0bd1afef3 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2c08b9ac71f :: I2c0 = unsafe { gdi2c08b9ac71f :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c08b9ac71f :: I2c0 = unsafe { gdi2c08b9ac71f :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu1342a8f5 :: Pmu = unsafe { gdpmu1342a8f5 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const SYSCFG : gdsyscfg99035fab :: Syscfg = unsafe { gdsyscfg99035fab :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmpfe28bbad :: Cmp = unsafe { gdcmpfe28bbad :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdexti83469f4a :: Exti = unsafe { gdexti83469f4a :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc47519d73 :: Adc = unsafe { gdadc47519d73 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer039d8e338 :: Timer0 = unsafe { gdtimer039d8e338 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0e52b5b99 :: Spi0 = unsafe { gdspi0e52b5b99 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart0bd1afef3 :: Usart0 = unsafe { gdusart0bd1afef3 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer144dec44bb :: Timer14 = unsafe { gdtimer144dec44bb :: Timer14 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const DBGMCU : gddbgmcu751e9bc8 :: Dbgmcu = unsafe { gddbgmcu751e9bc8 :: Dbgmcu :: from_ptr (0x4001_5800usize as _) } ; pub const DMA : gddma3c234e7c :: Dma = unsafe { gddma3c234e7c :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const RCU : gdrcu2eec98a2 :: Rcu = unsafe { gdrcu2eec98a2 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc66eac118 :: Fmc = unsafe { gdfmc66eac118 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc8a4036fe :: Crc = unsafe { gdcrc8a4036fe :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob3a01cf30 :: Gpiob = unsafe { gdgpiob3a01cf30 :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioc88586c6c :: Gpioc = unsafe { gdgpioc88586c6c :: Gpioc :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOF : gdgpiof41ef0f55 :: Gpiof = unsafe { gdgpiof41ef0f55 :: Gpiof :: from_ptr (0x4800_1400usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc47519d73_v1.rs"] pub mod gdadc47519d73;
#[path="../../peripherals/gdcmpfe28bbad_v1.rs"] pub mod gdcmpfe28bbad;
#[path="../../peripherals/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../../peripherals/gddbgmcu751e9bc8_v1.rs"] pub mod gddbgmcu751e9bc8;
#[path="../../peripherals/gddma3c234e7c_v1.rs"] pub mod gddma3c234e7c;
#[path="../../peripherals/gdexti83469f4a_v1.rs"] pub mod gdexti83469f4a;
#[path="../../peripherals/gdfmc66eac118_v1.rs"] pub mod gdfmc66eac118;
#[path="../../peripherals/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../../peripherals/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../../peripherals/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../../peripherals/gdgpioc88586c6c_v1.rs"] pub mod gdgpioc88586c6c;
#[path="../../peripherals/gdgpiof41ef0f55_v1.rs"] pub mod gdgpiof41ef0f55;
#[path="../../peripherals/gdi2c08b9ac71f_v1.rs"] pub mod gdi2c08b9ac71f;
#[path="../../peripherals/gdpmu1342a8f5_v1.rs"] pub mod gdpmu1342a8f5;
#[path="../../peripherals/gdrcu2eec98a2_v1.rs"] pub mod gdrcu2eec98a2;
#[path="../../peripherals/gdrtc335eb78b_v1.rs"] pub mod gdrtc335eb78b;
#[path="../../peripherals/gdspi0e52b5b99_v1.rs"] pub mod gdspi0e52b5b99;
#[path="../../peripherals/gdspi1de3ba8e1_v1.rs"] pub mod gdspi1de3ba8e1;
#[path="../../peripherals/gdsyscfg99035fab_v1.rs"] pub mod gdsyscfg99035fab;
#[path="../../peripherals/gdtimer039d8e338_v1.rs"] pub mod gdtimer039d8e338;
#[path="../../peripherals/gdtimer1309b6b8cc_v1.rs"] pub mod gdtimer1309b6b8cc;
#[path="../../peripherals/gdtimer144dec44bb_v1.rs"] pub mod gdtimer144dec44bb;
#[path="../../peripherals/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../../peripherals/gdtimer20cf13e9f_v1.rs"] pub mod gdtimer20cf13e9f;
#[path="../../peripherals/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../../peripherals/gdusart0bd1afef3_v1.rs"] pub mod gdusart0bd1afef3;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
