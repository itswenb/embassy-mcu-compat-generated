




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
ADC0_1 = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI5_9"]
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
RTC_ALARM = 41 , # [doc = "42 - USBHS_WKUP"]
USBHS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2_I2S2ADD"]
SPI2_I2S2ADD = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC"]
TIMER5_DAC = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - ENET"]
ENET = 61 , # [doc = "62 - ENET_WKUP"]
ENET_WKUP = 62 , # [doc = "63 - CAN1_TX"]
CAN1_TX = 63 , # [doc = "64 - CAN1_RX0"]
CAN1_RX0 = 64 , # [doc = "65 - CAN1_RX1"]
CAN1_RX1 = 65 , # [doc = "66 - CAN1_EWMC"]
CAN1_EWMC = 66 , # [doc = "67 - USBHS"]
USBHS = 67 , # [doc = "69 - SHRTIMER_IRQ2"]
SHRTIMER_IRQ2 = 69 , # [doc = "70 - SHRTIMER_IRQ3"]
SHRTIMER_IRQ3 = 70 , # [doc = "71 - SHRTIMER_IRQ4"]
SHRTIMER_IRQ4 = 71 , # [doc = "72 - SHRTIMER_IRQ5"]
SHRTIMER_IRQ5 = 72 , # [doc = "73 - SHRTIMER_IRQ6"]
SHRTIMER_IRQ6 = 73 , # [doc = "74 - USBHS_EP1_OUT"]
USBHS_EP1_OUT = 74 , # [doc = "75 - USBHS_EP1_IN"]
USBHS_EP1_IN = 75 , # [doc = "76 - SHRTIMER_IRQ0"]
SHRTIMER_IRQ0 = 76 , # [doc = "77 - SHRTIMER_IRQ1"]
SHRTIMER_IRQ1 = 77 , # [doc = "78 - CAN2_TX"]
CAN2_TX = 78 , # [doc = "79 - CAN2_RX0"]
CAN2_RX0 = 79 , # [doc = "80 - CAN2_RX1"]
CAN2_RX1 = 80 , # [doc = "81 - CAN2_EWMC"]
CAN2_EWMC = 81 , # [doc = "82 - I2C2_EV"]
I2C2_EV = 82 , # [doc = "83 - I2C2_ER"]
I2C2_ER = 83 , # [doc = "84 - USART5"]
USART5 = 84 , # [doc = "85 - I2C2_WKUP"]
I2C2_WKUP = 85 , # [doc = "86 - USART5_WKUP"]
USART5_WKUP = 86 , # [doc = "87 - TMU"]
TMU = 87 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1_I2S1ADD () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBHS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CHANNEL () ; fn EXMC () ; fn TIMER4 () ; fn SPI2_I2S2ADD () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn USBHS () ; fn SHRTIMER_IRQ2 () ; fn SHRTIMER_IRQ3 () ; fn SHRTIMER_IRQ4 () ; fn SHRTIMER_IRQ5 () ; fn SHRTIMER_IRQ6 () ; fn USBHS_EP1_OUT () ; fn USBHS_EP1_IN () ; fn SHRTIMER_IRQ0 () ; fn SHRTIMER_IRQ1 () ; fn CAN2_TX () ; fn CAN2_RX0 () ; fn CAN2_RX1 () ; fn CAN2_EWMC () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USART5 () ; fn I2C2_WKUP () ; fn USART5_WKUP () ; fn TMU () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 88]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1_I2S1ADD } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBHS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CHANNEL } , Vector { _reserved : 0 } , Vector { _handler : EXMC } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2_I2S2ADD } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : USBHS } , Vector { _reserved : 0 } , Vector { _handler : SHRTIMER_IRQ2 } , Vector { _handler : SHRTIMER_IRQ3 } , Vector { _handler : SHRTIMER_IRQ4 } , Vector { _handler : SHRTIMER_IRQ5 } , Vector { _handler : SHRTIMER_IRQ6 } , Vector { _handler : USBHS_EP1_OUT } , Vector { _handler : USBHS_EP1_IN } , Vector { _handler : SHRTIMER_IRQ0 } , Vector { _handler : SHRTIMER_IRQ1 } , Vector { _handler : CAN2_TX } , Vector { _handler : CAN2_RX0 } , Vector { _handler : CAN2_RX1 } , Vector { _handler : CAN2_EWMC } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USART5 } , Vector { _handler : I2C2_WKUP } , Vector { _handler : USART5_WKUP } , Vector { _handler : TMU } ,]
; } pub const OB : gdob717f2361 :: Ob = unsafe { gdob717f2361 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const I2S2_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_4000usize as _) } ; pub const USART1 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c59ded4aa :: I2c = unsafe { gdi2c59ded4aa :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c59ded4aa :: I2c = unsafe { gdi2c59ded4aa :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcan52a0fbba :: Can = unsafe { gdcan52a0fbba :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan52a0fbba :: Can = unsafe { gdcan52a0fbba :: Can :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkpddaa24e5 :: Bkp = unsafe { gdbkpddaa24e5 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu5b735bb1 :: Pmu = unsafe { gdpmu5b735bb1 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac2c324d90 :: Dac = unsafe { gddac2c324d90 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const I2C2 : gdi2c2566026ac :: I2c2 = unsafe { gdi2c2566026ac :: I2c2 :: from_ptr (0x4000_c000usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const CAN2 : gdcan52a0fbba :: Can = unsafe { gdcan52a0fbba :: Can :: from_ptr (0x4000_cc00usize as _) } ; pub const AFIO : gdafio3260312d :: Afio = unsafe { gdafio3260312d :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti9fc5df87 :: Exti = unsafe { gdexti9fc5df87 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpio114d8126 :: Gpio = unsafe { gdgpio114d8126 :: Gpio :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadcebf84092 :: Adc = unsafe { gdadcebf84092 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcebf84092 :: Adc = unsafe { gdadcebf84092 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi20dc9722 :: Spi = unsafe { gdspi20dc9722 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusartf581e00c :: Usart = unsafe { gdusartf581e00c :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER8 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimer894282e9 :: Timer = unsafe { gdtimer894282e9 :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const USART5 : gdusart58135de6a :: Usart5 = unsafe { gdusart58135de6a :: Usart5 :: from_ptr (0x4001_7000usize as _) } ; pub const SHRTIMER0 : gdshrtimerea23ae38 :: Shrtimer = unsafe { gdshrtimerea23ae38 :: Shrtimer :: from_ptr (0x4001_7400usize as _) } ; pub const CMP : gdcmpa4fd7736 :: Cmp = unsafe { gdcmpa4fd7736 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const SDIO : gdsdioa16a5588 :: Sdio = unsafe { gdsdioa16a5588 :: Sdio :: from_ptr (0x4001_8000usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu162113ac :: Rcu = unsafe { gdrcu162113ac :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc09ec7384 :: Fmc = unsafe { gdfmc09ec7384 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET : gdenet4408cf6f :: Enet = unsafe { gdenet4408cf6f :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const TMU : gdtmu810f0d96 :: Tmu = unsafe { gdtmu810f0d96 :: Tmu :: from_ptr (0x4008_0000usize as _) } ; pub const EXMC : gdexmc6eb28b9f :: Exmc = unsafe { gdexmc6eb28b9f :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const SQPI : gdsqpi47688f21 :: Sqpi = unsafe { gdsqpi47688f21 :: Sqpi :: from_ptr (0xa000_1000usize as _) } ; pub const DBG : gddbg0c729a76 :: Dbg = unsafe { gddbg0c729a76 :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcebf84092_v1.rs"] pub mod gdadcebf84092;
#[path="../../peripherals/gdafio3260312d_v1.rs"] pub mod gdafio3260312d;
#[path="../../peripherals/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../../peripherals/gdcan52a0fbba_v1.rs"] pub mod gdcan52a0fbba;
#[path="../../peripherals/gdcmpa4fd7736_v1.rs"] pub mod gdcmpa4fd7736;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddac2c324d90_v1.rs"] pub mod gddac2c324d90;
#[path="../../peripherals/gddbg0c729a76_v1.rs"] pub mod gddbg0c729a76;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdenet4408cf6f_v1.rs"] pub mod gdenet4408cf6f;
#[path="../../peripherals/gdexmc6eb28b9f_v1.rs"] pub mod gdexmc6eb28b9f;
#[path="../../peripherals/gdexti9fc5df87_v1.rs"] pub mod gdexti9fc5df87;
#[path="../../peripherals/gdfmc09ec7384_v1.rs"] pub mod gdfmc09ec7384;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpio114d8126_v1.rs"] pub mod gdgpio114d8126;
#[path="../../peripherals/gdi2c2566026ac_v1.rs"] pub mod gdi2c2566026ac;
#[path="../../peripherals/gdi2c59ded4aa_v1.rs"] pub mod gdi2c59ded4aa;
#[path="../../peripherals/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../../peripherals/gdob717f2361_v1.rs"] pub mod gdob717f2361;
#[path="../../peripherals/gdpmu5b735bb1_v1.rs"] pub mod gdpmu5b735bb1;
#[path="../../peripherals/gdrcu162113ac_v1.rs"] pub mod gdrcu162113ac;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../../peripherals/gdshrtimerea23ae38_v1.rs"] pub mod gdshrtimerea23ae38;
#[path="../../peripherals/gdspi20dc9722_v1.rs"] pub mod gdspi20dc9722;
#[path="../../peripherals/gdsqpi47688f21_v1.rs"] pub mod gdsqpi47688f21;
#[path="../../peripherals/gdtimer894282e9_v1.rs"] pub mod gdtimer894282e9;
#[path="../../peripherals/gdtmu810f0d96_v1.rs"] pub mod gdtmu810f0d96;
#[path="../../peripherals/gdusart58135de6a_v1.rs"] pub mod gdusart58135de6a;
#[path="../../peripherals/gdusartf581e00c_v1.rs"] pub mod gdusartf581e00c;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
