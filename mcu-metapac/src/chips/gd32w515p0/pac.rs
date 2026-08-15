

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - TAMPER_STAMP"]
TAMPER_STAMP = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - DMA0_CHANNEL7"]
DMA0_CHANNEL7 = 18 , # [doc = "19 - ADC"]
ADC = 19 , # [doc = "20 - TAMPER_STAMP_S"]
TAMPER_STAMP_S = 20 , # [doc = "21 - RTC_WKUP_S"]
RTC_WKUP_S = 21 , # [doc = "22 - RTC_ALARM_S"]
RTC_ALARM_S = 22 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_CMT"]
TIMER0_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
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
RTC_ALARM = 41 , # [doc = "42 - VLVDF"]
VLVDF = 42 , # [doc = "44 - TIMER15"]
TIMER15 = 44 , # [doc = "45 - TIMER16"]
TIMER16 = 45 , # [doc = "49 - SDIO"]
SDIO = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - I2C0_WKUP"]
I2C0_WKUP = 51 , # [doc = "52 - USART0_WKUP"]
USART0_WKUP = 52 , # [doc = "53 - USART2_WKUP"]
USART2_WKUP = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 61 , # [doc = "62 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 62 , # [doc = "63 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 63 , # [doc = "66 - WIFI11N_WKUP"]
WIFI11N_WKUP = 66 , # [doc = "67 - USBFS"]
USBFS = 67 , # [doc = "76 - USBFS_WKUP"]
USBFS_WKUP = 76 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "89 - HPDF_INT0"]
HPDF_INT0 = 89 , # [doc = "90 - HPDF_INT1"]
HPDF_INT1 = 90 , # [doc = "91 - WIFI11N_INT0"]
WIFI11N_INT0 = 91 , # [doc = "92 - WIFI11N_INT1"]
WIFI11N_INT1 = 92 , # [doc = "93 - WIFI11N_INT2"]
WIFI11N_INT2 = 93 , # [doc = "94 - EFUSE"]
EFUSE = 94 , # [doc = "95 - QSPI"]
QSPI = 95 , # [doc = "96 - PKCAU"]
PKCAU = 96 , # [doc = "97 - TSI"]
TSI = 97 , # [doc = "98 - ICACHE"]
ICACHE = 98 , # [doc = "99 - TZIAC_S"]
TZIAC_S = 99 , # [doc = "100 - FMC_S"]
FMC_S = 100 , # [doc = "101 - QSPI_S"]
QSPI_S = 101 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER_STAMP () ; fn RTC_WKUP () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn DMA0_CHANNEL7 () ; fn ADC () ; fn TAMPER_STAMP_S () ; fn RTC_WKUP_S () ; fn RTC_ALARM_S () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn VLVDF () ; fn TIMER15 () ; fn TIMER16 () ; fn SDIO () ; fn TIMER4 () ; fn I2C0_WKUP () ; fn USART0_WKUP () ; fn USART2_WKUP () ; fn TIMER5 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn WIFI11N_WKUP () ; fn USBFS () ; fn USBFS_WKUP () ; fn DCI () ; fn CAU () ; fn HAU_TRNG () ; fn FPU () ; fn HPDF_INT0 () ; fn HPDF_INT1 () ; fn WIFI11N_INT0 () ; fn WIFI11N_INT1 () ; fn WIFI11N_INT2 () ; fn EFUSE () ; fn QSPI () ; fn PKCAU () ; fn TSI () ; fn ICACHE () ; fn TZIAC_S () ; fn FMC_S () ; fn QSPI_S () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 102]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER_STAMP } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : ADC } , Vector { _handler : TAMPER_STAMP_S } , Vector { _handler : RTC_WKUP_S } , Vector { _handler : RTC_ALARM_S } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : VLVDF } , Vector { _reserved : 0 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : I2C0_WKUP } , Vector { _handler : USART0_WKUP } , Vector { _handler : USART2_WKUP } , Vector { _handler : TIMER5 } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : WIFI11N_WKUP } , Vector { _handler : USBFS } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USBFS_WKUP } , Vector { _reserved : 0 } , Vector { _handler : DCI } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _handler : FPU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HPDF_INT0 } , Vector { _handler : HPDF_INT1 } , Vector { _handler : WIFI11N_INT0 } , Vector { _handler : WIFI11N_INT1 } , Vector { _handler : WIFI11N_INT2 } , Vector { _handler : EFUSE } , Vector { _handler : QSPI } , Vector { _handler : PKCAU } , Vector { _handler : TSI } , Vector { _handler : ICACHE } , Vector { _handler : TZIAC_S } , Vector { _handler : FMC_S } , Vector { _handler : QSPI_S } ,]
; } pub const TIMER1 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const RTC : gdrtcc5f2b32e :: Rtc = unsafe { gdrtcc5f2b32e :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi1c2f4e1e :: Spi = unsafe { gdspi1c2f4e1e :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART0 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const PMU : gdpmufda9360e :: Pmu = unsafe { gdpmufda9360e :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const TIMER0 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4001_0000usize as _) } ; pub const USART2 : gdusart7f24e647 :: Usart = unsafe { gdusart7f24e647 :: Usart :: from_ptr (0x4001_1000usize as _) } ; pub const ADC : gdadc35fc0029 :: Adc = unsafe { gdadc35fc0029 :: Adc :: from_ptr (0x4001_2000usize as _) } ; pub const SDIO : gdsdioa16a5588 :: Sdio = unsafe { gdsdioa16a5588 :: Sdio :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi1c2f4e1e :: Spi = unsafe { gdspi1c2f4e1e :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const SYSCFG : gdsyscfg24aecb07 :: Syscfg = unsafe { gdsyscfg24aecb07 :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdexti42cdb862 :: Exti = unsafe { gdexti42cdb862 :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const HPDF : gdhpdfc666e7e5 :: Hpdf = unsafe { gdhpdfc666e7e5 :: Hpdf :: from_ptr (0x4001_6000usize as _) } ; pub const TIMER15 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4001_8000usize as _) } ; pub const TIMER16 : gdtimera05861d6 :: Timer = unsafe { gdtimera05861d6 :: Timer :: from_ptr (0x4001_8400usize as _) } ; pub const GPIOA : gdgpio5d0b827e :: Gpio = unsafe { gdgpio5d0b827e :: Gpio :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpio5d0b827e :: Gpio = unsafe { gdgpio5d0b827e :: Gpio :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpio5d0b827e :: Gpio = unsafe { gdgpio5d0b827e :: Gpio :: from_ptr (0x4002_0800usize as _) } ; pub const FMC : gdfmc21870ef0 :: Fmc = unsafe { gdfmc21870ef0 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const EFUSE : gdefusef9100cc2 :: Efuse = unsafe { gdefusef9100cc2 :: Efuse :: from_ptr (0x4002_2800usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcu3ec51a72 :: Rcu = unsafe { gdrcu3ec51a72 :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const TSI : gdtsi75cc2319 :: Tsi = unsafe { gdtsi75cc2319 :: Tsi :: from_ptr (0x4002_4000usize as _) } ; pub const SQPI : gdsqpi47688f21 :: Sqpi = unsafe { gdsqpi47688f21 :: Sqpi :: from_ptr (0x4002_5400usize as _) } ; pub const QSPI : gdqspib6e42f6d :: Qspi = unsafe { gdqspib6e42f6d :: Qspi :: from_ptr (0x4002_5800usize as _) } ; pub const DMA0 : gddma03e09269 :: Dma = unsafe { gddma03e09269 :: Dma :: from_ptr (0x4002_6000usize as _) } ; pub const DMA1 : gddma03e09269 :: Dma = unsafe { gddma03e09269 :: Dma :: from_ptr (0x4002_6400usize as _) } ; pub const ICACHE : gdicache04a9739a :: Icache = unsafe { gdicache04a9739a :: Icache :: from_ptr (0x4008_0000usize as _) } ; pub const TZSPC : gdtzspcada29d50 :: Tzspc = unsafe { gdtzspcada29d50 :: Tzspc :: from_ptr (0x400a_0000usize as _) } ; pub const TZIAC : gdtziac6b01a7ab :: Tziac = unsafe { gdtziac6b01a7ab :: Tziac :: from_ptr (0x400a_0400usize as _) } ; pub const TZBMPC0 : gdtzbmpcafecfd82 :: Tzbmpc = unsafe { gdtzbmpcafecfd82 :: Tzbmpc :: from_ptr (0x400a_0800usize as _) } ; pub const TZBMPC1 : gdtzbmpcafecfd82 :: Tzbmpc = unsafe { gdtzbmpcafecfd82 :: Tzbmpc :: from_ptr (0x400a_0c00usize as _) } ; pub const TZBMPC2 : gdtzbmpcf4d2e8f3 :: Tzbmpc = unsafe { gdtzbmpcf4d2e8f3 :: Tzbmpc :: from_ptr (0x400b_0000usize as _) } ; pub const TZBMPC3 : gdtzbmpcfe6b3775 :: Tzbmpc = unsafe { gdtzbmpcfe6b3775 :: Tzbmpc :: from_ptr (0x400b_0400usize as _) } ; pub const DCI : gddci6728f4f7 :: Dci = unsafe { gddci6728f4f7 :: Dci :: from_ptr (0x4c05_0000usize as _) } ; pub const CAU : gdcauf29b21d6 :: Cau = unsafe { gdcauf29b21d6 :: Cau :: from_ptr (0x4c06_0000usize as _) } ; pub const HAU : gdhaub97c00c8 :: Hau = unsafe { gdhaub97c00c8 :: Hau :: from_ptr (0x4c06_0400usize as _) } ; pub const TRNG : gdtrng13872700 :: Trng = unsafe { gdtrng13872700 :: Trng :: from_ptr (0x4c06_0800usize as _) } ; pub const PKCAU : gdpkcau5848bf43 :: Pkcau = unsafe { gdpkcau5848bf43 :: Pkcau :: from_ptr (0x4c06_1000usize as _) } ; pub const DBG : gddbg9afe2c92 :: Dbg = unsafe { gddbg9afe2c92 :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc35fc0029_v1.rs"] pub mod gdadc35fc0029;
#[path="../../peripherals/gdcauf29b21d6_v1.rs"] pub mod gdcauf29b21d6;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gddbg9afe2c92_v1.rs"] pub mod gddbg9afe2c92;
#[path="../../peripherals/gddci6728f4f7_v1.rs"] pub mod gddci6728f4f7;
#[path="../../peripherals/gddma03e09269_v1.rs"] pub mod gddma03e09269;
#[path="../../peripherals/gdefusef9100cc2_v1.rs"] pub mod gdefusef9100cc2;
#[path="../../peripherals/gdexti42cdb862_v1.rs"] pub mod gdexti42cdb862;
#[path="../../peripherals/gdfmc21870ef0_v1.rs"] pub mod gdfmc21870ef0;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpio5d0b827e_v1.rs"] pub mod gdgpio5d0b827e;
#[path="../../peripherals/gdhaub97c00c8_v1.rs"] pub mod gdhaub97c00c8;
#[path="../../peripherals/gdhpdfc666e7e5_v1.rs"] pub mod gdhpdfc666e7e5;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../../peripherals/gdicache04a9739a_v1.rs"] pub mod gdicache04a9739a;
#[path="../../peripherals/gdpkcau5848bf43_v1.rs"] pub mod gdpkcau5848bf43;
#[path="../../peripherals/gdpmufda9360e_v1.rs"] pub mod gdpmufda9360e;
#[path="../../peripherals/gdqspib6e42f6d_v1.rs"] pub mod gdqspib6e42f6d;
#[path="../../peripherals/gdrcu3ec51a72_v1.rs"] pub mod gdrcu3ec51a72;
#[path="../../peripherals/gdrtcc5f2b32e_v1.rs"] pub mod gdrtcc5f2b32e;
#[path="../../peripherals/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../../peripherals/gdspi1c2f4e1e_v1.rs"] pub mod gdspi1c2f4e1e;
#[path="../../peripherals/gdsqpi47688f21_v1.rs"] pub mod gdsqpi47688f21;
#[path="../../peripherals/gdsyscfg24aecb07_v1.rs"] pub mod gdsyscfg24aecb07;
#[path="../../peripherals/gdtimera05861d6_v1.rs"] pub mod gdtimera05861d6;
#[path="../../peripherals/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../../peripherals/gdtsi75cc2319_v1.rs"] pub mod gdtsi75cc2319;
#[path="../../peripherals/gdtzbmpcafecfd82_v1.rs"] pub mod gdtzbmpcafecfd82;
#[path="../../peripherals/gdtzbmpcf4d2e8f3_v1.rs"] pub mod gdtzbmpcf4d2e8f3;
#[path="../../peripherals/gdtzbmpcfe6b3775_v1.rs"] pub mod gdtzbmpcfe6b3775;
#[path="../../peripherals/gdtziac6b01a7ab_v1.rs"] pub mod gdtziac6b01a7ab;
#[path="../../peripherals/gdtzspcada29d50_v1.rs"] pub mod gdtzspcada29d50;
#[path="../../peripherals/gdusart7f24e647_v1.rs"] pub mod gdusart7f24e647;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
