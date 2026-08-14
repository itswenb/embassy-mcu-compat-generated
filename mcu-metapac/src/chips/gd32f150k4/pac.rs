




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
EXTI4_15 = 7 , # [doc = "8 - TSI"]
TSI = 8 , # [doc = "9 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 9 , # [doc = "10 - DMA_CHANNEL1_2"]
DMA_CHANNEL1_2 = 10 , # [doc = "11 - DMA_CHANNEL3_4"]
DMA_CHANNEL3_4 = 11 , # [doc = "12 - ADC_CMP"]
ADC_CMP = 12 , # [doc = "13 - TIMER0_BRK_UP_TRG_COM"]
TIMER0_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 14 , # [doc = "15 - TIMER1"]
TIMER1 = 15 , # [doc = "16 - TIMER2"]
TIMER2 = 16 , # [doc = "17 - TIMER5_DAC"]
TIMER5_DAC = 17 , # [doc = "19 - TIMER13"]
TIMER13 = 19 , # [doc = "20 - TIMER14"]
TIMER14 = 20 , # [doc = "21 - TIMER15"]
TIMER15 = 21 , # [doc = "22 - TIMER16"]
TIMER16 = 22 , # [doc = "23 - I2C0_EV"]
I2C0_EV = 23 , # [doc = "24 - I2C1_EV"]
I2C1_EV = 24 , # [doc = "25 - SPI0"]
SPI0 = 25 , # [doc = "26 - SPI1"]
SPI1 = 26 , # [doc = "27 - USART0"]
USART0 = 27 , # [doc = "28 - USART1"]
USART1 = 28 , # [doc = "30 - CEC"]
CEC = 30 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - I2C2_EV"]
I2C2_EV = 35 , # [doc = "36 - I2C2_ER"]
I2C2_ER = 36 , # [doc = "37 - USBD_LP"]
USBD_LP = 37 , # [doc = "38 - USBD_HP"]
USBD_HP = 38 , # [doc = "42 - USBDWAKEUP"]
USBDWAKEUP = 42 , # [doc = "48 - DMA_CHANNEL5_6"]
DMA_CHANNEL5_6 = 48 , # [doc = "51 - SPI2"]
SPI2 = 51 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn TSI () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER5_DAC () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn CEC () ; fn I2C0_ER () ; fn I2C1_ER () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBD_LP () ; fn USBD_HP () ; fn USBDWAKEUP () ; fn DMA_CHANNEL5_6 () ; fn SPI2 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 52]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _handler : TSI } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER5_DAC } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _handler : CEC } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBD_LP } , Vector { _handler : USBD_HP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBDWAKEUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL5_6 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI2 } ,]
; } pub const OB : gdob3a9c1d77 :: Ob = unsafe { gdob3a9c1d77 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER13 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc218478ea :: Rtc = unsafe { gdrtc218478ea :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspif510bcbc :: Spi = unsafe { gdspif510bcbc :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspif510bcbc :: Spi = unsafe { gdspif510bcbc :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusartc0290a80 :: Usart = unsafe { gdusartc0290a80 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2c1522fa4f :: I2c = unsafe { gdi2c1522fa4f :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c1522fa4f :: I2c = unsafe { gdi2c1522fa4f :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmufd5a3deb :: Pmu = unsafe { gdpmufd5a3deb :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac17bb59ad :: Dac = unsafe { gddac17bb59ad :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CEC : gdcecfd93ba0d :: Cec = unsafe { gdcecfd93ba0d :: Cec :: from_ptr (0x4000_7800usize as _) } ; pub const SYSCFG : gdsyscfg0a0fd0b4 :: Syscfg = unsafe { gdsyscfg0a0fd0b4 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmpe12ecc50 :: Cmp = unsafe { gdcmpe12ecc50 :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdexti7b9b36c7 :: Exti = unsafe { gdexti7b9b36c7 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc03b2fd91 :: Adc = unsafe { gdadc03b2fd91 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspif510bcbc :: Spi = unsafe { gdspif510bcbc :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusartc0290a80 :: Usart = unsafe { gdusartc0290a80 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer9000ea71 :: Timer = unsafe { gdtimer9000ea71 :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const DMA : gddmaeced416e :: Dma = unsafe { gddmaeced416e :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA_CHXCTL_BASE : gddmachxctlbase9fc231ae :: DmaChxctlBase = unsafe { gddmachxctlbase9fc231ae :: DmaChxctlBase :: from_ptr (0x4002_0008usize as _) } ; pub const DMA_CHXCNT_BASE : gddmachxcntbased390cdb4 :: DmaChxcntBase = unsafe { gddmachxcntbased390cdb4 :: DmaChxcntBase :: from_ptr (0x4002_000cusize as _) } ; pub const DMA_CHXPADDR_BASE : gddmachxpaddrbase24a24737 :: DmaChxpaddrBase = unsafe { gddmachxpaddrbase24a24737 :: DmaChxpaddrBase :: from_ptr (0x4002_0010usize as _) } ; pub const DMA_CHXMADDR_BASE : gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase = unsafe { gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase :: from_ptr (0x4002_0014usize as _) } ; pub const RCU : gdrcu62b10011 :: Rcu = unsafe { gdrcu62b10011 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc14500709 :: Fmc = unsafe { gdfmc14500709 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrca48611ed :: Crc = unsafe { gdcrca48611ed :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const TSI : gdtsifaec6025 :: Tsi = unsafe { gdtsifaec6025 :: Tsi :: from_ptr (0x4002_4000usize as _) } ; pub const GPIOA : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const DBG : gddbg6afd90ea :: Dbg = unsafe { gddbg6afd90ea :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc03b2fd91_v1.rs"] pub mod gdadc03b2fd91;
#[path="../../peripherals/gdcecfd93ba0d_v1.rs"] pub mod gdcecfd93ba0d;
#[path="../../peripherals/gdcmpe12ecc50_v1.rs"] pub mod gdcmpe12ecc50;
#[path="../../peripherals/gdcrca48611ed_v1.rs"] pub mod gdcrca48611ed;
#[path="../../peripherals/gddac17bb59ad_v1.rs"] pub mod gddac17bb59ad;
#[path="../../peripherals/gddbg6afd90ea_v1.rs"] pub mod gddbg6afd90ea;
#[path="../../peripherals/gddmachxcntbased390cdb4_v1.rs"] pub mod gddmachxcntbased390cdb4;
#[path="../../peripherals/gddmachxctlbase9fc231ae_v1.rs"] pub mod gddmachxctlbase9fc231ae;
#[path="../../peripherals/gddmachxmaddrbase53fbca93_v1.rs"] pub mod gddmachxmaddrbase53fbca93;
#[path="../../peripherals/gddmachxpaddrbase24a24737_v1.rs"] pub mod gddmachxpaddrbase24a24737;
#[path="../../peripherals/gddmaeced416e_v1.rs"] pub mod gddmaeced416e;
#[path="../../peripherals/gdexti7b9b36c7_v1.rs"] pub mod gdexti7b9b36c7;
#[path="../../peripherals/gdfmc14500709_v1.rs"] pub mod gdfmc14500709;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpiob9f95038_v1.rs"] pub mod gdgpiob9f95038;
#[path="../../peripherals/gdi2c1522fa4f_v1.rs"] pub mod gdi2c1522fa4f;
#[path="../../peripherals/gdob3a9c1d77_v1.rs"] pub mod gdob3a9c1d77;
#[path="../../peripherals/gdpmufd5a3deb_v1.rs"] pub mod gdpmufd5a3deb;
#[path="../../peripherals/gdrcu62b10011_v1.rs"] pub mod gdrcu62b10011;
#[path="../../peripherals/gdrtc218478ea_v1.rs"] pub mod gdrtc218478ea;
#[path="../../peripherals/gdspif510bcbc_v1.rs"] pub mod gdspif510bcbc;
#[path="../../peripherals/gdsyscfg0a0fd0b4_v1.rs"] pub mod gdsyscfg0a0fd0b4;
#[path="../../peripherals/gdtimer9000ea71_v1.rs"] pub mod gdtimer9000ea71;
#[path="../../peripherals/gdtsifaec6025_v1.rs"] pub mod gdtsifaec6025;
#[path="../../peripherals/gdusartc0290a80_v1.rs"] pub mod gdusartc0290a80;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
