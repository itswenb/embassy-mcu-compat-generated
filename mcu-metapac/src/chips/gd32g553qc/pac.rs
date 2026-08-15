

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD_VAVD_VOVD_VUVD"]
LVD_VAVD_VOVD_VUVD = 1 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC_WKUP"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT_IDX"]
TIMER0_TRG_CMT_IDX = 26 , # [doc = "27 - TIMER0_CHANNEL"]
TIMER0_CHANNEL = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER2"]
TIMER2 = 29 , # [doc = "30 - TIMER3"]
TIMER3 = 30 , # [doc = "31 - I2C0_EV_WKUP"]
I2C0_EV_WKUP = 31 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "33 - I2C1_EV_WKUP"]
I2C1_EV_WKUP = 33 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - SPI0"]
SPI0 = 35 , # [doc = "36 - SPI1"]
SPI1 = 36 , # [doc = "37 - USART0"]
USART0 = 37 , # [doc = "38 - USART1"]
USART1 = 38 , # [doc = "39 - USART2"]
USART2 = 39 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "43 - TIMER7_BRK_TRS_IDX"]
TIMER7_BRK_TRS_IDX = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT_IDX"]
TIMER7_TRG_CMT_IDX = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - SYSCFG"]
SYSCFG = 48 , # [doc = "49 - LPTIMER"]
LPTIMER = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC0_2"]
TIMER5_DAC0_2 = 54 , # [doc = "55 - TIMER6_DAC1_3"]
TIMER6_DAC1_3 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - ADC3"]
ADC3 = 61 , # [doc = "63 - VUVD1_VOVD1"]
VUVD1_VOVD1 = 63 , # [doc = "64 - CMP0_3"]
CMP0_3 = 64 , # [doc = "65 - CMP4_7"]
CMP4_7 = 65 , # [doc = "66 - CMP"]
CMP = 66 , # [doc = "67 - HRTIMER_IRQ0"]
HRTIMER_IRQ0 = 67 , # [doc = "68 - HRTIMER_IRQ1"]
HRTIMER_IRQ1 = 68 , # [doc = "69 - HRTIMER_IRQ2"]
HRTIMER_IRQ2 = 69 , # [doc = "70 - HRTIMER_IRQ3"]
HRTIMER_IRQ3 = 70 , # [doc = "71 - HRTIMER_IRQ4"]
HRTIMER_IRQ4 = 71 , # [doc = "72 - HRTIMER_IRQ5"]
HRTIMER_IRQ5 = 72 , # [doc = "73 - HRTIMER_IRQ6"]
HRTIMER_IRQ6 = 73 , # [doc = "74 - HRTIMER_IRQ7"]
HRTIMER_IRQ7 = 74 , # [doc = "75 - HRTIMER_IRQ8"]
HRTIMER_IRQ8 = 75 , # [doc = "76 - HRTIMER_IRQ9"]
HRTIMER_IRQ9 = 76 , # [doc = "77 - TIMER19_BRK_TRS_IDX"]
TIMER19_BRK_TRS_IDX = 77 , # [doc = "78 - TIMER19_UP"]
TIMER19_UP = 78 , # [doc = "79 - TIMER19_TRG_CMT_IDX"]
TIMER19_TRG_CMT_IDX = 79 , # [doc = "80 - TIMER19_CHANNEL"]
TIMER19_CHANNEL = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "82 - I2C2_EV_WKUP"]
I2C2_EV_WKUP = 82 , # [doc = "83 - I2C2_ER"]
I2C2_ER = 83 , # [doc = "85 - CAU"]
CAU = 85 , # [doc = "90 - TRNG"]
TRNG = 90 , # [doc = "92 - I2C3_EV_WKUP"]
I2C3_EV_WKUP = 92 , # [doc = "93 - I2C3_ER"]
I2C3_ER = 93 , # [doc = "94 - DMAMUX_OVR"]
DMAMUX_OVR = 94 , # [doc = "95 - QSPI"]
QSPI = 95 , # [doc = "96 - FFT"]
FFT = 96 , # [doc = "97 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 97 , # [doc = "98 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 98 , # [doc = "99 - CLA"]
CLA = 99 , # [doc = "100 - TMU"]
TMU = 100 , # [doc = "101 - FAC"]
FAC = 101 , # [doc = "102 - HPDF0"]
HPDF0 = 102 , # [doc = "103 - HPDF1"]
HPDF1 = 103 , # [doc = "104 - HPDF2"]
HPDF2 = 104 , # [doc = "105 - HPDF3"]
HPDF3 = 105 , # [doc = "106 - TIMER14"]
TIMER14 = 106 , # [doc = "107 - TIMER15"]
TIMER15 = 107 , # [doc = "108 - TIMER16"]
TIMER16 = 108 , # [doc = "109 - CAN0_WKUP"]
CAN0_WKUP = 109 , # [doc = "110 - CAN0_MESSAGE"]
CAN0_MESSAGE = 110 , # [doc = "111 - CAN0_BUSOFF"]
CAN0_BUSOFF = 111 , # [doc = "112 - CAN0_ERROR"]
CAN0_ERROR = 112 , # [doc = "113 - CAN0_FASTERROR"]
CAN0_FASTERROR = 113 , # [doc = "114 - CAN0_TEC"]
CAN0_TEC = 114 , # [doc = "115 - CAN0_REC"]
CAN0_REC = 115 , # [doc = "116 - CAN1_WKUP"]
CAN1_WKUP = 116 , # [doc = "117 - CAN1_MESSAGE"]
CAN1_MESSAGE = 117 , # [doc = "118 - CAN1_BUSOFF"]
CAN1_BUSOFF = 118 , # [doc = "119 - CAN1_ERROR"]
CAN1_ERROR = 119 , # [doc = "120 - CAN1_FASTERROR"]
CAN1_FASTERROR = 120 , # [doc = "121 - CAN1_TEC"]
CAN1_TEC = 121 , # [doc = "122 - CAN1_REC"]
CAN1_REC = 122 , # [doc = "123 - CAN2_WKUP"]
CAN2_WKUP = 123 , # [doc = "124 - CAN2_MESSAGE"]
CAN2_MESSAGE = 124 , # [doc = "125 - CAN2_BUSOFF"]
CAN2_BUSOFF = 125 , # [doc = "126 - CAN2_ERROR"]
CAN2_ERROR = 126 , # [doc = "127 - CAN2_FASTERROR"]
CAN2_FASTERROR = 127 , # [doc = "128 - CAN2_TEC"]
CAN2_TEC = 128 , # [doc = "129 - CAN2_REC"]
CAN2_REC = 129 , # [doc = "130 - TIMER0_DEC"]
TIMER0_DEC = 130 , # [doc = "131 - TIMER1_DEC"]
TIMER1_DEC = 131 , # [doc = "132 - TIMER2_DEC"]
TIMER2_DEC = 132 , # [doc = "133 - TIMER3_DEC"]
TIMER3_DEC = 133 , # [doc = "134 - TIMER4_DEC"]
TIMER4_DEC = 134 , # [doc = "135 - TIMER7_DEC"]
TIMER7_DEC = 135 , # [doc = "136 - TIMER19_DEC"]
TIMER19_DEC = 136 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD_VAVD_VOVD_VUVD () ; fn TAMPER () ; fn RTC_WKUP () ; fn FMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT_IDX () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV_WKUP () ; fn I2C0_ER () ; fn I2C1_EV_WKUP () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn TIMER7_BRK_TRS_IDX () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT_IDX () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn SYSCFG () ; fn LPTIMER () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC0_2 () ; fn TIMER6_DAC1_3 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ADC3 () ; fn VUVD1_VOVD1 () ; fn CMP0_3 () ; fn CMP4_7 () ; fn CMP () ; fn HRTIMER_IRQ0 () ; fn HRTIMER_IRQ1 () ; fn HRTIMER_IRQ2 () ; fn HRTIMER_IRQ3 () ; fn HRTIMER_IRQ4 () ; fn HRTIMER_IRQ5 () ; fn HRTIMER_IRQ6 () ; fn HRTIMER_IRQ7 () ; fn HRTIMER_IRQ8 () ; fn HRTIMER_IRQ9 () ; fn TIMER19_BRK_TRS_IDX () ; fn TIMER19_UP () ; fn TIMER19_TRG_CMT_IDX () ; fn TIMER19_CHANNEL () ; fn FPU () ; fn I2C2_EV_WKUP () ; fn I2C2_ER () ; fn CAU () ; fn TRNG () ; fn I2C3_EV_WKUP () ; fn I2C3_ER () ; fn DMAMUX_OVR () ; fn QSPI () ; fn FFT () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn CLA () ; fn TMU () ; fn FAC () ; fn HPDF0 () ; fn HPDF1 () ; fn HPDF2 () ; fn HPDF3 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn CAN0_WKUP () ; fn CAN0_MESSAGE () ; fn CAN0_BUSOFF () ; fn CAN0_ERROR () ; fn CAN0_FASTERROR () ; fn CAN0_TEC () ; fn CAN0_REC () ; fn CAN1_WKUP () ; fn CAN1_MESSAGE () ; fn CAN1_BUSOFF () ; fn CAN1_ERROR () ; fn CAN1_FASTERROR () ; fn CAN1_TEC () ; fn CAN1_REC () ; fn CAN2_WKUP () ; fn CAN2_MESSAGE () ; fn CAN2_BUSOFF () ; fn CAN2_ERROR () ; fn CAN2_FASTERROR () ; fn CAN2_TEC () ; fn CAN2_REC () ; fn TIMER0_DEC () ; fn TIMER1_DEC () ; fn TIMER2_DEC () ; fn TIMER3_DEC () ; fn TIMER4_DEC () ; fn TIMER7_DEC () ; fn TIMER19_DEC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 137]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD_VAVD_VOVD_VUVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT_IDX } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV_WKUP } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV_WKUP } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK_TRS_IDX } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT_IDX } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _handler : SYSCFG } , Vector { _handler : LPTIMER } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC0_2 } , Vector { _handler : TIMER6_DAC1_3 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ADC3 } , Vector { _reserved : 0 } , Vector { _handler : VUVD1_VOVD1 } , Vector { _handler : CMP0_3 } , Vector { _handler : CMP4_7 } , Vector { _handler : CMP } , Vector { _handler : HRTIMER_IRQ0 } , Vector { _handler : HRTIMER_IRQ1 } , Vector { _handler : HRTIMER_IRQ2 } , Vector { _handler : HRTIMER_IRQ3 } , Vector { _handler : HRTIMER_IRQ4 } , Vector { _handler : HRTIMER_IRQ5 } , Vector { _handler : HRTIMER_IRQ6 } , Vector { _handler : HRTIMER_IRQ7 } , Vector { _handler : HRTIMER_IRQ8 } , Vector { _handler : HRTIMER_IRQ9 } , Vector { _handler : TIMER19_BRK_TRS_IDX } , Vector { _handler : TIMER19_UP } , Vector { _handler : TIMER19_TRG_CMT_IDX } , Vector { _handler : TIMER19_CHANNEL } , Vector { _handler : FPU } , Vector { _handler : I2C2_EV_WKUP } , Vector { _handler : I2C2_ER } , Vector { _reserved : 0 } , Vector { _handler : CAU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TRNG } , Vector { _reserved : 0 } , Vector { _handler : I2C3_EV_WKUP } , Vector { _handler : I2C3_ER } , Vector { _handler : DMAMUX_OVR } , Vector { _handler : QSPI } , Vector { _handler : FFT } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : CLA } , Vector { _handler : TMU } , Vector { _handler : FAC } , Vector { _handler : HPDF0 } , Vector { _handler : HPDF1 } , Vector { _handler : HPDF2 } , Vector { _handler : HPDF3 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : CAN0_WKUP } , Vector { _handler : CAN0_MESSAGE } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ERROR } , Vector { _handler : CAN0_FASTERROR } , Vector { _handler : CAN0_TEC } , Vector { _handler : CAN0_REC } , Vector { _handler : CAN1_WKUP } , Vector { _handler : CAN1_MESSAGE } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ERROR } , Vector { _handler : CAN1_FASTERROR } , Vector { _handler : CAN1_TEC } , Vector { _handler : CAN1_REC } , Vector { _handler : CAN2_WKUP } , Vector { _handler : CAN2_MESSAGE } , Vector { _handler : CAN2_BUSOFF } , Vector { _handler : CAN2_ERROR } , Vector { _handler : CAN2_FASTERROR } , Vector { _handler : CAN2_TEC } , Vector { _handler : CAN2_REC } , Vector { _handler : TIMER0_DEC } , Vector { _handler : TIMER1_DEC } , Vector { _handler : TIMER2_DEC } , Vector { _handler : TIMER3_DEC } , Vector { _handler : TIMER4_DEC } , Vector { _handler : TIMER7_DEC } , Vector { _handler : TIMER19_DEC } ,]
; } pub const TIMER1 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const RTC : gdrtcd5d8d4bc :: Rtc = unsafe { gdrtcd5d8d4bc :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspiea8c377b :: Spi = unsafe { gdspiea8c377b :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspiea8c377b :: Spi = unsafe { gdspiea8c377b :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusartc72580ea :: Usart = unsafe { gdusartc72580ea :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartc72580ea :: Usart = unsafe { gdusartc72580ea :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartc72580ea :: Usart = unsafe { gdusartc72580ea :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartc72580ea :: Usart = unsafe { gdusartc72580ea :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const I2C3 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5c00usize as _) } ; pub const PMU : gdpmu1dc33268 :: Pmu = unsafe { gdpmu1dc33268 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const LPTIMER : gdlptimer265af638 :: Lptimer = unsafe { gdlptimer265af638 :: Lptimer :: from_ptr (0x4000_9400usize as _) } ; pub const I2C2 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_c000usize as _) } ; pub const SYSCFG : gdsyscfgee183683 :: Syscfg = unsafe { gdsyscfgee183683 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti8937c1c8 :: Exti = unsafe { gdexti8937c1c8 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const TIMER0 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspiea8c377b :: Spi = unsafe { gdspiea8c377b :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusartc72580ea :: Usart = unsafe { gdusartc72580ea :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const TIMER19 : gdtimerd311b1a2 :: Timer = unsafe { gdtimerd311b1a2 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const HRTIMER0 : gdhrtimerc7445402 :: Hrtimer = unsafe { gdhrtimerc7445402 :: Hrtimer :: from_ptr (0x4001_5800usize as _) } ; pub const HPDF : gdhpdfdafb56e7 :: Hpdf = unsafe { gdhpdfdafb56e7 :: Hpdf :: from_ptr (0x4001_7000usize as _) } ; pub const HPDF_FLT0 : gdhpdfdafb56e7 :: Hpdf = unsafe { gdhpdfdafb56e7 :: Hpdf :: from_ptr (0x4001_7100usize as _) } ; pub const HPDF_FLT1 : gdhpdfdafb56e7 :: Hpdf = unsafe { gdhpdfdafb56e7 :: Hpdf :: from_ptr (0x4001_7180usize as _) } ; pub const HPDF_FLT2 : gdhpdfdafb56e7 :: Hpdf = unsafe { gdhpdfdafb56e7 :: Hpdf :: from_ptr (0x4001_7200usize as _) } ; pub const HPDF_FLT3 : gdhpdfdafb56e7 :: Hpdf = unsafe { gdhpdfdafb56e7 :: Hpdf :: from_ptr (0x4001_7280usize as _) } ; pub const VREF : gdvref779f5a9e :: Vref = unsafe { gdvref779f5a9e :: Vref :: from_ptr (0x4001_7800usize as _) } ; pub const CMP : gdcmp4263a684 :: Cmp = unsafe { gdcmp4263a684 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const TRIGSEL : gdtrigsel37febbbf :: Trigsel = unsafe { gdtrigsel37febbbf :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const CAN0 : gdcan22e45210 :: Can = unsafe { gdcan22e45210 :: Can :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcan22e45210 :: Can = unsafe { gdcan22e45210 :: Can :: from_ptr (0x4001_b000usize as _) } ; pub const CAN2 : gdcan22e45210 :: Can = unsafe { gdcan22e45210 :: Can :: from_ptr (0x4001_c000usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamuxd5ba02be :: Dmamux = unsafe { gddmamuxd5ba02be :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbase2ebc8a75 :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbase2ebc8a75 :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase1b4097c0 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcu8f196476 :: Rcu = unsafe { gdrcu8f196476 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc84e2d009 :: Fmc = unsafe { gdfmc84e2d009 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const FFT : gdfftc3717816 :: Fft = unsafe { gdfftc3717816 :: Fft :: from_ptr (0x4002_5000usize as _) } ; pub const CLA : gdclaa852113c :: Cla = unsafe { gdclaa852113c :: Cla :: from_ptr (0x4003_8000usize as _) } ; pub const GPIOA : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_1400usize as _) } ; pub const GPIOG : gdgpio2c42bb33 :: Gpio = unsafe { gdgpio2c42bb33 :: Gpio :: from_ptr (0x4800_1800usize as _) } ; pub const CAU : gdcau0732936f :: Cau = unsafe { gdcau0732936f :: Cau :: from_ptr (0x4802_1000usize as _) } ; pub const TRNG : gdtrng03d0dc9e :: Trng = unsafe { gdtrng03d0dc9e :: Trng :: from_ptr (0x4802_1800usize as _) } ; pub const CPDM : gdcpdm49df1052 :: Cpdm = unsafe { gdcpdm49df1052 :: Cpdm :: from_ptr (0x4802_2800usize as _) } ; pub const TMU : gdtmu6e5ec85c :: Tmu = unsafe { gdtmu6e5ec85c :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const FAC : gdfac70f604b5 :: Fac = unsafe { gdfac70f604b5 :: Fac :: from_ptr (0x4802_4800usize as _) } ; pub const ADC0 : gdadc01210ae24 :: Adc0 = unsafe { gdadc01210ae24 :: Adc0 :: from_ptr (0x5000_0000usize as _) } ; pub const ADC1 : gdadc7aed6f31 :: Adc = unsafe { gdadc7aed6f31 :: Adc :: from_ptr (0x5000_0400usize as _) } ; pub const ADC2 : gdadc7aed6f31 :: Adc = unsafe { gdadc7aed6f31 :: Adc :: from_ptr (0x5000_0800usize as _) } ; pub const ADC3 : gdadc7aed6f31 :: Adc = unsafe { gdadc7aed6f31 :: Adc :: from_ptr (0x5000_0c00usize as _) } ; pub const DAC0 : gddac4a2b738c :: Dac = unsafe { gddac4a2b738c :: Dac :: from_ptr (0x5000_1000usize as _) } ; pub const DAC1 : gddac4a2b738c :: Dac = unsafe { gddac4a2b738c :: Dac :: from_ptr (0x5000_1400usize as _) } ; pub const DAC2 : gddac4a2b738c :: Dac = unsafe { gddac4a2b738c :: Dac :: from_ptr (0x5000_1800usize as _) } ; pub const DAC3 : gddac4a2b738c :: Dac = unsafe { gddac4a2b738c :: Dac :: from_ptr (0x5000_1c00usize as _) } ; pub const EXMC : gdexmc6ac29ae0 :: Exmc = unsafe { gdexmc6ac29ae0 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const QSPI : gdqspi6bd3ed8c :: Qspi = unsafe { gdqspi6bd3ed8c :: Qspi :: from_ptr (0xa000_1000usize as _) } ; pub const DBG : gddbge900170a :: Dbg = unsafe { gddbge900170a :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc01210ae24_v1.rs"] pub mod gdadc01210ae24;
#[path="../../peripherals/gdadc7aed6f31_v1.rs"] pub mod gdadc7aed6f31;
#[path="../../peripherals/gdcan22e45210_v1.rs"] pub mod gdcan22e45210;
#[path="../../peripherals/gdcau0732936f_v1.rs"] pub mod gdcau0732936f;
#[path="../../peripherals/gdclaa852113c_v1.rs"] pub mod gdclaa852113c;
#[path="../../peripherals/gdcmp4263a684_v1.rs"] pub mod gdcmp4263a684;
#[path="../../peripherals/gdcpdm49df1052_v1.rs"] pub mod gdcpdm49df1052;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gddac4a2b738c_v1.rs"] pub mod gddac4a2b738c;
#[path="../../peripherals/gddbge900170a_v1.rs"] pub mod gddbge900170a;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gddmamuxd5ba02be_v1.rs"] pub mod gddmamuxd5ba02be;
#[path="../../peripherals/gddmamuxrgchxcfgbase1b4097c0_v1.rs"] pub mod gddmamuxrgchxcfgbase1b4097c0;
#[path="../../peripherals/gddmamuxrmchxcfgbase2ebc8a75_v1.rs"] pub mod gddmamuxrmchxcfgbase2ebc8a75;
#[path="../../peripherals/gdexmc6ac29ae0_v1.rs"] pub mod gdexmc6ac29ae0;
#[path="../../peripherals/gdexti8937c1c8_v1.rs"] pub mod gdexti8937c1c8;
#[path="../../peripherals/gdfac70f604b5_v1.rs"] pub mod gdfac70f604b5;
#[path="../../peripherals/gdfftc3717816_v1.rs"] pub mod gdfftc3717816;
#[path="../../peripherals/gdfmc84e2d009_v1.rs"] pub mod gdfmc84e2d009;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpio2c42bb33_v1.rs"] pub mod gdgpio2c42bb33;
#[path="../../peripherals/gdhpdfdafb56e7_v1.rs"] pub mod gdhpdfdafb56e7;
#[path="../../peripherals/gdhrtimerc7445402_v1.rs"] pub mod gdhrtimerc7445402;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdlptimer265af638_v1.rs"] pub mod gdlptimer265af638;
#[path="../../peripherals/gdpmu1dc33268_v1.rs"] pub mod gdpmu1dc33268;
#[path="../../peripherals/gdqspi6bd3ed8c_v1.rs"] pub mod gdqspi6bd3ed8c;
#[path="../../peripherals/gdrcu8f196476_v1.rs"] pub mod gdrcu8f196476;
#[path="../../peripherals/gdrtcd5d8d4bc_v1.rs"] pub mod gdrtcd5d8d4bc;
#[path="../../peripherals/gdspiea8c377b_v1.rs"] pub mod gdspiea8c377b;
#[path="../../peripherals/gdsyscfgee183683_v1.rs"] pub mod gdsyscfgee183683;
#[path="../../peripherals/gdtimerd311b1a2_v1.rs"] pub mod gdtimerd311b1a2;
#[path="../../peripherals/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../../peripherals/gdtrigsel37febbbf_v1.rs"] pub mod gdtrigsel37febbbf;
#[path="../../peripherals/gdtrng03d0dc9e_v1.rs"] pub mod gdtrng03d0dc9e;
#[path="../../peripherals/gdusartc72580ea_v1.rs"] pub mod gdusartc72580ea;
#[path="../../peripherals/gdvref779f5a9e_v1.rs"] pub mod gdvref779f5a9e;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
