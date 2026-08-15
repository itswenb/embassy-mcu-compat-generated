

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - RTC_TAMPER"]
RTC_TAMPER = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "5 - RCU"]
RCU = 5 , # [doc = "6 - EXTI_LINE0"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - DMA0_CHANNEL7"]
DMA0_CHANNEL7 = 18 , # [doc = "19 - ADC"]
ADC = 19 , # [doc = "20 - RTC_TAMPER_SEC"]
RTC_TAMPER_SEC = 20 , # [doc = "21 - RTC_WKUP_SEC"]
RTC_WKUP_SEC = 21 , # [doc = "22 - RTC_ALARM_SEC"]
RTC_ALARM_SEC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0COM"]
TIMER0COM = 26 , # [doc = "27 - TIMER0_CC"]
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
RTC_ALARM = 41 , # [doc = "44 - TIMER15"]
TIMER15 = 44 , # [doc = "45 - TIMER16"]
TIMER16 = 45 , # [doc = "49 - SDIO"]
SDIO = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - I2C0_WK"]
I2C0_WK = 51 , # [doc = "52 - USART0_WK"]
USART0_WK = 52 , # [doc = "53 - USART2_WK"]
USART2_WK = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 61 , # [doc = "62 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 62 , # [doc = "63 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 63 , # [doc = "67 - USB_FS"]
USB_FS = 67 , # [doc = "76 - USB_FS_WKUP"]
USB_FS_WKUP = 76 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "89 - HPDF0"]
HPDF0 = 89 , # [doc = "90 - HPDF1"]
HPDF1 = 90 , # [doc = "94 - EFUSE"]
EFUSE = 94 , # [doc = "96 - PKCAU"]
PKCAU = 96 , # [doc = "97 - TSI"]
TSI = 97 , # [doc = "98 - ICACHE"]
ICACHE = 98 , # [doc = "99 - TZIAC"]
TZIAC = 99 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn RTC_TAMPER () ; fn RTC_WKUP () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn DMA0_CHANNEL7 () ; fn ADC () ; fn RTC_TAMPER_SEC () ; fn RTC_WKUP_SEC () ; fn RTC_ALARM_SEC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0COM () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn TIMER15 () ; fn TIMER16 () ; fn SDIO () ; fn TIMER4 () ; fn I2C0_WK () ; fn USART0_WK () ; fn USART2_WK () ; fn TIMER5 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn USB_FS () ; fn USB_FS_WKUP () ; fn DCI () ; fn CAU () ; fn HAU_TRNG () ; fn HPDF0 () ; fn HPDF1 () ; fn EFUSE () ; fn PKCAU () ; fn TSI () ; fn ICACHE () ; fn TZIAC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 100]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : RTC_TAMPER } , Vector { _handler : RTC_WKUP } , Vector { _reserved : 0 } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : ADC } , Vector { _handler : RTC_TAMPER_SEC } , Vector { _handler : RTC_WKUP_SEC } , Vector { _handler : RTC_ALARM_SEC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0COM } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : I2C0_WK } , Vector { _handler : USART0_WK } , Vector { _handler : USART2_WK } , Vector { _handler : TIMER5 } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USB_FS } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USB_FS_WKUP } , Vector { _reserved : 0 } , Vector { _handler : DCI } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HPDF0 } , Vector { _handler : HPDF1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EFUSE } , Vector { _reserved : 0 } , Vector { _handler : PKCAU } , Vector { _handler : TSI } , Vector { _handler : ICACHE } , Vector { _handler : TZIAC } ,]
; } pub const TIMER1 : gdtimer1fffa800c :: Timer1 = unsafe { gdtimer1fffa800c :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer2eec52228 :: Timer2 = unsafe { gdtimer2eec52228 :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer393263af4 :: Timer3 = unsafe { gdtimer393263af4 :: Timer3 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer41e5d00d9 :: Timer4 = unsafe { gdtimer41e5d00d9 :: Timer4 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : gdrtc852192c0 :: Rtc = unsafe { gdrtc852192c0 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtb5a65d35 :: Fwdgt = unsafe { gdfwdgtb5a65d35 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart133efdba7 :: Usart1 = unsafe { gdusart133efdba7 :: Usart1 :: from_ptr (0x4000_4400usize as _) } ; pub const USART0 : gdusart0184abb20 :: Usart0 = unsafe { gdusart0184abb20 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c110f061e6 :: I2c1 = unsafe { gdi2c110f061e6 :: I2c1 :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmu60a17507 :: Pmu = unsafe { gdpmu60a17507 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const TIMER0 : gdtimer02909a1e0 :: Timer0 = unsafe { gdtimer02909a1e0 :: Timer0 :: from_ptr (0x4001_0000usize as _) } ; pub const USART2 : gdusart28472597e :: Usart2 = unsafe { gdusart28472597e :: Usart2 :: from_ptr (0x4001_1000usize as _) } ; pub const ADC : gdadcfa6e1d5e :: Adc = unsafe { gdadcfa6e1d5e :: Adc :: from_ptr (0x4001_2000usize as _) } ; pub const SDIO : gdsdioc3d573a1 :: Sdio = unsafe { gdsdioc3d573a1 :: Sdio :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0c6850d65 :: Spi0 = unsafe { gdspi0c6850d65 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const SYSCFG : gdsyscfg9b26c1e2 :: Syscfg = unsafe { gdsyscfg9b26c1e2 :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdextib5f0f6a2 :: Exti = unsafe { gdextib5f0f6a2 :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const HPDF : gdhpdf8229656b :: Hpdf = unsafe { gdhpdf8229656b :: Hpdf :: from_ptr (0x4001_6000usize as _) } ; pub const TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_8000usize as _) } ; pub const TIMER16 : gdtimer169b85af82 :: Timer16 = unsafe { gdtimer169b85af82 :: Timer16 :: from_ptr (0x4001_8400usize as _) } ; pub const GPIOA : gdgpioab160b911 :: Gpioa = unsafe { gdgpioab160b911 :: Gpioa :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpiobc8518b32 :: Gpiob = unsafe { gdgpiobc8518b32 :: Gpiob :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpioc27deb6bb :: Gpioc = unsafe { gdgpioc27deb6bb :: Gpioc :: from_ptr (0x4002_0800usize as _) } ; pub const FMC : gdfmcbf969006 :: Fmc = unsafe { gdfmcbf969006 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const EFUSE : gdefusecc10b21c :: Efuse = unsafe { gdefusecc10b21c :: Efuse :: from_ptr (0x4002_2800usize as _) } ; pub const CRC : gdcrc5215c95f :: Crc = unsafe { gdcrc5215c95f :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcu963f0fa8 :: Rcu = unsafe { gdrcu963f0fa8 :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const TSI : gdtsib2a02cb7 :: Tsi = unsafe { gdtsib2a02cb7 :: Tsi :: from_ptr (0x4002_4000usize as _) } ; pub const SQPI : gdsqpi725030b5 :: Sqpi = unsafe { gdsqpi725030b5 :: Sqpi :: from_ptr (0x4002_5400usize as _) } ; pub const QSPI : gdqspia40e0b50 :: Qspi = unsafe { gdqspia40e0b50 :: Qspi :: from_ptr (0x4002_5800usize as _) } ; pub const DMA0 : gddma0eceade8d :: Dma0 = unsafe { gddma0eceade8d :: Dma0 :: from_ptr (0x4002_6000usize as _) } ; pub const DMA1 : gddma1085df51b :: Dma1 = unsafe { gddma1085df51b :: Dma1 :: from_ptr (0x4002_6400usize as _) } ; pub const ICACHE : gdicachec9264397 :: Icache = unsafe { gdicachec9264397 :: Icache :: from_ptr (0x4008_0000usize as _) } ; pub const TZSPC : gdtzspc3282497c :: Tzspc = unsafe { gdtzspc3282497c :: Tzspc :: from_ptr (0x400a_0000usize as _) } ; pub const TZIAC : gdtziac54075a01 :: Tziac = unsafe { gdtziac54075a01 :: Tziac :: from_ptr (0x400a_0400usize as _) } ; pub const TZBMPC0 : gdtzbmpc0eeec4ae7 :: Tzbmpc0 = unsafe { gdtzbmpc0eeec4ae7 :: Tzbmpc0 :: from_ptr (0x400a_0800usize as _) } ; pub const TZBMPC1 : gdtzbmpc10aa1e7ec :: Tzbmpc1 = unsafe { gdtzbmpc10aa1e7ec :: Tzbmpc1 :: from_ptr (0x400a_0c00usize as _) } ; pub const TZBMPC2 : gdtzbmpc2314ce6dc :: Tzbmpc2 = unsafe { gdtzbmpc2314ce6dc :: Tzbmpc2 :: from_ptr (0x400b_0000usize as _) } ; pub const TZBMPC3 : gdtzbmpc3b4d6e427 :: Tzbmpc3 = unsafe { gdtzbmpc3b4d6e427 :: Tzbmpc3 :: from_ptr (0x400b_0400usize as _) } ; pub const FS_GLOBAL : gdfsglobale74e6f0e :: FsGlobal = unsafe { gdfsglobale74e6f0e :: FsGlobal :: from_ptr (0x4900_0000usize as _) } ; pub const FS_HOST : gdfshost44621b1c :: FsHost = unsafe { gdfshost44621b1c :: FsHost :: from_ptr (0x4900_0400usize as _) } ; pub const FS_DEVICE : gdfsdevice6e545085 :: FsDevice = unsafe { gdfsdevice6e545085 :: FsDevice :: from_ptr (0x4900_0800usize as _) } ; pub const FS_PWRCLK : gdfspwrclk87dcd48b :: FsPwrclk = unsafe { gdfspwrclk87dcd48b :: FsPwrclk :: from_ptr (0x4900_0e00usize as _) } ; pub const DCI : gddci704bb188 :: Dci = unsafe { gddci704bb188 :: Dci :: from_ptr (0x4c05_0000usize as _) } ; pub const CAU : gdcau9d384eea :: Cau = unsafe { gdcau9d384eea :: Cau :: from_ptr (0x4c06_0000usize as _) } ; pub const HAU : gdhaub6a8ba07 :: Hau = unsafe { gdhaub6a8ba07 :: Hau :: from_ptr (0x4c06_0400usize as _) } ; pub const TRNG : gdtrngbf61c352 :: Trng = unsafe { gdtrngbf61c352 :: Trng :: from_ptr (0x4c06_0800usize as _) } ; pub const PKCAU : gdpkcauf28c701a :: Pkcau = unsafe { gdpkcauf28c701a :: Pkcau :: from_ptr (0x4c06_1000usize as _) } ; pub const SEC_TIMER1 : gdtimer1fffa800c :: Timer1 = unsafe { gdtimer1fffa800c :: Timer1 :: from_ptr (0x5000_0000usize as _) } ; pub const SEC_TIMER2 : gdtimer2eec52228 :: Timer2 = unsafe { gdtimer2eec52228 :: Timer2 :: from_ptr (0x5000_0400usize as _) } ; pub const SEC_TIMER3 : gdtimer393263af4 :: Timer3 = unsafe { gdtimer393263af4 :: Timer3 :: from_ptr (0x5000_0800usize as _) } ; pub const SEC_TIMER4 : gdtimer41e5d00d9 :: Timer4 = unsafe { gdtimer41e5d00d9 :: Timer4 :: from_ptr (0x5000_0c00usize as _) } ; pub const SEC_TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x5000_1000usize as _) } ; pub const SEC_RTC : gdrtc852192c0 :: Rtc = unsafe { gdrtc852192c0 :: Rtc :: from_ptr (0x5000_2800usize as _) } ; pub const SEC_WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x5000_2c00usize as _) } ; pub const SEC_FWDGT : gdfwdgtb5a65d35 :: Fwdgt = unsafe { gdfwdgtb5a65d35 :: Fwdgt :: from_ptr (0x5000_3000usize as _) } ; pub const SEC_I2S1_ADD : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x5000_3400usize as _) } ; pub const SEC_SPI1 : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x5000_3800usize as _) } ; pub const SEC_USART1 : gdusart133efdba7 :: Usart1 = unsafe { gdusart133efdba7 :: Usart1 :: from_ptr (0x5000_4400usize as _) } ; pub const SEC_USART0 : gdusart0184abb20 :: Usart0 = unsafe { gdusart0184abb20 :: Usart0 :: from_ptr (0x5000_4800usize as _) } ; pub const SEC_I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x5000_5400usize as _) } ; pub const SEC_I2C1 : gdi2c110f061e6 :: I2c1 = unsafe { gdi2c110f061e6 :: I2c1 :: from_ptr (0x5000_5800usize as _) } ; pub const SEC_PMU : gdpmu60a17507 :: Pmu = unsafe { gdpmu60a17507 :: Pmu :: from_ptr (0x5000_7000usize as _) } ; pub const SEC_TIMER0 : gdtimer02909a1e0 :: Timer0 = unsafe { gdtimer02909a1e0 :: Timer0 :: from_ptr (0x5001_0000usize as _) } ; pub const SEC_USART2 : gdusart28472597e :: Usart2 = unsafe { gdusart28472597e :: Usart2 :: from_ptr (0x5001_1000usize as _) } ; pub const SEC_ADC : gdadcfa6e1d5e :: Adc = unsafe { gdadcfa6e1d5e :: Adc :: from_ptr (0x5001_2000usize as _) } ; pub const SEC_SDIO : gdsdioc3d573a1 :: Sdio = unsafe { gdsdioc3d573a1 :: Sdio :: from_ptr (0x5001_2c00usize as _) } ; pub const SEC_SPI0 : gdspi0c6850d65 :: Spi0 = unsafe { gdspi0c6850d65 :: Spi0 :: from_ptr (0x5001_3000usize as _) } ; pub const SEC_SYSCFG : gdsyscfg9b26c1e2 :: Syscfg = unsafe { gdsyscfg9b26c1e2 :: Syscfg :: from_ptr (0x5001_3800usize as _) } ; pub const SEC_EXTI : gdextib5f0f6a2 :: Exti = unsafe { gdextib5f0f6a2 :: Exti :: from_ptr (0x5001_3c00usize as _) } ; pub const SEC_HPDF : gdhpdf8229656b :: Hpdf = unsafe { gdhpdf8229656b :: Hpdf :: from_ptr (0x5001_6000usize as _) } ; pub const SEC_TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x5001_8000usize as _) } ; pub const SEC_TIMER16 : gdtimer169b85af82 :: Timer16 = unsafe { gdtimer169b85af82 :: Timer16 :: from_ptr (0x5001_8400usize as _) } ; pub const SEC_GPIOA : gdgpioab160b911 :: Gpioa = unsafe { gdgpioab160b911 :: Gpioa :: from_ptr (0x5002_0000usize as _) } ; pub const SEC_GPIOB : gdgpiobc8518b32 :: Gpiob = unsafe { gdgpiobc8518b32 :: Gpiob :: from_ptr (0x5002_0400usize as _) } ; pub const SEC_GPIOC : gdgpioc27deb6bb :: Gpioc = unsafe { gdgpioc27deb6bb :: Gpioc :: from_ptr (0x5002_0800usize as _) } ; pub const SEC_FMC : gdfmcbf969006 :: Fmc = unsafe { gdfmcbf969006 :: Fmc :: from_ptr (0x5002_2000usize as _) } ; pub const SEC_EFUSE : gdefusecc10b21c :: Efuse = unsafe { gdefusecc10b21c :: Efuse :: from_ptr (0x5002_2800usize as _) } ; pub const SEC_CRC : gdcrc5215c95f :: Crc = unsafe { gdcrc5215c95f :: Crc :: from_ptr (0x5002_3000usize as _) } ; pub const SEC_RCU : gdrcu963f0fa8 :: Rcu = unsafe { gdrcu963f0fa8 :: Rcu :: from_ptr (0x5002_3800usize as _) } ; pub const SEC_TSI : gdtsib2a02cb7 :: Tsi = unsafe { gdtsib2a02cb7 :: Tsi :: from_ptr (0x5002_4000usize as _) } ; pub const SEC_SQPI : gdsqpi725030b5 :: Sqpi = unsafe { gdsqpi725030b5 :: Sqpi :: from_ptr (0x5002_5400usize as _) } ; pub const SEC_QSPI : gdqspia40e0b50 :: Qspi = unsafe { gdqspia40e0b50 :: Qspi :: from_ptr (0x5002_5800usize as _) } ; pub const SEC_DMA0 : gddma0eceade8d :: Dma0 = unsafe { gddma0eceade8d :: Dma0 :: from_ptr (0x5002_6000usize as _) } ; pub const SEC_DMA1 : gddma1085df51b :: Dma1 = unsafe { gddma1085df51b :: Dma1 :: from_ptr (0x5002_6400usize as _) } ; pub const SEC_ICACHE : gdicachec9264397 :: Icache = unsafe { gdicachec9264397 :: Icache :: from_ptr (0x5008_0000usize as _) } ; pub const SEC_TZSPC : gdtzspc3282497c :: Tzspc = unsafe { gdtzspc3282497c :: Tzspc :: from_ptr (0x500a_0000usize as _) } ; pub const SEC_TZIAC : gdtziac54075a01 :: Tziac = unsafe { gdtziac54075a01 :: Tziac :: from_ptr (0x500a_0400usize as _) } ; pub const SEC_TZBMPC0 : gdtzbmpc0eeec4ae7 :: Tzbmpc0 = unsafe { gdtzbmpc0eeec4ae7 :: Tzbmpc0 :: from_ptr (0x500a_0800usize as _) } ; pub const SEC_TZBMPC1 : gdtzbmpc10aa1e7ec :: Tzbmpc1 = unsafe { gdtzbmpc10aa1e7ec :: Tzbmpc1 :: from_ptr (0x500a_0c00usize as _) } ; pub const SEC_TZBMPC2 : gdtzbmpc2314ce6dc :: Tzbmpc2 = unsafe { gdtzbmpc2314ce6dc :: Tzbmpc2 :: from_ptr (0x500b_0000usize as _) } ; pub const SEC_TZBMPC3 : gdtzbmpc3b4d6e427 :: Tzbmpc3 = unsafe { gdtzbmpc3b4d6e427 :: Tzbmpc3 :: from_ptr (0x500b_0400usize as _) } ; pub const SEC_FS_GLOBAL : gdfsglobale74e6f0e :: FsGlobal = unsafe { gdfsglobale74e6f0e :: FsGlobal :: from_ptr (0x5900_0000usize as _) } ; pub const SEC_FS_HOST : gdfshost44621b1c :: FsHost = unsafe { gdfshost44621b1c :: FsHost :: from_ptr (0x5900_0400usize as _) } ; pub const SEC_FS_DEVICE : gdfsdevice6e545085 :: FsDevice = unsafe { gdfsdevice6e545085 :: FsDevice :: from_ptr (0x5900_0800usize as _) } ; pub const SEC_FS_PWRCLK : gdfspwrclk87dcd48b :: FsPwrclk = unsafe { gdfspwrclk87dcd48b :: FsPwrclk :: from_ptr (0x5900_0e00usize as _) } ; pub const SEC_DCI : gddci704bb188 :: Dci = unsafe { gddci704bb188 :: Dci :: from_ptr (0x5c05_0000usize as _) } ; pub const SEC_CAU : gdcau9d384eea :: Cau = unsafe { gdcau9d384eea :: Cau :: from_ptr (0x5c06_0000usize as _) } ; pub const SEC_HAU : gdhaub6a8ba07 :: Hau = unsafe { gdhaub6a8ba07 :: Hau :: from_ptr (0x5c06_0400usize as _) } ; pub const SEC_TRNG : gdtrngbf61c352 :: Trng = unsafe { gdtrngbf61c352 :: Trng :: from_ptr (0x5c06_0800usize as _) } ; pub const SEC_PKCAU : gdpkcauf28c701a :: Pkcau = unsafe { gdpkcauf28c701a :: Pkcau :: from_ptr (0x5c06_1000usize as _) } ; pub const DCB : gddcbfca262ed :: Dcb = unsafe { gddcbfca262ed :: Dcb :: from_ptr (0xe000_ee08usize as _) } ; pub const DBG : gddbgf6015eed :: Dbg = unsafe { gddbgf6015eed :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcfa6e1d5e_v1.rs"] pub mod gdadcfa6e1d5e;
#[path="../../peripherals/gdcau9d384eea_v1.rs"] pub mod gdcau9d384eea;
#[path="../../peripherals/gdcrc5215c95f_v1.rs"] pub mod gdcrc5215c95f;
#[path="../../peripherals/gddbgf6015eed_v1.rs"] pub mod gddbgf6015eed;
#[path="../../peripherals/gddcbfca262ed_v1.rs"] pub mod gddcbfca262ed;
#[path="../../peripherals/gddci704bb188_v1.rs"] pub mod gddci704bb188;
#[path="../../peripherals/gddma0eceade8d_v1.rs"] pub mod gddma0eceade8d;
#[path="../../peripherals/gddma1085df51b_v1.rs"] pub mod gddma1085df51b;
#[path="../../peripherals/gdefusecc10b21c_v1.rs"] pub mod gdefusecc10b21c;
#[path="../../peripherals/gdextib5f0f6a2_v1.rs"] pub mod gdextib5f0f6a2;
#[path="../../peripherals/gdfmcbf969006_v1.rs"] pub mod gdfmcbf969006;
#[path="../../peripherals/gdfsdevice6e545085_v1.rs"] pub mod gdfsdevice6e545085;
#[path="../../peripherals/gdfsglobale74e6f0e_v1.rs"] pub mod gdfsglobale74e6f0e;
#[path="../../peripherals/gdfshost44621b1c_v1.rs"] pub mod gdfshost44621b1c;
#[path="../../peripherals/gdfspwrclk87dcd48b_v1.rs"] pub mod gdfspwrclk87dcd48b;
#[path="../../peripherals/gdfwdgtb5a65d35_v1.rs"] pub mod gdfwdgtb5a65d35;
#[path="../../peripherals/gdgpioab160b911_v1.rs"] pub mod gdgpioab160b911;
#[path="../../peripherals/gdgpiobc8518b32_v1.rs"] pub mod gdgpiobc8518b32;
#[path="../../peripherals/gdgpioc27deb6bb_v1.rs"] pub mod gdgpioc27deb6bb;
#[path="../../peripherals/gdhaub6a8ba07_v1.rs"] pub mod gdhaub6a8ba07;
#[path="../../peripherals/gdhpdf8229656b_v1.rs"] pub mod gdhpdf8229656b;
#[path="../../peripherals/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../../peripherals/gdi2c110f061e6_v1.rs"] pub mod gdi2c110f061e6;
#[path="../../peripherals/gdicachec9264397_v1.rs"] pub mod gdicachec9264397;
#[path="../../peripherals/gdpkcauf28c701a_v1.rs"] pub mod gdpkcauf28c701a;
#[path="../../peripherals/gdpmu60a17507_v1.rs"] pub mod gdpmu60a17507;
#[path="../../peripherals/gdqspia40e0b50_v1.rs"] pub mod gdqspia40e0b50;
#[path="../../peripherals/gdrcu963f0fa8_v1.rs"] pub mod gdrcu963f0fa8;
#[path="../../peripherals/gdrtc852192c0_v1.rs"] pub mod gdrtc852192c0;
#[path="../../peripherals/gdsdioc3d573a1_v1.rs"] pub mod gdsdioc3d573a1;
#[path="../../peripherals/gdspi0c6850d65_v1.rs"] pub mod gdspi0c6850d65;
#[path="../../peripherals/gdspi14e571efb_v1.rs"] pub mod gdspi14e571efb;
#[path="../../peripherals/gdsqpi725030b5_v1.rs"] pub mod gdsqpi725030b5;
#[path="../../peripherals/gdsyscfg9b26c1e2_v1.rs"] pub mod gdsyscfg9b26c1e2;
#[path="../../peripherals/gdtimer02909a1e0_v1.rs"] pub mod gdtimer02909a1e0;
#[path="../../peripherals/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../../peripherals/gdtimer169b85af82_v1.rs"] pub mod gdtimer169b85af82;
#[path="../../peripherals/gdtimer1fffa800c_v1.rs"] pub mod gdtimer1fffa800c;
#[path="../../peripherals/gdtimer2eec52228_v1.rs"] pub mod gdtimer2eec52228;
#[path="../../peripherals/gdtimer393263af4_v1.rs"] pub mod gdtimer393263af4;
#[path="../../peripherals/gdtimer41e5d00d9_v1.rs"] pub mod gdtimer41e5d00d9;
#[path="../../peripherals/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../../peripherals/gdtrngbf61c352_v1.rs"] pub mod gdtrngbf61c352;
#[path="../../peripherals/gdtsib2a02cb7_v1.rs"] pub mod gdtsib2a02cb7;
#[path="../../peripherals/gdtzbmpc0eeec4ae7_v1.rs"] pub mod gdtzbmpc0eeec4ae7;
#[path="../../peripherals/gdtzbmpc10aa1e7ec_v1.rs"] pub mod gdtzbmpc10aa1e7ec;
#[path="../../peripherals/gdtzbmpc2314ce6dc_v1.rs"] pub mod gdtzbmpc2314ce6dc;
#[path="../../peripherals/gdtzbmpc3b4d6e427_v1.rs"] pub mod gdtzbmpc3b4d6e427;
#[path="../../peripherals/gdtziac54075a01_v1.rs"] pub mod gdtziac54075a01;
#[path="../../peripherals/gdtzspc3282497c_v1.rs"] pub mod gdtzspc3282497c;
#[path="../../peripherals/gdusart0184abb20_v1.rs"] pub mod gdusart0184abb20;
#[path="../../peripherals/gdusart133efdba7_v1.rs"] pub mod gdusart133efdba7;
#[path="../../peripherals/gdusart28472597e_v1.rs"] pub mod gdusart28472597e;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
