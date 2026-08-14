




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU"]
RCU = 5 , # [doc = "6 - EXTI0"]
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
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - ENET"]
ENET = 61 , # [doc = "62 - ENET_WKUP"]
ENET_WKUP = 62 , # [doc = "63 - CAN1_TX"]
CAN1_TX = 63 , # [doc = "64 - CAN1_RX0"]
CAN1_RX0 = 64 , # [doc = "65 - CAN1_RX1"]
CAN1_RX1 = 65 , # [doc = "66 - CAN1_EWMC"]
CAN1_EWMC = 66 , # [doc = "67 - USBFS"]
USBFS = 67 , # [doc = "69 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 69 , # [doc = "70 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 70 , # [doc = "71 - USART5"]
USART5 = 71 , # [doc = "72 - I2C2_EV"]
I2C2_EV = 72 , # [doc = "73 - I2C2_ER"]
I2C2_ER = 73 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "82 - UART6"]
UART6 = 82 , # [doc = "83 - UART7"]
UART7 = 83 , # [doc = "88 - TLI"]
TLI = 88 , # [doc = "89 - TLI_ER"]
TLI_ER = 89 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn USBFS () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn DCI () ; fn CAU () ; fn HAU_TRNG () ; fn UART6 () ; fn UART7 () ; fn TLI () ; fn TLI_ER () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 90]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : USBFS } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DCI } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _reserved : 0 } , Vector { _handler : UART6 } , Vector { _handler : UART7 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TLI } , Vector { _handler : TLI_ER } ,]
; } pub const OB : gdob09cb4d52 :: Ob = unsafe { gdob09cb4d52 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi19423642 :: Spi = unsafe { gdspi19423642 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi19423642 :: Spi = unsafe { gdspi19423642 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2cfe81b9f6 :: I2c = unsafe { gdi2cfe81b9f6 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2cfe81b9f6 :: I2c = unsafe { gdi2cfe81b9f6 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkp6d5289c8 :: Bkp = unsafe { gdbkp6d5289c8 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu97892901 :: Pmu = unsafe { gdpmu97892901 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac55126433 :: Dac = unsafe { gddac55126433 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4000_7c00usize as _) } ; pub const I2C2 : gdi2cfe81b9f6 :: I2c = unsafe { gdi2cfe81b9f6 :: I2c :: from_ptr (0x4000_c000usize as _) } ; pub const AFIO : gdafio47ec3ad5 :: Afio = unsafe { gdafio47ec3ad5 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextia39263ea :: Exti = unsafe { gdextia39263ea :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadcae7321a4 :: Adc = unsafe { gdadcae7321a4 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcae7321a4 :: Adc = unsafe { gdadcae7321a4 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi19423642 :: Spi = unsafe { gdspi19423642 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadcae7321a4 :: Adc = unsafe { gdadcae7321a4 :: Adc :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const LAYER0 : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const TLI : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const LAYER1 : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6880usize as _) } ; pub const USART5 : gdusart464fea75 :: Usart = unsafe { gdusart464fea75 :: Usart :: from_ptr (0x4001_7000usize as _) } ; pub const GPIOH : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_7400usize as _) } ; pub const GPIOI : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_7800usize as _) } ; pub const SDIO : gdsdioa16a5588 :: Sdio = unsafe { gdsdioa16a5588 :: Sdio :: from_ptr (0x4001_8000usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma19c19bbe4 :: Dma1 = unsafe { gddma19c19bbe4 :: Dma1 :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu09599178 :: Rcu = unsafe { gdrcu09599178 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcf841e586 :: Fmc = unsafe { gdfmcf841e586 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET : gdenetb43a4ee4 :: Enet = unsafe { gdenetb43a4ee4 :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const DCI : gddci6728f4f7 :: Dci = unsafe { gddci6728f4f7 :: Dci :: from_ptr (0x5005_0000usize as _) } ; pub const CAU : gdcau90d317ec :: Cau = unsafe { gdcau90d317ec :: Cau :: from_ptr (0x5006_0000usize as _) } ; pub const HAU : gdhau9223b7da :: Hau = unsafe { gdhau9223b7da :: Hau :: from_ptr (0x5006_0400usize as _) } ; pub const TRNG : gdtrng4f75162f :: Trng = unsafe { gdtrng4f75162f :: Trng :: from_ptr (0x5006_0800usize as _) } ; pub const EXMC_NOR_PSRAM : gdexmc6b188277 :: Exmc = unsafe { gdexmc6b188277 :: Exmc :: from_ptr (0x6000_0000usize as _) } ; pub const EXMC_NAND : gdexmc6b188277 :: Exmc = unsafe { gdexmc6b188277 :: Exmc :: from_ptr (0x7000_0000usize as _) } ; pub const EXMC_PCCARD : gdexmc6b188277 :: Exmc = unsafe { gdexmc6b188277 :: Exmc :: from_ptr (0x9000_0000usize as _) } ; pub const EXMC : gdexmc6b188277 :: Exmc = unsafe { gdexmc6b188277 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const EXMC_SDRAM : gdexmc6b188277 :: Exmc = unsafe { gdexmc6b188277 :: Exmc :: from_ptr (0xc000_0000usize as _) } ; pub const DBG : gddbg78785c44 :: Dbg = unsafe { gddbg78785c44 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcae7321a4_v1.rs"] pub mod gdadcae7321a4;
#[path="../../peripherals/gdafio47ec3ad5_v1.rs"] pub mod gdafio47ec3ad5;
#[path="../../peripherals/gdbkp6d5289c8_v1.rs"] pub mod gdbkp6d5289c8;
#[path="../../peripherals/gdcan486a8ac4_v1.rs"] pub mod gdcan486a8ac4;
#[path="../../peripherals/gdcau90d317ec_v1.rs"] pub mod gdcau90d317ec;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gddac55126433_v1.rs"] pub mod gddac55126433;
#[path="../../peripherals/gddbg78785c44_v1.rs"] pub mod gddbg78785c44;
#[path="../../peripherals/gddci6728f4f7_v1.rs"] pub mod gddci6728f4f7;
#[path="../../peripherals/gddma19c19bbe4_v1.rs"] pub mod gddma19c19bbe4;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdenetb43a4ee4_v1.rs"] pub mod gdenetb43a4ee4;
#[path="../../peripherals/gdexmc6b188277_v1.rs"] pub mod gdexmc6b188277;
#[path="../../peripherals/gdextia39263ea_v1.rs"] pub mod gdextia39263ea;
#[path="../../peripherals/gdfmcf841e586_v1.rs"] pub mod gdfmcf841e586;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpioc14eca7d_v1.rs"] pub mod gdgpioc14eca7d;
#[path="../../peripherals/gdhau9223b7da_v1.rs"] pub mod gdhau9223b7da;
#[path="../../peripherals/gdi2cfe81b9f6_v1.rs"] pub mod gdi2cfe81b9f6;
#[path="../../peripherals/gdob09cb4d52_v1.rs"] pub mod gdob09cb4d52;
#[path="../../peripherals/gdpmu97892901_v1.rs"] pub mod gdpmu97892901;
#[path="../../peripherals/gdrcu09599178_v1.rs"] pub mod gdrcu09599178;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../../peripherals/gdspi19423642_v1.rs"] pub mod gdspi19423642;
#[path="../../peripherals/gdtimerb17b3660_v1.rs"] pub mod gdtimerb17b3660;
#[path="../../peripherals/gdtli94583ddb_v1.rs"] pub mod gdtli94583ddb;
#[path="../../peripherals/gdtrng4f75162f_v1.rs"] pub mod gdtrng4f75162f;
#[path="../../peripherals/gdusart464fea75_v1.rs"] pub mod gdusart464fea75;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
