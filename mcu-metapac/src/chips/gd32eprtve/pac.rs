




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
ADC0_1 = 18 , # [doc = "19 - USBD_HP"]
USBD_HP = 19 , # [doc = "20 - USBD_LP"]
USBD_LP = 20 , # [doc = "23 - EXTI5_9"]
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
SPI0 = 35 , # [doc = "36 - SPI1_I2S1ADD"]
SPI1_I2S1ADD = 36 , # [doc = "37 - USART0"]
USART0 = 37 , # [doc = "38 - USART1"]
USART1 = 38 , # [doc = "39 - USART2"]
USART2 = 39 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT"]
TIMER7_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2_I2S2ADD"]
SPI2_I2S2ADD = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC"]
TIMER5_DAC = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3_CHANNEL4"]
DMA1_CHANNEL3_CHANNEL4 = 59 , # [doc = "61 - ENET"]
ENET = 61 , # [doc = "62 - ENET_WKUP"]
ENET_WKUP = 62 , # [doc = "82 - I2C2_EV"]
I2C2_EV = 82 , # [doc = "83 - I2C2_ER"]
I2C2_ER = 83 , # [doc = "84 - USART5"]
USART5 = 84 , # [doc = "85 - I2C2_WKUP"]
I2C2_WKUP = 85 , # [doc = "86 - USART5_WKUP"]
USART5_WKUP = 86 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn USBD_HP () ; fn USBD_LP () ; fn EXTI5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1_I2S1ADD () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn EXMC () ; fn TIMER4 () ; fn SPI2_I2S2ADD () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USART5 () ; fn I2C2_WKUP () ; fn USART5_WKUP () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 87]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : USBD_HP } , Vector { _handler : USBD_LP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1_I2S1ADD } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2_I2S2ADD } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USART5 } , Vector { _handler : I2C2_WKUP } , Vector { _handler : USART5_WKUP } ,]
; } pub const OB : gdob6b3dc746 :: Ob = unsafe { gdob6b3dc746 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const I2S2_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_4000usize as _) } ; pub const USART1 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c642b76e5 :: I2c = unsafe { gdi2c642b76e5 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c642b76e5 :: I2c = unsafe { gdi2c642b76e5 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkpddaa24e5 :: Bkp = unsafe { gdbkpddaa24e5 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu5b735bb1 :: Pmu = unsafe { gdpmu5b735bb1 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddacb803d6dd :: Dac = unsafe { gddacb803d6dd :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const I2C2 : gdi2c2566026ac :: I2c2 = unsafe { gdi2c2566026ac :: I2c2 :: from_ptr (0x4000_c000usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const DAC1 : gddacb803d6dd :: Dac = unsafe { gddacb803d6dd :: Dac :: from_ptr (0x4000_d000usize as _) } ; pub const AFIO : gdafio16e5c907 :: Afio = unsafe { gdafio16e5c907 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti9fc5df87 :: Exti = unsafe { gdexti9fc5df87 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadcebf84092 :: Adc = unsafe { gdadcebf84092 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcebf84092 :: Adc = unsafe { gdadcebf84092 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadcebf84092 :: Adc = unsafe { gdadcebf84092 :: Adc :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER14 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const TIMER8 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimer8c4a32fc :: Timer = unsafe { gdtimer8c4a32fc :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const USART5 : gdusart58135de6a :: Usart5 = unsafe { gdusart58135de6a :: Usart5 :: from_ptr (0x4001_7000usize as _) } ; pub const SHRTIMER0 : gdshrtimerea23ae38 :: Shrtimer = unsafe { gdshrtimerea23ae38 :: Shrtimer :: from_ptr (0x4001_7400usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu8160eaaf :: Rcu = unsafe { gdrcu8160eaaf :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmccdfbdbb8 :: Fmc = unsafe { gdfmccdfbdbb8 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET : gdenet4408cf6f :: Enet = unsafe { gdenet4408cf6f :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const EXMC : gdexmc6eb28b9f :: Exmc = unsafe { gdexmc6eb28b9f :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const SQPI : gdsqpi47688f21 :: Sqpi = unsafe { gdsqpi47688f21 :: Sqpi :: from_ptr (0xa000_1000usize as _) } ; pub const DBG : gddbg95ed6038 :: Dbg = unsafe { gddbg95ed6038 :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcebf84092_v1.rs"] pub mod gdadcebf84092;
#[path="../../peripherals/gdafio16e5c907_v1.rs"] pub mod gdafio16e5c907;
#[path="../../peripherals/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddacb803d6dd_v1.rs"] pub mod gddacb803d6dd;
#[path="../../peripherals/gddbg95ed6038_v1.rs"] pub mod gddbg95ed6038;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdenet4408cf6f_v1.rs"] pub mod gdenet4408cf6f;
#[path="../../peripherals/gdexmc6eb28b9f_v1.rs"] pub mod gdexmc6eb28b9f;
#[path="../../peripherals/gdexti9fc5df87_v1.rs"] pub mod gdexti9fc5df87;
#[path="../../peripherals/gdfmccdfbdbb8_v1.rs"] pub mod gdfmccdfbdbb8;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpio114d8126_v1.rs"] pub mod gdgpio114d8126;
#[path="../../peripherals/gdi2c2566026ac_v1.rs"] pub mod gdi2c2566026ac;
#[path="../../peripherals/gdi2c642b76e5_v1.rs"] pub mod gdi2c642b76e5;
#[path="../../peripherals/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../../peripherals/gdob6b3dc746_v1.rs"] pub mod gdob6b3dc746;
#[path="../../peripherals/gdpmu5b735bb1_v1.rs"] pub mod gdpmu5b735bb1;
#[path="../../peripherals/gdrcu8160eaaf_v1.rs"] pub mod gdrcu8160eaaf;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdshrtimerea23ae38_v1.rs"] pub mod gdshrtimerea23ae38;
#[path="../../peripherals/gdspi20dc9722_v1.rs"] pub mod gdspi20dc9722;
#[path="../../peripherals/gdsqpi47688f21_v1.rs"] pub mod gdsqpi47688f21;
#[path="../../peripherals/gdtimer8c4a32fc_v1.rs"] pub mod gdtimer8c4a32fc;
#[path="../../peripherals/gdusart58135de6a_v1.rs"] pub mod gdusart58135de6a;
#[path="../../peripherals/gdusartf581e00c_v1.rs"] pub mod gdusartf581e00c;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
