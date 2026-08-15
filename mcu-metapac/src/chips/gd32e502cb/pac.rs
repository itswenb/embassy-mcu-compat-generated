

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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC"]
ADC = 18 , # [doc = "19 - CAN0_MF"]
CAN0_MF = 19 , # [doc = "20 - CAN0_BUSOFF"]
CAN0_BUSOFF = 20 , # [doc = "21 - CAN0_ER"]
CAN0_ER = 21 , # [doc = "22 - CAN0_ERFT"]
CAN0_ERFT = 22 , # [doc = "23 - CAN0_TW"]
CAN0_TW = 23 , # [doc = "24 - CAN0_RW"]
CAN0_RW = 24 , # [doc = "25 - CAN0_WAKE"]
CAN0_WAKE = 25 , # [doc = "26 - TIMER0_BRK_UP_TR_CM"]
TIMER0_BRK_UP_TR_CM = 26 , # [doc = "27 - TIMER0_CAP"]
TIMER0_CAP = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER19_BRK_UP_TR_CM"]
TIMER19_BRK_UP_TR_CM = 29 , # [doc = "30 - TIMER19_CAP"]
TIMER19_CAP = 30 , # [doc = "31 - I2C0_EV"]
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
TAMPER = 42 , # [doc = "43 - TIMER20_BRK_UP_TR_CM"]
TIMER20_BRK_UP_TR_CM = 43 , # [doc = "44 - TIMER20_CAP"]
TIMER20_CAP = 44 , # [doc = "45 - TIMER7_BRK_UP_TR_CM"]
TIMER7_BRK_UP_TR_CM = 45 , # [doc = "46 - TIMER7_CAP"]
TIMER7_CAP = 46 , # [doc = "47 - DMA_MUX"]
DMA_MUX = 47 , # [doc = "49 - CMP"]
CMP = 49 , # [doc = "51 - OVD"]
OVD = 51 , # [doc = "54 - TIMER5_DAC"]
TIMER5_DAC = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "62 - CAN1_WAKE"]
CAN1_WAKE = 62 , # [doc = "63 - CAN1_MF"]
CAN1_MF = 63 , # [doc = "64 - CAN1_BUSOFF"]
CAN1_BUSOFF = 64 , # [doc = "65 - CAN1_ER"]
CAN1_ER = 65 , # [doc = "66 - CAN1_ERFT"]
CAN1_ERFT = 66 , # [doc = "67 - CAN1_TW"]
CAN1_TW = 67 , # [doc = "68 - CAN1_RW"]
CAN1_RW = 68 , # [doc = "70 - MFCOM"]
MFCOM = 70 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC () ; fn CAN0_MF () ; fn CAN0_BUSOFF () ; fn CAN0_ER () ; fn CAN0_ERFT () ; fn CAN0_TW () ; fn CAN0_RW () ; fn CAN0_WAKE () ; fn TIMER0_BRK_UP_TR_CM () ; fn TIMER0_CAP () ; fn TIMER1 () ; fn TIMER19_BRK_UP_TR_CM () ; fn TIMER19_CAP () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn EXTI5_9 () ; fn TAMPER () ; fn TIMER20_BRK_UP_TR_CM () ; fn TIMER20_CAP () ; fn TIMER7_BRK_UP_TR_CM () ; fn TIMER7_CAP () ; fn DMA_MUX () ; fn CMP () ; fn OVD () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn CAN1_WAKE () ; fn CAN1_MF () ; fn CAN1_BUSOFF () ; fn CAN1_ER () ; fn CAN1_ERFT () ; fn CAN1_TW () ; fn CAN1_RW () ; fn MFCOM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 71]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _reserved : 0 } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC } , Vector { _handler : CAN0_MF } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ER } , Vector { _handler : CAN0_ERFT } , Vector { _handler : CAN0_TW } , Vector { _handler : CAN0_RW } , Vector { _handler : CAN0_WAKE } , Vector { _handler : TIMER0_BRK_UP_TR_CM } , Vector { _handler : TIMER0_CAP } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER19_BRK_UP_TR_CM } , Vector { _handler : TIMER19_CAP } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TAMPER } , Vector { _handler : TIMER20_BRK_UP_TR_CM } , Vector { _handler : TIMER20_CAP } , Vector { _handler : TIMER7_BRK_UP_TR_CM } , Vector { _handler : TIMER7_CAP } , Vector { _handler : DMA_MUX } , Vector { _reserved : 0 } , Vector { _handler : CMP } , Vector { _reserved : 0 } , Vector { _handler : OVD } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _handler : CAN1_WAKE } , Vector { _handler : CAN1_MF } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ER } , Vector { _handler : CAN1_ERFT } , Vector { _handler : CAN1_TW } , Vector { _handler : CAN1_RW } , Vector { _reserved : 0 } , Vector { _handler : MFCOM } ,]
; } pub const TIMER1 : gdtimer1cb2db824 :: Timer1 = unsafe { gdtimer1cb2db824 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER5 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer642c906a4 :: Timer6 = unsafe { gdtimer642c906a4 :: Timer6 :: from_ptr (0x4000_1400usize as _) } ; pub const RTC : gdrtc6b0c077c :: Rtc = unsafe { gdrtc6b0c077c :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt9ccc125f :: Fwdgt = unsafe { gdfwdgt9ccc125f :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi14e571efb :: Spi1 = unsafe { gdspi14e571efb :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const USART1 : gdusart042a368e4 :: Usart0 = unsafe { gdusart042a368e4 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart042a368e4 :: Usart0 = unsafe { gdusart042a368e4 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const I2C0 : gdi2c000bb4e12 :: I2c0 = unsafe { gdi2c000bb4e12 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c19c81d997 :: I2c1 = unsafe { gdi2c19c81d997 :: I2c1 :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkp726e313a :: Bkp = unsafe { gdbkp726e313a :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu2d47d9c6 :: Pmu = unsafe { gdpmu2d47d9c6 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddacde26c92b :: Dac = unsafe { gddacde26c92b :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const SYSCFG : gdsyscfg9b2fb855 :: Syscfg = unsafe { gdsyscfg9b2fb855 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextibdc5df6b :: Exti = unsafe { gdextibdc5df6b :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC0 : gdadc09259a092 :: Adc0 = unsafe { gdadc09259a092 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc1bf095765 :: Adc1 = unsafe { gdadc1bf095765 :: Adc1 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0a62f20ab :: Timer0 = unsafe { gdtimer0a62f20ab :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0c6850d65 :: Spi0 = unsafe { gdspi0c6850d65 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer0a62f20ab :: Timer0 = unsafe { gdtimer0a62f20ab :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart042a368e4 :: Usart0 = unsafe { gdusart042a368e4 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER19 : gdtimer0a62f20ab :: Timer0 = unsafe { gdtimer0a62f20ab :: Timer0 :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER20 : gdtimer0a62f20ab :: Timer0 = unsafe { gdtimer0a62f20ab :: Timer0 :: from_ptr (0x4001_5400usize as _) } ; pub const CMP : gdcmp8f451a3d :: Cmp = unsafe { gdcmp8f451a3d :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const TRIGSEL : gdtrigsel2179cb80 :: Trigsel = unsafe { gdtrigsel2179cb80 :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const CAN0 : gdcan0e1070584 :: Can0 = unsafe { gdcan0e1070584 :: Can0 :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcan0e1070584 :: Can0 = unsafe { gdcan0e1070584 :: Can0 :: from_ptr (0x4001_b000usize as _) } ; pub const DMA0 : gddma082147e2f :: Dma0 = unsafe { gddma082147e2f :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma1bb311858 :: Dma1 = unsafe { gddma1bb311858 :: Dma1 :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux7bf7f916 :: Dmamux = unsafe { gddmamux7bf7f916 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcue53fc96d :: Rcu = unsafe { gdrcue53fc96d :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcd32845c2 :: Fmc = unsafe { gdfmcd32845c2 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc8a4036fe :: Crc = unsafe { gdcrc8a4036fe :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const MFCOM : gdmfcomdfea6a59 :: Mfcom = unsafe { gdmfcomdfea6a59 :: Mfcom :: from_ptr (0x4003_8400usize as _) } ; pub const GPIOA : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob3a01cf30 :: Gpiob = unsafe { gdgpiob3a01cf30 :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpioa9804d271 :: Gpioa = unsafe { gdgpioa9804d271 :: Gpioa :: from_ptr (0x4800_1400usize as _) } ; pub const DBG : gddbg2b068fbb :: Dbg = unsafe { gddbg2b068fbb :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc09259a092_v1.rs"] pub mod gdadc09259a092;
#[path="../../peripherals/gdadc1bf095765_v1.rs"] pub mod gdadc1bf095765;
#[path="../../peripherals/gdbkp726e313a_v1.rs"] pub mod gdbkp726e313a;
#[path="../../peripherals/gdcan0e1070584_v1.rs"] pub mod gdcan0e1070584;
#[path="../../peripherals/gdcmp8f451a3d_v1.rs"] pub mod gdcmp8f451a3d;
#[path="../../peripherals/gdcrc8a4036fe_v1.rs"] pub mod gdcrc8a4036fe;
#[path="../../peripherals/gddacde26c92b_v1.rs"] pub mod gddacde26c92b;
#[path="../../peripherals/gddbg2b068fbb_v1.rs"] pub mod gddbg2b068fbb;
#[path="../../peripherals/gddma082147e2f_v1.rs"] pub mod gddma082147e2f;
#[path="../../peripherals/gddma1bb311858_v1.rs"] pub mod gddma1bb311858;
#[path="../../peripherals/gddmamux7bf7f916_v1.rs"] pub mod gddmamux7bf7f916;
#[path="../../peripherals/gdextibdc5df6b_v1.rs"] pub mod gdextibdc5df6b;
#[path="../../peripherals/gdfmcd32845c2_v1.rs"] pub mod gdfmcd32845c2;
#[path="../../peripherals/gdfwdgt9ccc125f_v1.rs"] pub mod gdfwdgt9ccc125f;
#[path="../../peripherals/gdgpioa9804d271_v1.rs"] pub mod gdgpioa9804d271;
#[path="../../peripherals/gdgpiob3a01cf30_v1.rs"] pub mod gdgpiob3a01cf30;
#[path="../../peripherals/gdi2c000bb4e12_v1.rs"] pub mod gdi2c000bb4e12;
#[path="../../peripherals/gdi2c19c81d997_v1.rs"] pub mod gdi2c19c81d997;
#[path="../../peripherals/gdmfcomdfea6a59_v1.rs"] pub mod gdmfcomdfea6a59;
#[path="../../peripherals/gdpmu2d47d9c6_v1.rs"] pub mod gdpmu2d47d9c6;
#[path="../../peripherals/gdrcue53fc96d_v1.rs"] pub mod gdrcue53fc96d;
#[path="../../peripherals/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../../peripherals/gdspi0c6850d65_v1.rs"] pub mod gdspi0c6850d65;
#[path="../../peripherals/gdspi14e571efb_v1.rs"] pub mod gdspi14e571efb;
#[path="../../peripherals/gdsyscfg9b2fb855_v1.rs"] pub mod gdsyscfg9b2fb855;
#[path="../../peripherals/gdtimer0a62f20ab_v1.rs"] pub mod gdtimer0a62f20ab;
#[path="../../peripherals/gdtimer1cb2db824_v1.rs"] pub mod gdtimer1cb2db824;
#[path="../../peripherals/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../../peripherals/gdtimer642c906a4_v1.rs"] pub mod gdtimer642c906a4;
#[path="../../peripherals/gdtrigsel2179cb80_v1.rs"] pub mod gdtrigsel2179cb80;
#[path="../../peripherals/gdusart042a368e4_v1.rs"] pub mod gdusart042a368e4;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
