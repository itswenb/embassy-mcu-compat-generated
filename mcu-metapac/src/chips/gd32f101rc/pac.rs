




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
ADC0_1 = 18 , # [doc = "19 - USBD_HP_CAN0_TX"]
USBD_HP_CAN0_TX = 19 , # [doc = "20 - USBD_LP_CAN0_RX0"]
USBD_LP_CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
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
RTC_ALARM = 41 , # [doc = "42 - USBD_WKUP"]
USBD_WKUP = 42 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT"]
TIMER7_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - ADC2"]
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
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3_CHANNEL4"]
DMA1_CHANNEL3_CHANNEL4 = 59 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn USBD_HP_CAN0_TX () ; fn USBD_LP_CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBD_WKUP () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3_CHANNEL4 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 60]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : USBD_HP_CAN0_TX } , Vector { _handler : USBD_LP_CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBD_WKUP } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3_CHANNEL4 } ,]
; } pub const OB : gdob09cb4d52 :: Ob = unsafe { gdob09cb4d52 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi59720211 :: Spi = unsafe { gdspi59720211 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi59720211 :: Spi = unsafe { gdspi59720211 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2cfe81b9f6 :: I2c = unsafe { gdi2cfe81b9f6 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2cfe81b9f6 :: I2c = unsafe { gdi2cfe81b9f6 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcancb57dfc5 :: Can = unsafe { gdcancb57dfc5 :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcancb57dfc5 :: Can = unsafe { gdcancb57dfc5 :: Can :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkp7944b1bc :: Bkp = unsafe { gdbkp7944b1bc :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu97892901 :: Pmu = unsafe { gdpmu97892901 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac55126433 :: Dac = unsafe { gddac55126433 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const AFIO : gdafioe8de1fbd :: Afio = unsafe { gdafioe8de1fbd :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextia39263ea :: Exti = unsafe { gdextia39263ea :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadc3e4f2cd4 :: Adc = unsafe { gdadc3e4f2cd4 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc3e4f2cd4 :: Adc = unsafe { gdadc3e4f2cd4 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi59720211 :: Spi = unsafe { gdspi59720211 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadc3e4f2cd4 :: Adc = unsafe { gdadc3e4f2cd4 :: Adc :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const SDIO : gdsdio64bb917d :: Sdio = unsafe { gdsdio64bb917d :: Sdio :: from_ptr (0x4001_8000usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcub1e53c68 :: Rcu = unsafe { gdrcub1e53c68 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcf841e586 :: Fmc = unsafe { gdfmcf841e586 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET : gdenetb43a4ee4 :: Enet = unsafe { gdenetb43a4ee4 :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const EXMC : gdexmc1c05e84d :: Exmc = unsafe { gdexmc1c05e84d :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbgd3014d6f :: Dbg = unsafe { gddbgd3014d6f :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc3e4f2cd4_v1.rs"] pub mod gdadc3e4f2cd4;
#[path="../../peripherals/gdafioe8de1fbd_v1.rs"] pub mod gdafioe8de1fbd;
#[path="../../peripherals/gdbkp7944b1bc_v1.rs"] pub mod gdbkp7944b1bc;
#[path="../../peripherals/gdcancb57dfc5_v1.rs"] pub mod gdcancb57dfc5;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gddac55126433_v1.rs"] pub mod gddac55126433;
#[path="../../peripherals/gddbgd3014d6f_v1.rs"] pub mod gddbgd3014d6f;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdenetb43a4ee4_v1.rs"] pub mod gdenetb43a4ee4;
#[path="../../peripherals/gdexmc1c05e84d_v1.rs"] pub mod gdexmc1c05e84d;
#[path="../../peripherals/gdextia39263ea_v1.rs"] pub mod gdextia39263ea;
#[path="../../peripherals/gdfmcf841e586_v1.rs"] pub mod gdfmcf841e586;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpioc14eca7d_v1.rs"] pub mod gdgpioc14eca7d;
#[path="../../peripherals/gdi2cfe81b9f6_v1.rs"] pub mod gdi2cfe81b9f6;
#[path="../../peripherals/gdob09cb4d52_v1.rs"] pub mod gdob09cb4d52;
#[path="../../peripherals/gdpmu97892901_v1.rs"] pub mod gdpmu97892901;
#[path="../../peripherals/gdrcub1e53c68_v1.rs"] pub mod gdrcub1e53c68;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdsdio64bb917d_v1.rs"] pub mod gdsdio64bb917d;
#[path="../../peripherals/gdspi59720211_v1.rs"] pub mod gdspi59720211;
#[path="../../peripherals/gdtimerb17b3660_v1.rs"] pub mod gdtimerb17b3660;
#[path="../../peripherals/gdusartd2819c58_v1.rs"] pub mod gdusartd2819c58;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
