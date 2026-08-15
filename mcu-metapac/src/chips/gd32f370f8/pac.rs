

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
I2C1_ER = 34 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "48 - DMA_CHANNEL5_6"]
DMA_CHANNEL5_6 = 48 , # [doc = "67 - USBFS"]
USBFS = 67 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn TSI () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1_2 () ; fn DMA_CHANNEL3_4 () ; fn ADC_CMP () ; fn TIMER0_BRK_UP_TRG_COM () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER5_DAC () ; fn TIMER13 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn I2C0_EV () ; fn I2C1_EV () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn CEC () ; fn I2C0_ER () ; fn I2C1_ER () ; fn USBFS_WKUP () ; fn DMA_CHANNEL5_6 () ; fn USBFS () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 68]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _handler : TSI } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1_2 } , Vector { _handler : DMA_CHANNEL3_4 } , Vector { _handler : ADC_CMP } , Vector { _handler : TIMER0_BRK_UP_TRG_COM } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER5_DAC } , Vector { _reserved : 0 } , Vector { _handler : TIMER13 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C1_EV } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _reserved : 0 } , Vector { _handler : CEC } , Vector { _reserved : 0 } , Vector { _handler : I2C0_ER } , Vector { _reserved : 0 } , Vector { _handler : I2C1_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBFS_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL5_6 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBFS } ,]
; } pub const TIMER1 : gdtimer16a9c1bb0 :: Timer1 = unsafe { gdtimer16a9c1bb0 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer16a9c1bb0 :: Timer1 = unsafe { gdtimer16a9c1bb0 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER13 : gdtimer1309b6b8cc :: Timer13 = unsafe { gdtimer1309b6b8cc :: Timer13 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc7ef316ca :: Rtc = unsafe { gdrtc7ef316ca :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgtfa76971a :: Wwdgt = unsafe { gdwwdgtfa76971a :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtcbc843d4 :: Fwdgt = unsafe { gdfwdgtcbc843d4 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi04f926fdd :: Spi0 = unsafe { gdspi04f926fdd :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart08bc22e17 :: Usart0 = unsafe { gdusart08bc22e17 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const I2C0 : gdi2c00d742485 :: I2c0 = unsafe { gdi2c00d742485 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c00d742485 :: I2c0 = unsafe { gdi2c00d742485 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmuf173c0ef :: Pmu = unsafe { gdpmuf173c0ef :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddacc6b1bb98 :: Dac = unsafe { gddacc6b1bb98 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CEC : gdcec9fb29752 :: Cec = unsafe { gdcec9fb29752 :: Cec :: from_ptr (0x4000_7800usize as _) } ; pub const CTC : gdctc57a0fbe5 :: Ctc = unsafe { gdctc57a0fbe5 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const SYSCFG : gdsyscfg595878d5 :: Syscfg = unsafe { gdsyscfg595878d5 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const CMP : gdcmp6176059a :: Cmp = unsafe { gdcmp6176059a :: Cmp :: from_ptr (0x4001_001cusize as _) } ; pub const EXTI : gdexti6214ef6d :: Exti = unsafe { gdexti6214ef6d :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadcb7217899 :: Adc = unsafe { gdadcb7217899 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer0d9a58b68 :: Timer0 = unsafe { gdtimer0d9a58b68 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi04f926fdd :: Spi0 = unsafe { gdspi04f926fdd :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart08bc22e17 :: Usart0 = unsafe { gdusart08bc22e17 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer144dec44bb :: Timer14 = unsafe { gdtimer144dec44bb :: Timer14 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const DMA : gddma0f758611 :: Dma = unsafe { gddma0f758611 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const RCU : gdrcu5258fdf2 :: Rcu = unsafe { gdrcu5258fdf2 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc20e9ec99 :: Fmc = unsafe { gdfmc20e9ec99 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc8a4036fe :: Crc = unsafe { gdcrc8a4036fe :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const TSI : gdtsid83e70fb :: Tsi = unsafe { gdtsid83e70fb :: Tsi :: from_ptr (0x4002_4000usize as _) } ; pub const GPIOA : gdgpioa54a0be2b :: Gpioa = unsafe { gdgpioa54a0be2b :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiobcfe71f6a :: Gpiob = unsafe { gdgpiobcfe71f6a :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioc47392aee :: Gpioc = unsafe { gdgpioc47392aee :: Gpioc :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiodc257f1c6 :: Gpiod = unsafe { gdgpiodc257f1c6 :: Gpiod :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiof564f1005 :: Gpiof = unsafe { gdgpiof564f1005 :: Gpiof :: from_ptr (0x4800_1400usize as _) } ; pub const USBFS_GLOBAL : gdusbfsglobal1a7549aa :: UsbfsGlobal = unsafe { gdusbfsglobal1a7549aa :: UsbfsGlobal :: from_ptr (0x5000_0000usize as _) } ; pub const USBFS_HOST : gdusbfshost5f42a79e :: UsbfsHost = unsafe { gdusbfshost5f42a79e :: UsbfsHost :: from_ptr (0x5000_0400usize as _) } ; pub const USBFS_DEVICE : gdusbfsdevice6d1906cf :: UsbfsDevice = unsafe { gdusbfsdevice6d1906cf :: UsbfsDevice :: from_ptr (0x5000_0800usize as _) } ; pub const USBFS_PWRCLK : gdusbfspwrclk2ac667f0 :: UsbfsPwrclk = unsafe { gdusbfspwrclk2ac667f0 :: UsbfsPwrclk :: from_ptr (0x5000_0e00usize as _) } ; pub const DBG : gddbg7f4c1511 :: Dbg = unsafe { gddbg7f4c1511 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcb7217899_v1.rs"] pub mod gdadcb7217899;
#[path="../../peripherals/gdcec9fb29752_v1.rs"] pub mod gdcec9fb29752;
#[path="../../peripherals/gdcmp6176059a_v1.rs"] pub mod gdcmp6176059a;
#[path="../../peripherals/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../../peripherals/gdctc57a0fbe5_v1.rs"] pub mod gdctc57a0fbe5;
#[path="../../peripherals/gddacc6b1bb98_v1.rs"] pub mod gddacc6b1bb98;
#[path="../../peripherals/gddbg7f4c1511_v1.rs"] pub mod gddbg7f4c1511;
#[path="../../peripherals/gddma0f758611_v1.rs"] pub mod gddma0f758611;
#[path="../../peripherals/gdexti6214ef6d_v1.rs"] pub mod gdexti6214ef6d;
#[path="../../peripherals/gdfmc20e9ec99_v1.rs"] pub mod gdfmc20e9ec99;
#[path="../../peripherals/gdfwdgtcbc843d4_v1.rs"] pub mod gdfwdgtcbc843d4;
#[path="../../peripherals/gdgpioa54a0be2b_v1.rs"] pub mod gdgpioa54a0be2b;
#[path="../../peripherals/gdgpiobcfe71f6a_v1.rs"] pub mod gdgpiobcfe71f6a;
#[path="../../peripherals/gdgpioc47392aee_v1.rs"] pub mod gdgpioc47392aee;
#[path="../../peripherals/gdgpiodc257f1c6_v1.rs"] pub mod gdgpiodc257f1c6;
#[path="../../peripherals/gdgpiof564f1005_v1.rs"] pub mod gdgpiof564f1005;
#[path="../../peripherals/gdi2c00d742485_v1.rs"] pub mod gdi2c00d742485;
#[path="../../peripherals/gdpmuf173c0ef_v1.rs"] pub mod gdpmuf173c0ef;
#[path="../../peripherals/gdrcu5258fdf2_v1.rs"] pub mod gdrcu5258fdf2;
#[path="../../peripherals/gdrtc7ef316ca_v1.rs"] pub mod gdrtc7ef316ca;
#[path="../../peripherals/gdspi04f926fdd_v1.rs"] pub mod gdspi04f926fdd;
#[path="../../peripherals/gdsyscfg595878d5_v1.rs"] pub mod gdsyscfg595878d5;
#[path="../../peripherals/gdtimer0d9a58b68_v1.rs"] pub mod gdtimer0d9a58b68;
#[path="../../peripherals/gdtimer1309b6b8cc_v1.rs"] pub mod gdtimer1309b6b8cc;
#[path="../../peripherals/gdtimer144dec44bb_v1.rs"] pub mod gdtimer144dec44bb;
#[path="../../peripherals/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../../peripherals/gdtimer16a9c1bb0_v1.rs"] pub mod gdtimer16a9c1bb0;
#[path="../../peripherals/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../../peripherals/gdtsid83e70fb_v1.rs"] pub mod gdtsid83e70fb;
#[path="../../peripherals/gdusart08bc22e17_v1.rs"] pub mod gdusart08bc22e17;
#[path="../../peripherals/gdusbfsdevice6d1906cf_v1.rs"] pub mod gdusbfsdevice6d1906cf;
#[path="../../peripherals/gdusbfsglobal1a7549aa_v1.rs"] pub mod gdusbfsglobal1a7549aa;
#[path="../../peripherals/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../../peripherals/gdwwdgtfa76971a_v1.rs"] pub mod gdwwdgtfa76971a;
