




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "3 - RTC"]
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
ADC0_1 = 18 , # [doc = "19 - CAN0_MESSAGE"]
CAN0_MESSAGE = 19 , # [doc = "20 - CAN0_BUSOFF"]
CAN0_BUSOFF = 20 , # [doc = "21 - CAN0_ERROR"]
CAN0_ERROR = 21 , # [doc = "22 - CAN0_FASTERROR"]
CAN0_FASTERROR = 22 , # [doc = "23 - CAN0_TEC"]
CAN0_TEC = 23 , # [doc = "24 - CAN0_REC"]
CAN0_REC = 24 , # [doc = "25 - CAN0_WKUP"]
CAN0_WKUP = 25 , # [doc = "26 - TIMER0_BRK_UP_TRG_CMT"]
TIMER0_BRK_UP_TRG_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER19_BRK_UP_TRG_CMT"]
TIMER19_BRK_UP_TRG_CMT = 29 , # [doc = "30 - TIMER19_CHANNEL"]
TIMER19_CHANNEL = 30 , # [doc = "31 - I2C0_EV"]
I2C0_EV = 31 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "33 - I2C1_EV"]
I2C1_EV = 33 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - SPI0"]
SPI0 = 35 , # [doc = "36 - SPI1"]
SPI1 = 36 , # [doc = "37 - USART0"]
USART0 = 37 , # [doc = "38 - USART1"]
USART1 = 38 , # [doc = "39 - USART2"]
USART2 = 39 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "41 - EXTI5_9"]
EXTI5_9 = 41 , # [doc = "42 - TAMPER"]
TAMPER = 42 , # [doc = "43 - TIMER20_BRK_UP_TRG_CMT"]
TIMER20_BRK_UP_TRG_CMT = 43 , # [doc = "44 - TIMER20_CHANNEL"]
TIMER20_CHANNEL = 44 , # [doc = "45 - TIMER7_BRK_UP_TRG_CMT"]
TIMER7_BRK_UP_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - DMAMUX"]
DMAMUX = 47 , # [doc = "48 - SRAMC_ECCSE"]
SRAMC_ECCSE = 48 , # [doc = "49 - CMP"]
CMP = 49 , # [doc = "51 - OVD"]
OVD = 51 , # [doc = "54 - TIMER5_DAC"]
TIMER5_DAC = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "62 - CAN1_WKUP"]
CAN1_WKUP = 62 , # [doc = "63 - CAN1_MESSAGE"]
CAN1_MESSAGE = 63 , # [doc = "64 - CAN1_BUSOFF"]
CAN1_BUSOFF = 64 , # [doc = "65 - CAN1_ERROR"]
CAN1_ERROR = 65 , # [doc = "66 - CAN1_FASTERROR"]
CAN1_FASTERROR = 66 , # [doc = "67 - CAN1_TEC"]
CAN1_TEC = 67 , # [doc = "68 - CAN1_REC"]
CAN1_REC = 68 , # [doc = "69 - FPU"]
FPU = 69 , # [doc = "70 - MFCOM"]
MFCOM = 70 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_MESSAGE () ; fn CAN0_BUSOFF () ; fn CAN0_ERROR () ; fn CAN0_FASTERROR () ; fn CAN0_TEC () ; fn CAN0_REC () ; fn CAN0_WKUP () ; fn TIMER0_BRK_UP_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER19_BRK_UP_TRG_CMT () ; fn TIMER19_CHANNEL () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn EXTI5_9 () ; fn TAMPER () ; fn TIMER20_BRK_UP_TRG_CMT () ; fn TIMER20_CHANNEL () ; fn TIMER7_BRK_UP_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn DMAMUX () ; fn SRAMC_ECCSE () ; fn CMP () ; fn OVD () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn CAN1_WKUP () ; fn CAN1_MESSAGE () ; fn CAN1_BUSOFF () ; fn CAN1_ERROR () ; fn CAN1_FASTERROR () ; fn CAN1_TEC () ; fn CAN1_REC () ; fn FPU () ; fn MFCOM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 71]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_MESSAGE } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ERROR } , Vector { _handler : CAN0_FASTERROR } , Vector { _handler : CAN0_TEC } , Vector { _handler : CAN0_REC } , Vector { _handler : CAN0_WKUP } , Vector { _handler : TIMER0_BRK_UP_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER19_BRK_UP_TRG_CMT } , Vector { _handler : TIMER19_CHANNEL } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TAMPER } , Vector { _handler : TIMER20_BRK_UP_TRG_CMT } , Vector { _handler : TIMER20_CHANNEL } , Vector { _handler : TIMER7_BRK_UP_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : DMAMUX } , Vector { _handler : SRAMC_ECCSE } , Vector { _handler : CMP } , Vector { _reserved : 0 } , Vector { _handler : OVD } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _handler : CAN1_WKUP } , Vector { _handler : CAN1_MESSAGE } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ERROR } , Vector { _handler : CAN1_FASTERROR } , Vector { _handler : CAN1_TEC } , Vector { _handler : CAN1_REC } , Vector { _handler : FPU } , Vector { _handler : MFCOM } ,]
; } pub const OB : gdob28e8bc27 :: Ob = unsafe { gdob28e8bc27 :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER5 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi5d70f94a :: Spi = unsafe { gdspi5d70f94a :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart6d0b41a3 :: Usart = unsafe { gdusart6d0b41a3 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart6d0b41a3 :: Usart = unsafe { gdusart6d0b41a3 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2cecd631a5 :: I2c = unsafe { gdi2cecd631a5 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2cecd631a5 :: I2c = unsafe { gdi2cecd631a5 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkp33ccb7c2 :: Bkp = unsafe { gdbkp33ccb7c2 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmubc000a45 :: Pmu = unsafe { gdpmubc000a45 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac32e8d501 :: Dac = unsafe { gddac32e8d501 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const SYSCFG : gdsyscfg39dede78 :: Syscfg = unsafe { gdsyscfg39dede78 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti29f744de :: Exti = unsafe { gdexti29f744de :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC0 : gdadcaec32a72 :: Adc = unsafe { gdadcaec32a72 :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcaec32a72 :: Adc = unsafe { gdadcaec32a72 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi5d70f94a :: Spi = unsafe { gdspi5d70f94a :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart6d0b41a3 :: Usart = unsafe { gdusart6d0b41a3 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER19 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER20 : gdtimer5e574d75 :: Timer = unsafe { gdtimer5e574d75 :: Timer :: from_ptr (0x4001_5400usize as _) } ; pub const CMP : gdcmpd1466266 :: Cmp = unsafe { gdcmpd1466266 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const TRIGSEL : gdtrigseld16b35fa :: Trigsel = unsafe { gdtrigseld16b35fa :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const CAN0 : gdcanf6d1de49 :: Can = unsafe { gdcanf6d1de49 :: Can :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcanf6d1de49 :: Can = unsafe { gdcanf6d1de49 :: Can :: from_ptr (0x4001_b000usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux4c40dca9 :: Dmamux = unsafe { gddmamux4c40dca9 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbase7797637e :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbase7797637e :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcu1b817abe :: Rcu = unsafe { gdrcu1b817abe :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcee4535d3 :: Fmc = unsafe { gdfmcee4535d3 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const MFCOM : gdmfcomff1dc6ee :: Mfcom = unsafe { gdmfcomff1dc6ee :: Mfcom :: from_ptr (0x4003_8400usize as _) } ; pub const GPIOA : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const DBG : gddbg3f036c14 :: Dbg = unsafe { gddbg3f036c14 :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcaec32a72_v1.rs"] pub mod gdadcaec32a72;
#[path="../../peripherals/gdbkp33ccb7c2_v1.rs"] pub mod gdbkp33ccb7c2;
#[path="../../peripherals/gdcanf6d1de49_v1.rs"] pub mod gdcanf6d1de49;
#[path="../../peripherals/gdcmpd1466266_v1.rs"] pub mod gdcmpd1466266;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gddac32e8d501_v1.rs"] pub mod gddac32e8d501;
#[path="../../peripherals/gddbg3f036c14_v1.rs"] pub mod gddbg3f036c14;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gddmamux4c40dca9_v1.rs"] pub mod gddmamux4c40dca9;
#[path="../../peripherals/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../../peripherals/gddmamuxrmchxcfgbase7797637e_v1.rs"] pub mod gddmamuxrmchxcfgbase7797637e;
#[path="../../peripherals/gdexti29f744de_v1.rs"] pub mod gdexti29f744de;
#[path="../../peripherals/gdfmcee4535d3_v1.rs"] pub mod gdfmcee4535d3;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../../peripherals/gdi2cecd631a5_v1.rs"] pub mod gdi2cecd631a5;
#[path="../../peripherals/gdmfcomff1dc6ee_v1.rs"] pub mod gdmfcomff1dc6ee;
#[path="../../peripherals/gdob28e8bc27_v1.rs"] pub mod gdob28e8bc27;
#[path="../../peripherals/gdpmubc000a45_v1.rs"] pub mod gdpmubc000a45;
#[path="../../peripherals/gdrcu1b817abe_v1.rs"] pub mod gdrcu1b817abe;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdspi5d70f94a_v1.rs"] pub mod gdspi5d70f94a;
#[path="../../peripherals/gdsyscfg39dede78_v1.rs"] pub mod gdsyscfg39dede78;
#[path="../../peripherals/gdtimer5e574d75_v1.rs"] pub mod gdtimer5e574d75;
#[path="../../peripherals/gdtrigseld16b35fa_v1.rs"] pub mod gdtrigseld16b35fa;
#[path="../../peripherals/gdusart6d0b41a3_v1.rs"] pub mod gdusart6d0b41a3;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
