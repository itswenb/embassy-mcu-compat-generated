

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - AVD_PVD"]
AVD_PVD = 1 , # [doc = "2 - RTC_TAMPER_TIMESTAMP"]
RTC_TAMPER_TIMESTAMP = 2 , # [doc = "3 - RTC_WAKE"]
RTC_WAKE = 3 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TR_CM"]
TIMER0_TR_CM = 26 , # [doc = "27 - TIMER0_CAP"]
TIMER0_CAP = 27 , # [doc = "28 - TIMER1"]
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
RTC_ALARM = 41 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TR_CM"]
TIMER7_TR_CM = 45 , # [doc = "46 - TIMER7_CAP"]
TIMER7_CAP = 46 , # [doc = "47 - DMA0_CHANNEL7"]
DMA0_CHANNEL7 = 47 , # [doc = "48 - EXMC_GLOBAL"]
EXMC_GLOBAL = 48 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC"]
TIMER5_DAC = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "68 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 68 , # [doc = "69 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 69 , # [doc = "70 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 70 , # [doc = "71 - USART5"]
USART5 = 71 , # [doc = "72 - I2C2_EV"]
I2C2_EV = 72 , # [doc = "73 - I2C2_ER"]
I2C2_ER = 73 , # [doc = "74 - USBHS0_EP1_OUT"]
USBHS0_EP1_OUT = 74 , # [doc = "75 - USBHS0_EP1_IN"]
USBHS0_EP1_IN = 75 , # [doc = "76 - USBHS0_WAKEUP"]
USBHS0_WAKEUP = 76 , # [doc = "77 - USBHS0_GLOBAL"]
USBHS0_GLOBAL = 77 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "82 - UART6"]
UART6 = 82 , # [doc = "83 - UART7"]
UART7 = 83 , # [doc = "84 - SPI3"]
SPI3 = 84 , # [doc = "85 - SPI4"]
SPI4 = 85 , # [doc = "86 - SPI5"]
SPI5 = 86 , # [doc = "92 - OSPI0"]
OSPI0 = 92 , # [doc = "95 - I2C3_EV"]
I2C3_EV = 95 , # [doc = "96 - I2C3_ER"]
I2C3_ER = 96 , # [doc = "102 - DMAMUX0"]
DMAMUX0 = 102 , # [doc = "110 - HPDF_GLOBAL0"]
HPDF_GLOBAL0 = 110 , # [doc = "111 - HPDF_GLOBAL1"]
HPDF_GLOBAL1 = 111 , # [doc = "112 - HPDF_GLOBAL2"]
HPDF_GLOBAL2 = 112 , # [doc = "113 - HPDF_GLOBAL3"]
HPDF_GLOBAL3 = 113 , # [doc = "116 - TIMER14"]
TIMER14 = 116 , # [doc = "117 - TIMER15"]
TIMER15 = 117 , # [doc = "118 - TIMER16"]
TIMER16 = 118 , # [doc = "122 - MDMA_GLOBAL"]
MDMA_GLOBAL = 122 , # [doc = "127 - ADC2"]
ADC2 = 127 , # [doc = "137 - CPM_GLOBAL"]
CPM_GLOBAL = 137 , # [doc = "144 - CTC"]
CTC = 144 , # [doc = "145 - ECC"]
ECC = 145 , # [doc = "150 - OSPI1"]
OSPI1 = 150 , # [doc = "153 - FAC_GLOBAL"]
FAC_GLOBAL = 153 , # [doc = "154 - TMU_GLOBAL"]
TMU_GLOBAL = 154 , # [doc = "161 - TIMER22"]
TIMER22 = 161 , # [doc = "162 - TIMER23"]
TIMER23 = 162 , # [doc = "165 - TIMER40"]
TIMER40 = 165 , # [doc = "166 - TIMER41"]
TIMER41 = 166 , # [doc = "167 - TIMER42"]
TIMER42 = 167 , # [doc = "168 - TIMER43"]
TIMER43 = 168 , # [doc = "169 - TIMER44"]
TIMER44 = 169 , # [doc = "170 - TIMER50"]
TIMER50 = 170 , # [doc = "171 - TIMER51"]
TIMER51 = 171 , # [doc = "172 - USBHS1_EP1_OUT"]
USBHS1_EP1_OUT = 172 , # [doc = "173 - USBHS1_EP1_IN"]
USBHS1_EP1_IN = 173 , # [doc = "174 - USBHS1_WAKEUP"]
USBHS1_WAKEUP = 174 , # [doc = "175 - USBHS1_GLOBAL"]
USBHS1_GLOBAL = 175 , # [doc = "179 - CAN0_WK"]
CAN0_WK = 179 , # [doc = "180 - CAN0_BUFF"]
CAN0_BUFF = 180 , # [doc = "181 - CAN0_BUSOFF"]
CAN0_BUSOFF = 181 , # [doc = "182 - CAN0_ERROR"]
CAN0_ERROR = 182 , # [doc = "183 - CAN0_ERROR_FTX"]
CAN0_ERROR_FTX = 183 , # [doc = "184 - CAN0_WARNING_TX"]
CAN0_WARNING_TX = 184 , # [doc = "185 - CAN0_WARNING_RX"]
CAN0_WARNING_RX = 185 , # [doc = "186 - CAN1_WK"]
CAN1_WK = 186 , # [doc = "187 - CAN1_BUFF"]
CAN1_BUFF = 187 , # [doc = "188 - CAN1_BUSOFF"]
CAN1_BUSOFF = 188 , # [doc = "189 - CAN1_ERROR"]
CAN1_ERROR = 189 , # [doc = "190 - CAN1_ERROR_FTX"]
CAN1_ERROR_FTX = 190 , # [doc = "191 - CAN1_WARNING_TX"]
CAN1_WARNING_TX = 191 , # [doc = "192 - CAN1_WARNING_RX"]
CAN1_WARNING_RX = 192 , # [doc = "193 - CAN2_WK"]
CAN2_WK = 193 , # [doc = "194 - CAN2_BUFF"]
CAN2_BUFF = 194 , # [doc = "195 - CAN2_BUSOFF"]
CAN2_BUSOFF = 195 , # [doc = "196 - CAN2_ERROR"]
CAN2_ERROR = 196 , # [doc = "197 - CAN2_ERROR_FTX"]
CAN2_ERROR_FTX = 197 , # [doc = "198 - CAN2_WARNING_TX"]
CAN2_WARNING_TX = 198 , # [doc = "199 - CAN2_WARNING_RX"]
CAN2_WARNING_RX = 199 , # [doc = "200 - EFUSE"]
EFUSE = 200 , # [doc = "201 - I2C0_WAKE"]
I2C0_WAKE = 201 , # [doc = "202 - I2C1_WAKE"]
I2C1_WAKE = 202 , # [doc = "203 - I2C2_WAKE"]
I2C2_WAKE = 203 , # [doc = "204 - I2C3_WAKE"]
I2C3_WAKE = 204 , # [doc = "205 - LPDTS"]
LPDTS = 205 , # [doc = "206 - LPDTS_WAKE"]
LPDTS_WAKE = 206 , # [doc = "207 - TIMER0_DEC"]
TIMER0_DEC = 207 , # [doc = "208 - TIMER7_DEC"]
TIMER7_DEC = 208 , # [doc = "209 - TIMER1_DEC"]
TIMER1_DEC = 209 , # [doc = "210 - TIMER2_DEC"]
TIMER2_DEC = 210 , # [doc = "211 - TIMER3_DEC"]
TIMER3_DEC = 211 , # [doc = "212 - TIMER4_DEC"]
TIMER4_DEC = 212 , # [doc = "213 - TIMER22_DEC"]
TIMER22_DEC = 213 , # [doc = "214 - TIMER23_DEC"]
TIMER23_DEC = 214 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn AVD_PVD () ; fn RTC_TAMPER_TIMESTAMP () ; fn RTC_WAKE () ; fn FMC () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TR_CM () ; fn TIMER0_CAP () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TR_CM () ; fn TIMER7_CAP () ; fn DMA0_CHANNEL7 () ; fn EXMC_GLOBAL () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBHS0_EP1_OUT () ; fn USBHS0_EP1_IN () ; fn USBHS0_WAKEUP () ; fn USBHS0_GLOBAL () ; fn HAU_TRNG () ; fn UART6 () ; fn UART7 () ; fn SPI3 () ; fn SPI4 () ; fn SPI5 () ; fn OSPI0 () ; fn I2C3_EV () ; fn I2C3_ER () ; fn DMAMUX0 () ; fn HPDF_GLOBAL0 () ; fn HPDF_GLOBAL1 () ; fn HPDF_GLOBAL2 () ; fn HPDF_GLOBAL3 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn MDMA_GLOBAL () ; fn ADC2 () ; fn CPM_GLOBAL () ; fn CTC () ; fn ECC () ; fn OSPI1 () ; fn FAC_GLOBAL () ; fn TMU_GLOBAL () ; fn TIMER22 () ; fn TIMER23 () ; fn TIMER40 () ; fn TIMER41 () ; fn TIMER42 () ; fn TIMER43 () ; fn TIMER44 () ; fn TIMER50 () ; fn TIMER51 () ; fn USBHS1_EP1_OUT () ; fn USBHS1_EP1_IN () ; fn USBHS1_WAKEUP () ; fn USBHS1_GLOBAL () ; fn CAN0_WK () ; fn CAN0_BUFF () ; fn CAN0_BUSOFF () ; fn CAN0_ERROR () ; fn CAN0_ERROR_FTX () ; fn CAN0_WARNING_TX () ; fn CAN0_WARNING_RX () ; fn CAN1_WK () ; fn CAN1_BUFF () ; fn CAN1_BUSOFF () ; fn CAN1_ERROR () ; fn CAN1_ERROR_FTX () ; fn CAN1_WARNING_TX () ; fn CAN1_WARNING_RX () ; fn CAN2_WK () ; fn CAN2_BUFF () ; fn CAN2_BUSOFF () ; fn CAN2_ERROR () ; fn CAN2_ERROR_FTX () ; fn CAN2_WARNING_TX () ; fn CAN2_WARNING_RX () ; fn EFUSE () ; fn I2C0_WAKE () ; fn I2C1_WAKE () ; fn I2C2_WAKE () ; fn I2C3_WAKE () ; fn LPDTS () ; fn LPDTS_WAKE () ; fn TIMER0_DEC () ; fn TIMER7_DEC () ; fn TIMER1_DEC () ; fn TIMER2_DEC () ; fn TIMER3_DEC () ; fn TIMER4_DEC () ; fn TIMER22_DEC () ; fn TIMER23_DEC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 215]
= [Vector { _handler : WWDGT } , Vector { _handler : AVD_PVD } , Vector { _handler : RTC_TAMPER_TIMESTAMP } , Vector { _handler : RTC_WAKE } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TR_CM } , Vector { _handler : TIMER0_CAP } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TR_CM } , Vector { _handler : TIMER7_CAP } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : EXMC_GLOBAL } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBHS0_EP1_OUT } , Vector { _handler : USBHS0_EP1_IN } , Vector { _handler : USBHS0_WAKEUP } , Vector { _handler : USBHS0_GLOBAL } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HAU_TRNG } , Vector { _reserved : 0 } , Vector { _handler : UART6 } , Vector { _handler : UART7 } , Vector { _handler : SPI3 } , Vector { _handler : SPI4 } , Vector { _handler : SPI5 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : OSPI0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C3_EV } , Vector { _handler : I2C3_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMAMUX0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HPDF_GLOBAL0 } , Vector { _handler : HPDF_GLOBAL1 } , Vector { _handler : HPDF_GLOBAL2 } , Vector { _handler : HPDF_GLOBAL3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : MDMA_GLOBAL } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CPM_GLOBAL } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CTC } , Vector { _handler : ECC } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : OSPI1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : FAC_GLOBAL } , Vector { _handler : TMU_GLOBAL } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER22 } , Vector { _handler : TIMER23 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER40 } , Vector { _handler : TIMER41 } , Vector { _handler : TIMER42 } , Vector { _handler : TIMER43 } , Vector { _handler : TIMER44 } , Vector { _handler : TIMER50 } , Vector { _handler : TIMER51 } , Vector { _handler : USBHS1_EP1_OUT } , Vector { _handler : USBHS1_EP1_IN } , Vector { _handler : USBHS1_WAKEUP } , Vector { _handler : USBHS1_GLOBAL } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CAN0_WK } , Vector { _handler : CAN0_BUFF } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ERROR } , Vector { _handler : CAN0_ERROR_FTX } , Vector { _handler : CAN0_WARNING_TX } , Vector { _handler : CAN0_WARNING_RX } , Vector { _handler : CAN1_WK } , Vector { _handler : CAN1_BUFF } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ERROR } , Vector { _handler : CAN1_ERROR_FTX } , Vector { _handler : CAN1_WARNING_TX } , Vector { _handler : CAN1_WARNING_RX } , Vector { _handler : CAN2_WK } , Vector { _handler : CAN2_BUFF } , Vector { _handler : CAN2_BUSOFF } , Vector { _handler : CAN2_ERROR } , Vector { _handler : CAN2_ERROR_FTX } , Vector { _handler : CAN2_WARNING_TX } , Vector { _handler : CAN2_WARNING_RX } , Vector { _handler : EFUSE } , Vector { _handler : I2C0_WAKE } , Vector { _handler : I2C1_WAKE } , Vector { _handler : I2C2_WAKE } , Vector { _handler : I2C3_WAKE } , Vector { _handler : LPDTS } , Vector { _handler : LPDTS_WAKE } , Vector { _handler : TIMER0_DEC } , Vector { _handler : TIMER7_DEC } , Vector { _handler : TIMER1_DEC } , Vector { _handler : TIMER2_DEC } , Vector { _handler : TIMER3_DEC } , Vector { _handler : TIMER4_DEC } , Vector { _handler : TIMER22_DEC } , Vector { _handler : TIMER23_DEC } ,]
; } pub const TIMER1 : gdtimer1da3bc56a :: Timer1 = unsafe { gdtimer1da3bc56a :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer27201f8c9 :: Timer2 = unsafe { gdtimer27201f8c9 :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer27201f8c9 :: Timer2 = unsafe { gdtimer27201f8c9 :: Timer2 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer1da3bc56a :: Timer1 = unsafe { gdtimer1da3bc56a :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer5330a987e :: Timer5 = unsafe { gdtimer5330a987e :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer5330a987e :: Timer5 = unsafe { gdtimer5330a987e :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const SPI1 : gdspi1356222e3 :: Spi1 = unsafe { gdspi1356222e3 :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi255cb8c1f :: Spi2 = unsafe { gdspi255cb8c1f :: Spi2 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart0626fb765 :: Usart0 = unsafe { gdusart0626fb765 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart0626fb765 :: Usart0 = unsafe { gdusart0626fb765 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart330e38640 :: Uart3 = unsafe { gduart330e38640 :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart330e38640 :: Uart3 = unsafe { gduart330e38640 :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const I2C3 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_5c00usize as _) } ; pub const DAC : gddac555b6194 :: Dac = unsafe { gddac555b6194 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gduart330e38640 :: Uart3 = unsafe { gduart330e38640 :: Uart3 :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gduart330e38640 :: Uart3 = unsafe { gduart330e38640 :: Uart3 :: from_ptr (0x4000_7c00usize as _) } ; pub const CTC : gdctcdb80f1ce :: Ctc = unsafe { gdctcdb80f1ce :: Ctc :: from_ptr (0x4000_8400usize as _) } ; pub const I2C2 : gdi2c0cd973dc4 :: I2c0 = unsafe { gdi2c0cd973dc4 :: I2c0 :: from_ptr (0x4000_c000usize as _) } ; pub const TIMER22 : gdtimer1da3bc56a :: Timer1 = unsafe { gdtimer1da3bc56a :: Timer1 :: from_ptr (0x4000_e000usize as _) } ; pub const TIMER23 : gdtimer1da3bc56a :: Timer1 = unsafe { gdtimer1da3bc56a :: Timer1 :: from_ptr (0x4000_e400usize as _) } ; pub const TIMER50 : gdtimer5071732508 :: Timer50 = unsafe { gdtimer5071732508 :: Timer50 :: from_ptr (0x4000_f000usize as _) } ; pub const TIMER51 : gdtimer5071732508 :: Timer50 = unsafe { gdtimer5071732508 :: Timer50 :: from_ptr (0x4000_f400usize as _) } ; pub const TIMER0 : gdtimer03afad14d :: Timer0 = unsafe { gdtimer03afad14d :: Timer0 :: from_ptr (0x4001_0000usize as _) } ; pub const TIMER7 : gdtimer03afad14d :: Timer0 = unsafe { gdtimer03afad14d :: Timer0 :: from_ptr (0x4001_0400usize as _) } ; pub const USART0 : gdusart0626fb765 :: Usart0 = unsafe { gdusart0626fb765 :: Usart0 :: from_ptr (0x4001_1000usize as _) } ; pub const USART5 : gdusart0626fb765 :: Usart0 = unsafe { gdusart0626fb765 :: Usart0 :: from_ptr (0x4001_1400usize as _) } ; pub const ADC0 : gdadc06d279556 :: Adc0 = unsafe { gdadc06d279556 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc1425a4aff :: Adc1 = unsafe { gdadc1425a4aff :: Adc1 :: from_ptr (0x4001_2800usize as _) } ; pub const ADC2 : gdadc2efea3dc8 :: Adc2 = unsafe { gdadc2efea3dc8 :: Adc2 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0a7377dd5 :: Spi0 = unsafe { gdspi0a7377dd5 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const SPI3 : gdspi3e9b78823 :: Spi3 = unsafe { gdspi3e9b78823 :: Spi3 :: from_ptr (0x4001_3400usize as _) } ; pub const SPI5 : gdspi5c82f56e6 :: Spi5 = unsafe { gdspi5c82f56e6 :: Spi5 :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimer155d5134ba :: Timer15 = unsafe { gdtimer155d5134ba :: Timer15 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimer155d5134ba :: Timer15 = unsafe { gdtimer155d5134ba :: Timer15 :: from_ptr (0x4001_4800usize as _) } ; pub const SPI4 : gdspi4af049e38 :: Spi4 = unsafe { gdspi4af049e38 :: Spi4 :: from_ptr (0x4001_5000usize as _) } ; pub const HPDF : gdhpdffd9de252 :: Hpdf = unsafe { gdhpdffd9de252 :: Hpdf :: from_ptr (0x4001_7000usize as _) } ; pub const TRIGSEL : gdtrigsel9d4a38fd :: Trigsel = unsafe { gdtrigsel9d4a38fd :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const EDOUT : gdedoutfebca4f4 :: Edout = unsafe { gdedoutfebca4f4 :: Edout :: from_ptr (0x4001_8800usize as _) } ; pub const CAN0 : gdcan0ab6ea0b5 :: Can0 = unsafe { gdcan0ab6ea0b5 :: Can0 :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcan0ab6ea0b5 :: Can0 = unsafe { gdcan0ab6ea0b5 :: Can0 :: from_ptr (0x4001_b000usize as _) } ; pub const CAN2 : gdcan0ab6ea0b5 :: Can0 = unsafe { gdcan0ab6ea0b5 :: Can0 :: from_ptr (0x4001_c000usize as _) } ; pub const TIMER40 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_d000usize as _) } ; pub const TIMER41 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_d400usize as _) } ; pub const TIMER42 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_d800usize as _) } ; pub const TIMER43 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_dc00usize as _) } ; pub const TIMER44 : gdtimer1457881844 :: Timer14 = unsafe { gdtimer1457881844 :: Timer14 :: from_ptr (0x4001_f000usize as _) } ; pub const DMA0 : gddma09f21797a :: Dma0 = unsafe { gddma09f21797a :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma09f21797a :: Dma0 = unsafe { gddma09f21797a :: Dma0 :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamuxeaace10d :: Dmamux = unsafe { gddmamuxeaace10d :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const EFUSE : gdefuseee44bd8c :: Efuse = unsafe { gdefuseee44bd8c :: Efuse :: from_ptr (0x4002_2800usize as _) } ; pub const USBHS0_GLOBAL : gdusbhs0globalbee3a389 :: Usbhs0Global = unsafe { gdusbhs0globalbee3a389 :: Usbhs0Global :: from_ptr (0x4004_0000usize as _) } ; pub const USBHS0_HOST : gdusbhs0host663109ac :: Usbhs0Host = unsafe { gdusbhs0host663109ac :: Usbhs0Host :: from_ptr (0x4004_0400usize as _) } ; pub const USBHS0_DEVICE : gdusbhs0deviced0449d15 :: Usbhs0Device = unsafe { gdusbhs0deviced0449d15 :: Usbhs0Device :: from_ptr (0x4004_0800usize as _) } ; pub const USBHS0_PWRCLK : gdusbhs0pwrclk0f97dd8b :: Usbhs0Pwrclk = unsafe { gdusbhs0pwrclk0f97dd8b :: Usbhs0Pwrclk :: from_ptr (0x4004_0e00usize as _) } ; pub const USBHS1_GLOBAL : gdusbhs1globalb3d6824e :: Usbhs1Global = unsafe { gdusbhs1globalb3d6824e :: Usbhs1Global :: from_ptr (0x4008_0000usize as _) } ; pub const USBHS1_HOST : gdusbhs1host14113081 :: Usbhs1Host = unsafe { gdusbhs1host14113081 :: Usbhs1Host :: from_ptr (0x4008_0400usize as _) } ; pub const USBHS1_DEVICE : gdusbhs1device9d406887 :: Usbhs1Device = unsafe { gdusbhs1device9d406887 :: Usbhs1Device :: from_ptr (0x4008_0800usize as _) } ; pub const USBHS1_PWRCLK : gdusbhs1pwrclk29fc276e :: Usbhs1Pwrclk = unsafe { gdusbhs1pwrclk29fc276e :: Usbhs1Pwrclk :: from_ptr (0x4008_0e00usize as _) } ; pub const TRNG : gdtrng6bc6a907 :: Trng = unsafe { gdtrng6bc6a907 :: Trng :: from_ptr (0x4802_1800usize as _) } ; pub const RAMECCMU1 : gdrameccmu1ba654536 :: Rameccmu1 = unsafe { gdrameccmu1ba654536 :: Rameccmu1 :: from_ptr (0x4802_3000usize as _) } ; pub const TMU : gdtmucbc214df :: Tmu = unsafe { gdtmucbc214df :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const FAC : gdfac96d60f19 :: Fac = unsafe { gdfac96d60f19 :: Fac :: from_ptr (0x4802_4800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x5000_3000usize as _) } ; pub const MDMA : gdmdmab9a60aaf :: Mdma = unsafe { gdmdmab9a60aaf :: Mdma :: from_ptr (0x5200_0000usize as _) } ; pub const FMC : gdfmca3be0d1e :: Fmc = unsafe { gdfmca3be0d1e :: Fmc :: from_ptr (0x5200_2000usize as _) } ; pub const EXMC : gdexmc293e9145 :: Exmc = unsafe { gdexmc293e9145 :: Exmc :: from_ptr (0x5200_4000usize as _) } ; pub const OSPI0 : gdospi0439e0312 :: Ospi0 = unsafe { gdospi0439e0312 :: Ospi0 :: from_ptr (0x5200_5000usize as _) } ; pub const RAMECCMU0 : gdrameccmu0d260ef4c :: Rameccmu0 = unsafe { gdrameccmu0d260ef4c :: Rameccmu0 :: from_ptr (0x5200_9000usize as _) } ; pub const OSPI1 : gdospi0439e0312 :: Ospi0 = unsafe { gdospi0439e0312 :: Ospi0 :: from_ptr (0x5200_a000usize as _) } ; pub const OSPIM : gdospimeed3ce76 :: Ospim = unsafe { gdospimeed3ce76 :: Ospim :: from_ptr (0x5200_b400usize as _) } ; pub const EXTI : gdextiacb27c36 :: Exti = unsafe { gdextiacb27c36 :: Exti :: from_ptr (0x5800_0000usize as _) } ; pub const SYSCFG : gdsyscfgb64b8ea8 :: Syscfg = unsafe { gdsyscfgb64b8ea8 :: Syscfg :: from_ptr (0x5800_0400usize as _) } ; pub const CMP : gdcmpb718ae34 :: Cmp = unsafe { gdcmpb718ae34 :: Cmp :: from_ptr (0x5800_3800usize as _) } ; pub const VREF : gdvref193fa1c3 :: Vref = unsafe { gdvref193fa1c3 :: Vref :: from_ptr (0x5800_3c00usize as _) } ; pub const RTC : gdrtcc8139290 :: Rtc = unsafe { gdrtcc8139290 :: Rtc :: from_ptr (0x5800_4000usize as _) } ; pub const FWDGT : gdfwdgt5932fb56 :: Fwdgt = unsafe { gdfwdgt5932fb56 :: Fwdgt :: from_ptr (0x5800_4800usize as _) } ; pub const PMU : gdpmu04713c6c :: Pmu = unsafe { gdpmu04713c6c :: Pmu :: from_ptr (0x5800_5800usize as _) } ; pub const LPDTS : gdlpdtsa3b40577 :: Lpdts = unsafe { gdlpdtsa3b40577 :: Lpdts :: from_ptr (0x5800_6800usize as _) } ; pub const GPIOA : gdgpioa9b446375 :: Gpioa = unsafe { gdgpioa9b446375 :: Gpioa :: from_ptr (0x5802_0000usize as _) } ; pub const GPIOB : gdgpiob3479439a :: Gpiob = unsafe { gdgpiob3479439a :: Gpiob :: from_ptr (0x5802_0400usize as _) } ; pub const GPIOC : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_0800usize as _) } ; pub const GPIOD : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_0c00usize as _) } ; pub const GPIOE : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_1000usize as _) } ; pub const GPIOF : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_1400usize as _) } ; pub const GPIOG : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_1800usize as _) } ; pub const GPIOH : gdgpioc18dfc69f :: Gpioc = unsafe { gdgpioc18dfc69f :: Gpioc :: from_ptr (0x5802_1c00usize as _) } ; pub const RCU : gdrcufed5390f :: Rcu = unsafe { gdrcufed5390f :: Rcu :: from_ptr (0x5802_4400usize as _) } ; pub const CRC : gdcrc1ff07d05 :: Crc = unsafe { gdcrc1ff07d05 :: Crc :: from_ptr (0x5802_4c00usize as _) } ; pub const DBG : gddbg21d11b7d :: Dbg = unsafe { gddbg21d11b7d :: Dbg :: from_ptr (0xe00e_1000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc06d279556_v1.rs"] pub mod gdadc06d279556;
#[path="../../peripherals/gdadc1425a4aff_v1.rs"] pub mod gdadc1425a4aff;
#[path="../../peripherals/gdadc2efea3dc8_v1.rs"] pub mod gdadc2efea3dc8;
#[path="../../peripherals/gdcan0ab6ea0b5_v1.rs"] pub mod gdcan0ab6ea0b5;
#[path="../../peripherals/gdcmpb718ae34_v1.rs"] pub mod gdcmpb718ae34;
#[path="../../peripherals/gdcrc1ff07d05_v1.rs"] pub mod gdcrc1ff07d05;
#[path="../../peripherals/gdctcdb80f1ce_v1.rs"] pub mod gdctcdb80f1ce;
#[path="../../peripherals/gddac555b6194_v1.rs"] pub mod gddac555b6194;
#[path="../../peripherals/gddbg21d11b7d_v1.rs"] pub mod gddbg21d11b7d;
#[path="../../peripherals/gddma09f21797a_v1.rs"] pub mod gddma09f21797a;
#[path="../../peripherals/gddmamuxeaace10d_v1.rs"] pub mod gddmamuxeaace10d;
#[path="../../peripherals/gdedoutfebca4f4_v1.rs"] pub mod gdedoutfebca4f4;
#[path="../../peripherals/gdefuseee44bd8c_v1.rs"] pub mod gdefuseee44bd8c;
#[path="../../peripherals/gdexmc293e9145_v1.rs"] pub mod gdexmc293e9145;
#[path="../../peripherals/gdextiacb27c36_v1.rs"] pub mod gdextiacb27c36;
#[path="../../peripherals/gdfac96d60f19_v1.rs"] pub mod gdfac96d60f19;
#[path="../../peripherals/gdfmca3be0d1e_v1.rs"] pub mod gdfmca3be0d1e;
#[path="../../peripherals/gdfwdgt5932fb56_v1.rs"] pub mod gdfwdgt5932fb56;
#[path="../../peripherals/gdgpioa9b446375_v1.rs"] pub mod gdgpioa9b446375;
#[path="../../peripherals/gdgpiob3479439a_v1.rs"] pub mod gdgpiob3479439a;
#[path="../../peripherals/gdgpioc18dfc69f_v1.rs"] pub mod gdgpioc18dfc69f;
#[path="../../peripherals/gdhpdffd9de252_v1.rs"] pub mod gdhpdffd9de252;
#[path="../../peripherals/gdi2c0cd973dc4_v1.rs"] pub mod gdi2c0cd973dc4;
#[path="../../peripherals/gdlpdtsa3b40577_v1.rs"] pub mod gdlpdtsa3b40577;
#[path="../../peripherals/gdmdmab9a60aaf_v1.rs"] pub mod gdmdmab9a60aaf;
#[path="../../peripherals/gdospi0439e0312_v1.rs"] pub mod gdospi0439e0312;
#[path="../../peripherals/gdospimeed3ce76_v1.rs"] pub mod gdospimeed3ce76;
#[path="../../peripherals/gdpmu04713c6c_v1.rs"] pub mod gdpmu04713c6c;
#[path="../../peripherals/gdrameccmu0d260ef4c_v1.rs"] pub mod gdrameccmu0d260ef4c;
#[path="../../peripherals/gdrameccmu1ba654536_v1.rs"] pub mod gdrameccmu1ba654536;
#[path="../../peripherals/gdrcufed5390f_v1.rs"] pub mod gdrcufed5390f;
#[path="../../peripherals/gdrtcc8139290_v1.rs"] pub mod gdrtcc8139290;
#[path="../../peripherals/gdspi0a7377dd5_v1.rs"] pub mod gdspi0a7377dd5;
#[path="../../peripherals/gdspi1356222e3_v1.rs"] pub mod gdspi1356222e3;
#[path="../../peripherals/gdspi255cb8c1f_v1.rs"] pub mod gdspi255cb8c1f;
#[path="../../peripherals/gdspi3e9b78823_v1.rs"] pub mod gdspi3e9b78823;
#[path="../../peripherals/gdspi4af049e38_v1.rs"] pub mod gdspi4af049e38;
#[path="../../peripherals/gdspi5c82f56e6_v1.rs"] pub mod gdspi5c82f56e6;
#[path="../../peripherals/gdsyscfgb64b8ea8_v1.rs"] pub mod gdsyscfgb64b8ea8;
#[path="../../peripherals/gdtimer03afad14d_v1.rs"] pub mod gdtimer03afad14d;
#[path="../../peripherals/gdtimer1457881844_v1.rs"] pub mod gdtimer1457881844;
#[path="../../peripherals/gdtimer155d5134ba_v1.rs"] pub mod gdtimer155d5134ba;
#[path="../../peripherals/gdtimer1da3bc56a_v1.rs"] pub mod gdtimer1da3bc56a;
#[path="../../peripherals/gdtimer27201f8c9_v1.rs"] pub mod gdtimer27201f8c9;
#[path="../../peripherals/gdtimer5071732508_v1.rs"] pub mod gdtimer5071732508;
#[path="../../peripherals/gdtimer5330a987e_v1.rs"] pub mod gdtimer5330a987e;
#[path="../../peripherals/gdtmucbc214df_v1.rs"] pub mod gdtmucbc214df;
#[path="../../peripherals/gdtrigsel9d4a38fd_v1.rs"] pub mod gdtrigsel9d4a38fd;
#[path="../../peripherals/gdtrng6bc6a907_v1.rs"] pub mod gdtrng6bc6a907;
#[path="../../peripherals/gduart330e38640_v1.rs"] pub mod gduart330e38640;
#[path="../../peripherals/gdusart0626fb765_v1.rs"] pub mod gdusart0626fb765;
#[path="../../peripherals/gdusbhs0deviced0449d15_v1.rs"] pub mod gdusbhs0deviced0449d15;
#[path="../../peripherals/gdusbhs0globalbee3a389_v1.rs"] pub mod gdusbhs0globalbee3a389;
#[path="../../peripherals/gdusbhs0host663109ac_v1.rs"] pub mod gdusbhs0host663109ac;
#[path="../../peripherals/gdusbhs0pwrclk0f97dd8b_v1.rs"] pub mod gdusbhs0pwrclk0f97dd8b;
#[path="../../peripherals/gdusbhs1device9d406887_v1.rs"] pub mod gdusbhs1device9d406887;
#[path="../../peripherals/gdusbhs1globalb3d6824e_v1.rs"] pub mod gdusbhs1globalb3d6824e;
#[path="../../peripherals/gdusbhs1host14113081_v1.rs"] pub mod gdusbhs1host14113081;
#[path="../../peripherals/gdusbhs1pwrclk29fc276e_v1.rs"] pub mod gdusbhs1pwrclk29fc276e;
#[path="../../peripherals/gdvref193fa1c3_v1.rs"] pub mod gdvref193fa1c3;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
