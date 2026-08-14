




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
TIMER0_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 14 , # [doc = "16 - TIMER2"]
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
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CHANNEL () ; fn TIMER2 () ; fn TIMER5 () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn I2C0_ER () ; fn I2C1_ER () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 35]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CHANNEL } , Vector { _reserved : 0 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER5 } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } ,]
; } pub const OB : gdobd8b8edf3 :: Ob = unsafe { gdobd8b8edf3 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER2 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER13 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc218478ea :: Rtc = unsafe { gdrtc218478ea :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspic7bc14a6 :: Spi = unsafe { gdspic7bc14a6 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2c2414824a :: I2c = unsafe { gdi2c2414824a :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c2414824a :: I2c = unsafe { gdi2c2414824a :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu9ff5b6df :: Pmu = unsafe { gdpmu9ff5b6df :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const SYSCFG : gdsyscfg8890ad57 :: Syscfg = unsafe { gdsyscfg8890ad57 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmpf9aa0875 :: Cmp = unsafe { gdcmpf9aa0875 :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdexti59df8d27 :: Exti = unsafe { gdexti59df8d27 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc517245d2 :: Adc = unsafe { gdadc517245d2 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspic7bc14a6 :: Spi = unsafe { gdspic7bc14a6 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer7ebd44eb :: Timer = unsafe { gdtimer7ebd44eb :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const DBG : gddbg1b652252 :: Dbg = unsafe { gddbg1b652252 :: Dbg :: from_ptr (0x4001_5800usize as _) } ; pub const DMA : gddma0d635cd0 :: Dma = unsafe { gddma0d635cd0 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA_CHXCTL_BASE : gddmachxctlbase49e16ead :: DmaChxctlBase = unsafe { gddmachxctlbase49e16ead :: DmaChxctlBase :: from_ptr (0x4002_0008usize as _) } ; pub const DMA_CHXCNT_BASE : gddmachxcntbase50b58da3 :: DmaChxcntBase = unsafe { gddmachxcntbase50b58da3 :: DmaChxcntBase :: from_ptr (0x4002_000cusize as _) } ; pub const DMA_CHXPADDR_BASE : gddmachxpaddrbasec13afd42 :: DmaChxpaddrBase = unsafe { gddmachxpaddrbasec13afd42 :: DmaChxpaddrBase :: from_ptr (0x4002_0010usize as _) } ; pub const DMA_CHXMADDR_BASE : gddmachxmaddrbase0f5bcd0c :: DmaChxmaddrBase = unsafe { gddmachxmaddrbase0f5bcd0c :: DmaChxmaddrBase :: from_ptr (0x4002_0014usize as _) } ; pub const RCU : gdrcucca4dd31 :: Rcu = unsafe { gdrcucca4dd31 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc95e51906 :: Fmc = unsafe { gdfmc95e51906 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOF : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_1400usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc517245d2_v1.rs"] pub mod gdadc517245d2;
#[path="../../peripherals/gdcmpf9aa0875_v1.rs"] pub mod gdcmpf9aa0875;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gddbg1b652252_v1.rs"] pub mod gddbg1b652252;
#[path="../../peripherals/gddma0d635cd0_v1.rs"] pub mod gddma0d635cd0;
#[path="../../peripherals/gddmachxcntbase50b58da3_v1.rs"] pub mod gddmachxcntbase50b58da3;
#[path="../../peripherals/gddmachxctlbase49e16ead_v1.rs"] pub mod gddmachxctlbase49e16ead;
#[path="../../peripherals/gddmachxmaddrbase0f5bcd0c_v1.rs"] pub mod gddmachxmaddrbase0f5bcd0c;
#[path="../../peripherals/gddmachxpaddrbasec13afd42_v1.rs"] pub mod gddmachxpaddrbasec13afd42;
#[path="../../peripherals/gdexti59df8d27_v1.rs"] pub mod gdexti59df8d27;
#[path="../../peripherals/gdfmc95e51906_v1.rs"] pub mod gdfmc95e51906;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../../peripherals/gdi2c2414824a_v1.rs"] pub mod gdi2c2414824a;
#[path="../../peripherals/gdobd8b8edf3_v1.rs"] pub mod gdobd8b8edf3;
#[path="../../peripherals/gdpmu9ff5b6df_v1.rs"] pub mod gdpmu9ff5b6df;
#[path="../../peripherals/gdrcucca4dd31_v1.rs"] pub mod gdrcucca4dd31;
#[path="../../peripherals/gdrtc218478ea_v1.rs"] pub mod gdrtc218478ea;
#[path="../../peripherals/gdspic7bc14a6_v1.rs"] pub mod gdspic7bc14a6;
#[path="../../peripherals/gdsyscfg8890ad57_v1.rs"] pub mod gdsyscfg8890ad57;
#[path="../../peripherals/gdtimer7ebd44eb_v1.rs"] pub mod gdtimer7ebd44eb;
#[path="../../peripherals/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
