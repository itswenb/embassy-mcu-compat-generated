

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - RTC_TAMPER_TIMESTAMP"]
RTC_TAMPER_TIMESTAMP = 2 , # [doc = "3 - RTC_WAKE"]
RTC_WAKE = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI0"]
EXTI0 = 6 , # [doc = "7 - EXTI01"]
EXTI01 = 7 , # [doc = "8 - EXTI2"]
EXTI2 = 8 , # [doc = "9 - EXTI3"]
EXTI3 = 9 , # [doc = "10 - EXTI4"]
EXTI4 = 10 , # [doc = "11 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 11 , # [doc = "12 - DMA_CHANNEL1"]
DMA_CHANNEL1 = 12 , # [doc = "13 - DMA_CHANNEL2"]
DMA_CHANNEL2 = 13 , # [doc = "14 - DMA_CHANNEL3"]
DMA_CHANNEL3 = 14 , # [doc = "15 - DMA_CHANNEL4"]
DMA_CHANNEL4 = 15 , # [doc = "16 - DMA_CHANNEL5"]
DMA_CHANNEL5 = 16 , # [doc = "17 - DMA_CHANNEL6"]
DMA_CHANNEL6 = 17 , # [doc = "18 - ADC"]
ADC = 18 , # [doc = "19 - USBD_HP"]
USBD_HP = 19 , # [doc = "20 - USBD_LP"]
USBD_LP = 20 , # [doc = "21 - TIMER1"]
TIMER1 = 21 , # [doc = "22 - TIMER2"]
TIMER2 = 22 , # [doc = "23 - TIMER0_BRK_TIMER8"]
TIMER0_BRK_TIMER8 = 23 , # [doc = "24 - TIMER11"]
TIMER11 = 24 , # [doc = "25 - TIMER5"]
TIMER5 = 25 , # [doc = "26 - TIMER6"]
TIMER6 = 26 , # [doc = "27 - USART0"]
USART0 = 27 , # [doc = "28 - USART1"]
USART1 = 28 , # [doc = "29 - USART3"]
USART3 = 29 , # [doc = "30 - UART4"]
UART4 = 30 , # [doc = "31 - I2C0_EV"]
I2C0_EV = 31 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "33 - I2C1_EV"]
I2C1_EV = 33 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - SPI0"]
SPI0 = 35 , # [doc = "36 - SPI1"]
SPI1 = 36 , # [doc = "37 - DAC"]
DAC = 37 , # [doc = "39 - I2C2_EV"]
I2C2_EV = 39 , # [doc = "40 - I2C2_ER"]
I2C2_ER = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "43 - EXTI5_9"]
EXTI5_9 = 43 , # [doc = "47 - EXTI10_15"]
EXTI10_15 = 47 , # [doc = "55 - DMA_MUX"]
DMA_MUX = 55 , # [doc = "56 - CMP0"]
CMP0 = 56 , # [doc = "57 - CMP1"]
CMP1 = 57 , # [doc = "58 - I2C0_WAKE"]
I2C0_WAKE = 58 , # [doc = "59 - I2C2_WAKE"]
I2C2_WAKE = 59 , # [doc = "60 - USART0_WAKE"]
USART0_WAKE = 60 , # [doc = "61 - LPUART"]
LPUART = 61 , # [doc = "62 - CAU"]
CAU = 62 , # [doc = "63 - TRNG"]
TRNG = 63 , # [doc = "64 - SLCD"]
SLCD = 64 , # [doc = "65 - USART1_WAKE"]
USART1_WAKE = 65 , # [doc = "66 - I2C1_WAKE"]
I2C1_WAKE = 66 , # [doc = "67 - LPUART_WAKE"]
LPUART_WAKE = 67 , # [doc = "68 - LPTIMER"]
LPTIMER = 68 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC_TAMPER_TIMESTAMP () ; fn RTC_WAKE () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI01 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1 () ; fn DMA_CHANNEL2 () ; fn DMA_CHANNEL3 () ; fn DMA_CHANNEL4 () ; fn DMA_CHANNEL5 () ; fn DMA_CHANNEL6 () ; fn ADC () ; fn USBD_HP () ; fn USBD_LP () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER11 () ; fn TIMER5 () ; fn TIMER6 () ; fn USART0 () ; fn USART1 () ; fn USART3 () ; fn UART4 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn DAC () ; fn I2C2_EV () ; fn I2C2_ER () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn EXTI5_9 () ; fn EXTI10_15 () ; fn DMA_MUX () ; fn CMP0 () ; fn CMP1 () ; fn I2C0_WAKE () ; fn I2C2_WAKE () ; fn USART0_WAKE () ; fn LPUART () ; fn CAU () ; fn TRNG () ; fn SLCD () ; fn USART1_WAKE () ; fn I2C1_WAKE () ; fn LPUART_WAKE () ; fn LPTIMER () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 69]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC_TAMPER_TIMESTAMP } , Vector { _handler : RTC_WAKE } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI01 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1 } , Vector { _handler : DMA_CHANNEL2 } , Vector { _handler : DMA_CHANNEL3 } , Vector { _handler : DMA_CHANNEL4 } , Vector { _handler : DMA_CHANNEL5 } , Vector { _handler : DMA_CHANNEL6 } , Vector { _handler : ADC } , Vector { _handler : USBD_HP } , Vector { _handler : USBD_LP } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER11 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART3 } , Vector { _handler : UART4 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : DAC } , Vector { _reserved : 0 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _handler : EXTI5_9 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI10_15 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA_MUX } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : I2C0_WAKE } , Vector { _handler : I2C2_WAKE } , Vector { _handler : USART0_WAKE } , Vector { _handler : LPUART } , Vector { _handler : CAU } , Vector { _handler : TRNG } , Vector { _handler : SLCD } , Vector { _handler : USART1_WAKE } , Vector { _handler : I2C1_WAKE } , Vector { _handler : LPUART_WAKE } , Vector { _handler : LPTIMER } ,]
; } pub const TIMER1 : gdtimer15f311eaa :: Timer1 = unsafe { gdtimer15f311eaa :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer2000ed3f4 :: Timer2 = unsafe { gdtimer2000ed3f4 :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer6b465bb6b :: Timer6 = unsafe { gdtimer6b465bb6b :: Timer6 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer83f762be9 :: Timer8 = unsafe { gdtimer83f762be9 :: Timer8 :: from_ptr (0x4000_1800usize as _) } ; pub const SLCD : gdslcd8086d68f :: Slcd = unsafe { gdslcd8086d68f :: Slcd :: from_ptr (0x4000_2400usize as _) } ; pub const RTC : gdrtc30fffb52 :: Rtc = unsafe { gdrtc30fffb52 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgtdd622579 :: Wwdgt = unsafe { gdwwdgtdd622579 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt5932fb56 :: Fwdgt = unsafe { gdfwdgt5932fb56 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi19358bf74 :: Spi1 = unsafe { gdspi19358bf74 :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart0184abb20 :: Usart0 = unsafe { gdusart0184abb20 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const UART3 : gduart37add471e :: Uart3 = unsafe { gduart37add471e :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart4f478961c :: Uart4 = unsafe { gduart4f478961c :: Uart4 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const USBD : gdusbd3c6a50b5 :: Usbd = unsafe { gdusbd3c6a50b5 :: Usbd :: from_ptr (0x4000_5c00usize as _) } ; pub const PMU : gdpmu4fa21ce6 :: Pmu = unsafe { gdpmu4fa21ce6 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac7e57a629 :: Dac = unsafe { gddac7e57a629 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const LPUART : gdlpuart3ad1937d :: Lpuart = unsafe { gdlpuart3ad1937d :: Lpuart :: from_ptr (0x4000_8000usize as _) } ; pub const LPTIMER : gdlptimer81986a0b :: Lptimer = unsafe { gdlptimer81986a0b :: Lptimer :: from_ptr (0x4000_9400usize as _) } ; pub const I2C2 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_c000usize as _) } ; pub const CTC : gdctceaaaf458 :: Ctc = unsafe { gdctceaaaf458 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const SYSCFG : gdsyscfgce05548e :: Syscfg = unsafe { gdsyscfgce05548e :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const VREF : gdvref8ca405d5 :: Vref = unsafe { gdvref8ca405d5 :: Vref :: from_ptr (0x4001_0030usize as _) } ; pub const EXTI : gdexti30fc9668 :: Exti = unsafe { gdexti30fc9668 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadc34b106d5 :: Adc = unsafe { gdadc34b106d5 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const SPI0 : gdspi0cf000376 :: Spi0 = unsafe { gdspi0cf000376 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart0184abb20 :: Usart0 = unsafe { gdusart0184abb20 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER8 : gdtimer83f762be9 :: Timer8 = unsafe { gdtimer83f762be9 :: Timer8 :: from_ptr (0x4001_4c00usize as _) } ; pub const DBGMCU : gddbgmcu738c6f06 :: Dbgmcu = unsafe { gddbgmcu738c6f06 :: Dbgmcu :: from_ptr (0x4001_5800usize as _) } ; pub const CMP : gdcmpd90af10b :: Cmp = unsafe { gdcmpd90af10b :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const DMA : gddmaaff99f21 :: Dma = unsafe { gddmaaff99f21 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMAMUX : gddmamux75bc37af :: Dmamux = unsafe { gddmamux75bc37af :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcufeeb139f :: Rcu = unsafe { gdrcufeeb139f :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcef1b902c :: Fmc = unsafe { gdfmcef1b902c :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc67d273cb :: Crc = unsafe { gdcrc67d273cb :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob3a01cf30 :: Gpiob = unsafe { gdgpiob3a01cf30 :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioc0fba06c4 :: Gpioc = unsafe { gdgpioc0fba06c4 :: Gpioc :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpiod7229d923 :: Gpiod = unsafe { gdgpiod7229d923 :: Gpiod :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpiof7c6237df :: Gpiof = unsafe { gdgpiof7c6237df :: Gpiof :: from_ptr (0x4800_1400usize as _) } ; pub const CAU : gdcaue9e51f0c :: Cau = unsafe { gdcaue9e51f0c :: Cau :: from_ptr (0x5006_0000usize as _) } ; pub const TRNG : gdtrngbf61c352 :: Trng = unsafe { gdtrngbf61c352 :: Trng :: from_ptr (0x5006_0800usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc34b106d5_v1.rs"] pub mod gdadc34b106d5;
#[path="../../peripherals/gdcaue9e51f0c_v1.rs"] pub mod gdcaue9e51f0c;
#[path="../../peripherals/gdcmpd90af10b_v1.rs"] pub mod gdcmpd90af10b;
#[path="../../peripherals/gdcrc67d273cb_v1.rs"] pub mod gdcrc67d273cb;
#[path="../../peripherals/gdctceaaaf458_v1.rs"] pub mod gdctceaaaf458;
#[path="../../peripherals/gddac7e57a629_v1.rs"] pub mod gddac7e57a629;
#[path="../../peripherals/gddbgmcu738c6f06_v1.rs"] pub mod gddbgmcu738c6f06;
#[path="../../peripherals/gddmaaff99f21_v1.rs"] pub mod gddmaaff99f21;
#[path="../../peripherals/gddmamux75bc37af_v1.rs"] pub mod gddmamux75bc37af;
#[path="../../peripherals/gdexti30fc9668_v1.rs"] pub mod gdexti30fc9668;
#[path="../../peripherals/gdfmcef1b902c_v1.rs"] pub mod gdfmcef1b902c;
#[path="../../peripherals/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../../peripherals/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../../peripherals/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../../peripherals/gdgpioc0fba06c4_v1.rs"] pub mod gdgpioc0fba06c4;
#[path="../../peripherals/gdgpiod7229d923_v1.rs"] pub mod gdgpiod7229d923;
#[path="../../peripherals/gdgpiof7c6237df_v1.rs"] pub mod gdgpiof7c6237df;
#[path="../../peripherals/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../../peripherals/gdlptimer81986a0b_v1.rs"] pub mod gdlptimer81986a0b;
#[path="../../peripherals/gdlpuart3ad1937d_v1.rs"] pub mod gdlpuart3ad1937d;
#[path="../../peripherals/gdpmu4fa21ce6_v1.rs"] pub mod gdpmu4fa21ce6;
#[path="../../peripherals/gdrcufeeb139f_v1.rs"] pub mod gdrcufeeb139f;
#[path="../../peripherals/gdrtc30fffb52_v1.rs"] pub mod gdrtc30fffb52;
#[path="../../peripherals/gdslcd8086d68f_v1.rs"] pub mod gdslcd8086d68f;
#[path="../../peripherals/gdspi0cf000376_v1.rs"] pub mod gdspi0cf000376;
#[path="../../peripherals/gdspi19358bf74_v1.rs"] pub mod gdspi19358bf74;
#[path="../../peripherals/gdsyscfgce05548e_v1.rs"] pub mod gdsyscfgce05548e;
#[path="../../peripherals/gdtimer15f311eaa_v1.rs"] pub mod gdtimer15f311eaa;
#[path="../../peripherals/gdtimer2000ed3f4_v1.rs"] pub mod gdtimer2000ed3f4;
#[path="../../peripherals/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../../peripherals/gdtimer6b465bb6b_v1.rs"] pub mod gdtimer6b465bb6b;
#[path="../../peripherals/gdtimer83f762be9_v1.rs"] pub mod gdtimer83f762be9;
#[path="../../peripherals/gdtrngbf61c352_v1.rs"] pub mod gdtrngbf61c352;
#[path="../../peripherals/gduart37add471e_v1.rs"] pub mod gduart37add471e;
#[path="../../peripherals/gduart4f478961c_v1.rs"] pub mod gduart4f478961c;
#[path="../../peripherals/gdusart0184abb20_v1.rs"] pub mod gdusart0184abb20;
#[path="../../peripherals/gdusbd3c6a50b5_v1.rs"] pub mod gdusbd3c6a50b5;
#[path="../../peripherals/gdvref8ca405d5_v1.rs"] pub mod gdvref8ca405d5;
#[path="../../peripherals/gdwwdgtdd622579_v1.rs"] pub mod gdwwdgtdd622579;
