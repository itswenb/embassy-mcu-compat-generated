

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "19 - WWDGT"]
WWDGT = 19 , # [doc = "20 - LVD"]
LVD = 20 , # [doc = "21 - TAMPER"]
TAMPER = 21 , # [doc = "22 - RTC"]
RTC = 22 , # [doc = "23 - FMC"]
FMC = 23 , # [doc = "24 - RCU_CTC"]
RCU_CTC = 24 , # [doc = "25 - EXTI0"]
EXTI0 = 25 , # [doc = "26 - EXTI1"]
EXTI1 = 26 , # [doc = "27 - EXTI2"]
EXTI2 = 27 , # [doc = "28 - EXTI3"]
EXTI3 = 28 , # [doc = "29 - EXTI4"]
EXTI4 = 29 , # [doc = "30 - DMA0_CHANNEL0"]
DMA0_CHANNEL0 = 30 , # [doc = "31 - DMA0_CHANNEL1"]
DMA0_CHANNEL1 = 31 , # [doc = "32 - DMA0_CHANNEL2"]
DMA0_CHANNEL2 = 32 , # [doc = "33 - DMA0_CHANNEL3"]
DMA0_CHANNEL3 = 33 , # [doc = "34 - DMA0_CHANNEL4"]
DMA0_CHANNEL4 = 34 , # [doc = "35 - DMA0_CHANNEL5"]
DMA0_CHANNEL5 = 35 , # [doc = "36 - DMA0_CHANNEL6"]
DMA0_CHANNEL6 = 36 , # [doc = "37 - ADC0_1"]
ADC0_1 = 37 , # [doc = "38 - CAN0_TX"]
CAN0_TX = 38 , # [doc = "39 - CAN0_RX0"]
CAN0_RX0 = 39 , # [doc = "40 - CAN0_RX1"]
CAN0_RX1 = 40 , # [doc = "41 - CAN0_EWMC"]
CAN0_EWMC = 41 , # [doc = "42 - EXTI5_9"]
EXTI5_9 = 42 , # [doc = "43 - TIMER0_BRK"]
TIMER0_BRK = 43 , # [doc = "44 - TIMER0_UP"]
TIMER0_UP = 44 , # [doc = "45 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 45 , # [doc = "46 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 46 , # [doc = "47 - TIMER1"]
TIMER1 = 47 , # [doc = "48 - TIMER2"]
TIMER2 = 48 , # [doc = "49 - TIMER3"]
TIMER3 = 49 , # [doc = "50 - I2C0_EV"]
I2C0_EV = 50 , # [doc = "51 - I2C0_ER"]
I2C0_ER = 51 , # [doc = "52 - I2C1_EV"]
I2C1_EV = 52 , # [doc = "53 - I2C1_ER"]
I2C1_ER = 53 , # [doc = "54 - SPI0"]
SPI0 = 54 , # [doc = "55 - SPI1"]
SPI1 = 55 , # [doc = "56 - USART0"]
USART0 = 56 , # [doc = "57 - USART1"]
USART1 = 57 , # [doc = "58 - USART2"]
USART2 = 58 , # [doc = "59 - EXTI10_15"]
EXTI10_15 = 59 , # [doc = "60 - RTC_ALARM"]
RTC_ALARM = 60 , # [doc = "61 - USBFS_WKUP"]
USBFS_WKUP = 61 , # [doc = "69 - TIMER4"]
TIMER4 = 69 , # [doc = "70 - SPI2"]
SPI2 = 70 , # [doc = "71 - UART3"]
UART3 = 71 , # [doc = "72 - UART4"]
UART4 = 72 , # [doc = "73 - TIMER5"]
TIMER5 = 73 , # [doc = "74 - TIMER6"]
TIMER6 = 74 , # [doc = "75 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 75 , # [doc = "76 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 76 , # [doc = "77 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 77 , # [doc = "78 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 78 , # [doc = "79 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 79 , # [doc = "82 - CAN1_TX"]
CAN1_TX = 82 , # [doc = "83 - CAN1_RX0"]
CAN1_RX0 = 83 , # [doc = "84 - CAN1_RX1"]
CAN1_RX1 = 84 , # [doc = "85 - CAN1_EWMC"]
CAN1_EWMC = 85 , # [doc = "86 - USBFS"]
USBFS = 86 , } impl Interrupt { #[inline(always)] pub const fn number(self) -> u16 { self as u16 } }  pub const OB : gdob09cb4d52 :: Ob = unsafe { gdob09cb4d52 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi239d2229 :: Spi = unsafe { gdspi239d2229 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi239d2229 :: Spi = unsafe { gdspi239d2229 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c734aaed5 :: I2c = unsafe { gdi2c734aaed5 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c734aaed5 :: I2c = unsafe { gdi2c734aaed5 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkp7944b1bc :: Bkp = unsafe { gdbkp7944b1bc :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu97892901 :: Pmu = unsafe { gdpmu97892901 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac55126433 :: Dac = unsafe { gddac55126433 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const AFIO : gdafioa27eefcd :: Afio = unsafe { gdafioa27eefcd :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextif95225bb :: Exti = unsafe { gdextif95225bb :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioc14eca7d :: Gpio = unsafe { gdgpioc14eca7d :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const ADC0 : gdadce30ea086 :: Adc = unsafe { gdadce30ea086 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadce30ea086 :: Adc = unsafe { gdadce30ea086 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimerb17b3660 :: Timer = unsafe { gdtimerb17b3660 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi239d2229 :: Spi = unsafe { gdspi239d2229 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART0 : gdusartd2819c58 :: Usart = unsafe { gdusartd2819c58 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu5361e546 :: Rcu = unsafe { gdrcu5361e546 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcdc407917 :: Fmc = unsafe { gdfmcdc407917 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const EXMC : gdexmc9f914e53 :: Exmc = unsafe { gdexmc9f914e53 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg8cc9fb0b :: Dbg = unsafe { gddbg8cc9fb0b :: Dbg :: from_ptr (0xe004_2000usize as _) } ;  #[path="../../peripherals/gdadce30ea086_v1.rs"] pub mod gdadce30ea086;
#[path="../../peripherals/gdafioa27eefcd_v1.rs"] pub mod gdafioa27eefcd;
#[path="../../peripherals/gdbkp7944b1bc_v1.rs"] pub mod gdbkp7944b1bc;
#[path="../../peripherals/gdcan486a8ac4_v1.rs"] pub mod gdcan486a8ac4;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gddac55126433_v1.rs"] pub mod gddac55126433;
#[path="../../peripherals/gddbg8cc9fb0b_v1.rs"] pub mod gddbg8cc9fb0b;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gdexmc9f914e53_v1.rs"] pub mod gdexmc9f914e53;
#[path="../../peripherals/gdextif95225bb_v1.rs"] pub mod gdextif95225bb;
#[path="../../peripherals/gdfmcdc407917_v1.rs"] pub mod gdfmcdc407917;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpioc14eca7d_v1.rs"] pub mod gdgpioc14eca7d;
#[path="../../peripherals/gdi2c734aaed5_v1.rs"] pub mod gdi2c734aaed5;
#[path="../../peripherals/gdob09cb4d52_v1.rs"] pub mod gdob09cb4d52;
#[path="../../peripherals/gdpmu97892901_v1.rs"] pub mod gdpmu97892901;
#[path="../../peripherals/gdrcu5361e546_v1.rs"] pub mod gdrcu5361e546;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdspi239d2229_v1.rs"] pub mod gdspi239d2229;
#[path="../../peripherals/gdtimerb17b3660_v1.rs"] pub mod gdtimerb17b3660;
#[path="../../peripherals/gdusartd2819c58_v1.rs"] pub mod gdusartd2819c58;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
