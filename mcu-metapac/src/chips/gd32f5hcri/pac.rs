

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - RTC_TAMPER"]
RTC_TAMPER = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU"]
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
RTC_ALARM = 41 , # [doc = "42 - VLVDF"]
VLVDF = 42 , # [doc = "44 - TIMER15"]
TIMER15 = 44 , # [doc = "45 - TIMER16"]
TIMER16 = 45 , # [doc = "50 - TIMER4"]
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
DMA1_CHANNEL7 = 63 , # [doc = "65 - I2C1_WK"]
I2C1_WK = 65 , # [doc = "67 - USB_FS"]
USB_FS = 67 , # [doc = "76 - USB_FS_WKUP"]
USB_FS_WKUP = 76 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "94 - EFUSE"]
EFUSE = 94 , # [doc = "95 - QSPI"]
QSPI = 95 , # [doc = "96 - PKCAU"]
PKCAU = 96 , # [doc = "98 - ICACHE"]
ICACHE = 98 , # [doc = "99 - TZIAC_SEC"]
TZIAC_SEC = 99 , # [doc = "100 - FMC_SEC"]
FMC_SEC = 100 , # [doc = "101 - QSPI_SEC"]
QSPI_SEC = 101 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC_TAMPER () ; fn RTC_WKUP () ; fn FMC () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn DMA0_CHANNEL7 () ; fn ADC () ; fn RTC_TAMPER_SEC () ; fn RTC_WKUP_SEC () ; fn RTC_ALARM_SEC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0COM () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn VLVDF () ; fn TIMER15 () ; fn TIMER16 () ; fn TIMER4 () ; fn I2C0_WK () ; fn USART0_WK () ; fn USART2_WK () ; fn TIMER5 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn I2C1_WK () ; fn USB_FS () ; fn USB_FS_WKUP () ; fn CAU () ; fn HAU_TRNG () ; fn FPU () ; fn EFUSE () ; fn QSPI () ; fn PKCAU () ; fn ICACHE () ; fn TZIAC_SEC () ; fn FMC_SEC () ; fn QSPI_SEC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 102]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : RTC_TAMPER } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : ADC } , Vector { _handler : RTC_TAMPER_SEC } , Vector { _handler : RTC_WKUP_SEC } , Vector { _handler : RTC_ALARM_SEC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0COM } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : VLVDF } , Vector { _reserved : 0 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : I2C0_WK } , Vector { _handler : USART0_WK } , Vector { _handler : USART2_WK } , Vector { _handler : TIMER5 } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _reserved : 0 } , Vector { _handler : I2C1_WK } , Vector { _reserved : 0 } , Vector { _handler : USB_FS } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USB_FS_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _handler : FPU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EFUSE } , Vector { _handler : QSPI } , Vector { _handler : PKCAU } , Vector { _reserved : 0 } , Vector { _handler : ICACHE } , Vector { _handler : TZIAC_SEC } , Vector { _handler : FMC_SEC } , Vector { _handler : QSPI_SEC } ,]
; } pub const TIMER1 : gdtimer19ab75ca7 :: Timer1 = unsafe { gdtimer19ab75ca7 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer225dfb15c :: Timer2 = unsafe { gdtimer225dfb15c :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer3f6465302 :: Timer3 = unsafe { gdtimer3f6465302 :: Timer3 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer4443e83bb :: Timer4 = unsafe { gdtimer4443e83bb :: Timer4 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : gdrtc852192c0 :: Rtc = unsafe { gdrtc852192c0 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt9ccc125f :: Fwdgt = unsafe { gdfwdgt9ccc125f :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart18dc51ae1 :: Usart1 = unsafe { gdusart18dc51ae1 :: Usart1 :: from_ptr (0x4000_4400usize as _) } ; pub const USART0 : gdusart0d14079e1 :: Usart0 = unsafe { gdusart0d14079e1 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c0fe7c9466 :: I2c0 = unsafe { gdi2c0fe7c9466 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c1b7b495ac :: I2c1 = unsafe { gdi2c1b7b495ac :: I2c1 :: from_ptr (0x4000_5800usize as _) } ; pub const CTC : gdctc02e788fd :: Ctc = unsafe { gdctc02e788fd :: Ctc :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmucc20757f :: Pmu = unsafe { gdpmucc20757f :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const TIMER0 : gdtimer0855d28e7 :: Timer0 = unsafe { gdtimer0855d28e7 :: Timer0 :: from_ptr (0x4001_0000usize as _) } ; pub const USART2 : gdusart296769238 :: Usart2 = unsafe { gdusart296769238 :: Usart2 :: from_ptr (0x4001_1000usize as _) } ; pub const ADC : gdadcbceaa202 :: Adc = unsafe { gdadcbceaa202 :: Adc :: from_ptr (0x4001_2000usize as _) } ; pub const SPI0 : gdspi0177d35b9 :: Spi0 = unsafe { gdspi0177d35b9 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const SYSCFG : gdsyscfg1ff84976 :: Syscfg = unsafe { gdsyscfg1ff84976 :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdexti0eb3d8fb :: Exti = unsafe { gdexti0eb3d8fb :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x4001_8000usize as _) } ; pub const TIMER16 : gdtimer169b85af82 :: Timer16 = unsafe { gdtimer169b85af82 :: Timer16 :: from_ptr (0x4001_8400usize as _) } ; pub const GPIOA : gdgpioab160b911 :: Gpioa = unsafe { gdgpioab160b911 :: Gpioa :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpiobc8518b32 :: Gpiob = unsafe { gdgpiobc8518b32 :: Gpiob :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpioc27deb6bb :: Gpioc = unsafe { gdgpioc27deb6bb :: Gpioc :: from_ptr (0x4002_0800usize as _) } ; pub const GPIOD : gdgpiodf1ff9739 :: Gpiod = unsafe { gdgpiodf1ff9739 :: Gpiod :: from_ptr (0x4002_0c00usize as _) } ; pub const FMC : gdfmca00b7f1e :: Fmc = unsafe { gdfmca00b7f1e :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const EFUSE : gdefusecc10b21c :: Efuse = unsafe { gdefusecc10b21c :: Efuse :: from_ptr (0x4002_2800usize as _) } ; pub const CRC : gdcrc5215c95f :: Crc = unsafe { gdcrc5215c95f :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcu547eb6ba :: Rcu = unsafe { gdrcu547eb6ba :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const SQPI : gdsqpi725030b5 :: Sqpi = unsafe { gdsqpi725030b5 :: Sqpi :: from_ptr (0x4002_5400usize as _) } ; pub const QSPI : gdqspi768e2ece :: Qspi = unsafe { gdqspi768e2ece :: Qspi :: from_ptr (0x4002_5800usize as _) } ; pub const DMA0 : gddma08933c762 :: Dma0 = unsafe { gddma08933c762 :: Dma0 :: from_ptr (0x4002_6000usize as _) } ; pub const DMA1 : gddma13bf5564e :: Dma1 = unsafe { gddma13bf5564e :: Dma1 :: from_ptr (0x4002_6400usize as _) } ; pub const ICACHE : gdicachec9264397 :: Icache = unsafe { gdicachec9264397 :: Icache :: from_ptr (0x4008_0000usize as _) } ; pub const TZSPC : gdtzspc02c11fa8 :: Tzspc = unsafe { gdtzspc02c11fa8 :: Tzspc :: from_ptr (0x400a_0000usize as _) } ; pub const TZIAC : gdtziaccd5888a5 :: Tziac = unsafe { gdtziaccd5888a5 :: Tziac :: from_ptr (0x400a_0400usize as _) } ; pub const TZBMPC0 : gdtzbmpc0eeec4ae7 :: Tzbmpc0 = unsafe { gdtzbmpc0eeec4ae7 :: Tzbmpc0 :: from_ptr (0x400a_0800usize as _) } ; pub const TZBMPC1 : gdtzbmpc10aa1e7ec :: Tzbmpc1 = unsafe { gdtzbmpc10aa1e7ec :: Tzbmpc1 :: from_ptr (0x400a_0c00usize as _) } ; pub const TZBMPC2 : gdtzbmpc2ffd9cc81 :: Tzbmpc2 = unsafe { gdtzbmpc2ffd9cc81 :: Tzbmpc2 :: from_ptr (0x400b_0000usize as _) } ; pub const TZBMPC3 : gdtzbmpc39033fa90 :: Tzbmpc3 = unsafe { gdtzbmpc39033fa90 :: Tzbmpc3 :: from_ptr (0x400b_0400usize as _) } ; pub const FS_GLOBAL : gdfsglobale74e6f0e :: FsGlobal = unsafe { gdfsglobale74e6f0e :: FsGlobal :: from_ptr (0x4900_0000usize as _) } ; pub const FS_HOST : gdfshost44621b1c :: FsHost = unsafe { gdfshost44621b1c :: FsHost :: from_ptr (0x4900_0400usize as _) } ; pub const FS_DEVICE : gdfsdevice6e545085 :: FsDevice = unsafe { gdfsdevice6e545085 :: FsDevice :: from_ptr (0x4900_0800usize as _) } ; pub const FS_PWRCLK : gdfspwrclk87dcd48b :: FsPwrclk = unsafe { gdfspwrclk87dcd48b :: FsPwrclk :: from_ptr (0x4900_0e00usize as _) } ; pub const CAU : gdcaub94774ba :: Cau = unsafe { gdcaub94774ba :: Cau :: from_ptr (0x4c06_0000usize as _) } ; pub const HAU : gdhaub6a8ba07 :: Hau = unsafe { gdhaub6a8ba07 :: Hau :: from_ptr (0x4c06_0400usize as _) } ; pub const TRNG : gdtrng550089c9 :: Trng = unsafe { gdtrng550089c9 :: Trng :: from_ptr (0x4c06_0800usize as _) } ; pub const PKCAU : gdpkcauf28c701a :: Pkcau = unsafe { gdpkcauf28c701a :: Pkcau :: from_ptr (0x4c06_1000usize as _) } ; pub const SEC_TIMER1 : gdtimer19ab75ca7 :: Timer1 = unsafe { gdtimer19ab75ca7 :: Timer1 :: from_ptr (0x5000_0000usize as _) } ; pub const SEC_TIMER2 : gdtimer225dfb15c :: Timer2 = unsafe { gdtimer225dfb15c :: Timer2 :: from_ptr (0x5000_0400usize as _) } ; pub const SEC_TIMER3 : gdtimer3f6465302 :: Timer3 = unsafe { gdtimer3f6465302 :: Timer3 :: from_ptr (0x5000_0800usize as _) } ; pub const SEC_TIMER4 : gdtimer4443e83bb :: Timer4 = unsafe { gdtimer4443e83bb :: Timer4 :: from_ptr (0x5000_0c00usize as _) } ; pub const SEC_TIMER5 : gdtimer5183dba8f :: Timer5 = unsafe { gdtimer5183dba8f :: Timer5 :: from_ptr (0x5000_1000usize as _) } ; pub const SEC_RTC : gdrtc852192c0 :: Rtc = unsafe { gdrtc852192c0 :: Rtc :: from_ptr (0x5000_2800usize as _) } ; pub const SEC_WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x5000_2c00usize as _) } ; pub const SEC_FWDGT : gdfwdgt9ccc125f :: Fwdgt = unsafe { gdfwdgt9ccc125f :: Fwdgt :: from_ptr (0x5000_3000usize as _) } ; pub const SEC_I2S1_ADD : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x5000_3400usize as _) } ; pub const SEC_SPI1 : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x5000_3800usize as _) } ; pub const SEC_USART1 : gdusart18dc51ae1 :: Usart1 = unsafe { gdusart18dc51ae1 :: Usart1 :: from_ptr (0x5000_4400usize as _) } ; pub const SEC_USART0 : gdusart0d14079e1 :: Usart0 = unsafe { gdusart0d14079e1 :: Usart0 :: from_ptr (0x5000_4800usize as _) } ; pub const SEC_I2C0 : gdi2c0fe7c9466 :: I2c0 = unsafe { gdi2c0fe7c9466 :: I2c0 :: from_ptr (0x5000_5400usize as _) } ; pub const SEC_I2C1 : gdi2c1b7b495ac :: I2c1 = unsafe { gdi2c1b7b495ac :: I2c1 :: from_ptr (0x5000_5800usize as _) } ; pub const SEC_CTC : gdctc02e788fd :: Ctc = unsafe { gdctc02e788fd :: Ctc :: from_ptr (0x5000_6c00usize as _) } ; pub const SEC_PMU : gdpmucc20757f :: Pmu = unsafe { gdpmucc20757f :: Pmu :: from_ptr (0x5000_7000usize as _) } ; pub const SEC_TIMER0 : gdtimer0855d28e7 :: Timer0 = unsafe { gdtimer0855d28e7 :: Timer0 :: from_ptr (0x5001_0000usize as _) } ; pub const SEC_USART2 : gdusart296769238 :: Usart2 = unsafe { gdusart296769238 :: Usart2 :: from_ptr (0x5001_1000usize as _) } ; pub const SEC_ADC : gdadcbceaa202 :: Adc = unsafe { gdadcbceaa202 :: Adc :: from_ptr (0x5001_2000usize as _) } ; pub const SEC_SPI0 : gdspi0177d35b9 :: Spi0 = unsafe { gdspi0177d35b9 :: Spi0 :: from_ptr (0x5001_3000usize as _) } ; pub const SEC_SYSCFG : gdsyscfg1ff84976 :: Syscfg = unsafe { gdsyscfg1ff84976 :: Syscfg :: from_ptr (0x5001_3800usize as _) } ; pub const SEC_EXTI : gdexti0eb3d8fb :: Exti = unsafe { gdexti0eb3d8fb :: Exti :: from_ptr (0x5001_3c00usize as _) } ; pub const SEC_TIMER15 : gdtimer15dc6fd783 :: Timer15 = unsafe { gdtimer15dc6fd783 :: Timer15 :: from_ptr (0x5001_8000usize as _) } ; pub const SEC_TIMER16 : gdtimer169b85af82 :: Timer16 = unsafe { gdtimer169b85af82 :: Timer16 :: from_ptr (0x5001_8400usize as _) } ; pub const SEC_GPIOA : gdgpioab160b911 :: Gpioa = unsafe { gdgpioab160b911 :: Gpioa :: from_ptr (0x5002_0000usize as _) } ; pub const SEC_GPIOB : gdgpiobc8518b32 :: Gpiob = unsafe { gdgpiobc8518b32 :: Gpiob :: from_ptr (0x5002_0400usize as _) } ; pub const SEC_GPIOC : gdgpioc27deb6bb :: Gpioc = unsafe { gdgpioc27deb6bb :: Gpioc :: from_ptr (0x5002_0800usize as _) } ; pub const SEC_GPIOD : gdgpiodf1ff9739 :: Gpiod = unsafe { gdgpiodf1ff9739 :: Gpiod :: from_ptr (0x5002_0c00usize as _) } ; pub const SEC_FMC : gdfmca00b7f1e :: Fmc = unsafe { gdfmca00b7f1e :: Fmc :: from_ptr (0x5002_2000usize as _) } ; pub const SEC_EFUSE : gdefusecc10b21c :: Efuse = unsafe { gdefusecc10b21c :: Efuse :: from_ptr (0x5002_2800usize as _) } ; pub const SEC_CRC : gdcrc5215c95f :: Crc = unsafe { gdcrc5215c95f :: Crc :: from_ptr (0x5002_3000usize as _) } ; pub const SEC_RCU : gdrcu547eb6ba :: Rcu = unsafe { gdrcu547eb6ba :: Rcu :: from_ptr (0x5002_3800usize as _) } ; pub const SEC_SQPI : gdsqpi725030b5 :: Sqpi = unsafe { gdsqpi725030b5 :: Sqpi :: from_ptr (0x5002_5400usize as _) } ; pub const SEC_QSPI : gdqspi768e2ece :: Qspi = unsafe { gdqspi768e2ece :: Qspi :: from_ptr (0x5002_5800usize as _) } ; pub const SEC_DMA0 : gddma08933c762 :: Dma0 = unsafe { gddma08933c762 :: Dma0 :: from_ptr (0x5002_6000usize as _) } ; pub const SEC_DMA1 : gddma13bf5564e :: Dma1 = unsafe { gddma13bf5564e :: Dma1 :: from_ptr (0x5002_6400usize as _) } ; pub const SEC_ICACHE : gdicachec9264397 :: Icache = unsafe { gdicachec9264397 :: Icache :: from_ptr (0x5008_0000usize as _) } ; pub const SEC_TZSPC : gdtzspc02c11fa8 :: Tzspc = unsafe { gdtzspc02c11fa8 :: Tzspc :: from_ptr (0x500a_0000usize as _) } ; pub const SEC_TZIAC : gdtziaccd5888a5 :: Tziac = unsafe { gdtziaccd5888a5 :: Tziac :: from_ptr (0x500a_0400usize as _) } ; pub const SEC_TZBMPC0 : gdtzbmpc0eeec4ae7 :: Tzbmpc0 = unsafe { gdtzbmpc0eeec4ae7 :: Tzbmpc0 :: from_ptr (0x500a_0800usize as _) } ; pub const SEC_TZBMPC1 : gdtzbmpc10aa1e7ec :: Tzbmpc1 = unsafe { gdtzbmpc10aa1e7ec :: Tzbmpc1 :: from_ptr (0x500a_0c00usize as _) } ; pub const SEC_TZBMPC2 : gdtzbmpc2ffd9cc81 :: Tzbmpc2 = unsafe { gdtzbmpc2ffd9cc81 :: Tzbmpc2 :: from_ptr (0x500b_0000usize as _) } ; pub const SEC_TZBMPC3 : gdtzbmpc39033fa90 :: Tzbmpc3 = unsafe { gdtzbmpc39033fa90 :: Tzbmpc3 :: from_ptr (0x500b_0400usize as _) } ; pub const SEC_FS_GLOBAL : gdfsglobale74e6f0e :: FsGlobal = unsafe { gdfsglobale74e6f0e :: FsGlobal :: from_ptr (0x5900_0000usize as _) } ; pub const SEC_FS_HOST : gdfshost44621b1c :: FsHost = unsafe { gdfshost44621b1c :: FsHost :: from_ptr (0x5900_0400usize as _) } ; pub const SEC_FS_DEVICE : gdfsdevice6e545085 :: FsDevice = unsafe { gdfsdevice6e545085 :: FsDevice :: from_ptr (0x5900_0800usize as _) } ; pub const SEC_FS_PWRCLK : gdfspwrclk87dcd48b :: FsPwrclk = unsafe { gdfspwrclk87dcd48b :: FsPwrclk :: from_ptr (0x5900_0e00usize as _) } ; pub const SEC_CAU : gdcaub94774ba :: Cau = unsafe { gdcaub94774ba :: Cau :: from_ptr (0x5c06_0000usize as _) } ; pub const SEC_HAU : gdhaub6a8ba07 :: Hau = unsafe { gdhaub6a8ba07 :: Hau :: from_ptr (0x5c06_0400usize as _) } ; pub const SEC_TRNG : gdtrng550089c9 :: Trng = unsafe { gdtrng550089c9 :: Trng :: from_ptr (0x5c06_0800usize as _) } ; pub const SEC_PKCAU : gdpkcauf28c701a :: Pkcau = unsafe { gdpkcauf28c701a :: Pkcau :: from_ptr (0x5c06_1000usize as _) } ; pub const DCB : gddcbfca262ed :: Dcb = unsafe { gddcbfca262ed :: Dcb :: from_ptr (0xe000_ee08usize as _) } ; pub const DBG : gddbgb68ad2cd :: Dbg = unsafe { gddbgb68ad2cd :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcbceaa202_v1.rs"] pub mod gdadcbceaa202;
#[path="../../peripherals/gdcaub94774ba_v1.rs"] pub mod gdcaub94774ba;
#[path="../../peripherals/gdcrc5215c95f_v1.rs"] pub mod gdcrc5215c95f;
#[path="../../peripherals/gdctc02e788fd_v1.rs"] pub mod gdctc02e788fd;
#[path="../../peripherals/gddbgb68ad2cd_v1.rs"] pub mod gddbgb68ad2cd;
#[path="../../peripherals/gddcbfca262ed_v1.rs"] pub mod gddcbfca262ed;
#[path="../../peripherals/gddma08933c762_v1.rs"] pub mod gddma08933c762;
#[path="../../peripherals/gddma13bf5564e_v1.rs"] pub mod gddma13bf5564e;
#[path="../../peripherals/gdefusecc10b21c_v1.rs"] pub mod gdefusecc10b21c;
#[path="../../peripherals/gdexti0eb3d8fb_v1.rs"] pub mod gdexti0eb3d8fb;
#[path="../../peripherals/gdfmca00b7f1e_v1.rs"] pub mod gdfmca00b7f1e;
#[path="../../peripherals/gdfsdevice6e545085_v1.rs"] pub mod gdfsdevice6e545085;
#[path="../../peripherals/gdfsglobale74e6f0e_v1.rs"] pub mod gdfsglobale74e6f0e;
#[path="../../peripherals/gdfshost44621b1c_v1.rs"] pub mod gdfshost44621b1c;
#[path="../../peripherals/gdfspwrclk87dcd48b_v1.rs"] pub mod gdfspwrclk87dcd48b;
#[path="../../peripherals/gdfwdgt9ccc125f_v1.rs"] pub mod gdfwdgt9ccc125f;
#[path="../../peripherals/gdgpioab160b911_v1.rs"] pub mod gdgpioab160b911;
#[path="../../peripherals/gdgpiobc8518b32_v1.rs"] pub mod gdgpiobc8518b32;
#[path="../../peripherals/gdgpioc27deb6bb_v1.rs"] pub mod gdgpioc27deb6bb;
#[path="../../peripherals/gdgpiodf1ff9739_v1.rs"] pub mod gdgpiodf1ff9739;
#[path="../../peripherals/gdhaub6a8ba07_v1.rs"] pub mod gdhaub6a8ba07;
#[path="../../peripherals/gdi2c0fe7c9466_v1.rs"] pub mod gdi2c0fe7c9466;
#[path="../../peripherals/gdi2c1b7b495ac_v1.rs"] pub mod gdi2c1b7b495ac;
#[path="../../peripherals/gdicachec9264397_v1.rs"] pub mod gdicachec9264397;
#[path="../../peripherals/gdpkcauf28c701a_v1.rs"] pub mod gdpkcauf28c701a;
#[path="../../peripherals/gdpmucc20757f_v1.rs"] pub mod gdpmucc20757f;
#[path="../../peripherals/gdqspi768e2ece_v1.rs"] pub mod gdqspi768e2ece;
#[path="../../peripherals/gdrcu547eb6ba_v1.rs"] pub mod gdrcu547eb6ba;
#[path="../../peripherals/gdrtc852192c0_v1.rs"] pub mod gdrtc852192c0;
#[path="../../peripherals/gdspi0177d35b9_v1.rs"] pub mod gdspi0177d35b9;
#[path="../../peripherals/gdspi14e571efb_v1.rs"] pub mod gdspi14e571efb;
#[path="../../peripherals/gdsqpi725030b5_v1.rs"] pub mod gdsqpi725030b5;
#[path="../../peripherals/gdsyscfg1ff84976_v1.rs"] pub mod gdsyscfg1ff84976;
#[path="../../peripherals/gdtimer0855d28e7_v1.rs"] pub mod gdtimer0855d28e7;
#[path="../../peripherals/gdtimer15dc6fd783_v1.rs"] pub mod gdtimer15dc6fd783;
#[path="../../peripherals/gdtimer169b85af82_v1.rs"] pub mod gdtimer169b85af82;
#[path="../../peripherals/gdtimer19ab75ca7_v1.rs"] pub mod gdtimer19ab75ca7;
#[path="../../peripherals/gdtimer225dfb15c_v1.rs"] pub mod gdtimer225dfb15c;
#[path="../../peripherals/gdtimer3f6465302_v1.rs"] pub mod gdtimer3f6465302;
#[path="../../peripherals/gdtimer4443e83bb_v1.rs"] pub mod gdtimer4443e83bb;
#[path="../../peripherals/gdtimer5183dba8f_v1.rs"] pub mod gdtimer5183dba8f;
#[path="../../peripherals/gdtrng550089c9_v1.rs"] pub mod gdtrng550089c9;
#[path="../../peripherals/gdtzbmpc0eeec4ae7_v1.rs"] pub mod gdtzbmpc0eeec4ae7;
#[path="../../peripherals/gdtzbmpc10aa1e7ec_v1.rs"] pub mod gdtzbmpc10aa1e7ec;
#[path="../../peripherals/gdtzbmpc2ffd9cc81_v1.rs"] pub mod gdtzbmpc2ffd9cc81;
#[path="../../peripherals/gdtzbmpc39033fa90_v1.rs"] pub mod gdtzbmpc39033fa90;
#[path="../../peripherals/gdtziaccd5888a5_v1.rs"] pub mod gdtziaccd5888a5;
#[path="../../peripherals/gdtzspc02c11fa8_v1.rs"] pub mod gdtzspc02c11fa8;
#[path="../../peripherals/gdusart0d14079e1_v1.rs"] pub mod gdusart0d14079e1;
#[path="../../peripherals/gdusart18dc51ae1_v1.rs"] pub mod gdusart18dc51ae1;
#[path="../../peripherals/gdusart296769238_v1.rs"] pub mod gdusart296769238;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
