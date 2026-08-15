

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI_LINE0"]
EXTI_LINE0 = 6 , # [doc = "7 - EXTI_LINE1"]
EXTI_LINE1 = 7 , # [doc = "8 - EXTI_LINE2"]
EXTI_LINE2 = 8 , # [doc = "9 - EXTI_LINE3"]
EXTI_LINE3 = 9 , # [doc = "10 - EXTI_LINE4"]
EXTI_LINE4 = 10 , # [doc = "11 - DMA0_CHANNEL0"]
DMA0_CHANNEL0 = 11 , # [doc = "12 - DMA0_CHANNEL1"]
DMA0_CHANNEL1 = 12 , # [doc = "13 - DMA0_CHANNEL2"]
DMA0_CHANNEL2 = 13 , # [doc = "14 - DMA0_CHANNEL3"]
DMA0_CHANNEL3 = 14 , # [doc = "15 - DMA0_CHANNEL4"]
DMA0_CHANNEL4 = 15 , # [doc = "16 - DMA0_CHANNEL5"]
DMA0_CHANNEL5 = 16 , # [doc = "17 - DMA0_CHANNEL6"]
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "19 - USBD_HP_CAN0_TX"]
USBD_HP_CAN0_TX = 19 , # [doc = "20 - USBD_LP_CAN0_RX0"]
USBD_LP_CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK_TIMER8"]
TIMER0_BRK_TIMER8 = 24 , # [doc = "25 - TIMER0_UP_TIMER9"]
TIMER0_UP_TIMER9 = 25 , # [doc = "26 - TIMER0_TRG_CMT_TIMER10"]
TIMER0_TRG_CMT_TIMER10 = 26 , # [doc = "27 - TIMER0_CC"]
TIMER0_CC = 27 , # [doc = "28 - TIMER1"]
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
USART2 = 39 , # [doc = "40 - EXTI_LINE15_10"]
EXTI_LINE15_10 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CC"]
TIMER7_CC = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "49 - SDIO"]
SDIO = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3_4"]
DMA1_CHANNEL3_4 = 59 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn USBD_HP_CAN0_TX () ; fn USBD_LP_CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CC () ; fn ADC2 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3_4 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 60]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : USBD_HP_CAN0_TX } , Vector { _handler : USBD_LP_CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CC } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3_4 } ,]
; } pub const TIMER1 : gdtimer1b47aad48 :: Timer1 = unsafe { gdtimer1b47aad48 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer1b47aad48 :: Timer1 = unsafe { gdtimer1b47aad48 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer1b47aad48 :: Timer1 = unsafe { gdtimer1b47aad48 :: Timer1 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer1b47aad48 :: Timer1 = unsafe { gdtimer1b47aad48 :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer895e47fd0 :: Timer8 = unsafe { gdtimer895e47fd0 :: Timer8 :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer911ce6ab6 :: Timer9 = unsafe { gdtimer911ce6ab6 :: Timer9 :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer911ce6ab6 :: Timer9 = unsafe { gdtimer911ce6ab6 :: Timer9 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc6b0c077c :: Rtc = unsafe { gdrtc6b0c077c :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtdc3d0d7a :: Fwdgt = unsafe { gdfwdgtdc3d0d7a :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi0946ced9d :: Spi0 = unsafe { gdspi0946ced9d :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi0946ced9d :: Spi0 = unsafe { gdspi0946ced9d :: Spi0 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart082eeb6ea :: Usart0 = unsafe { gdusart082eeb6ea :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart082eeb6ea :: Usart0 = unsafe { gdusart082eeb6ea :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart3c678fe30 :: Uart3 = unsafe { gduart3c678fe30 :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart3c678fe30 :: Uart3 = unsafe { gduart3c678fe30 :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c037cbcb65 :: I2c0 = unsafe { gdi2c037cbcb65 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c037cbcb65 :: I2c0 = unsafe { gdi2c037cbcb65 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const USBD : gdusbd81a0e1ed :: Usbd = unsafe { gdusbd81a0e1ed :: Usbd :: from_ptr (0x4000_5c00usize as _) } ; pub const CAN0 : gdcan050e9510d :: Can0 = unsafe { gdcan050e9510d :: Can0 :: from_ptr (0x4000_6400usize as _) } ; pub const BKP : gdbkp16a620e0 :: Bkp = unsafe { gdbkp16a620e0 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu3bb0ce43 :: Pmu = unsafe { gdpmu3bb0ce43 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac1e2c6112 :: Dac = unsafe { gddac1e2c6112 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CTC : gdctc0cad8643 :: Ctc = unsafe { gdctc0cad8643 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const AFIO : gdafio0b6b322d :: Afio = unsafe { gdafio0b6b322d :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti11a1be47 :: Exti = unsafe { gdexti11a1be47 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadc0206d7fed :: Adc0 = unsafe { gdadc0206d7fed :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc1152dce23 :: Adc1 = unsafe { gdadc1152dce23 :: Adc1 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0a0aa2af0 :: Timer0 = unsafe { gdtimer0a0aa2af0 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0946ced9d :: Spi0 = unsafe { gdspi0946ced9d :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer0a0aa2af0 :: Timer0 = unsafe { gdtimer0a0aa2af0 :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart082eeb6ea :: Usart0 = unsafe { gdusart082eeb6ea :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadc1152dce23 :: Adc1 = unsafe { gdadc1152dce23 :: Adc1 :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimer895e47fd0 :: Timer8 = unsafe { gdtimer895e47fd0 :: Timer8 :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimer911ce6ab6 :: Timer9 = unsafe { gdtimer911ce6ab6 :: Timer9 :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimer911ce6ab6 :: Timer9 = unsafe { gdtimer911ce6ab6 :: Timer9 :: from_ptr (0x4001_5400usize as _) } ; pub const SDIO : gdsdio3e5180d0 :: Sdio = unsafe { gdsdio3e5180d0 :: Sdio :: from_ptr (0x4001_8000usize as _) } ; pub const DMA0 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu15523ba1 :: Rcu = unsafe { gdrcu15523ba1 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc2c447638 :: Fmc = unsafe { gdfmc2c447638 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc66a4f78d :: Crc = unsafe { gdcrc66a4f78d :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const EXMC : gdexmc9f6a36f3 :: Exmc = unsafe { gdexmc9f6a36f3 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg895e48a2 :: Dbg = unsafe { gddbg895e48a2 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc0206d7fed_v1.rs"] pub mod gdadc0206d7fed;
#[path="../../peripherals/gdadc1152dce23_v1.rs"] pub mod gdadc1152dce23;
#[path="../../peripherals/gdafio0b6b322d_v1.rs"] pub mod gdafio0b6b322d;
#[path="../../peripherals/gdbkp16a620e0_v1.rs"] pub mod gdbkp16a620e0;
#[path="../../peripherals/gdcan050e9510d_v1.rs"] pub mod gdcan050e9510d;
#[path="../../peripherals/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../../peripherals/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../../peripherals/gddac1e2c6112_v1.rs"] pub mod gddac1e2c6112;
#[path="../../peripherals/gddbg895e48a2_v1.rs"] pub mod gddbg895e48a2;
#[path="../../peripherals/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../../peripherals/gdexmc9f6a36f3_v1.rs"] pub mod gdexmc9f6a36f3;
#[path="../../peripherals/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../../peripherals/gdfmc2c447638_v1.rs"] pub mod gdfmc2c447638;
#[path="../../peripherals/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../../peripherals/gdgpioac3e5c224_v1.rs"] pub mod gdgpioac3e5c224;
#[path="../../peripherals/gdi2c037cbcb65_v1.rs"] pub mod gdi2c037cbcb65;
#[path="../../peripherals/gdpmu3bb0ce43_v1.rs"] pub mod gdpmu3bb0ce43;
#[path="../../peripherals/gdrcu15523ba1_v1.rs"] pub mod gdrcu15523ba1;
#[path="../../peripherals/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../../peripherals/gdsdio3e5180d0_v1.rs"] pub mod gdsdio3e5180d0;
#[path="../../peripherals/gdspi0946ced9d_v1.rs"] pub mod gdspi0946ced9d;
#[path="../../peripherals/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../../peripherals/gdtimer1b47aad48_v1.rs"] pub mod gdtimer1b47aad48;
#[path="../../peripherals/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../../peripherals/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../../peripherals/gdtimer911ce6ab6_v1.rs"] pub mod gdtimer911ce6ab6;
#[path="../../peripherals/gduart3c678fe30_v1.rs"] pub mod gduart3c678fe30;
#[path="../../peripherals/gdusart082eeb6ea_v1.rs"] pub mod gdusart082eeb6ea;
#[path="../../peripherals/gdusbd81a0e1ed_v1.rs"] pub mod gdusbd81a0e1ed;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
