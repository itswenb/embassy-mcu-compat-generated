

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD_AVD_OVD"]
LVD_AVD_OVD = 1 , # [doc = "2 - RTC_TAMPER_TIMESTAMP_LXTAL_STUCK"]
RTC_TAMPER_TIMESTAMP_LXTAL_STUCK = 2 , # [doc = "3 - RTC_WAKE"]
RTC_WAKE = 3 , # [doc = "4 - FMC_GLOBAL"]
FMC_GLOBAL = 4 , # [doc = "5 - RCU_GLOBAL"]
RCU_GLOBAL = 5 , # [doc = "6 - EXTI_LINE0"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TR_CM_DEC_ZERO"]
TIMER0_TR_CM_DEC_ZERO = 26 , # [doc = "27 - TIMER0_CAP"]
TIMER0_CAP = 27 , # [doc = "28 - TIMER1"]
TIMER1 = 28 , # [doc = "29 - TIMER2"]
TIMER2 = 29 , # [doc = "30 - TIMER3"]
TIMER3 = 30 , # [doc = "31 - I2C0_EV_WAKE"]
I2C0_EV_WAKE = 31 , # [doc = "32 - I2C0_ER"]
I2C0_ER = 32 , # [doc = "33 - I2C1_EV_WAKE"]
I2C1_EV_WAKE = 33 , # [doc = "34 - I2C1_ER"]
I2C1_ER = 34 , # [doc = "35 - SPI0"]
SPI0 = 35 , # [doc = "36 - SPI1"]
SPI1 = 36 , # [doc = "37 - USART0"]
USART0 = 37 , # [doc = "38 - USART1"]
USART1 = 38 , # [doc = "39 - USART2"]
USART2 = 39 , # [doc = "40 - EXTI_LINE15_10"]
EXTI_LINE15_10 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "43 - TIMER7_BRK_TE_ZE"]
TIMER7_BRK_TE_ZE = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TR_CM_DEC_ZERO"]
TIMER7_TR_CM_DEC_ZERO = 45 , # [doc = "46 - TIMER7_CAP"]
TIMER7_CAP = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - SYSCFG"]
SYSCFG = 48 , # [doc = "49 - LPTIMER"]
LPTIMER = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC_0_2"]
TIMER5_DAC_0_2 = 54 , # [doc = "55 - TIMER6_DAC_1_3"]
TIMER6_DAC_1_3 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - ADC3"]
ADC3 = 61 , # [doc = "63 - UVD2_OVD2"]
UVD2_OVD2 = 63 , # [doc = "64 - CMP0_1_2_3"]
CMP0_1_2_3 = 64 , # [doc = "65 - CMP4_5_6_7"]
CMP4_5_6_7 = 65 , # [doc = "66 - CMP_GLOBAL"]
CMP_GLOBAL = 66 , # [doc = "67 - HRTIMER_IRQ0"]
HRTIMER_IRQ0 = 67 , # [doc = "68 - HRTIMER_IRQ1"]
HRTIMER_IRQ1 = 68 , # [doc = "69 - HRTIMER_IRQ2"]
HRTIMER_IRQ2 = 69 , # [doc = "70 - HRTIMER_IRQ3"]
HRTIMER_IRQ3 = 70 , # [doc = "71 - HRTIMER_IRQ4"]
HRTIMER_IRQ4 = 71 , # [doc = "72 - HRTIMER_IRQ5"]
HRTIMER_IRQ5 = 72 , # [doc = "73 - HRTIMER_IRQ6"]
HRTIMER_IRQ6 = 73 , # [doc = "74 - HRTIMER_IRQ7"]
HRTIMER_IRQ7 = 74 , # [doc = "75 - HRTIMER_IRQ8"]
HRTIMER_IRQ8 = 75 , # [doc = "76 - HRTIMER_IRQ9"]
HRTIMER_IRQ9 = 76 , # [doc = "77 - TIMER19_BRK_TE_ZE"]
TIMER19_BRK_TE_ZE = 77 , # [doc = "78 - TIMER19_UP"]
TIMER19_UP = 78 , # [doc = "79 - TIMER19_TR_CM_DEC_ZERO"]
TIMER19_TR_CM_DEC_ZERO = 79 , # [doc = "80 - TIMER19_CAP"]
TIMER19_CAP = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "82 - I2C2_EV_WAKE"]
I2C2_EV_WAKE = 82 , # [doc = "83 - I2C2_ER"]
I2C2_ER = 83 , # [doc = "85 - CAU"]
CAU = 85 , # [doc = "90 - TRNG"]
TRNG = 90 , # [doc = "92 - I2C3_EV_WAKE"]
I2C3_EV_WAKE = 92 , # [doc = "93 - I2C3_ER"]
I2C3_ER = 93 , # [doc = "94 - DMA_MUX"]
DMA_MUX = 94 , # [doc = "95 - QSPI"]
QSPI = 95 , # [doc = "96 - FFT_GLOBAL"]
FFT_GLOBAL = 96 , # [doc = "97 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 97 , # [doc = "98 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 98 , # [doc = "99 - CLA"]
CLA = 99 , # [doc = "100 - TMU"]
TMU = 100 , # [doc = "101 - FAC_GLOBAL"]
FAC_GLOBAL = 101 , # [doc = "102 - HPDF_GLOBAL0"]
HPDF_GLOBAL0 = 102 , # [doc = "103 - HPDF_GLOBAL1"]
HPDF_GLOBAL1 = 103 , # [doc = "104 - HPDF_GLOBAL2"]
HPDF_GLOBAL2 = 104 , # [doc = "105 - HPDF_GLOBAL3"]
HPDF_GLOBAL3 = 105 , # [doc = "106 - TIMER14"]
TIMER14 = 106 , # [doc = "107 - TIMER15"]
TIMER15 = 107 , # [doc = "108 - TIMER16"]
TIMER16 = 108 , # [doc = "109 - CAN0_WK"]
CAN0_WK = 109 , # [doc = "110 - CAN0_BUFF"]
CAN0_BUFF = 110 , # [doc = "111 - CAN0_BUSOFF"]
CAN0_BUSOFF = 111 , # [doc = "112 - CAN0_ERROR"]
CAN0_ERROR = 112 , # [doc = "113 - CAN0_ERROR_FTX"]
CAN0_ERROR_FTX = 113 , # [doc = "114 - CAN0_WARNING_TX"]
CAN0_WARNING_TX = 114 , # [doc = "115 - CAN0_WARNING_RX"]
CAN0_WARNING_RX = 115 , # [doc = "116 - CAN1_WK"]
CAN1_WK = 116 , # [doc = "117 - CAN1_BUFF"]
CAN1_BUFF = 117 , # [doc = "118 - CAN1_BUSOFF"]
CAN1_BUSOFF = 118 , # [doc = "119 - CAN1_ERROR"]
CAN1_ERROR = 119 , # [doc = "120 - CAN1_ERROR_FTX"]
CAN1_ERROR_FTX = 120 , # [doc = "121 - CAN1_WARNING_TX"]
CAN1_WARNING_TX = 121 , # [doc = "122 - CAN1_WARNING_RX"]
CAN1_WARNING_RX = 122 , # [doc = "123 - CAN2_WK"]
CAN2_WK = 123 , # [doc = "124 - CAN2_BUFF"]
CAN2_BUFF = 124 , # [doc = "125 - CAN2_BUSOFF"]
CAN2_BUSOFF = 125 , # [doc = "126 - CAN2_ERROR"]
CAN2_ERROR = 126 , # [doc = "127 - CAN2_ERROR_FTX"]
CAN2_ERROR_FTX = 127 , # [doc = "128 - CAN2_WARNING_TX"]
CAN2_WARNING_TX = 128 , # [doc = "129 - CAN2_WARNING_RX"]
CAN2_WARNING_RX = 129 , # [doc = "130 - TIMER0_DEC"]
TIMER0_DEC = 130 , # [doc = "131 - TIMER1_DEC"]
TIMER1_DEC = 131 , # [doc = "132 - TIMER2_DEC"]
TIMER2_DEC = 132 , # [doc = "133 - TIMER3_DEC"]
TIMER3_DEC = 133 , # [doc = "134 - TIMER4_DEC"]
TIMER4_DEC = 134 , # [doc = "135 - TIMER7_DEC"]
TIMER7_DEC = 135 , # [doc = "136 - TIMER19_DEC"]
TIMER19_DEC = 136 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD_AVD_OVD () ; fn RTC_TAMPER_TIMESTAMP_LXTAL_STUCK () ; fn RTC_WAKE () ; fn FMC_GLOBAL () ; fn RCU_GLOBAL () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TR_CM_DEC_ZERO () ; fn TIMER0_CAP () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV_WAKE () ; fn I2C0_ER () ; fn I2C1_EV_WAKE () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn TIMER7_BRK_TE_ZE () ; fn TIMER7_UP () ; fn TIMER7_TR_CM_DEC_ZERO () ; fn TIMER7_CAP () ; fn ADC2 () ; fn SYSCFG () ; fn LPTIMER () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC_0_2 () ; fn TIMER6_DAC_1_3 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ADC3 () ; fn UVD2_OVD2 () ; fn CMP0_1_2_3 () ; fn CMP4_5_6_7 () ; fn CMP_GLOBAL () ; fn HRTIMER_IRQ0 () ; fn HRTIMER_IRQ1 () ; fn HRTIMER_IRQ2 () ; fn HRTIMER_IRQ3 () ; fn HRTIMER_IRQ4 () ; fn HRTIMER_IRQ5 () ; fn HRTIMER_IRQ6 () ; fn HRTIMER_IRQ7 () ; fn HRTIMER_IRQ8 () ; fn HRTIMER_IRQ9 () ; fn TIMER19_BRK_TE_ZE () ; fn TIMER19_UP () ; fn TIMER19_TR_CM_DEC_ZERO () ; fn TIMER19_CAP () ; fn FPU () ; fn I2C2_EV_WAKE () ; fn I2C2_ER () ; fn CAU () ; fn TRNG () ; fn I2C3_EV_WAKE () ; fn I2C3_ER () ; fn DMA_MUX () ; fn QSPI () ; fn FFT_GLOBAL () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn CLA () ; fn TMU () ; fn FAC_GLOBAL () ; fn HPDF_GLOBAL0 () ; fn HPDF_GLOBAL1 () ; fn HPDF_GLOBAL2 () ; fn HPDF_GLOBAL3 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn CAN0_WK () ; fn CAN0_BUFF () ; fn CAN0_BUSOFF () ; fn CAN0_ERROR () ; fn CAN0_ERROR_FTX () ; fn CAN0_WARNING_TX () ; fn CAN0_WARNING_RX () ; fn CAN1_WK () ; fn CAN1_BUFF () ; fn CAN1_BUSOFF () ; fn CAN1_ERROR () ; fn CAN1_ERROR_FTX () ; fn CAN1_WARNING_TX () ; fn CAN1_WARNING_RX () ; fn CAN2_WK () ; fn CAN2_BUFF () ; fn CAN2_BUSOFF () ; fn CAN2_ERROR () ; fn CAN2_ERROR_FTX () ; fn CAN2_WARNING_TX () ; fn CAN2_WARNING_RX () ; fn TIMER0_DEC () ; fn TIMER1_DEC () ; fn TIMER2_DEC () ; fn TIMER3_DEC () ; fn TIMER4_DEC () ; fn TIMER7_DEC () ; fn TIMER19_DEC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 137]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD_AVD_OVD } , Vector { _handler : RTC_TAMPER_TIMESTAMP_LXTAL_STUCK } , Vector { _handler : RTC_WAKE } , Vector { _handler : FMC_GLOBAL } , Vector { _handler : RCU_GLOBAL } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TR_CM_DEC_ZERO } , Vector { _handler : TIMER0_CAP } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV_WAKE } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV_WAKE } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK_TE_ZE } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TR_CM_DEC_ZERO } , Vector { _handler : TIMER7_CAP } , Vector { _handler : ADC2 } , Vector { _handler : SYSCFG } , Vector { _handler : LPTIMER } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC_0_2 } , Vector { _handler : TIMER6_DAC_1_3 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ADC3 } , Vector { _reserved : 0 } , Vector { _handler : UVD2_OVD2 } , Vector { _handler : CMP0_1_2_3 } , Vector { _handler : CMP4_5_6_7 } , Vector { _handler : CMP_GLOBAL } , Vector { _handler : HRTIMER_IRQ0 } , Vector { _handler : HRTIMER_IRQ1 } , Vector { _handler : HRTIMER_IRQ2 } , Vector { _handler : HRTIMER_IRQ3 } , Vector { _handler : HRTIMER_IRQ4 } , Vector { _handler : HRTIMER_IRQ5 } , Vector { _handler : HRTIMER_IRQ6 } , Vector { _handler : HRTIMER_IRQ7 } , Vector { _handler : HRTIMER_IRQ8 } , Vector { _handler : HRTIMER_IRQ9 } , Vector { _handler : TIMER19_BRK_TE_ZE } , Vector { _handler : TIMER19_UP } , Vector { _handler : TIMER19_TR_CM_DEC_ZERO } , Vector { _handler : TIMER19_CAP } , Vector { _handler : FPU } , Vector { _handler : I2C2_EV_WAKE } , Vector { _handler : I2C2_ER } , Vector { _reserved : 0 } , Vector { _handler : CAU } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TRNG } , Vector { _reserved : 0 } , Vector { _handler : I2C3_EV_WAKE } , Vector { _handler : I2C3_ER } , Vector { _handler : DMA_MUX } , Vector { _handler : QSPI } , Vector { _handler : FFT_GLOBAL } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : CLA } , Vector { _handler : TMU } , Vector { _handler : FAC_GLOBAL } , Vector { _handler : HPDF_GLOBAL0 } , Vector { _handler : HPDF_GLOBAL1 } , Vector { _handler : HPDF_GLOBAL2 } , Vector { _handler : HPDF_GLOBAL3 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : CAN0_WK } , Vector { _handler : CAN0_BUFF } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ERROR } , Vector { _handler : CAN0_ERROR_FTX } , Vector { _handler : CAN0_WARNING_TX } , Vector { _handler : CAN0_WARNING_RX } , Vector { _handler : CAN1_WK } , Vector { _handler : CAN1_BUFF } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ERROR } , Vector { _handler : CAN1_ERROR_FTX } , Vector { _handler : CAN1_WARNING_TX } , Vector { _handler : CAN1_WARNING_RX } , Vector { _handler : CAN2_WK } , Vector { _handler : CAN2_BUFF } , Vector { _handler : CAN2_BUSOFF } , Vector { _handler : CAN2_ERROR } , Vector { _handler : CAN2_ERROR_FTX } , Vector { _handler : CAN2_WARNING_TX } , Vector { _handler : CAN2_WARNING_RX } , Vector { _handler : TIMER0_DEC } , Vector { _handler : TIMER1_DEC } , Vector { _handler : TIMER2_DEC } , Vector { _handler : TIMER3_DEC } , Vector { _handler : TIMER4_DEC } , Vector { _handler : TIMER7_DEC } , Vector { _handler : TIMER19_DEC } ,]
; } pub const TIMER1 : gdtimer103a746fb :: Timer1 = unsafe { gdtimer103a746fb :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer291200e8c :: Timer2 = unsafe { gdtimer291200e8c :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer291200e8c :: Timer2 = unsafe { gdtimer291200e8c :: Timer2 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer103a746fb :: Timer1 = unsafe { gdtimer103a746fb :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer58fe8734a :: Timer5 = unsafe { gdtimer58fe8734a :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer58fe8734a :: Timer5 = unsafe { gdtimer58fe8734a :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const RTC : gdrtca0f4d2cf :: Rtc = unsafe { gdrtca0f4d2cf :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt5932fb56 :: Fwdgt = unsafe { gdfwdgt5932fb56 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi16544be1a :: Spi1 = unsafe { gdspi16544be1a :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi2112e160c :: Spi2 = unsafe { gdspi2112e160c :: Spi2 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart001f99729 :: Usart0 = unsafe { gdusart001f99729 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart001f99729 :: Usart0 = unsafe { gdusart001f99729 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart38ee66329 :: Uart3 = unsafe { gduart38ee66329 :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart38ee66329 :: Uart3 = unsafe { gduart38ee66329 :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const I2C3 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5c00usize as _) } ; pub const PMU : gdpmu38e55ba3 :: Pmu = unsafe { gdpmu38e55ba3 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const LPTIMER : gdlptimer1f47bc76 :: Lptimer = unsafe { gdlptimer1f47bc76 :: Lptimer :: from_ptr (0x4000_9400usize as _) } ; pub const I2C2 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_c000usize as _) } ; pub const SYSCFG : gdsyscfgc16069c6 :: Syscfg = unsafe { gdsyscfgc16069c6 :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextiaf81da6f :: Exti = unsafe { gdextiaf81da6f :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const TIMER0 : gdtimer00fb2a8b3 :: Timer0 = unsafe { gdtimer00fb2a8b3 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0d1cf2b57 :: Spi0 = unsafe { gdspi0d1cf2b57 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer00fb2a8b3 :: Timer0 = unsafe { gdtimer00fb2a8b3 :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart001f99729 :: Usart0 = unsafe { gdusart001f99729 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer14eb20ecc0 :: Timer14 = unsafe { gdtimer14eb20ecc0 :: Timer14 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer15f7745da8 :: Timer15 = unsafe { gdtimer15f7745da8 :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer15f7745da8 :: Timer15 = unsafe { gdtimer15f7745da8 :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const TIMER19 : gdtimer00fb2a8b3 :: Timer0 = unsafe { gdtimer00fb2a8b3 :: Timer0 :: from_ptr (0x4001_5000usize as _) } ; pub const HRTIMER_MASTER_TIMER : gdhrtimermastertimer46dd88dd :: HrtimerMasterTimer = unsafe { gdhrtimermastertimer46dd88dd :: HrtimerMasterTimer :: from_ptr (0x4001_5800usize as _) } ; pub const HRTIMER_SLAVE_TIMER0 : gdhrtimerslavetimer067581449 :: HrtimerSlaveTimer0 = unsafe { gdhrtimerslavetimer067581449 :: HrtimerSlaveTimer0 :: from_ptr (0x4001_5880usize as _) } ; pub const HRTIMER_SLAVE_TIMER1 : gdhrtimerslavetimer14d8109aa :: HrtimerSlaveTimer1 = unsafe { gdhrtimerslavetimer14d8109aa :: HrtimerSlaveTimer1 :: from_ptr (0x4001_5900usize as _) } ; pub const HRTIMER_SLAVE_TIMER2 : gdhrtimerslavetimer20b2b3389 :: HrtimerSlaveTimer2 = unsafe { gdhrtimerslavetimer20b2b3389 :: HrtimerSlaveTimer2 :: from_ptr (0x4001_5980usize as _) } ; pub const HRTIMER_SLAVE_TIMER3 : gdhrtimerslavetimer34217e6d5 :: HrtimerSlaveTimer3 = unsafe { gdhrtimerslavetimer34217e6d5 :: HrtimerSlaveTimer3 :: from_ptr (0x4001_5a00usize as _) } ; pub const HRTIMER_SLAVE_TIMER4 : gdhrtimerslavetimer44f2ae72f :: HrtimerSlaveTimer4 = unsafe { gdhrtimerslavetimer44f2ae72f :: HrtimerSlaveTimer4 :: from_ptr (0x4001_5a80usize as _) } ; pub const HRTIMER_SLAVE_TIMER5 : gdhrtimerslavetimer5093157b0 :: HrtimerSlaveTimer5 = unsafe { gdhrtimerslavetimer5093157b0 :: HrtimerSlaveTimer5 :: from_ptr (0x4001_5b00usize as _) } ; pub const HRTIMER_COMMON : gdhrtimercommone8f80cd9 :: HrtimerCommon = unsafe { gdhrtimercommone8f80cd9 :: HrtimerCommon :: from_ptr (0x4001_5b80usize as _) } ; pub const HRTIMER_SLAVE_TIMER6 : gdhrtimerslavetimer66a7539e7 :: HrtimerSlaveTimer6 = unsafe { gdhrtimerslavetimer66a7539e7 :: HrtimerSlaveTimer6 :: from_ptr (0x4001_6000usize as _) } ; pub const HRTIMER_SLAVE_TIMER7 : gdhrtimerslavetimer7b4a9b9e4 :: HrtimerSlaveTimer7 = unsafe { gdhrtimerslavetimer7b4a9b9e4 :: HrtimerSlaveTimer7 :: from_ptr (0x4001_6080usize as _) } ; pub const HPDF : gdhpdf7e0cd818 :: Hpdf = unsafe { gdhpdf7e0cd818 :: Hpdf :: from_ptr (0x4001_7000usize as _) } ; pub const VREF : gdvref193fa1c3 :: Vref = unsafe { gdvref193fa1c3 :: Vref :: from_ptr (0x4001_7800usize as _) } ; pub const CMP : gdcmp5553b816 :: Cmp = unsafe { gdcmp5553b816 :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const TRIGSEL : gdtrigsel75c0668b :: Trigsel = unsafe { gdtrigsel75c0668b :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const CAN0 : gdcan00d9f65e8 :: Can0 = unsafe { gdcan00d9f65e8 :: Can0 :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcan00d9f65e8 :: Can0 = unsafe { gdcan00d9f65e8 :: Can0 :: from_ptr (0x4001_b000usize as _) } ; pub const CAN2 : gdcan00d9f65e8 :: Can0 = unsafe { gdcan00d9f65e8 :: Can0 :: from_ptr (0x4001_c000usize as _) } ; pub const DMA0 : gddma0ff389860 :: Dma0 = unsafe { gddma0ff389860 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma0ff389860 :: Dma0 = unsafe { gddma0ff389860 :: Dma0 :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux1fdc5150 :: Dmamux = unsafe { gddmamux1fdc5150 :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcu35f5457f :: Rcu = unsafe { gdrcu35f5457f :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcd425e751 :: Fmc = unsafe { gdfmcd425e751 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrccf345d41 :: Crc = unsafe { gdcrccf345d41 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const FFT : gdfft4a1b8727 :: Fft = unsafe { gdfft4a1b8727 :: Fft :: from_ptr (0x4002_5000usize as _) } ; pub const CLA : gdclabbe1d8c5 :: Cla = unsafe { gdclabbe1d8c5 :: Cla :: from_ptr (0x4003_8000usize as _) } ; pub const GPIOA : gdgpioa041672dd :: Gpioa = unsafe { gdgpioa041672dd :: Gpioa :: from_ptr (0x4800_0000usize as _) } ; pub const GPIOB : gdgpiob2a39d6c5 :: Gpiob = unsafe { gdgpiob2a39d6c5 :: Gpiob :: from_ptr (0x4800_0400usize as _) } ; pub const GPIOC : gdgpioc62f10237 :: Gpioc = unsafe { gdgpioc62f10237 :: Gpioc :: from_ptr (0x4800_0800usize as _) } ; pub const GPIOD : gdgpioc62f10237 :: Gpioc = unsafe { gdgpioc62f10237 :: Gpioc :: from_ptr (0x4800_0c00usize as _) } ; pub const GPIOE : gdgpioc62f10237 :: Gpioc = unsafe { gdgpioc62f10237 :: Gpioc :: from_ptr (0x4800_1000usize as _) } ; pub const GPIOF : gdgpioc62f10237 :: Gpioc = unsafe { gdgpioc62f10237 :: Gpioc :: from_ptr (0x4800_1400usize as _) } ; pub const GPIOG : gdgpioc62f10237 :: Gpioc = unsafe { gdgpioc62f10237 :: Gpioc :: from_ptr (0x4800_1800usize as _) } ; pub const CAU : gdcaue3899f2b :: Cau = unsafe { gdcaue3899f2b :: Cau :: from_ptr (0x4802_1000usize as _) } ; pub const TRNG : gdtrnga8e0c4c0 :: Trng = unsafe { gdtrnga8e0c4c0 :: Trng :: from_ptr (0x4802_1800usize as _) } ; pub const CPDM : gdcpdm0270b1b1 :: Cpdm = unsafe { gdcpdm0270b1b1 :: Cpdm :: from_ptr (0x4802_2800usize as _) } ; pub const TMU : gdtmuf06b7fd1 :: Tmu = unsafe { gdtmuf06b7fd1 :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const FAC : gdfac8bc94bdf :: Fac = unsafe { gdfac8bc94bdf :: Fac :: from_ptr (0x4802_4800usize as _) } ; pub const ADC0 : gdadc0e44d6214 :: Adc0 = unsafe { gdadc0e44d6214 :: Adc0 :: from_ptr (0x5000_0000usize as _) } ; pub const ADC1 : gdadc1d46cf375 :: Adc1 = unsafe { gdadc1d46cf375 :: Adc1 :: from_ptr (0x5000_0400usize as _) } ; pub const ADC2 : gdadc2ffb56179 :: Adc2 = unsafe { gdadc2ffb56179 :: Adc2 :: from_ptr (0x5000_0800usize as _) } ; pub const ADC3 : gdadc3412cd97d :: Adc3 = unsafe { gdadc3412cd97d :: Adc3 :: from_ptr (0x5000_0c00usize as _) } ; pub const DAC0 : gddac07216986f :: Dac0 = unsafe { gddac07216986f :: Dac0 :: from_ptr (0x5000_1000usize as _) } ; pub const DAC1 : gddac07216986f :: Dac0 = unsafe { gddac07216986f :: Dac0 :: from_ptr (0x5000_1400usize as _) } ; pub const DAC2 : gddac07216986f :: Dac0 = unsafe { gddac07216986f :: Dac0 :: from_ptr (0x5000_1800usize as _) } ; pub const DAC3 : gddac07216986f :: Dac0 = unsafe { gddac07216986f :: Dac0 :: from_ptr (0x5000_1c00usize as _) } ; pub const EXMC : gdexmc15e63ec9 :: Exmc = unsafe { gdexmc15e63ec9 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const QSPI : gdqspic71428c9 :: Qspi = unsafe { gdqspic71428c9 :: Qspi :: from_ptr (0xa000_1000usize as _) } ; pub const DBG : gddbg91aeb23f :: Dbg = unsafe { gddbg91aeb23f :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc0e44d6214_v1.rs"] pub mod gdadc0e44d6214;
#[path="../../peripherals/gdadc1d46cf375_v1.rs"] pub mod gdadc1d46cf375;
#[path="../../peripherals/gdadc2ffb56179_v1.rs"] pub mod gdadc2ffb56179;
#[path="../../peripherals/gdadc3412cd97d_v1.rs"] pub mod gdadc3412cd97d;
#[path="../../peripherals/gdcan00d9f65e8_v1.rs"] pub mod gdcan00d9f65e8;
#[path="../../peripherals/gdcaue3899f2b_v1.rs"] pub mod gdcaue3899f2b;
#[path="../../peripherals/gdclabbe1d8c5_v1.rs"] pub mod gdclabbe1d8c5;
#[path="../../peripherals/gdcmp5553b816_v1.rs"] pub mod gdcmp5553b816;
#[path="../../peripherals/gdcpdm0270b1b1_v1.rs"] pub mod gdcpdm0270b1b1;
#[path="../../peripherals/gdcrccf345d41_v1.rs"] pub mod gdcrccf345d41;
#[path="../../peripherals/gddac07216986f_v1.rs"] pub mod gddac07216986f;
#[path="../../peripherals/gddbg91aeb23f_v1.rs"] pub mod gddbg91aeb23f;
#[path="../../peripherals/gddma0ff389860_v1.rs"] pub mod gddma0ff389860;
#[path="../../peripherals/gddmamux1fdc5150_v1.rs"] pub mod gddmamux1fdc5150;
#[path="../../peripherals/gdexmc15e63ec9_v1.rs"] pub mod gdexmc15e63ec9;
#[path="../../peripherals/gdextiaf81da6f_v1.rs"] pub mod gdextiaf81da6f;
#[path="../../peripherals/gdfac8bc94bdf_v1.rs"] pub mod gdfac8bc94bdf;
#[path="../../peripherals/gdfft4a1b8727_v1.rs"] pub mod gdfft4a1b8727;
#[path="../../peripherals/gdfmcd425e751_v1.rs"] pub mod gdfmcd425e751;
#[path="../../peripherals/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../../peripherals/gdgpioa041672dd_v1.rs"] pub mod gdgpioa041672dd;
#[path="../../peripherals/gdgpiob2a39d6c5_v1.rs"] pub mod gdgpiob2a39d6c5;
#[path="../../peripherals/gdgpioc62f10237_v1.rs"] pub mod gdgpioc62f10237;
#[path="../../peripherals/gdhpdf7e0cd818_v1.rs"] pub mod gdhpdf7e0cd818;
#[path="../../peripherals/gdhrtimercommone8f80cd9_v1.rs"] pub mod gdhrtimercommone8f80cd9;
#[path="../../peripherals/gdhrtimermastertimer46dd88dd_v1.rs"] pub mod gdhrtimermastertimer46dd88dd;
#[path="../../peripherals/gdhrtimerslavetimer067581449_v1.rs"] pub mod gdhrtimerslavetimer067581449;
#[path="../../peripherals/gdhrtimerslavetimer14d8109aa_v1.rs"] pub mod gdhrtimerslavetimer14d8109aa;
#[path="../../peripherals/gdhrtimerslavetimer20b2b3389_v1.rs"] pub mod gdhrtimerslavetimer20b2b3389;
#[path="../../peripherals/gdhrtimerslavetimer34217e6d5_v1.rs"] pub mod gdhrtimerslavetimer34217e6d5;
#[path="../../peripherals/gdhrtimerslavetimer44f2ae72f_v1.rs"] pub mod gdhrtimerslavetimer44f2ae72f;
#[path="../../peripherals/gdhrtimerslavetimer5093157b0_v1.rs"] pub mod gdhrtimerslavetimer5093157b0;
#[path="../../peripherals/gdhrtimerslavetimer66a7539e7_v1.rs"] pub mod gdhrtimerslavetimer66a7539e7;
#[path="../../peripherals/gdhrtimerslavetimer7b4a9b9e4_v1.rs"] pub mod gdhrtimerslavetimer7b4a9b9e4;
#[path="../../peripherals/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../../peripherals/gdlptimer1f47bc76_v1.rs"] pub mod gdlptimer1f47bc76;
#[path="../../peripherals/gdpmu38e55ba3_v1.rs"] pub mod gdpmu38e55ba3;
#[path="../../peripherals/gdqspic71428c9_v1.rs"] pub mod gdqspic71428c9;
#[path="../../peripherals/gdrcu35f5457f_v1.rs"] pub mod gdrcu35f5457f;
#[path="../../peripherals/gdrtca0f4d2cf_v1.rs"] pub mod gdrtca0f4d2cf;
#[path="../../peripherals/gdspi0d1cf2b57_v1.rs"] pub mod gdspi0d1cf2b57;
#[path="../../peripherals/gdspi16544be1a_v1.rs"] pub mod gdspi16544be1a;
#[path="../../peripherals/gdspi2112e160c_v1.rs"] pub mod gdspi2112e160c;
#[path="../../peripherals/gdsyscfgc16069c6_v1.rs"] pub mod gdsyscfgc16069c6;
#[path="../../peripherals/gdtimer00fb2a8b3_v1.rs"] pub mod gdtimer00fb2a8b3;
#[path="../../peripherals/gdtimer103a746fb_v1.rs"] pub mod gdtimer103a746fb;
#[path="../../peripherals/gdtimer14eb20ecc0_v1.rs"] pub mod gdtimer14eb20ecc0;
#[path="../../peripherals/gdtimer15f7745da8_v1.rs"] pub mod gdtimer15f7745da8;
#[path="../../peripherals/gdtimer291200e8c_v1.rs"] pub mod gdtimer291200e8c;
#[path="../../peripherals/gdtimer58fe8734a_v1.rs"] pub mod gdtimer58fe8734a;
#[path="../../peripherals/gdtmuf06b7fd1_v1.rs"] pub mod gdtmuf06b7fd1;
#[path="../../peripherals/gdtrigsel75c0668b_v1.rs"] pub mod gdtrigsel75c0668b;
#[path="../../peripherals/gdtrnga8e0c4c0_v1.rs"] pub mod gdtrnga8e0c4c0;
#[path="../../peripherals/gduart38ee66329_v1.rs"] pub mod gduart38ee66329;
#[path="../../peripherals/gdusart001f99729_v1.rs"] pub mod gdusart001f99729;
#[path="../../peripherals/gdvref193fa1c3_v1.rs"] pub mod gdvref193fa1c3;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
