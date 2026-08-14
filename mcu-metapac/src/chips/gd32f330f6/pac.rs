




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - RTC"]
RTC = 2 , # [doc = "3 - FMC"]
FMC = 3 , # [doc = "4 - RCU_CTC"]
RCU_CTC = 4 , # [doc = "5 - EXTI0_1"]
EXTI0_1 = 5 , # [doc = "6 - EXTI2_3"]
EXTI2_3 = 6 , # [doc = "7 - EXTI4_15"]
EXTI4_15 = 7 , # [doc = "8 - TSI"]
TSI = 8 , # [doc = "9 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 9 , # [doc = "10 - DMA_CHANNEL1_2"]
DMA_CHANNEL1_2 = 10 , # [doc = "11 - DMA_CHANNEL3_4"]
DMA_CHANNEL3_4 = 11 , # [doc = "12 - ADC_CMP"]
ADC_CMP = 12 , # [doc = "13 - TIMER0_BRK_UP_TRG_COM"]
TIMER0_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 14 , # [doc = "15 - TIMER1"]
TIMER1 = 15 , # [doc = "16 - TIMER2"]
TIMER2 = 16 , # [doc = "19 - TIMER13"]
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
I2C1_ER = 34 , # [doc = "48 - DMA_CHANNEL5_6"]
DMA_CHANNEL5_6 = 48 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn TSI () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn I2C0_ER () ; fn I2C1_ER () ; fn DMA_CHANNEL5_6 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 49]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _handler : TSI } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL5_6 } ,]
; } pub const OB : gdob8669b02b :: Ob = unsafe { gdob8669b02b :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER13 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc218478ea :: Rtc = unsafe { gdrtc218478ea :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspib2f7804e :: Spi = unsafe { gdspib2f7804e :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusartf414f7c3 :: Usart = unsafe { gdusartf414f7c3 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2cd2b8dbf2 :: I2c = unsafe { gdi2cd2b8dbf2 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2cd2b8dbf2 :: I2c = unsafe { gdi2cd2b8dbf2 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmuda3a6d7f :: Pmu = unsafe { gdpmuda3a6d7f :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const SYSCFG : gdsyscfg4d7d59e9 :: Syscfg = unsafe { gdsyscfg4d7d59e9 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmpe12ecc50 :: Cmp = unsafe { gdcmpe12ecc50 :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdexti7b9b36c7 :: Exti = unsafe { gdexti7b9b36c7 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc939a9095 :: Adc = unsafe { gdadc939a9095 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspib2f7804e :: Spi = unsafe { gdspib2f7804e :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusartf414f7c3 :: Usart = unsafe { gdusartf414f7c3 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer58330829 :: Timer = unsafe { gdtimer58330829 :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const DMA : gddma203b2e8a :: Dma = unsafe { gddma203b2e8a :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA_CHXCTL_BASE : gddmachxctlbase9fc231ae :: DmaChxctlBase = unsafe { gddmachxctlbase9fc231ae :: DmaChxctlBase :: from_ptr (0x4002_0008usize as _) } ; pub const DMA_CHXCNT_BASE : gddmachxcntbased390cdb4 :: DmaChxcntBase = unsafe { gddmachxcntbased390cdb4 :: DmaChxcntBase :: from_ptr (0x4002_000cusize as _) } ; pub const DMA_CHXPADDR_BASE : gddmachxpaddrbase24a24737 :: DmaChxpaddrBase = unsafe { gddmachxpaddrbase24a24737 :: DmaChxpaddrBase :: from_ptr (0x4002_0010usize as _) } ; pub const DMA_CHXMADDR_BASE : gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase = unsafe { gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase :: from_ptr (0x4002_0014usize as _) } ; pub const RCU : gdrcuc6ee4fe0 :: Rcu = unsafe { gdrcuc6ee4fe0 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcb0f6c6c8 :: Fmc = unsafe { gdfmcb0f6c6c8 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpioe3950126 :: Gpio = unsafe { gdgpioe3950126 :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpioe3950126 :: Gpio = unsafe { gdgpioe3950126 :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioe3950126 :: Gpio = unsafe { gdgpioe3950126 :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpioe3950126 :: Gpio = unsafe { gdgpioe3950126 :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpioe3950126 :: Gpio = unsafe { gdgpioe3950126 :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const DBG : gddbg2bf2258f :: Dbg = unsafe { gddbg2bf2258f :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc939a9095_v1.rs"] pub mod gdadc939a9095;
#[path="../../peripherals/gdcmpe12ecc50_v1.rs"] pub mod gdcmpe12ecc50;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddbg2bf2258f_v1.rs"] pub mod gddbg2bf2258f;
#[path="../../peripherals/gddma203b2e8a_v1.rs"] pub mod gddma203b2e8a;
#[path="../../peripherals/gddmachxcntbased390cdb4_v1.rs"] pub mod gddmachxcntbased390cdb4;
#[path="../../peripherals/gddmachxctlbase9fc231ae_v1.rs"] pub mod gddmachxctlbase9fc231ae;
#[path="../../peripherals/gddmachxmaddrbase53fbca93_v1.rs"] pub mod gddmachxmaddrbase53fbca93;
#[path="../../peripherals/gddmachxpaddrbase24a24737_v1.rs"] pub mod gddmachxpaddrbase24a24737;
#[path="../../peripherals/gdexti7b9b36c7_v1.rs"] pub mod gdexti7b9b36c7;
#[path="../../peripherals/gdfmcb0f6c6c8_v1.rs"] pub mod gdfmcb0f6c6c8;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpioe3950126_v1.rs"] pub mod gdgpioe3950126;
#[path="../../peripherals/gdi2cd2b8dbf2_v1.rs"] pub mod gdi2cd2b8dbf2;
#[path="../../peripherals/gdob8669b02b_v1.rs"] pub mod gdob8669b02b;
#[path="../../peripherals/gdpmuda3a6d7f_v1.rs"] pub mod gdpmuda3a6d7f;
#[path="../../peripherals/gdrcuc6ee4fe0_v1.rs"] pub mod gdrcuc6ee4fe0;
#[path="../../peripherals/gdrtc218478ea_v1.rs"] pub mod gdrtc218478ea;
#[path="../../peripherals/gdspib2f7804e_v1.rs"] pub mod gdspib2f7804e;
#[path="../../peripherals/gdsyscfg4d7d59e9_v1.rs"] pub mod gdsyscfg4d7d59e9;
#[path="../../peripherals/gdtimer58330829_v1.rs"] pub mod gdtimer58330829;
#[path="../../peripherals/gdusartf414f7c3_v1.rs"] pub mod gdusartf414f7c3;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
