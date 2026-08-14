




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - VAVD_LVD_VOVD_VUVD"]
VAVD_LVD_VOVD_VUVD = 1 , # [doc = "2 - TAMPER_STAMP_LXTAL"]
TAMPER_STAMP_LXTAL = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - NVMC"]
NVMC = 4 , # [doc = "5 - RCU"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1_RDCM"]
ADC0_1_RDCM = 18 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 26 , # [doc = "27 - TIMER0_CHANNEL"]
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
RTC_ALARM = 41 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT"]
TIMER7_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - DMA0_CHANNEL7"]
DMA0_CHANNEL7 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "49 - SDIO0"]
SDIO0 = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5_DAC_UDR"]
TIMER5_DAC_UDR = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - ENET0"]
ENET0 = 61 , # [doc = "62 - ENET0_WKUP"]
ENET0_WKUP = 62 , # [doc = "68 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 68 , # [doc = "69 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 69 , # [doc = "70 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 70 , # [doc = "71 - USART5"]
USART5 = 71 , # [doc = "72 - I2C2_EV"]
I2C2_EV = 72 , # [doc = "73 - I2C2_ER"]
I2C2_ER = 73 , # [doc = "74 - USBHS0_EP1_OUT"]
USBHS0_EP1_OUT = 74 , # [doc = "75 - USBHS0_EP1_IN"]
USBHS0_EP1_IN = 75 , # [doc = "76 - USBHS0_WKUP"]
USBHS0_WKUP = 76 , # [doc = "77 - USBHS0"]
USBHS0 = 77 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "82 - UART6"]
UART6 = 82 , # [doc = "83 - UART7"]
UART7 = 83 , # [doc = "84 - SPI3"]
SPI3 = 84 , # [doc = "85 - SPI4"]
SPI4 = 85 , # [doc = "86 - SPI5"]
SPI5 = 86 , # [doc = "87 - SAI0"]
SAI0 = 87 , # [doc = "88 - TLI"]
TLI = 88 , # [doc = "89 - TLI_ER"]
TLI_ER = 89 , # [doc = "90 - IPA"]
IPA = 90 , # [doc = "91 - SAI1"]
SAI1 = 91 , # [doc = "92 - OSPI0"]
OSPI0 = 92 , # [doc = "95 - I2C3_EV"]
I2C3_EV = 95 , # [doc = "96 - I2C3_ER"]
I2C3_ER = 96 , # [doc = "97 - RSPDIF"]
RSPDIF = 97 , # [doc = "102 - DMAMUX_OVR"]
DMAMUX_OVR = 102 , # [doc = "109 - HPDF_INT4"]
HPDF_INT4 = 109 , # [doc = "110 - HPDF_INT0"]
HPDF_INT0 = 110 , # [doc = "111 - HPDF_INT1"]
HPDF_INT1 = 111 , # [doc = "112 - HPDF_INT2"]
HPDF_INT2 = 112 , # [doc = "113 - HPDF_INT3"]
HPDF_INT3 = 113 , # [doc = "114 - SAI2"]
SAI2 = 114 , # [doc = "116 - TIMER14"]
TIMER14 = 116 , # [doc = "117 - TIMER15"]
TIMER15 = 117 , # [doc = "118 - TIMER16"]
TIMER16 = 118 , # [doc = "120 - MDIO"]
MDIO = 120 , # [doc = "122 - MDMA"]
MDMA = 122 , # [doc = "124 - SDIO1"]
SDIO1 = 124 , # [doc = "125 - HWSEM"]
HWSEM = 125 , # [doc = "127 - ADC2"]
ADC2 = 127 , # [doc = "137 - CMP0_1"]
CMP0_1 = 137 , # [doc = "144 - CTC"]
CTC = 144 , # [doc = "145 - RAMECCMU"]
RAMECCMU = 145 , # [doc = "146 - SAI3"]
SAI3 = 146 , # [doc = "150 - OSPI1"]
OSPI1 = 150 , # [doc = "151 - RTDEC0"]
RTDEC0 = 151 , # [doc = "152 - RTDEC1"]
RTDEC1 = 152 , # [doc = "153 - FAC"]
FAC = 153 , # [doc = "154 - TMU"]
TMU = 154 , # [doc = "155 - FFT"]
FFT = 155 , # [doc = "156 - ESC_SYNC0"]
ESC_SYNC0 = 156 , # [doc = "157 - ESC_SYNC1"]
ESC_SYNC1 = 157 , # [doc = "158 - ESC"]
ESC = 158 , # [doc = "161 - TIMER22"]
TIMER22 = 161 , # [doc = "162 - TIMER23"]
TIMER23 = 162 , # [doc = "163 - TIMER30"]
TIMER30 = 163 , # [doc = "164 - TIMER31"]
TIMER31 = 164 , # [doc = "165 - TIMER40"]
TIMER40 = 165 , # [doc = "166 - TIMER41"]
TIMER41 = 166 , # [doc = "167 - TIMER42"]
TIMER42 = 167 , # [doc = "168 - TIMER43"]
TIMER43 = 168 , # [doc = "169 - TIMER44"]
TIMER44 = 169 , # [doc = "170 - TIMER50"]
TIMER50 = 170 , # [doc = "171 - TIMER51"]
TIMER51 = 171 , # [doc = "172 - USBHS1_EP1_OUT"]
USBHS1_EP1_OUT = 172 , # [doc = "173 - USBHS1_EP1_IN"]
USBHS1_EP1_IN = 173 , # [doc = "174 - USBHS1_WKUP"]
USBHS1_WKUP = 174 , # [doc = "175 - USBHS1"]
USBHS1 = 175 , # [doc = "176 - ENET1"]
ENET1 = 176 , # [doc = "177 - ENET1_WKUP"]
ENET1_WKUP = 177 , # [doc = "179 - CAN0_WKUP"]
CAN0_WKUP = 179 , # [doc = "180 - CAN0_MESSAGE"]
CAN0_MESSAGE = 180 , # [doc = "181 - CAN0_BUSOFF"]
CAN0_BUSOFF = 181 , # [doc = "182 - CAN0_ERROR"]
CAN0_ERROR = 182 , # [doc = "183 - CAN0_FASTERROR"]
CAN0_FASTERROR = 183 , # [doc = "184 - CAN0_TEC"]
CAN0_TEC = 184 , # [doc = "185 - CAN0_REC"]
CAN0_REC = 185 , # [doc = "186 - CAN1_WKUP"]
CAN1_WKUP = 186 , # [doc = "187 - CAN1_MESSAGE"]
CAN1_MESSAGE = 187 , # [doc = "188 - CAN1_BUSOFF"]
CAN1_BUSOFF = 188 , # [doc = "189 - CAN1_ERROR"]
CAN1_ERROR = 189 , # [doc = "190 - CAN1_FASTERROR"]
CAN1_FASTERROR = 190 , # [doc = "191 - CAN1_TEC"]
CAN1_TEC = 191 , # [doc = "192 - CAN1_REC"]
CAN1_REC = 192 , # [doc = "193 - CAN2_WKUP"]
CAN2_WKUP = 193 , # [doc = "194 - CAN2_MESSAGE"]
CAN2_MESSAGE = 194 , # [doc = "195 - CAN2_BUSOFF"]
CAN2_BUSOFF = 195 , # [doc = "196 - CAN2_ERROR"]
CAN2_ERROR = 196 , # [doc = "197 - CAN2_FASTERROR"]
CAN2_FASTERROR = 197 , # [doc = "198 - CAN2_TEC"]
CAN2_TEC = 198 , # [doc = "199 - CAN2_REC"]
CAN2_REC = 199 , # [doc = "200 - OTP"]
OTP = 200 , # [doc = "201 - I2C0_WKUP"]
I2C0_WKUP = 201 , # [doc = "202 - I2C1_WKUP"]
I2C1_WKUP = 202 , # [doc = "203 - I2C2_WKUP"]
I2C2_WKUP = 203 , # [doc = "204 - I2C3_WKUP"]
I2C3_WKUP = 204 , # [doc = "205 - LPDTS"]
LPDTS = 205 , # [doc = "206 - LPDTS_WKUP"]
LPDTS_WKUP = 206 , # [doc = "207 - TIMER0_DEC"]
TIMER0_DEC = 207 , # [doc = "208 - TIMER7_DEC"]
TIMER7_DEC = 208 , # [doc = "209 - TIMER1_DEC"]
TIMER1_DEC = 209 , # [doc = "210 - TIMER2_DEC"]
TIMER2_DEC = 210 , # [doc = "211 - TIMER3_DEC"]
TIMER3_DEC = 211 , # [doc = "212 - TIMER4_DEC"]
TIMER4_DEC = 212 , # [doc = "213 - TIMER22_DEC"]
TIMER22_DEC = 213 , # [doc = "214 - TIMER23_DEC"]
TIMER23_DEC = 214 , # [doc = "215 - TIMER30_DEC"]
TIMER30_DEC = 215 , # [doc = "216 - TIMER31_DEC"]
TIMER31_DEC = 216 , # [doc = "217 - CPU_DCACHE_ERR"]
CPU_DCACHE_ERR = 217 , # [doc = "218 - CPU_ICACHE_ERR"]
CPU_ICACHE_ERR = 218 , # [doc = "219 - VOVD_VUVD_FILTER"]
VOVD_VUVD_FILTER = 219 , # [doc = "221 - TIMER0_CHANNEL_BRK"]
TIMER0_CHANNEL_BRK = 221 , # [doc = "222 - TIMER7_CHANNEL_BRK"]
TIMER7_CHANNEL_BRK = 222 , # [doc = "223 - DSI"]
DSI = 223 , # [doc = "224 - PKCAU"]
PKCAU = 224 , # [doc = "225 - ESC_BRIDGE"]
ESC_BRIDGE = 225 , # [doc = "226 - EDIM"]
EDIM = 226 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn VAVD_LVD_VOVD_VUVD () ; fn TAMPER_STAMP_LXTAL () ; fn RTC_WKUP () ; fn NVMC () ; fn RCU () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1_RDCM () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn DMA0_CHANNEL7 () ; fn EXMC () ; fn SDIO0 () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC_UDR () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET0 () ; fn ENET0_WKUP () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBHS0_EP1_OUT () ; fn USBHS0_EP1_IN () ; fn USBHS0_WKUP () ; fn USBHS0 () ; fn DCI () ; fn CAU () ; fn HAU_TRNG () ; fn FPU () ; fn UART6 () ; fn UART7 () ; fn SPI3 () ; fn SPI4 () ; fn SPI5 () ; fn SAI0 () ; fn TLI () ; fn TLI_ER () ; fn IPA () ; fn SAI1 () ; fn OSPI0 () ; fn I2C3_EV () ; fn I2C3_ER () ; fn RSPDIF () ; fn DMAMUX_OVR () ; fn HPDF_INT4 () ; fn HPDF_INT0 () ; fn HPDF_INT1 () ; fn HPDF_INT2 () ; fn HPDF_INT3 () ; fn SAI2 () ; fn TIMER14 () ; fn TIMER15 () ; fn TIMER16 () ; fn MDIO () ; fn MDMA () ; fn SDIO1 () ; fn HWSEM () ; fn ADC2 () ; fn CMP0_1 () ; fn CTC () ; fn RAMECCMU () ; fn SAI3 () ; fn OSPI1 () ; fn RTDEC0 () ; fn RTDEC1 () ; fn FAC () ; fn TMU () ; fn FFT () ; fn ESC_SYNC0 () ; fn ESC_SYNC1 () ; fn ESC () ; fn TIMER22 () ; fn TIMER23 () ; fn TIMER30 () ; fn TIMER31 () ; fn TIMER40 () ; fn TIMER41 () ; fn TIMER42 () ; fn TIMER43 () ; fn TIMER44 () ; fn TIMER50 () ; fn TIMER51 () ; fn USBHS1_EP1_OUT () ; fn USBHS1_EP1_IN () ; fn USBHS1_WKUP () ; fn USBHS1 () ; fn ENET1 () ; fn ENET1_WKUP () ; fn CAN0_WKUP () ; fn CAN0_MESSAGE () ; fn CAN0_BUSOFF () ; fn CAN0_ERROR () ; fn CAN0_FASTERROR () ; fn CAN0_TEC () ; fn CAN0_REC () ; fn CAN1_WKUP () ; fn CAN1_MESSAGE () ; fn CAN1_BUSOFF () ; fn CAN1_ERROR () ; fn CAN1_FASTERROR () ; fn CAN1_TEC () ; fn CAN1_REC () ; fn CAN2_WKUP () ; fn CAN2_MESSAGE () ; fn CAN2_BUSOFF () ; fn CAN2_ERROR () ; fn CAN2_FASTERROR () ; fn CAN2_TEC () ; fn CAN2_REC () ; fn OTP () ; fn I2C0_WKUP () ; fn I2C1_WKUP () ; fn I2C2_WKUP () ; fn I2C3_WKUP () ; fn LPDTS () ; fn LPDTS_WKUP () ; fn TIMER0_DEC () ; fn TIMER7_DEC () ; fn TIMER1_DEC () ; fn TIMER2_DEC () ; fn TIMER3_DEC () ; fn TIMER4_DEC () ; fn TIMER22_DEC () ; fn TIMER23_DEC () ; fn TIMER30_DEC () ; fn TIMER31_DEC () ; fn CPU_DCACHE_ERR () ; fn CPU_ICACHE_ERR () ; fn VOVD_VUVD_FILTER () ; fn TIMER0_CHANNEL_BRK () ; fn TIMER7_CHANNEL_BRK () ; fn DSI () ; fn PKCAU () ; fn ESC_BRIDGE () ; fn EDIM () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 227]
= [Vector { _handler : WWDGT } , Vector { _handler : VAVD_LVD_VOVD_VUVD } , Vector { _handler : TAMPER_STAMP_LXTAL } , Vector { _handler : RTC_WKUP } , Vector { _handler : NVMC } , Vector { _handler : RCU } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1_RDCM } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _reserved : 0 } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : EXMC } , Vector { _handler : SDIO0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC_UDR } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET0 } , Vector { _handler : ENET0_WKUP } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBHS0_EP1_OUT } , Vector { _handler : USBHS0_EP1_IN } , Vector { _handler : USBHS0_WKUP } , Vector { _handler : USBHS0 } , Vector { _handler : DCI } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _handler : FPU } , Vector { _handler : UART6 } , Vector { _handler : UART7 } , Vector { _handler : SPI3 } , Vector { _handler : SPI4 } , Vector { _handler : SPI5 } , Vector { _handler : SAI0 } , Vector { _handler : TLI } , Vector { _handler : TLI_ER } , Vector { _handler : IPA } , Vector { _handler : SAI1 } , Vector { _handler : OSPI0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : I2C3_EV } , Vector { _handler : I2C3_ER } , Vector { _handler : RSPDIF } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DMAMUX_OVR } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : HPDF_INT4 } , Vector { _handler : HPDF_INT0 } , Vector { _handler : HPDF_INT1 } , Vector { _handler : HPDF_INT2 } , Vector { _handler : HPDF_INT3 } , Vector { _handler : SAI2 } , Vector { _reserved : 0 } , Vector { _handler : TIMER14 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _reserved : 0 } , Vector { _handler : MDIO } , Vector { _reserved : 0 } , Vector { _handler : MDMA } , Vector { _reserved : 0 } , Vector { _handler : SDIO1 } , Vector { _handler : HWSEM } , Vector { _reserved : 0 } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CMP0_1 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : CTC } , Vector { _handler : RAMECCMU } , Vector { _handler : SAI3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : OSPI1 } , Vector { _handler : RTDEC0 } , Vector { _handler : RTDEC1 } , Vector { _handler : FAC } , Vector { _handler : TMU } , Vector { _handler : FFT } , Vector { _handler : ESC_SYNC0 } , Vector { _handler : ESC_SYNC1 } , Vector { _handler : ESC } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIMER22 } , Vector { _handler : TIMER23 } , Vector { _handler : TIMER30 } , Vector { _handler : TIMER31 } , Vector { _handler : TIMER40 } , Vector { _handler : TIMER41 } , Vector { _handler : TIMER42 } , Vector { _handler : TIMER43 } , Vector { _handler : TIMER44 } , Vector { _handler : TIMER50 } , Vector { _handler : TIMER51 } , Vector { _handler : USBHS1_EP1_OUT } , Vector { _handler : USBHS1_EP1_IN } , Vector { _handler : USBHS1_WKUP } , Vector { _handler : USBHS1 } , Vector { _handler : ENET1 } , Vector { _handler : ENET1_WKUP } , Vector { _reserved : 0 } , Vector { _handler : CAN0_WKUP } , Vector { _handler : CAN0_MESSAGE } , Vector { _handler : CAN0_BUSOFF } , Vector { _handler : CAN0_ERROR } , Vector { _handler : CAN0_FASTERROR } , Vector { _handler : CAN0_TEC } , Vector { _handler : CAN0_REC } , Vector { _handler : CAN1_WKUP } , Vector { _handler : CAN1_MESSAGE } , Vector { _handler : CAN1_BUSOFF } , Vector { _handler : CAN1_ERROR } , Vector { _handler : CAN1_FASTERROR } , Vector { _handler : CAN1_TEC } , Vector { _handler : CAN1_REC } , Vector { _handler : CAN2_WKUP } , Vector { _handler : CAN2_MESSAGE } , Vector { _handler : CAN2_BUSOFF } , Vector { _handler : CAN2_ERROR } , Vector { _handler : CAN2_FASTERROR } , Vector { _handler : CAN2_TEC } , Vector { _handler : CAN2_REC } , Vector { _handler : OTP } , Vector { _handler : I2C0_WKUP } , Vector { _handler : I2C1_WKUP } , Vector { _handler : I2C2_WKUP } , Vector { _handler : I2C3_WKUP } , Vector { _handler : LPDTS } , Vector { _handler : LPDTS_WKUP } , Vector { _handler : TIMER0_DEC } , Vector { _handler : TIMER7_DEC } , Vector { _handler : TIMER1_DEC } , Vector { _handler : TIMER2_DEC } , Vector { _handler : TIMER3_DEC } , Vector { _handler : TIMER4_DEC } , Vector { _handler : TIMER22_DEC } , Vector { _handler : TIMER23_DEC } , Vector { _handler : TIMER30_DEC } , Vector { _handler : TIMER31_DEC } , Vector { _handler : CPU_DCACHE_ERR } , Vector { _handler : CPU_ICACHE_ERR } , Vector { _handler : VOVD_VUVD_FILTER } , Vector { _reserved : 0 } , Vector { _handler : TIMER0_CHANNEL_BRK } , Vector { _handler : TIMER7_CHANNEL_BRK } , Vector { _handler : DSI } , Vector { _handler : PKCAU } , Vector { _handler : ESC_BRIDGE } , Vector { _handler : EDIM } ,]
; } pub const TIMER1 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const SPI1 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const RSPDIF : gdrspdif22daf900 :: Rspdif = unsafe { gdrspdif22daf900 :: Rspdif :: from_ptr (0x4000_4000usize as _) } ; pub const USART1 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const I2C3 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_5c00usize as _) } ; pub const DAC0 : gddac523caf1a :: Dac = unsafe { gddac523caf1a :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4000_7c00usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_8400usize as _) } ; pub const MDIO : gdmdio596b2b27 :: Mdio = unsafe { gdmdio596b2b27 :: Mdio :: from_ptr (0x4000_9400usize as _) } ; pub const I2C2 : gdi2c4e73acde :: I2c = unsafe { gdi2c4e73acde :: I2c :: from_ptr (0x4000_c000usize as _) } ; pub const EDIM_AFMT : gdedimafmtc755000c :: EdimAfmt = unsafe { gdedimafmtc755000c :: EdimAfmt :: from_ptr (0x4000_c400usize as _) } ; pub const EDIM_TFMT : gdedimtfmt3eab6256 :: EdimTfmt = unsafe { gdedimtfmt3eab6256 :: EdimTfmt :: from_ptr (0x4000_c800usize as _) } ; pub const EDIM_ENDAT : gdedimendat15ceb0e5 :: EdimEndat = unsafe { gdedimendat15ceb0e5 :: EdimEndat :: from_ptr (0x4000_cc00usize as _) } ; pub const EDIM_BISS : gdedimbiss1ab60e08 :: EdimBiss = unsafe { gdedimbiss1ab60e08 :: EdimBiss :: from_ptr (0x4000_d000usize as _) } ; pub const EDIM_HDSL : gdedimhdsl8f02a454 :: EdimHdsl = unsafe { gdedimhdsl8f02a454 :: EdimHdsl :: from_ptr (0x4000_d800usize as _) } ; pub const TIMER22 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_e000usize as _) } ; pub const TIMER23 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_e400usize as _) } ; pub const TIMER30 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_e800usize as _) } ; pub const TIMER31 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_ec00usize as _) } ; pub const TIMER50 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_f000usize as _) } ; pub const TIMER51 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4000_f400usize as _) } ; pub const TIMER0 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_0000usize as _) } ; pub const TIMER7 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_0400usize as _) } ; pub const USART0 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4001_1000usize as _) } ; pub const USART5 : gdusarte31b633e :: Usart = unsafe { gdusarte31b633e :: Usart :: from_ptr (0x4001_1400usize as _) } ; pub const ADC0 : gdadc07ce7e972 :: Adc0 = unsafe { gdadc07ce7e972 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc446b6462 :: Adc = unsafe { gdadc446b6462 :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const ADC2 : gdadc446b6462 :: Adc = unsafe { gdadc446b6462 :: Adc :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const SPI3 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4001_3400usize as _) } ; pub const SPI5 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4001_3800usize as _) } ; pub const TIMER14 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER15 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER16 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const SPI4 : gdspia2feaee7 :: Spi = unsafe { gdspia2feaee7 :: Spi :: from_ptr (0x4001_5000usize as _) } ; pub const SAI0 : gdsai81edec52 :: Sai = unsafe { gdsai81edec52 :: Sai :: from_ptr (0x4001_5800usize as _) } ; pub const SAI1 : gdsai81edec52 :: Sai = unsafe { gdsai81edec52 :: Sai :: from_ptr (0x4001_5c00usize as _) } ; pub const SAI2 : gdsai81edec52 :: Sai = unsafe { gdsai81edec52 :: Sai :: from_ptr (0x4001_6000usize as _) } ; pub const RDCM : gdrdcme04ef535 :: Rdcm = unsafe { gdrdcme04ef535 :: Rdcm :: from_ptr (0x4001_6400usize as _) } ; pub const SAI3 : gdsai81edec52 :: Sai = unsafe { gdsai81edec52 :: Sai :: from_ptr (0x4001_6800usize as _) } ; pub const HPDF : gdhpdffe1368b3 :: Hpdf = unsafe { gdhpdffe1368b3 :: Hpdf :: from_ptr (0x4001_7000usize as _) } ; pub const HPDF_FLT0 : gdhpdffe1368b3 :: Hpdf = unsafe { gdhpdffe1368b3 :: Hpdf :: from_ptr (0x4001_7100usize as _) } ; pub const HPDF_FLT1 : gdhpdffe1368b3 :: Hpdf = unsafe { gdhpdffe1368b3 :: Hpdf :: from_ptr (0x4001_7180usize as _) } ; pub const HPDF_FLT2 : gdhpdffe1368b3 :: Hpdf = unsafe { gdhpdffe1368b3 :: Hpdf :: from_ptr (0x4001_7200usize as _) } ; pub const HPDF_FLT3 : gdhpdffe1368b3 :: Hpdf = unsafe { gdhpdffe1368b3 :: Hpdf :: from_ptr (0x4001_7280usize as _) } ; pub const TRIGSEL : gdtrigselc40c9296 :: Trigsel = unsafe { gdtrigselc40c9296 :: Trigsel :: from_ptr (0x4001_8400usize as _) } ; pub const EDOUT : gdedout6974cab7 :: Edout = unsafe { gdedout6974cab7 :: Edout :: from_ptr (0x4001_8800usize as _) } ; pub const CAN0 : gdcana7cd7781 :: Can = unsafe { gdcana7cd7781 :: Can :: from_ptr (0x4001_a000usize as _) } ; pub const CAN1 : gdcana7cd7781 :: Can = unsafe { gdcana7cd7781 :: Can :: from_ptr (0x4001_b000usize as _) } ; pub const CAN2 : gdcana7cd7781 :: Can = unsafe { gdcana7cd7781 :: Can :: from_ptr (0x4001_c000usize as _) } ; pub const TIMER40 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_d000usize as _) } ; pub const TIMER41 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_d400usize as _) } ; pub const TIMER42 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_d800usize as _) } ; pub const TIMER43 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_dc00usize as _) } ; pub const TIMER44 : gdtimereb2fc375 :: Timer = unsafe { gdtimereb2fc375 :: Timer :: from_ptr (0x4001_f000usize as _) } ; pub const DMA0 : gddmaf3ee856f :: Dma = unsafe { gddmaf3ee856f :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmaf3ee856f :: Dma = unsafe { gddmaf3ee856f :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux489b88ce :: Dmamux = unsafe { gddmamux489b88ce :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbase3c558169 :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbase3c558169 :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbase95458840 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbase95458840 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const ENET0 : gdenet38297eab :: Enet = unsafe { gdenet38297eab :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const ENET1 : gdenet38297eab :: Enet = unsafe { gdenet38297eab :: Enet :: from_ptr (0x4002_a000usize as _) } ; pub const PKCAU : gdpkcauf9e1d63d :: Pkcau = unsafe { gdpkcauf9e1d63d :: Pkcau :: from_ptr (0x4800_0000usize as _) } ; pub const DCI : gddcid3cb6fbe :: Dci = unsafe { gddcid3cb6fbe :: Dci :: from_ptr (0x4802_0000usize as _) } ; pub const GPSI : gdgpsi8a02f8c0 :: Gpsi = unsafe { gdgpsi8a02f8c0 :: Gpsi :: from_ptr (0x4802_0200usize as _) } ; pub const CAU : gdcaufd3db2fb :: Cau = unsafe { gdcaufd3db2fb :: Cau :: from_ptr (0x4802_1000usize as _) } ; pub const HAU : gdhau7c50811e :: Hau = unsafe { gdhau7c50811e :: Hau :: from_ptr (0x4802_1400usize as _) } ; pub const TRNG : gdtrng89e226cf :: Trng = unsafe { gdtrng89e226cf :: Trng :: from_ptr (0x4802_1800usize as _) } ; pub const SDIO1 : gdsdiobd66f984 :: Sdio = unsafe { gdsdiobd66f984 :: Sdio :: from_ptr (0x4802_2400usize as _) } ; pub const CPDM_SDIO1 : gdcpdm37c60c16 :: Cpdm = unsafe { gdcpdm37c60c16 :: Cpdm :: from_ptr (0x4802_2800usize as _) } ; pub const RAMECCMU1 : gdrameccmu8cae911f :: Rameccmu = unsafe { gdrameccmu8cae911f :: Rameccmu :: from_ptr (0x4802_3000usize as _) } ; pub const FFT : gdfftc3717816 :: Fft = unsafe { gdfftc3717816 :: Fft :: from_ptr (0x4802_4000usize as _) } ; pub const TMU : gdtmu6e5ec85c :: Tmu = unsafe { gdtmu6e5ec85c :: Tmu :: from_ptr (0x4802_4400usize as _) } ; pub const FAC : gdfac70f604b5 :: Fac = unsafe { gdfac70f604b5 :: Fac :: from_ptr (0x4802_4800usize as _) } ; pub const TLI : gdtli1ba8a397 :: Tli = unsafe { gdtli1ba8a397 :: Tli :: from_ptr (0x5000_1000usize as _) } ; pub const LAYER0 : gdtli1ba8a397 :: Tli = unsafe { gdtli1ba8a397 :: Tli :: from_ptr (0x5000_1084usize as _) } ; pub const LAYER1 : gdtli1ba8a397 :: Tli = unsafe { gdtli1ba8a397 :: Tli :: from_ptr (0x5000_1104usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x5000_3000usize as _) } ; pub const AXIIM : gdaxiimafbe6630 :: Axiim = unsafe { gdaxiimafbe6630 :: Axiim :: from_ptr (0x5100_0000usize as _) } ; pub const MDMA : gdmdma7993bd09 :: Mdma = unsafe { gdmdma7993bd09 :: Mdma :: from_ptr (0x5200_0000usize as _) } ; pub const IPA : gdipa61f09f1e :: Ipa = unsafe { gdipa61f09f1e :: Ipa :: from_ptr (0x5200_1000usize as _) } ; pub const NVMC : gdnvmc9a5d07ef :: Nvmc = unsafe { gdnvmc9a5d07ef :: Nvmc :: from_ptr (0x5200_2000usize as _) } ; pub const DSI : gddsi76441603 :: Dsi = unsafe { gddsi76441603 :: Dsi :: from_ptr (0x5200_3000usize as _) } ; pub const EXMC : gdexmc2e387b8a :: Exmc = unsafe { gdexmc2e387b8a :: Exmc :: from_ptr (0x5200_4000usize as _) } ; pub const OSPI0 : gdospi71203c17 :: Ospi = unsafe { gdospi71203c17 :: Ospi :: from_ptr (0x5200_5000usize as _) } ; pub const CPDM_OSPI0 : gdcpdm37c60c16 :: Cpdm = unsafe { gdcpdm37c60c16 :: Cpdm :: from_ptr (0x5200_6000usize as _) } ; pub const SDIO0 : gdsdiobd66f984 :: Sdio = unsafe { gdsdiobd66f984 :: Sdio :: from_ptr (0x5200_7000usize as _) } ; pub const CPDM_SDIO0 : gdcpdm37c60c16 :: Cpdm = unsafe { gdcpdm37c60c16 :: Cpdm :: from_ptr (0x5200_8000usize as _) } ; pub const RAMECCMU0 : gdrameccmu05ad7d731 :: Rameccmu0 = unsafe { gdrameccmu05ad7d731 :: Rameccmu0 :: from_ptr (0x5200_9000usize as _) } ; pub const OSPI1 : gdospi71203c17 :: Ospi = unsafe { gdospi71203c17 :: Ospi :: from_ptr (0x5200_a000usize as _) } ; pub const CPDM_OSPI1 : gdcpdm37c60c16 :: Cpdm = unsafe { gdcpdm37c60c16 :: Cpdm :: from_ptr (0x5200_b000usize as _) } ; pub const OSPIM : gdospimad46f02a :: Ospim = unsafe { gdospimad46f02a :: Ospim :: from_ptr (0x5200_b400usize as _) } ; pub const RTDEC0 : gdrtdeca2be341e :: Rtdec = unsafe { gdrtdeca2be341e :: Rtdec :: from_ptr (0x5200_b800usize as _) } ; pub const RTDEC1 : gdrtdeca2be341e :: Rtdec = unsafe { gdrtdeca2be341e :: Rtdec :: from_ptr (0x5200_bc00usize as _) } ; pub const EXTI : gdexti049f6534 :: Exti = unsafe { gdexti049f6534 :: Exti :: from_ptr (0x5800_0000usize as _) } ; pub const SYSCFG : gdsyscfgd19ad75d :: Syscfg = unsafe { gdsyscfgd19ad75d :: Syscfg :: from_ptr (0x5800_0400usize as _) } ; pub const CMP : gdcmp9ee5ac9c :: Cmp = unsafe { gdcmp9ee5ac9c :: Cmp :: from_ptr (0x5800_3800usize as _) } ; pub const VREF : gdvref779f5a9e :: Vref = unsafe { gdvref779f5a9e :: Vref :: from_ptr (0x5800_3c00usize as _) } ; pub const RTC : gdrtc4f2c5d98 :: Rtc = unsafe { gdrtc4f2c5d98 :: Rtc :: from_ptr (0x5800_4000usize as _) } ; pub const FWDGT : gdfwdgtc7bc9588 :: Fwdgt = unsafe { gdfwdgtc7bc9588 :: Fwdgt :: from_ptr (0x5800_4800usize as _) } ; pub const PMU : gdpmu64058cb9 :: Pmu = unsafe { gdpmu64058cb9 :: Pmu :: from_ptr (0x5800_5800usize as _) } ; pub const LPDTS : gdlpdts3307f0fd :: Lpdts = unsafe { gdlpdts3307f0fd :: Lpdts :: from_ptr (0x5800_6800usize as _) } ; pub const GPIOA : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_0000usize as _) } ; pub const GPIOB : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_0400usize as _) } ; pub const GPIOC : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_0800usize as _) } ; pub const GPIOD : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_0c00usize as _) } ; pub const GPIOE : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_1000usize as _) } ; pub const GPIOF : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_1400usize as _) } ; pub const GPIOG : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_1800usize as _) } ; pub const GPIOH : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_1c00usize as _) } ; pub const GPIOJ : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_2400usize as _) } ; pub const GPIOK : gdgpio9a89eec9 :: Gpio = unsafe { gdgpio9a89eec9 :: Gpio :: from_ptr (0x5802_2800usize as _) } ; pub const RCU : gdrcu0591d20f :: Rcu = unsafe { gdrcu0591d20f :: Rcu :: from_ptr (0x5802_4400usize as _) } ; pub const CRC : gdcrc491c92d0 :: Crc = unsafe { gdcrc491c92d0 :: Crc :: from_ptr (0x5802_4c00usize as _) } ; pub const HWSEM : gdhwsem7ea96c46 :: Hwsem = unsafe { gdhwsem7ea96c46 :: Hwsem :: from_ptr (0x5802_6400usize as _) } ; pub const ESC : gdescd7927785 :: Esc = unsafe { gdescd7927785 :: Esc :: from_ptr (0x5f00_0000usize as _) } ; pub const DBG : gddbgde2c0489 :: Dbg = unsafe { gddbgde2c0489 :: Dbg :: from_ptr (0xe00e_1000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc07ce7e972_v1.rs"] pub mod gdadc07ce7e972;
#[path="../../peripherals/gdadc446b6462_v1.rs"] pub mod gdadc446b6462;
#[path="../../peripherals/gdaxiimafbe6630_v1.rs"] pub mod gdaxiimafbe6630;
#[path="../../peripherals/gdcana7cd7781_v1.rs"] pub mod gdcana7cd7781;
#[path="../../peripherals/gdcaufd3db2fb_v1.rs"] pub mod gdcaufd3db2fb;
#[path="../../peripherals/gdcmp9ee5ac9c_v1.rs"] pub mod gdcmp9ee5ac9c;
#[path="../../peripherals/gdcpdm37c60c16_v1.rs"] pub mod gdcpdm37c60c16;
#[path="../../peripherals/gdcrc491c92d0_v1.rs"] pub mod gdcrc491c92d0;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddac523caf1a_v1.rs"] pub mod gddac523caf1a;
#[path="../../peripherals/gddbgde2c0489_v1.rs"] pub mod gddbgde2c0489;
#[path="../../peripherals/gddcid3cb6fbe_v1.rs"] pub mod gddcid3cb6fbe;
#[path="../../peripherals/gddmaf3ee856f_v1.rs"] pub mod gddmaf3ee856f;
#[path="../../peripherals/gddmamux489b88ce_v1.rs"] pub mod gddmamux489b88ce;
#[path="../../peripherals/gddmamuxrgchxcfgbase95458840_v1.rs"] pub mod gddmamuxrgchxcfgbase95458840;
#[path="../../peripherals/gddmamuxrmchxcfgbase3c558169_v1.rs"] pub mod gddmamuxrmchxcfgbase3c558169;
#[path="../../peripherals/gddsi76441603_v1.rs"] pub mod gddsi76441603;
#[path="../../peripherals/gdedimafmtc755000c_v1.rs"] pub mod gdedimafmtc755000c;
#[path="../../peripherals/gdedimbiss1ab60e08_v1.rs"] pub mod gdedimbiss1ab60e08;
#[path="../../peripherals/gdedimendat15ceb0e5_v1.rs"] pub mod gdedimendat15ceb0e5;
#[path="../../peripherals/gdedimhdsl8f02a454_v1.rs"] pub mod gdedimhdsl8f02a454;
#[path="../../peripherals/gdedimtfmt3eab6256_v1.rs"] pub mod gdedimtfmt3eab6256;
#[path="../../peripherals/gdedout6974cab7_v1.rs"] pub mod gdedout6974cab7;
#[path="../../peripherals/gdenet38297eab_v1.rs"] pub mod gdenet38297eab;
#[path="../../peripherals/gdescd7927785_v1.rs"] pub mod gdescd7927785;
#[path="../../peripherals/gdexmc2e387b8a_v1.rs"] pub mod gdexmc2e387b8a;
#[path="../../peripherals/gdexti049f6534_v1.rs"] pub mod gdexti049f6534;
#[path="../../peripherals/gdfac70f604b5_v1.rs"] pub mod gdfac70f604b5;
#[path="../../peripherals/gdfftc3717816_v1.rs"] pub mod gdfftc3717816;
#[path="../../peripherals/gdfwdgtc7bc9588_v1.rs"] pub mod gdfwdgtc7bc9588;
#[path="../../peripherals/gdgpio9a89eec9_v1.rs"] pub mod gdgpio9a89eec9;
#[path="../../peripherals/gdgpsi8a02f8c0_v1.rs"] pub mod gdgpsi8a02f8c0;
#[path="../../peripherals/gdhau7c50811e_v1.rs"] pub mod gdhau7c50811e;
#[path="../../peripherals/gdhpdffe1368b3_v1.rs"] pub mod gdhpdffe1368b3;
#[path="../../peripherals/gdhwsem7ea96c46_v1.rs"] pub mod gdhwsem7ea96c46;
#[path="../../peripherals/gdi2c4e73acde_v1.rs"] pub mod gdi2c4e73acde;
#[path="../../peripherals/gdipa61f09f1e_v1.rs"] pub mod gdipa61f09f1e;
#[path="../../peripherals/gdlpdts3307f0fd_v1.rs"] pub mod gdlpdts3307f0fd;
#[path="../../peripherals/gdmdio596b2b27_v1.rs"] pub mod gdmdio596b2b27;
#[path="../../peripherals/gdmdma7993bd09_v1.rs"] pub mod gdmdma7993bd09;
#[path="../../peripherals/gdnvmc9a5d07ef_v1.rs"] pub mod gdnvmc9a5d07ef;
#[path="../../peripherals/gdospi71203c17_v1.rs"] pub mod gdospi71203c17;
#[path="../../peripherals/gdospimad46f02a_v1.rs"] pub mod gdospimad46f02a;
#[path="../../peripherals/gdpkcauf9e1d63d_v1.rs"] pub mod gdpkcauf9e1d63d;
#[path="../../peripherals/gdpmu64058cb9_v1.rs"] pub mod gdpmu64058cb9;
#[path="../../peripherals/gdrameccmu05ad7d731_v1.rs"] pub mod gdrameccmu05ad7d731;
#[path="../../peripherals/gdrameccmu8cae911f_v1.rs"] pub mod gdrameccmu8cae911f;
#[path="../../peripherals/gdrcu0591d20f_v1.rs"] pub mod gdrcu0591d20f;
#[path="../../peripherals/gdrdcme04ef535_v1.rs"] pub mod gdrdcme04ef535;
#[path="../../peripherals/gdrspdif22daf900_v1.rs"] pub mod gdrspdif22daf900;
#[path="../../peripherals/gdrtc4f2c5d98_v1.rs"] pub mod gdrtc4f2c5d98;
#[path="../../peripherals/gdrtdeca2be341e_v1.rs"] pub mod gdrtdeca2be341e;
#[path="../../peripherals/gdsai81edec52_v1.rs"] pub mod gdsai81edec52;
#[path="../../peripherals/gdsdiobd66f984_v1.rs"] pub mod gdsdiobd66f984;
#[path="../../peripherals/gdspia2feaee7_v1.rs"] pub mod gdspia2feaee7;
#[path="../../peripherals/gdsyscfgd19ad75d_v1.rs"] pub mod gdsyscfgd19ad75d;
#[path="../../peripherals/gdtimereb2fc375_v1.rs"] pub mod gdtimereb2fc375;
#[path="../../peripherals/gdtli1ba8a397_v1.rs"] pub mod gdtli1ba8a397;
#[path="../../peripherals/gdtmu6e5ec85c_v1.rs"] pub mod gdtmu6e5ec85c;
#[path="../../peripherals/gdtrigselc40c9296_v1.rs"] pub mod gdtrigselc40c9296;
#[path="../../peripherals/gdtrng89e226cf_v1.rs"] pub mod gdtrng89e226cf;
#[path="../../peripherals/gdusarte31b633e_v1.rs"] pub mod gdusarte31b633e;
#[path="../../peripherals/gdvref779f5a9e_v1.rs"] pub mod gdvref779f5a9e;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
