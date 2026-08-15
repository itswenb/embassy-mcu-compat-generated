

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
TIMER0_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIMER0_CC"]
TIMER0_CC = 14 , # [doc = "15 - TIMER1"]
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
USBD_HP = 38 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "48 - DMA_CHANNEL5_6"]
DMA_CHANNEL5_6 = 48 , # [doc = "51 - SPI2"]
SPI2 = 51 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn TSI () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER5_DAC () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn CEC () ; fn I2C0_ER () ; fn I2C1_ER () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBD_LP () ; fn USBD_HP () ; fn USBD_WKUP () ; fn DMA_CHANNEL5_6 () ; fn SPI2 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 52]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _handler : TSI } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER5_DAC } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _handler : CEC } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBD_LP } , Vector { _handler : USBD_HP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBD_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL5_6 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI2 } ,]
; } pub const TIMER1 : gdtimer1ac32c839 :: Timer1 = unsafe { gdtimer1ac32c839 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer1ac32c839 :: Timer1 = unsafe { gdtimer1ac32c839 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer539366dab :: Timer5 = unsafe { gdtimer539366dab :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER13 : gdtimer131b1f9b88 :: Timer13 = unsafe { gdtimer131b1f9b88 :: Timer13 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtce98f095e :: Rtc = unsafe { gdrtce98f095e :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgta2f29825 :: Wwdgt = unsafe { gdwwdgta2f29825 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgte0a44d28 :: Fwdgt = unsafe { gdfwdgte0a44d28 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi031a1ec7f :: Spi0 = unsafe { gdspi031a1ec7f :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi031a1ec7f :: Spi0 = unsafe { gdspi031a1ec7f :: Spi0 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart13604d2cd :: Usart1 = unsafe { gdusart13604d2cd :: Usart1 :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2c004a0a4d9 :: I2c0 = unsafe { gdi2c004a0a4d9 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c004a0a4d9 :: I2c0 = unsafe { gdi2c004a0a4d9 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const USBD : gdusbd0273a115 :: Usbd = unsafe { gdusbd0273a115 :: Usbd :: from_ptr (0x4000_5c00usize as _) } ; pub const PMU : gdpmu808687e1 :: Pmu = unsafe { gdpmu808687e1 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac2ce5879d :: Dac = unsafe { gddac2ce5879d :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CEC : gdcecade85f56 :: Cec = unsafe { gdcecade85f56 :: Cec :: from_ptr (0x4000_7800usize as _) } ; pub const I2C2 : gdi2c004a0a4d9 :: I2c0 = unsafe { gdi2c004a0a4d9 :: I2c0 :: from_ptr (0x4000_c000usize as _) } ; pub const SYSCFG : gdsyscfg501b84a6 :: Syscfg = unsafe { gdsyscfg501b84a6 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmp23924063 :: Cmp = unsafe { gdcmp23924063 :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdextiab091bb7 :: Exti = unsafe { gdextiab091bb7 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc018ab876 :: Adc = unsafe { gdadc018ab876 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer096c7099a :: Timer0 = unsafe { gdtimer096c7099a :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi031a1ec7f :: Spi0 = unsafe { gdspi031a1ec7f :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart06f0b00f6 :: Usart0 = unsafe { gdusart06f0b00f6 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer143daae142 :: Timer14 = unsafe { gdtimer143daae142 :: Timer14 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer1504ddc856 :: Timer15 = unsafe { gdtimer1504ddc856 :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer1504ddc856 :: Timer15 = unsafe { gdtimer1504ddc856 :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const DMA : gddma9472c5b9 :: Dma = unsafe { gddma9472c5b9 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const RCU : gdrcu70c8037a :: Rcu = unsafe { gdrcu70c8037a :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcb49be91b :: Fmc = unsafe { gdfmcb49be91b :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc7d8cea52 :: Crc = unsafe { gdcrc7d8cea52 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const TSI : gdtsie9c86076 :: Tsi = unsafe { gdtsie9c86076 :: Tsi :: from_ptr (0x4002_4000usize as _) } ; pub const GPIOA : gdgpioa3815acae :: Gpioa = unsafe { gdgpioa3815acae :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob0e35583c :: Gpiob = unsafe { gdgpiob0e35583c :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioc797149ba :: Gpioc = unsafe { gdgpioc797149ba :: Gpioc :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiod7a8beedc :: Gpiod = unsafe { gdgpiod7a8beedc :: Gpiod :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiof4abb2202 :: Gpiof = unsafe { gdgpiof4abb2202 :: Gpiof :: from_ptr (0x4800_1400usize as _) } ; pub const DBG : gddbg1876a7bc :: Dbg = unsafe { gddbg1876a7bc :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc018ab876_v1.rs"] pub mod gdadc018ab876;
#[path="../../peripherals/gdcecade85f56_v1.rs"] pub mod gdcecade85f56;
#[path="../../peripherals/gdcmp23924063_v1.rs"] pub mod gdcmp23924063;
#[path="../../peripherals/gdcrc7d8cea52_v1.rs"] pub mod gdcrc7d8cea52;
#[path="../../peripherals/gddac2ce5879d_v1.rs"] pub mod gddac2ce5879d;
#[path="../../peripherals/gddbg1876a7bc_v1.rs"] pub mod gddbg1876a7bc;
#[path="../../peripherals/gddma9472c5b9_v1.rs"] pub mod gddma9472c5b9;
#[path="../../peripherals/gdextiab091bb7_v1.rs"] pub mod gdextiab091bb7;
#[path="../../peripherals/gdfmcb49be91b_v1.rs"] pub mod gdfmcb49be91b;
#[path="../../peripherals/gdfwdgte0a44d28_v1.rs"] pub mod gdfwdgte0a44d28;
#[path="../../peripherals/gdgpioa3815acae_v1.rs"] pub mod gdgpioa3815acae;
#[path="../../peripherals/gdgpiob0e35583c_v1.rs"] pub mod gdgpiob0e35583c;
#[path="../../peripherals/gdgpioc797149ba_v1.rs"] pub mod gdgpioc797149ba;
#[path="../../peripherals/gdgpiod7a8beedc_v1.rs"] pub mod gdgpiod7a8beedc;
#[path="../../peripherals/gdgpiof4abb2202_v1.rs"] pub mod gdgpiof4abb2202;
#[path="../../peripherals/gdi2c004a0a4d9_v1.rs"] pub mod gdi2c004a0a4d9;
#[path="../../peripherals/gdpmu808687e1_v1.rs"] pub mod gdpmu808687e1;
#[path="../../peripherals/gdrcu70c8037a_v1.rs"] pub mod gdrcu70c8037a;
#[path="../../peripherals/gdrtce98f095e_v1.rs"] pub mod gdrtce98f095e;
#[path="../../peripherals/gdspi031a1ec7f_v1.rs"] pub mod gdspi031a1ec7f;
#[path="../../peripherals/gdsyscfg501b84a6_v1.rs"] pub mod gdsyscfg501b84a6;
#[path="../../peripherals/gdtimer096c7099a_v1.rs"] pub mod gdtimer096c7099a;
#[path="../../peripherals/gdtimer131b1f9b88_v1.rs"] pub mod gdtimer131b1f9b88;
#[path="../../peripherals/gdtimer143daae142_v1.rs"] pub mod gdtimer143daae142;
#[path="../../peripherals/gdtimer1504ddc856_v1.rs"] pub mod gdtimer1504ddc856;
#[path="../../peripherals/gdtimer1ac32c839_v1.rs"] pub mod gdtimer1ac32c839;
#[path="../../peripherals/gdtimer539366dab_v1.rs"] pub mod gdtimer539366dab;
#[path="../../peripherals/gdtsie9c86076_v1.rs"] pub mod gdtsie9c86076;
#[path="../../peripherals/gdusart06f0b00f6_v1.rs"] pub mod gdusart06f0b00f6;
#[path="../../peripherals/gdusart13604d2cd_v1.rs"] pub mod gdusart13604d2cd;
#[path="../../peripherals/gdusbd0273a115_v1.rs"] pub mod gdusbd0273a115;
#[path="../../peripherals/gdwwdgta2f29825_v1.rs"] pub mod gdwwdgta2f29825;
