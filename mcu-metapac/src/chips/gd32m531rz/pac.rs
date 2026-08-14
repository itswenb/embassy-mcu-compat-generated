




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD1"]
LVD1 = 1 , # [doc = "2 - LVD2"]
LVD2 = 2 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL5 = 16 , # [doc = "18 - ADC0"]
ADC0 = 18 , # [doc = "19 - CAN_TX"]
CAN_TX = 19 , # [doc = "20 - CAN_RX0"]
CAN_RX0 = 20 , # [doc = "21 - CAN_RX1"]
CAN_RX1 = 21 , # [doc = "22 - CAN_EWMC"]
CAN_EWMC = 22 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER2"]
TIMER2 = 29 , # [doc = "30 - GPTIMER0"]
GPTIMER0 = 30 , # [doc = "31 - I2C_EV"]
I2C_EV = 31 , # [doc = "32 - I2C_ER"]
I2C_ER = 32 , # [doc = "35 - SPI"]
SPI = 35 , # [doc = "37 - UART0"]
UART0 = 37 , # [doc = "38 - UART1"]
UART1 = 38 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT"]
TIMER7_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - TMU"]
TMU = 47 , # [doc = "50 - GPTIMER1"]
GPTIMER1 = 50 , # [doc = "52 - UART2"]
UART2 = 52 , # [doc = "53 - UART3"]
UART3 = 53 , # [doc = "54 - CPTIMER0"]
CPTIMER0 = 54 , # [doc = "55 - CPTIMER1"]
CPTIMER1 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 61 , # [doc = "62 - DMAMUX_OVERRUN"]
DMAMUX_OVERRUN = 62 , # [doc = "63 - CPTIMERW"]
CPTIMERW = 63 , # [doc = "65 - CFMU"]
CFMU = 65 , # [doc = "66 - I2C_WKUP"]
I2C_WKUP = 66 , # [doc = "67 - FWDGT"]
FWDGT = 67 , # [doc = "70 - CMP0"]
CMP0 = 70 , # [doc = "71 - CMP1"]
CMP1 = 71 , # [doc = "72 - CMP2"]
CMP2 = 72 , # [doc = "73 - CMP3"]
CMP3 = 73 , # [doc = "75 - ADC2"]
ADC2 = 75 , # [doc = "77 - POC"]
POC = 77 , # [doc = "79 - GTOC0"]
GTOC0 = 79 , # [doc = "80 - GTOC1"]
GTOC1 = 80 , # [doc = "81 - GTOC2"]
GTOC2 = 81 , # [doc = "82 - GTOC3"]
GTOC3 = 82 , # [doc = "85 - CMP0_EXTI"]
CMP0_EXTI = 85 , # [doc = "86 - CMP1_EXTI"]
CMP1_EXTI = 86 , # [doc = "87 - CMP2_EXTI"]
CMP2_EXTI = 87 , # [doc = "88 - CMP3_EXTI"]
CMP3_EXTI = 88 , # [doc = "92 - SRAMC_ECC"]
SRAMC_ECC = 92 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD1 () ; fn LVD2 () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn ADC0 () ; fn CAN_TX () ; fn CAN_RX0 () ; fn CAN_RX1 () ; fn CAN_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn GPTIMER0 () ; fn I2C_EV () ; fn I2C_ER () ; fn SPI () ; fn UART0 () ; fn UART1 () ; fn EXTI10_15 () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn TMU () ; fn GPTIMER1 () ; fn UART2 () ; fn UART3 () ; fn CPTIMER0 () ; fn CPTIMER1 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMAMUX_OVERRUN () ; fn CPTIMERW () ; fn CFMU () ; fn I2C_WKUP () ; fn FWDGT () ; fn CMP0 () ; fn CMP1 () ; fn CMP2 () ; fn CMP3 () ; fn ADC2 () ; fn POC () ; fn GTOC0 () ; fn GTOC1 () ; fn GTOC2 () ; fn GTOC3 () ; fn CMP0_EXTI () ; fn CMP1_EXTI () ; fn CMP2_EXTI () ; fn CMP3_EXTI () ; fn SRAMC_ECC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 93]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD1 } , Vector { _handler : LVD2 } , Vector { _reserved : 0 } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _reserved : 0 } , Vector { _handler : ADC0 } , Vector { _handler : CAN_TX } , Vector { _handler : CAN_RX0 } , Vector { _handler : CAN_RX1 } , Vector { _handler : CAN_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : GPTIMER0 } , Vector { _handler : I2C_EV } , Vector { _handler : I2C_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI } , Vector { _reserved : 0 } , Vector { _handler : UART0 } , Vector { _handler : UART1 } , Vector { _reserved : 0 } , Vector { _handler : EXTI10_15 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : TMU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : GPTIMER1 } , Vector { _reserved : 0 } , Vector { _handler : UART2 } , Vector { _handler : UART3 } , Vector { _handler : CPTIMER0 } , Vector { _handler : CPTIMER1 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMAMUX_OVERRUN } , Vector { _handler : CPTIMERW } , Vector { _reserved : 0 } , Vector { _handler : CFMU } , Vector { _handler : I2C_WKUP } , Vector { _handler : FWDGT } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : CMP2 } , Vector { _handler : CMP3 } , Vector { _reserved : 0 } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _handler : POC } , Vector { _reserved : 0 } , Vector { _handler : GTOC0 } , Vector { _handler : GTOC1 } , Vector { _handler : GTOC2 } , Vector { _handler : GTOC3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CMP0_EXTI } , Vector { _handler : CMP1_EXTI } , Vector { _handler : CMP2_EXTI } , Vector { _handler : CMP3_EXTI } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SRAMC_ECC } ,]
; } pub const OB : gdobecba84d3 :: Ob = unsafe { gdobecba84d3 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const CPTIMER0 : gdcptimeree6db8d9 :: Cptimer = unsafe { gdcptimeree6db8d9 :: Cptimer :: from_ptr (0x4000_0000usize as _) } ; pub const CPTIMER1 : gdcptimeree6db8d9 :: Cptimer = unsafe { gdcptimeree6db8d9 :: Cptimer :: from_ptr (0x4000_0400usize as _) } ; pub const WWDGT : gdwwdgt6968988b :: Wwdgt = unsafe { gdwwdgt6968988b :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt9caf0879 :: Fwdgt = unsafe { gdfwdgt9caf0879 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const UART0 : gduartaa5a7938 :: Uart = unsafe { gduartaa5a7938 :: Uart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART1 : gduartaa5a7938 :: Uart = unsafe { gduartaa5a7938 :: Uart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C : gdi2c8ede78f7 :: I2c = unsafe { gdi2c8ede78f7 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const PMU : gdpmu3925f692 :: Pmu = unsafe { gdpmu3925f692 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac4a56ea36 :: Dac = unsafe { gddac4a56ea36 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART2 : gduartaa5a7938 :: Uart = unsafe { gduartaa5a7938 :: Uart :: from_ptr (0x4000_7800usize as _) } ; pub const UART3 : gduartaa5a7938 :: Uart = unsafe { gduartaa5a7938 :: Uart :: from_ptr (0x4000_7c00usize as _) } ; pub const CFMU : gdcfmu50e649be :: Cfmu = unsafe { gdcfmu50e649be :: Cfmu :: from_ptr (0x4000_c800usize as _) } ; pub const CPTIMERW : gdcptimerw3395205e :: Cptimerw = unsafe { gdcptimerw3395205e :: Cptimerw :: from_ptr (0x4000_e000usize as _) } ; pub const SYSCFG : gdsyscfge224963c :: Syscfg = unsafe { gdsyscfge224963c :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti13fbcd68 :: Exti = unsafe { gdexti13fbcd68 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC0 : gdadc07b93ae04 :: Adc0 = unsafe { gdadc07b93ae04 :: Adc0 :: from_ptr (0x4001_2000usize as _) } ; pub const ADC2 : gdadc2e242f52 :: Adc = unsafe { gdadc2e242f52 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer9c3b27bc :: Timer = unsafe { gdtimer9c3b27bc :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI : gdspi59cf958b :: Spi = unsafe { gdspi59cf958b :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer9c3b27bc :: Timer = unsafe { gdtimer9c3b27bc :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const TIMER1 : gdtimer9c3b27bc :: Timer = unsafe { gdtimer9c3b27bc :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER2 : gdtimer9c3b27bc :: Timer = unsafe { gdtimer9c3b27bc :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const GPTIMER0 : gdgptimer95f98022 :: Gptimer = unsafe { gdgptimer95f98022 :: Gptimer :: from_ptr (0x4001_6000usize as _) } ; pub const GPTIMER1 : gdgptimer95f98022 :: Gptimer = unsafe { gdgptimer95f98022 :: Gptimer :: from_ptr (0x4001_6100usize as _) } ; pub const CMP : gdcmp73832aeb :: Cmp = unsafe { gdcmp73832aeb :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const EVIC : gdevic5169c252 :: Evic = unsafe { gdevic5169c252 :: Evic :: from_ptr (0x4001_8400usize as _) } ; pub const CAN : gdcan60cfeb95 :: Can = unsafe { gdcan60cfeb95 :: Can :: from_ptr (0x4001_a000usize as _) } ; pub const DMA0 : gddmaa3a4fde0 :: Dma = unsafe { gddmaa3a4fde0 :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmaa3a4fde0 :: Dma = unsafe { gddmaa3a4fde0 :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux4c40dca9 :: Dmamux = unsafe { gddmamux4c40dca9 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbase7797637e :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbase7797637e :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcu5a71bf8b :: Rcu = unsafe { gdrcu5a71bf8b :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc5edddd6f :: Fmc = unsafe { gdfmc5edddd6f :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrcba782523 :: Crc = unsafe { gdcrcba782523 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const GPIOG : gdgpio9495ea81 :: Gpio = unsafe { gdgpio9495ea81 :: Gpio :: from_ptr (0x4800_1800usize as _) } ; pub const GPION : gdgpionff502c14 :: Gpion = unsafe { gdgpionff502c14 :: Gpion :: from_ptr (0x4800_4000usize as _) } ; pub const POC : gdpocc3ca8581 :: Poc = unsafe { gdpocc3ca8581 :: Poc :: from_ptr (0x4800_4400usize as _) } ; pub const GTOC0 : gdgtoc78b2467f :: Gtoc = unsafe { gdgtoc78b2467f :: Gtoc :: from_ptr (0x4800_4800usize as _) } ; pub const GTOC1 : gdgtoc78b2467f :: Gtoc = unsafe { gdgtoc78b2467f :: Gtoc :: from_ptr (0x4800_4810usize as _) } ; pub const GTOC2 : gdgtoc78b2467f :: Gtoc = unsafe { gdgtoc78b2467f :: Gtoc :: from_ptr (0x4800_4820usize as _) } ; pub const GTOC3 : gdgtoc78b2467f :: Gtoc = unsafe { gdgtoc78b2467f :: Gtoc :: from_ptr (0x4800_4830usize as _) } ; pub const SVPWM : gdsvpwm8128db8b :: Svpwm = unsafe { gdsvpwm8128db8b :: Svpwm :: from_ptr (0x4802_4000usize as _) } ; pub const TMU : gdtmu6e5ec85c :: Tmu = unsafe { gdtmu6e5ec85c :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const DBG : gddbgbc5f12bf :: Dbg = unsafe { gddbgbc5f12bf :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc07b93ae04_v1.rs"] pub mod gdadc07b93ae04;
#[path="../../peripherals/gdadc2e242f52_v1.rs"] pub mod gdadc2e242f52;
#[path="../../peripherals/gdcan60cfeb95_v1.rs"] pub mod gdcan60cfeb95;
#[path="../../peripherals/gdcfmu50e649be_v1.rs"] pub mod gdcfmu50e649be;
#[path="../../peripherals/gdcmp73832aeb_v1.rs"] pub mod gdcmp73832aeb;
#[path="../../peripherals/gdcptimeree6db8d9_v1.rs"] pub mod gdcptimeree6db8d9;
#[path="../../peripherals/gdcptimerw3395205e_v1.rs"] pub mod gdcptimerw3395205e;
#[path="../../peripherals/gdcrcba782523_v1.rs"] pub mod gdcrcba782523;
#[path="../../peripherals/gddac4a56ea36_v1.rs"] pub mod gddac4a56ea36;
#[path="../../peripherals/gddbgbc5f12bf_v1.rs"] pub mod gddbgbc5f12bf;
#[path="../../peripherals/gddmaa3a4fde0_v1.rs"] pub mod gddmaa3a4fde0;
#[path="../../peripherals/gddmamux4c40dca9_v1.rs"] pub mod gddmamux4c40dca9;
#[path="../../peripherals/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../../peripherals/gddmamuxrmchxcfgbase7797637e_v1.rs"] pub mod gddmamuxrmchxcfgbase7797637e;
#[path="../../peripherals/gdevic5169c252_v1.rs"] pub mod gdevic5169c252;
#[path="../../peripherals/gdexti13fbcd68_v1.rs"] pub mod gdexti13fbcd68;
#[path="../../peripherals/gdfmc5edddd6f_v1.rs"] pub mod gdfmc5edddd6f;
#[path="../../peripherals/gdfwdgt9caf0879_v1.rs"] pub mod gdfwdgt9caf0879;
#[path="../../peripherals/gdgpio9495ea81_v1.rs"] pub mod gdgpio9495ea81;
#[path="../../peripherals/gdgpionff502c14_v1.rs"] pub mod gdgpionff502c14;
#[path="../../peripherals/gdgptimer95f98022_v1.rs"] pub mod gdgptimer95f98022;
#[path="../../peripherals/gdgtoc78b2467f_v1.rs"] pub mod gdgtoc78b2467f;
#[path="../../peripherals/gdi2c8ede78f7_v1.rs"] pub mod gdi2c8ede78f7;
#[path="../../peripherals/gdobecba84d3_v1.rs"] pub mod gdobecba84d3;
#[path="../../peripherals/gdpmu3925f692_v1.rs"] pub mod gdpmu3925f692;
#[path="../../peripherals/gdpocc3ca8581_v1.rs"] pub mod gdpocc3ca8581;
#[path="../../peripherals/gdrcu5a71bf8b_v1.rs"] pub mod gdrcu5a71bf8b;
#[path="../../peripherals/gdspi59cf958b_v1.rs"] pub mod gdspi59cf958b;
#[path="../../peripherals/gdsvpwm8128db8b_v1.rs"] pub mod gdsvpwm8128db8b;
#[path="../../peripherals/gdsyscfge224963c_v1.rs"] pub mod gdsyscfge224963c;
#[path="../../peripherals/gdtimer9c3b27bc_v1.rs"] pub mod gdtimer9c3b27bc;
#[path="../../peripherals/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../../peripherals/gduartaa5a7938_v1.rs"] pub mod gduartaa5a7938;
#[path="../../peripherals/gdwwdgt6968988b_v1.rs"] pub mod gdwwdgt6968988b;
