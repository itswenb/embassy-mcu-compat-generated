




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - TAMPER_STAMP"]
TAMPER_STAMP = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI0"]
EXTI0 = 6 , # [doc = "7 - EXTI1"]
EXTI1 = 7 , # [doc = "8 - EXTI2"]
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
ADC = 18 , # [doc = "19 - USBD_HP_CAN_TX"]
USBD_HP_CAN_TX = 19 , # [doc = "20 - USBD_LP_CAN_RX0"]
USBD_LP_CAN_RX0 = 20 , # [doc = "21 - TIMER1"]
TIMER1 = 21 , # [doc = "22 - TIMER2"]
TIMER2 = 22 , # [doc = "23 - TIMER8"]
TIMER8 = 23 , # [doc = "24 - TIMER11"]
TIMER11 = 24 , # [doc = "25 - TIMER5"]
TIMER5 = 25 , # [doc = "26 - TIMER6"]
TIMER6 = 26 , # [doc = "27 - USART0"]
USART0 = 27 , # [doc = "28 - USART1"]
USART1 = 28 , # [doc = "29 - UART3"]
UART3 = 29 , # [doc = "30 - UART4"]
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
EXTI5_9 = 43 , # [doc = "44 - TIMER0_TRG_CMT_UP_BRK"]
TIMER0_TRG_CMT_UP_BRK = 44 , # [doc = "45 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 45 , # [doc = "46 - TIMER14"]
TIMER14 = 46 , # [doc = "47 - EXTI10_15"]
EXTI10_15 = 47 , # [doc = "48 - TIMER40"]
TIMER40 = 48 , # [doc = "49 - CAN_RX1"]
CAN_RX1 = 49 , # [doc = "50 - CAN_EWMC"]
CAN_EWMC = 50 , # [doc = "55 - DMAMUX"]
DMAMUX = 55 , # [doc = "56 - CMP0"]
CMP0 = 56 , # [doc = "57 - CMP1"]
CMP1 = 57 , # [doc = "58 - I2C0_WKUP"]
I2C0_WKUP = 58 , # [doc = "59 - I2C2_WKUP"]
I2C2_WKUP = 59 , # [doc = "60 - USART0_WKUP"]
USART0_WKUP = 60 , # [doc = "61 - LPUART0"]
LPUART0 = 61 , # [doc = "62 - CAU"]
CAU = 62 , # [doc = "63 - TRNG"]
TRNG = 63 , # [doc = "64 - SLCD"]
SLCD = 64 , # [doc = "65 - USART1_WKUP"]
USART1_WKUP = 65 , # [doc = "66 - I2C1_WKUP"]
I2C1_WKUP = 66 , # [doc = "67 - LPUART0_WKUP"]
LPUART0_WKUP = 67 , # [doc = "68 - LPTIMER0"]
LPTIMER0 = 68 , # [doc = "69 - LPUART1_WKUP"]
LPUART1_WKUP = 69 , # [doc = "70 - LPTIMER1"]
LPTIMER1 = 70 , # [doc = "71 - LPUART1"]
LPUART1 = 71 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER_STAMP () ; fn RTC_WKUP () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA_CHANNEL0 () ; fn DMA_CHANNEL1 () ; fn DMA_CHANNEL2 () ; fn DMA_CHANNEL3 () ; fn DMA_CHANNEL4 () ; fn DMA_CHANNEL5 () ; fn DMA_CHANNEL6 () ; fn ADC () ; fn USBD_HP_CAN_TX () ; fn USBD_LP_CAN_RX0 () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER8 () ; fn TIMER11 () ; fn TIMER5 () ; fn TIMER6 () ; fn USART0 () ; fn USART1 () ; fn UART3 () ; fn UART4 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn DAC () ; fn I2C2_EV () ; fn I2C2_ER () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn EXTI5_9 () ; fn TIMER0_TRG_CMT_UP_BRK () ; fn TIMER0_CHANNEL () ; fn TIMER14 () ; fn EXTI10_15 () ; fn TIMER40 () ; fn CAN_RX1 () ; fn CAN_EWMC () ; fn DMAMUX () ; fn CMP0 () ; fn CMP1 () ; fn I2C0_WKUP () ; fn I2C2_WKUP () ; fn USART0_WKUP () ; fn LPUART0 () ; fn CAU () ; fn TRNG () ; fn SLCD () ; fn USART1_WKUP () ; fn I2C1_WKUP () ; fn LPUART0_WKUP () ; fn LPTIMER0 () ; fn LPUART1_WKUP () ; fn LPTIMER1 () ; fn LPUART1 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 72]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER_STAMP } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA_CHANNEL0 } , Vector { _handler : DMA_CHANNEL1 } , Vector { _handler : DMA_CHANNEL2 } , Vector { _handler : DMA_CHANNEL3 } , Vector { _handler : DMA_CHANNEL4 } , Vector { _handler : DMA_CHANNEL5 } , Vector { _handler : DMA_CHANNEL6 } , Vector { _handler : ADC } , Vector { _handler : USBD_HP_CAN_TX } , Vector { _handler : USBD_LP_CAN_RX0 } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER8 } , Vector { _handler : TIMER11 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : DAC } , Vector { _reserved : 0 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_TRG_CMT_UP_BRK } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER14 } , Vector { _handler : EXTI10_15 } , Vector { _handler : TIMER40 } , Vector { _handler : CAN_RX1 } , Vector { _handler : CAN_EWMC } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMAMUX } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : I2C0_WKUP } , Vector { _handler : I2C2_WKUP } , Vector { _handler : USART0_WKUP } , Vector { _handler : LPUART0 } , Vector { _handler : CAU } , Vector { _handler : TRNG } , Vector { _handler : SLCD } , Vector { _handler : USART1_WKUP } , Vector { _handler : I2C1_WKUP } , Vector { _handler : LPUART0_WKUP } , Vector { _handler : LPTIMER0 } , Vector { _handler : LPUART1_WKUP } , Vector { _handler : LPTIMER1 } , Vector { _handler : LPUART1 } ,]
; } pub const OB : gdob52d5f4ba :: Ob = unsafe { gdob52d5f4ba :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const SLCD : gdslcdedd3ff4a :: Slcd = unsafe { gdslcdedd3ff4a :: Slcd :: from_ptr (0x4000_2400usize as _) } ; pub const RTC : gdrtca0051ad5 :: Rtc = unsafe { gdrtca0051ad5 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgtf694703e :: Wwdgt = unsafe { gdwwdgtf694703e :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi3e72f252 :: Spi = unsafe { gdspi3e72f252 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const LPUART1 : gdlpuart39bfce16 :: Lpuart = unsafe { gdlpuart39bfce16 :: Lpuart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const CAN : gdcana4e40015 :: Can = unsafe { gdcana4e40015 :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const PMU : gdpmuc5ab2b18 :: Pmu = unsafe { gdpmuc5ab2b18 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac79dc5606 :: Dac = unsafe { gddac79dc5606 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const LPTIMER1 : gdlptimerea2749f2 :: Lptimer = unsafe { gdlptimerea2749f2 :: Lptimer :: from_ptr (0x4000_7c00usize as _) } ; pub const LPUART0 : gdlpuart39bfce16 :: Lpuart = unsafe { gdlpuart39bfce16 :: Lpuart :: from_ptr (0x4000_8000usize as _) } ; pub const LPTIMER0 : gdlptimerea2749f2 :: Lptimer = unsafe { gdlptimerea2749f2 :: Lptimer :: from_ptr (0x4000_9400usize as _) } ; pub const I2C2 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_c000usize as _) } ; pub const CTC : gdctc99079953 :: Ctc = unsafe { gdctc99079953 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const SYSCFG : gdsyscfg1c7962aa :: Syscfg = unsafe { gdsyscfg1c7962aa :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const VREF : gdvref997d2775 :: Vref = unsafe { gdvref997d2775 :: Vref :: from_ptr (0x4001_0030usize as _) } ; pub const EXTI : gdextib9f9062f :: Exti = unsafe { gdextib9f9062f :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC : gdadcdc796783 :: Adc = unsafe { gdadcdc796783 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi3e72f252 :: Spi = unsafe { gdspi3e72f252 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER8 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const DBG : gddbg170bd90d :: Dbg = unsafe { gddbg170bd90d :: Dbg :: from_ptr (0x4001_5800usize as _) } ; pub const CMP : gdcmp13366a93 :: Cmp = unsafe { gdcmp13366a93 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const TIMER40 : gdtimer50c14f09 :: Timer = unsafe { gdtimer50c14f09 :: Timer :: from_ptr (0x4001_d000usize as _) } ; pub const DMA : gddma203b2e8a :: Dma = unsafe { gddma203b2e8a :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA_CHXCTL_BASE : gddmachxctlbase9fc231ae :: DmaChxctlBase = unsafe { gddmachxctlbase9fc231ae :: DmaChxctlBase :: from_ptr (0x4002_0008usize as _) } ; pub const DMA_CHXCNT_BASE : gddmachxcntbased390cdb4 :: DmaChxcntBase = unsafe { gddmachxcntbased390cdb4 :: DmaChxcntBase :: from_ptr (0x4002_000cusize as _) } ; pub const DMA_CHXPADDR_BASE : gddmachxpaddrbase24a24737 :: DmaChxpaddrBase = unsafe { gddmachxpaddrbase24a24737 :: DmaChxpaddrBase :: from_ptr (0x4002_0010usize as _) } ; pub const DMA_CHXMADDR_BASE : gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase = unsafe { gddmachxmaddrbase53fbca93 :: DmaChxmaddrBase :: from_ptr (0x4002_0014usize as _) } ; pub const DMAMUX : gddmamuxed8f0489 :: Dmamux = unsafe { gddmamuxed8f0489 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbasebf2fe519 :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbasebf2fe519 :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcu345af9ab :: Rcu = unsafe { gdrcu345af9ab :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc54bf11b7 :: Fmc = unsafe { gdfmc54bf11b7 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOF : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const CAU : gdcau0732936f :: Cau = unsafe { gdcau0732936f :: Cau :: from_ptr (0x5006_0000usize as _) } ; pub const TRNG : gdtrng13872700 :: Trng = unsafe { gdtrng13872700 :: Trng :: from_ptr (0x5006_0800usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcdc796783_v1.rs"] pub mod gdadcdc796783;
#[path="../../peripherals/gdcana4e40015_v1.rs"] pub mod gdcana4e40015;
#[path="../../peripherals/gdcau0732936f_v1.rs"] pub mod gdcau0732936f;
#[path="../../peripherals/gdcmp13366a93_v1.rs"] pub mod gdcmp13366a93;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gdctc99079953_v1.rs"] pub mod gdctc99079953;
#[path="../../peripherals/gddac79dc5606_v1.rs"] pub mod gddac79dc5606;
#[path="../../peripherals/gddbg170bd90d_v1.rs"] pub mod gddbg170bd90d;
#[path="../../peripherals/gddma203b2e8a_v1.rs"] pub mod gddma203b2e8a;
#[path="../../peripherals/gddmachxcntbased390cdb4_v1.rs"] pub mod gddmachxcntbased390cdb4;
#[path="../../peripherals/gddmachxctlbase9fc231ae_v1.rs"] pub mod gddmachxctlbase9fc231ae;
#[path="../../peripherals/gddmachxmaddrbase53fbca93_v1.rs"] pub mod gddmachxmaddrbase53fbca93;
#[path="../../peripherals/gddmachxpaddrbase24a24737_v1.rs"] pub mod gddmachxpaddrbase24a24737;
#[path="../../peripherals/gddmamuxed8f0489_v1.rs"] pub mod gddmamuxed8f0489;
#[path="../../peripherals/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../../peripherals/gddmamuxrmchxcfgbasebf2fe519_v1.rs"] pub mod gddmamuxrmchxcfgbasebf2fe519;
#[path="../../peripherals/gdextib9f9062f_v1.rs"] pub mod gdextib9f9062f;
#[path="../../peripherals/gdfmc54bf11b7_v1.rs"] pub mod gdfmc54bf11b7;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdlptimerea2749f2_v1.rs"] pub mod gdlptimerea2749f2;
#[path="../../peripherals/gdlpuart39bfce16_v1.rs"] pub mod gdlpuart39bfce16;
#[path="../../peripherals/gdob52d5f4ba_v1.rs"] pub mod gdob52d5f4ba;
#[path="../../peripherals/gdpmuc5ab2b18_v1.rs"] pub mod gdpmuc5ab2b18;
#[path="../../peripherals/gdrcu345af9ab_v1.rs"] pub mod gdrcu345af9ab;
#[path="../../peripherals/gdrtca0051ad5_v1.rs"] pub mod gdrtca0051ad5;
#[path="../../peripherals/gdslcdedd3ff4a_v1.rs"] pub mod gdslcdedd3ff4a;
#[path="../../peripherals/gdspi3e72f252_v1.rs"] pub mod gdspi3e72f252;
#[path="../../peripherals/gdsyscfg1c7962aa_v1.rs"] pub mod gdsyscfg1c7962aa;
#[path="../../peripherals/gdtimer50c14f09_v1.rs"] pub mod gdtimer50c14f09;
#[path="../../peripherals/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../../peripherals/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../../peripherals/gdvref997d2775_v1.rs"] pub mod gdvref997d2775;
#[path="../../peripherals/gdwwdgtf694703e_v1.rs"] pub mod gdwwdgtf694703e;
