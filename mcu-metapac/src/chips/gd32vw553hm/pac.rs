

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "19 - WWDGT"]
WWDGT = 19 , # [doc = "20 - LVD"]
LVD = 20 , # [doc = "21 - TAMPER_STAMP"]
TAMPER_STAMP = 21 , # [doc = "22 - RTC_WKUP"]
RTC_WKUP = 22 , # [doc = "23 - FMC"]
FMC = 23 , # [doc = "24 - RCU"]
RCU = 24 , # [doc = "25 - EXTI0"]
EXTI0 = 25 , # [doc = "26 - EXTI1"]
EXTI1 = 26 , # [doc = "27 - EXTI2"]
EXTI2 = 27 , # [doc = "28 - EXTI3"]
EXTI3 = 28 , # [doc = "29 - EXTI4"]
EXTI4 = 29 , # [doc = "30 - DMA_CHANNEL0"]
DMA_CHANNEL0 = 30 , # [doc = "31 - DMA_CHANNEL1"]
DMA_CHANNEL1 = 31 , # [doc = "32 - DMA_CHANNEL2"]
DMA_CHANNEL2 = 32 , # [doc = "33 - DMA_CHANNEL3"]
DMA_CHANNEL3 = 33 , # [doc = "34 - DMA_CHANNEL4"]
DMA_CHANNEL4 = 34 , # [doc = "35 - DMA_CHANNEL5"]
DMA_CHANNEL5 = 35 , # [doc = "36 - DMA_CHANNEL6"]
DMA_CHANNEL6 = 36 , # [doc = "37 - DMA_CHANNEL7"]
DMA_CHANNEL7 = 37 , # [doc = "38 - ADC"]
ADC = 38 , # [doc = "42 - EXTI5_9"]
EXTI5_9 = 42 , # [doc = "43 - TIMER0_BRK"]
TIMER0_BRK = 43 , # [doc = "44 - TIMER0_UP"]
TIMER0_UP = 44 , # [doc = "45 - TIMER0_CMT"]
TIMER0_CMT = 45 , # [doc = "46 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 46 , # [doc = "47 - TIMER1"]
TIMER1 = 47 , # [doc = "48 - TIMER2"]
TIMER2 = 48 , # [doc = "50 - I2C0_EV"]
I2C0_EV = 50 , # [doc = "51 - I2C0_ER"]
I2C0_ER = 51 , # [doc = "52 - I2C1_EV"]
I2C1_EV = 52 , # [doc = "53 - I2C1_ER"]
I2C1_ER = 53 , # [doc = "54 - SPI"]
SPI = 54 , # [doc = "56 - USART0"]
USART0 = 56 , # [doc = "57 - UART1"]
UART1 = 57 , # [doc = "58 - UART2"]
UART2 = 58 , # [doc = "59 - EXTI10_15"]
EXTI10_15 = 59 , # [doc = "60 - RTC_ALARM"]
RTC_ALARM = 60 , # [doc = "61 - VLVDF"]
VLVDF = 61 , # [doc = "63 - TIMER15"]
TIMER15 = 63 , # [doc = "64 - TIMER16"]
TIMER16 = 64 , # [doc = "70 - I2C0_WKUP"]
I2C0_WKUP = 70 , # [doc = "71 - USART0_WKUP"]
USART0_WKUP = 71 , # [doc = "73 - TIMER5"]
TIMER5 = 73 , # [doc = "74 - WIFI_TRIGGER"]
WIFI_TRIGGER = 74 , # [doc = "75 - WIFI_MAC"]
WIFI_MAC = 75 , # [doc = "76 - WIFI_TX"]
WIFI_TX = 76 , # [doc = "77 - WIFI_RX"]
WIFI_RX = 77 , # [doc = "83 - LA"]
LA = 83 , # [doc = "84 - WIFI_WKUP"]
WIFI_WKUP = 84 , # [doc = "85 - BLE_WKUP"]
BLE_WKUP = 85 , # [doc = "86 - PLATFORM_WAKE"]
PLATFORM_WAKE = 86 , # [doc = "87 - ISO_BT_STAMP0"]
ISO_BT_STAMP0 = 87 , # [doc = "88 - ISO_BT_STAMP1"]
ISO_BT_STAMP1 = 88 , # [doc = "89 - ISO_BT_STAMP2"]
ISO_BT_STAMP2 = 89 , # [doc = "90 - ISO_BT_STAMP3"]
ISO_BT_STAMP3 = 90 , # [doc = "91 - ISO_BT_STAMP4"]
ISO_BT_STAMP4 = 91 , # [doc = "92 - ISO_BT_STAMP5"]
ISO_BT_STAMP5 = 92 , # [doc = "93 - ISO_BT_STAMP6"]
ISO_BT_STAMP6 = 93 , # [doc = "94 - ISO_BT_STAMP7"]
ISO_BT_STAMP7 = 94 , # [doc = "95 - PMU"]
PMU = 95 , # [doc = "98 - CAU"]
CAU = 98 , # [doc = "99 - HAU_TRNG"]
HAU_TRNG = 99 , # [doc = "101 - WIFI_INT"]
WIFI_INT = 101 , # [doc = "102 - WIFI_SW_TRIG"]
WIFI_SW_TRIG = 102 , # [doc = "103 - WIFI_FINE_TIMER_TARGET"]
WIFI_FINE_TIMER_TARGET = 103 , # [doc = "104 - WIFI_STAMP_TARGET1"]
WIFI_STAMP_TARGET1 = 104 , # [doc = "105 - WIFI_STAMP_TARGET2"]
WIFI_STAMP_TARGET2 = 105 , # [doc = "106 - WIFI_STAMP_TARGET3"]
WIFI_STAMP_TARGET3 = 106 , # [doc = "107 - WIFI_ENCRYPTION_ENGINE"]
WIFI_ENCRYPTION_ENGINE = 107 , # [doc = "108 - WIFI_SLEEP_MODE"]
WIFI_SLEEP_MODE = 108 , # [doc = "109 - WIFI_HALF_SLOT"]
WIFI_HALF_SLOT = 109 , # [doc = "110 - WIFI_FIFO_ACTIVITY"]
WIFI_FIFO_ACTIVITY = 110 , # [doc = "111 - WIFI_ERROR"]
WIFI_ERROR = 111 , # [doc = "112 - WIFI_FREQ_SELECT"]
WIFI_FREQ_SELECT = 112 , # [doc = "113 - EFUSE"]
EFUSE = 113 , # [doc = "114 - QSPI"]
QSPI = 114 , # [doc = "115 - PKCAU"]
PKCAU = 115 , } impl Interrupt { #[inline(always)] pub const fn number(self) -> u16 { self as u16 } }  pub const TIMER1 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER5 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : gdrtc7c0d047d :: Rtc = unsafe { gdrtc7c0d047d :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const UART1 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART0 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu8ef89808 :: Pmu = unsafe { gdpmu8ef89808 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const TIMER0 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4001_0000usize as _) } ; pub const UART2 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4001_1000usize as _) } ; pub const ADC : gdadca9050599 :: Adc = unsafe { gdadca9050599 :: Adc :: from_ptr (0x4001_2000usize as _) } ; pub const SPI : gdspi25816acd :: Spi = unsafe { gdspi25816acd :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const SYSCFG : gdsyscfg51b128a8 :: Syscfg = unsafe { gdsyscfg51b128a8 :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdextia48fbf2e :: Exti = unsafe { gdextia48fbf2e :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER15 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4001_8000usize as _) } ; pub const TIMER16 : gdtimer5e62b6e6 :: Timer = unsafe { gdtimer5e62b6e6 :: Timer :: from_ptr (0x4001_8400usize as _) } ; pub const GPIOA : gdgpioe6fca7d9 :: Gpio = unsafe { gdgpioe6fca7d9 :: Gpio :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpioe6fca7d9 :: Gpio = unsafe { gdgpioe6fca7d9 :: Gpio :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpioe6fca7d9 :: Gpio = unsafe { gdgpioe6fca7d9 :: Gpio :: from_ptr (0x4002_0800usize as _) } ; pub const FMC : gdfmc523fbb53 :: Fmc = unsafe { gdfmc523fbb53 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const EFUSE : gdefuse4f36246c :: Efuse = unsafe { gdefuse4f36246c :: Efuse :: from_ptr (0x4002_2800usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcu0b3fa95b :: Rcu = unsafe { gdrcu0b3fa95b :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const QSPI : gdqspi9ca258b7 :: Qspi = unsafe { gdqspi9ca258b7 :: Qspi :: from_ptr (0x4002_5800usize as _) } ; pub const DMA : gddma1cf53887 :: Dma = unsafe { gddma1cf53887 :: Dma :: from_ptr (0x4002_6000usize as _) } ; pub const CAU : gdcau95f6f36a :: Cau = unsafe { gdcau95f6f36a :: Cau :: from_ptr (0x4c06_0000usize as _) } ; pub const HAU : gdhaub97c00c8 :: Hau = unsafe { gdhaub97c00c8 :: Hau :: from_ptr (0x4c06_0400usize as _) } ; pub const TRNG : gdtrng1f3ad225 :: Trng = unsafe { gdtrng1f3ad225 :: Trng :: from_ptr (0x4c06_0800usize as _) } ; pub const PKCAU : gdpkcauf9e1d63d :: Pkcau = unsafe { gdpkcauf9e1d63d :: Pkcau :: from_ptr (0x4c06_1000usize as _) } ; pub const DBG : gddbg4e46e6a0 :: Dbg = unsafe { gddbg4e46e6a0 :: Dbg :: from_ptr (0xe004_4000usize as _) } ;  #[path="../../peripherals/gdadca9050599_v1.rs"] pub mod gdadca9050599;
#[path="../../peripherals/gdcau95f6f36a_v1.rs"] pub mod gdcau95f6f36a;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gddbg4e46e6a0_v1.rs"] pub mod gddbg4e46e6a0;
#[path="../../peripherals/gddma1cf53887_v1.rs"] pub mod gddma1cf53887;
#[path="../../peripherals/gdefuse4f36246c_v1.rs"] pub mod gdefuse4f36246c;
#[path="../../peripherals/gdextia48fbf2e_v1.rs"] pub mod gdextia48fbf2e;
#[path="../../peripherals/gdfmc523fbb53_v1.rs"] pub mod gdfmc523fbb53;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpioe6fca7d9_v1.rs"] pub mod gdgpioe6fca7d9;
#[path="../../peripherals/gdhaub97c00c8_v1.rs"] pub mod gdhaub97c00c8;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdpkcauf9e1d63d_v1.rs"] pub mod gdpkcauf9e1d63d;
#[path="../../peripherals/gdpmu8ef89808_v1.rs"] pub mod gdpmu8ef89808;
#[path="../../peripherals/gdqspi9ca258b7_v1.rs"] pub mod gdqspi9ca258b7;
#[path="../../peripherals/gdrcu0b3fa95b_v1.rs"] pub mod gdrcu0b3fa95b;
#[path="../../peripherals/gdrtc7c0d047d_v1.rs"] pub mod gdrtc7c0d047d;
#[path="../../peripherals/gdspi25816acd_v1.rs"] pub mod gdspi25816acd;
#[path="../../peripherals/gdsyscfg51b128a8_v1.rs"] pub mod gdsyscfg51b128a8;
#[path="../../peripherals/gdtimer5e62b6e6_v1.rs"] pub mod gdtimer5e62b6e6;
#[path="../../peripherals/gdtrng1f3ad225_v1.rs"] pub mod gdtrng1f3ad225;
#[path="../../peripherals/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
