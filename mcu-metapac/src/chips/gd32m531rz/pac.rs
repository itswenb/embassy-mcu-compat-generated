

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD1"]
LVD1 = 1 , # [doc = "2 - LVD2"]
LVD2 = 2 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL5 = 16 , # [doc = "18 - ADC0"]
ADC0 = 18 , # [doc = "19 - CAN_TX"]
CAN_TX = 19 , # [doc = "20 - CAN_RX0"]
CAN_RX0 = 20 , # [doc = "21 - CAN_RX1"]
CAN_RX1 = 21 , # [doc = "22 - CAN_EWMC"]
CAN_EWMC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CM"]
TIMER0_TRG_CM = 26 , # [doc = "27 - TIMER0_CC"]
TIMER0_CC = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER2"]
TIMER2 = 29 , # [doc = "30 - GPTIMER0"]
GPTIMER0 = 30 , # [doc = "31 - I2C_EV"]
I2C_EV = 31 , # [doc = "32 - I2C_ER"]
I2C_ER = 32 , # [doc = "35 - SPI"]
SPI = 35 , # [doc = "37 - UART0"]
UART0 = 37 , # [doc = "38 - UART1"]
UART1 = 38 , # [doc = "40 - EXTI_LINE15_10"]
EXTI_LINE15_10 = 40 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TR_CM"]
TIMER7_TR_CM = 45 , # [doc = "46 - TIMER7_CC"]
TIMER7_CC = 46 , # [doc = "47 - TMU"]
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
DMA1_CHANNEL5 = 61 , # [doc = "62 - DMA_MUX"]
DMA_MUX = 62 , # [doc = "63 - CPTIMERW"]
CPTIMERW = 63 , # [doc = "65 - CFMU"]
CFMU = 65 , # [doc = "66 - I2C_WAKEUP_FROM_EXTI_LINE23"]
I2C_WAKEUP_FROM_EXTI_LINE23 = 66 , # [doc = "67 - FWDGT_FROM_EXTI_LINE22"]
FWDGT_FROM_EXTI_LINE22 = 67 , # [doc = "70 - CMP0"]
CMP0 = 70 , # [doc = "71 - CMP1"]
CMP1 = 71 , # [doc = "72 - CMP2"]
CMP2 = 72 , # [doc = "73 - CMP3"]
CMP3 = 73 , # [doc = "75 - ADC2"]
ADC2 = 75 , # [doc = "77 - POC"]
POC = 77 , # [doc = "78 - EVIC"]
EVIC = 78 , # [doc = "79 - GTOC0"]
GTOC0 = 79 , # [doc = "80 - GTOC1"]
GTOC1 = 80 , # [doc = "81 - GTOC2"]
GTOC2 = 81 , # [doc = "82 - GTOC3"]
GTOC3 = 82 , # [doc = "85 - CMP0_FROM_EXTI"]
CMP0_FROM_EXTI = 85 , # [doc = "86 - CMP1_FROM_EXTI"]
CMP1_FROM_EXTI = 86 , # [doc = "87 - CMP2_FROM_EXTI"]
CMP2_FROM_EXTI = 87 , # [doc = "88 - CMP3_FROM_EXTI"]
CMP3_FROM_EXTI = 88 , # [doc = "92 - SRAM_ECC"]
SRAM_ECC = 92 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD1 () ; fn LVD2 () ; fn FMC () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn ADC0 () ; fn CAN_TX () ; fn CAN_RX0 () ; fn CAN_RX1 () ; fn CAN_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CM () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn GPTIMER0 () ; fn I2C_EV () ; fn I2C_ER () ; fn SPI () ; fn UART0 () ; fn UART1 () ; fn EXTI_LINE15_10 () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TR_CM () ; fn TIMER7_CC () ; fn TMU () ; fn GPTIMER1 () ; fn UART2 () ; fn UART3 () ; fn CPTIMER0 () ; fn CPTIMER1 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA_MUX () ; fn CPTIMERW () ; fn CFMU () ; fn I2C_WAKEUP_FROM_EXTI_LINE23 () ; fn FWDGT_FROM_EXTI_LINE22 () ; fn CMP0 () ; fn CMP1 () ; fn CMP2 () ; fn CMP3 () ; fn ADC2 () ; fn POC () ; fn EVIC () ; fn GTOC0 () ; fn GTOC1 () ; fn GTOC2 () ; fn GTOC3 () ; fn CMP0_FROM_EXTI () ; fn CMP1_FROM_EXTI () ; fn CMP2_FROM_EXTI () ; fn CMP3_FROM_EXTI () ; fn SRAM_ECC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 93]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD1 } , Vector { _handler : LVD2 } , Vector { _reserved : 0 } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _reserved : 0 } , Vector { _handler : ADC0 } , Vector { _handler : CAN_TX } , Vector { _handler : CAN_RX0 } , Vector { _handler : CAN_RX1 } , Vector { _handler : CAN_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CM } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : GPTIMER0 } , Vector { _handler : I2C_EV } , Vector { _handler : I2C_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SPI } , Vector { _reserved : 0 } , Vector { _handler : UART0 } , Vector { _handler : UART1 } , Vector { _reserved : 0 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TR_CM } , Vector { _handler : TIMER7_CC } , Vector { _handler : TMU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : GPTIMER1 } , Vector { _reserved : 0 } , Vector { _handler : UART2 } , Vector { _handler : UART3 } , Vector { _handler : CPTIMER0 } , Vector { _handler : CPTIMER1 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA_MUX } , Vector { _handler : CPTIMERW } , Vector { _reserved : 0 } , Vector { _handler : CFMU } , Vector { _handler : I2C_WAKEUP_FROM_EXTI_LINE23 } , Vector { _handler : FWDGT_FROM_EXTI_LINE22 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CMP0 } , Vector { _handler : CMP1 } , Vector { _handler : CMP2 } , Vector { _handler : CMP3 } , Vector { _reserved : 0 } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _handler : POC } , Vector { _handler : EVIC } , Vector { _handler : GTOC0 } , Vector { _handler : GTOC1 } , Vector { _handler : GTOC2 } , Vector { _handler : GTOC3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CMP0_FROM_EXTI } , Vector { _handler : CMP1_FROM_EXTI } , Vector { _handler : CMP2_FROM_EXTI } , Vector { _handler : CMP3_FROM_EXTI } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SRAM_ECC } ,]
; } pub const CPTIMER0 : gdcptimer0f537712b :: Cptimer0 = unsafe { gdcptimer0f537712b :: Cptimer0 :: from_ptr (0x4000_0000usize as _) } ; pub const CPTIMER1 : gdcptimer0f537712b :: Cptimer0 = unsafe { gdcptimer0f537712b :: Cptimer0 :: from_ptr (0x4000_0400usize as _) } ; pub const WWDGT : gdwwdgt7328a167 :: Wwdgt = unsafe { gdwwdgt7328a167 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgte0a44d28 :: Fwdgt = unsafe { gdfwdgte0a44d28 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const UART0 : gduart0d4cac493 :: Uart0 = unsafe { gduart0d4cac493 :: Uart0 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART1 : gduart0d4cac493 :: Uart0 = unsafe { gduart0d4cac493 :: Uart0 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C : gdi2ca6cc3474 :: I2c = unsafe { gdi2ca6cc3474 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const PMU : gdpmu4ecf2e55 :: Pmu = unsafe { gdpmu4ecf2e55 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddacb75238e9 :: Dac = unsafe { gddacb75238e9 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART2 : gduart0d4cac493 :: Uart0 = unsafe { gduart0d4cac493 :: Uart0 :: from_ptr (0x4000_7800usize as _) } ; pub const UART3 : gduart0d4cac493 :: Uart0 = unsafe { gduart0d4cac493 :: Uart0 :: from_ptr (0x4000_7c00usize as _) } ; pub const CFMU : gdcfmud735e759 :: Cfmu = unsafe { gdcfmud735e759 :: Cfmu :: from_ptr (0x4000_c800usize as _) } ; pub const CPTIMERW : gdcptimerwc742ef6c :: Cptimerw = unsafe { gdcptimerwc742ef6c :: Cptimerw :: from_ptr (0x4000_e000usize as _) } ; pub const SYSCFG : gdsyscfg8db51c8b :: Syscfg = unsafe { gdsyscfg8db51c8b :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextibab4ad71 :: Exti = unsafe { gdextibab4ad71 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const ADC0 : gdadc088c1e886 :: Adc0 = unsafe { gdadc088c1e886 :: Adc0 :: from_ptr (0x4001_2000usize as _) } ; pub const ADC2 : gdadc25692136a :: Adc2 = unsafe { gdadc25692136a :: Adc2 :: from_ptr (0x4001_2400usize as _) } ; pub const TIMER0 : gdtimer000cb8605 :: Timer0 = unsafe { gdtimer000cb8605 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI : gdspiea15830d :: Spi = unsafe { gdspiea15830d :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer000cb8605 :: Timer0 = unsafe { gdtimer000cb8605 :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const TIMER1 : gdtimer1ed17b6a8 :: Timer1 = unsafe { gdtimer1ed17b6a8 :: Timer1 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER2 : gdtimer1ed17b6a8 :: Timer1 = unsafe { gdtimer1ed17b6a8 :: Timer1 :: from_ptr (0x4001_4400usize as _) } ; pub const GPTIMER0 : gdgptimer002f92dbb :: Gptimer0 = unsafe { gdgptimer002f92dbb :: Gptimer0 :: from_ptr (0x4001_6000usize as _) } ; pub const GPTIMER1 : gdgptimer002f92dbb :: Gptimer0 = unsafe { gdgptimer002f92dbb :: Gptimer0 :: from_ptr (0x4001_6100usize as _) } ; pub const CMP : gdcmp6cf4a780 :: Cmp = unsafe { gdcmp6cf4a780 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const EVIC : gdevicf10e9e33 :: Evic = unsafe { gdevicf10e9e33 :: Evic :: from_ptr (0x4001_8400usize as _) } ; pub const CAN : gdcan8d97a339 :: Can = unsafe { gdcan8d97a339 :: Can :: from_ptr (0x4001_a000usize as _) } ; pub const DMA0 : gddma0586e39d1 :: Dma0 = unsafe { gddma0586e39d1 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma1b108675d :: Dma1 = unsafe { gddma1b108675d :: Dma1 :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux77665c6a :: Dmamux = unsafe { gddmamux77665c6a :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcu714c8771 :: Rcu = unsafe { gdrcu714c8771 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcd2f13365 :: Fmc = unsafe { gdfmcd2f13365 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc553be872 :: Crc = unsafe { gdcrc553be872 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_1400usize as _) } ; pub const GPIOG : gdgpioaf9033ab6 :: Gpioa = unsafe { gdgpioaf9033ab6 :: Gpioa :: from_ptr (0x4800_1800usize as _) } ; pub const GPION : gdgpionfd68a396 :: Gpion = unsafe { gdgpionfd68a396 :: Gpion :: from_ptr (0x4800_4000usize as _) } ; pub const POC : gdpocca2dbf68 :: Poc = unsafe { gdpocca2dbf68 :: Poc :: from_ptr (0x4800_4400usize as _) } ; pub const GTOC : gdgtoc1d40c5d1 :: Gtoc = unsafe { gdgtoc1d40c5d1 :: Gtoc :: from_ptr (0x4800_4800usize as _) } ; pub const SVPWM : gdsvpwmc75a0f03 :: Svpwm = unsafe { gdsvpwmc75a0f03 :: Svpwm :: from_ptr (0x4802_4000usize as _) } ; pub const TMU : gdtmuca711897 :: Tmu = unsafe { gdtmuca711897 :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const DBG : gddbg217d467b :: Dbg = unsafe { gddbg217d467b :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc088c1e886_v1.rs"] pub mod gdadc088c1e886;
#[path="../../peripherals/gdadc25692136a_v1.rs"] pub mod gdadc25692136a;
#[path="../../peripherals/gdcan8d97a339_v1.rs"] pub mod gdcan8d97a339;
#[path="../../peripherals/gdcfmud735e759_v1.rs"] pub mod gdcfmud735e759;
#[path="../../peripherals/gdcmp6cf4a780_v1.rs"] pub mod gdcmp6cf4a780;
#[path="../../peripherals/gdcptimer0f537712b_v1.rs"] pub mod gdcptimer0f537712b;
#[path="../../peripherals/gdcptimerwc742ef6c_v1.rs"] pub mod gdcptimerwc742ef6c;
#[path="../../peripherals/gdcrc553be872_v1.rs"] pub mod gdcrc553be872;
#[path="../../peripherals/gddacb75238e9_v1.rs"] pub mod gddacb75238e9;
#[path="../../peripherals/gddbg217d467b_v1.rs"] pub mod gddbg217d467b;
#[path="../../peripherals/gddma0586e39d1_v1.rs"] pub mod gddma0586e39d1;
#[path="../../peripherals/gddma1b108675d_v1.rs"] pub mod gddma1b108675d;
#[path="../../peripherals/gddmamux77665c6a_v1.rs"] pub mod gddmamux77665c6a;
#[path="../../peripherals/gdevicf10e9e33_v1.rs"] pub mod gdevicf10e9e33;
#[path="../../peripherals/gdextibab4ad71_v1.rs"] pub mod gdextibab4ad71;
#[path="../../peripherals/gdfmcd2f13365_v1.rs"] pub mod gdfmcd2f13365;
#[path="../../peripherals/gdfwdgte0a44d28_v1.rs"] pub mod gdfwdgte0a44d28;
#[path="../../peripherals/gdgpioaf9033ab6_v1.rs"] pub mod gdgpioaf9033ab6;
#[path="../../peripherals/gdgpionfd68a396_v1.rs"] pub mod gdgpionfd68a396;
#[path="../../peripherals/gdgptimer002f92dbb_v1.rs"] pub mod gdgptimer002f92dbb;
#[path="../../peripherals/gdgtoc1d40c5d1_v1.rs"] pub mod gdgtoc1d40c5d1;
#[path="../../peripherals/gdi2ca6cc3474_v1.rs"] pub mod gdi2ca6cc3474;
#[path="../../peripherals/gdpmu4ecf2e55_v1.rs"] pub mod gdpmu4ecf2e55;
#[path="../../peripherals/gdpocca2dbf68_v1.rs"] pub mod gdpocca2dbf68;
#[path="../../peripherals/gdrcu714c8771_v1.rs"] pub mod gdrcu714c8771;
#[path="../../peripherals/gdspiea15830d_v1.rs"] pub mod gdspiea15830d;
#[path="../../peripherals/gdsvpwmc75a0f03_v1.rs"] pub mod gdsvpwmc75a0f03;
#[path="../../peripherals/gdsyscfg8db51c8b_v1.rs"] pub mod gdsyscfg8db51c8b;
#[path="../../peripherals/gdtimer000cb8605_v1.rs"] pub mod gdtimer000cb8605;
#[path="../../peripherals/gdtimer1ed17b6a8_v1.rs"] pub mod gdtimer1ed17b6a8;
#[path="../../peripherals/gdtmuca711897_v1.rs"] pub mod gdtmuca711897;
#[path="../../peripherals/gduart0d4cac493_v1.rs"] pub mod gduart0d4cac493;
#[path="../../peripherals/gdwwdgt7328a167_v1.rs"] pub mod gdwwdgt7328a167;
