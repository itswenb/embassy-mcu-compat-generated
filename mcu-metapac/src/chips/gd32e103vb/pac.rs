




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI0"]
EXTI0 = 6 , # [doc = "7 - EXTI1"]
EXTI1 = 7 , # [doc = "8 - EXTI2"]
EXTI2 = 8 , # [doc = "9 - EXTI3"]
EXTI3 = 9 , # [doc = "10 - EXTI4"]
EXTI4 = 10 , # [doc = "11 - DMA0_CHANNEL0"]
DMA0_CHANNEL0 = 11 , # [doc = "12 - DMA0_CHANNEL1"]
DMA0_CHANNEL1 = 12 , # [doc = "13 - DMA0_CHANNEL2"]
DMA0_CHANNEL2 = 13 , # [doc = "14 - DMA0_CHANNEL3"]
DMA0_CHANNEL3 = 14 , # [doc = "15 - DMA0_CHANNEL4"]
DMA0_CHANNEL4 = 15 , # [doc = "16 - DMA0_CHANNEL5"]
DMA0_CHANNEL5 = 16 , # [doc = "17 - DMA0_CHANNEL6"]
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK_TIMER8"]
TIMER0_BRK_TIMER8 = 24 , # [doc = "25 - TIMER0_UP_TIMER9"]
TIMER0_UP_TIMER9 = 25 , # [doc = "26 - TIMER0_TRG_CMT_TIMER10"]
TIMER0_TRG_CMT_TIMER10 = 26 , # [doc = "27 - TIMER0_CHANNEL"]
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
USART2 = 39 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "67 - USBFS"]
USBFS = 67 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn EXTI5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CHANNEL () ; fn EXMC () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn USBFS () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 68]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CHANNEL } , Vector { _reserved : 0 } , Vector { _handler : EXMC } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBFS } ,]
; } pub const OB : gdobe7648570 :: Ob = unsafe { gdobe7648570 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspib2f7804e :: Spi = unsafe { gdspib2f7804e :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspib2f7804e :: Spi = unsafe { gdspib2f7804e :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart7382b2aa :: Usart = unsafe { gdusart7382b2aa :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart7382b2aa :: Usart = unsafe { gdusart7382b2aa :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusart7382b2aa :: Usart = unsafe { gdusart7382b2aa :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusart7382b2aa :: Usart = unsafe { gdusart7382b2aa :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c7bdbe2ea :: I2c = unsafe { gdi2c7bdbe2ea :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c7bdbe2ea :: I2c = unsafe { gdi2c7bdbe2ea :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkpddaa24e5 :: Bkp = unsafe { gdbkpddaa24e5 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu58a8b7f0 :: Pmu = unsafe { gdpmu58a8b7f0 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac55126433 :: Dac = unsafe { gddac55126433 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const AFIO : gdafiof2fe1665 :: Afio = unsafe { gdafiof2fe1665 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextif95225bb :: Exti = unsafe { gdextif95225bb :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const ADC0 : gdadcae7321a4 :: Adc = unsafe { gdadcae7321a4 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcae7321a4 :: Adc = unsafe { gdadcae7321a4 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspib2f7804e :: Spi = unsafe { gdspib2f7804e :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart7382b2aa :: Usart = unsafe { gdusart7382b2aa :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER8 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimera084f39c :: Timer = unsafe { gdtimera084f39c :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcue3131045 :: Rcu = unsafe { gdrcue3131045 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmce69bcfc6 :: Fmc = unsafe { gdfmce69bcfc6 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const EXMC : gdexmce31be54c :: Exmc = unsafe { gdexmce31be54c :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg8d4c8040 :: Dbg = unsafe { gddbg8d4c8040 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcae7321a4_v1.rs"] pub mod gdadcae7321a4;
#[path="../../peripherals/gdafiof2fe1665_v1.rs"] pub mod gdafiof2fe1665;
#[path="../../peripherals/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddac55126433_v1.rs"] pub mod gddac55126433;
#[path="../../peripherals/gddbg8d4c8040_v1.rs"] pub mod gddbg8d4c8040;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdexmce31be54c_v1.rs"] pub mod gdexmce31be54c;
#[path="../../peripherals/gdextif95225bb_v1.rs"] pub mod gdextif95225bb;
#[path="../../peripherals/gdfmce69bcfc6_v1.rs"] pub mod gdfmce69bcfc6;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpio114d8126_v1.rs"] pub mod gdgpio114d8126;
#[path="../../peripherals/gdi2c7bdbe2ea_v1.rs"] pub mod gdi2c7bdbe2ea;
#[path="../../peripherals/gdobe7648570_v1.rs"] pub mod gdobe7648570;
#[path="../../peripherals/gdpmu58a8b7f0_v1.rs"] pub mod gdpmu58a8b7f0;
#[path="../../peripherals/gdrcue3131045_v1.rs"] pub mod gdrcue3131045;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdspib2f7804e_v1.rs"] pub mod gdspib2f7804e;
#[path="../../peripherals/gdtimera084f39c_v1.rs"] pub mod gdtimera084f39c;
#[path="../../peripherals/gdusart7382b2aa_v1.rs"] pub mod gdusart7382b2aa;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
