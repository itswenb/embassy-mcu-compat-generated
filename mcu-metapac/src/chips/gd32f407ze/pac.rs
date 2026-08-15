

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - RTC_TAMPER"]
RTC_TAMPER = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI_LINE0"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC"]
ADC = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK_TIMER8"]
TIMER0_BRK_TIMER8 = 24 , # [doc = "25 - TIMER0_UP_TIMER9"]
TIMER0_UP_TIMER9 = 25 , # [doc = "26 - TIMER0_TRG_CMT_TIMER10"]
TIMER0_TRG_CMT_TIMER10 = 26 , # [doc = "27 - TIMER0_CC"]
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
RTC_ALARM = 41 , # [doc = "42 - OTG_FS_WKUP"]
OTG_FS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CC"]
TIMER7_CC = 46 , # [doc = "47 - DMA0_CHANNEL7"]
DMA0_CHANNEL7 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "49 - SDIO"]
SDIO = 49 , # [doc = "50 - TIMER4"]
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
DMA1_CHANNEL4 = 60 , # [doc = "61 - ENET"]
ENET = 61 , # [doc = "62 - ENET_WKUP"]
ENET_WKUP = 62 , # [doc = "63 - CAN1_TX"]
CAN1_TX = 63 , # [doc = "64 - CAN1_RX0"]
CAN1_RX0 = 64 , # [doc = "65 - CAN1_RX1"]
CAN1_RX1 = 65 , # [doc = "66 - CAN1_EWMC"]
CAN1_EWMC = 66 , # [doc = "67 - OTG_FS"]
OTG_FS = 67 , # [doc = "68 - DMA1_CHANNEL5"]
DMA1_CHANNEL5 = 68 , # [doc = "69 - DMA1_CHANNEL6"]
DMA1_CHANNEL6 = 69 , # [doc = "70 - DMA1_CHANNEL7"]
DMA1_CHANNEL7 = 70 , # [doc = "71 - USART5"]
USART5 = 71 , # [doc = "72 - I2C2_EV"]
I2C2_EV = 72 , # [doc = "73 - I2C2_ER"]
I2C2_ER = 73 , # [doc = "74 - USBHS_EP1_OUT"]
USBHS_EP1_OUT = 74 , # [doc = "75 - USBHS_EP1_IN"]
USBHS_EP1_IN = 75 , # [doc = "76 - USBHS_WKUP"]
USBHS_WKUP = 76 , # [doc = "77 - USBHS"]
USBHS = 77 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "80 - TRNG"]
TRNG = 80 , # [doc = "81 - FPU"]
FPU = 81 , # [doc = "82 - UART6"]
UART6 = 82 , # [doc = "83 - UART7"]
UART7 = 83 , # [doc = "84 - SPI3"]
SPI3 = 84 , # [doc = "85 - SPI4"]
SPI4 = 85 , # [doc = "86 - SPI5"]
SPI5 = 86 , # [doc = "88 - TLI"]
TLI = 88 , # [doc = "89 - TLI_ER"]
TLI_ER = 89 , # [doc = "90 - IPA"]
IPA = 90 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn RTC_TAMPER () ; fn RTC_WKUP () ; fn FMC () ; fn RCU_CTC () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn OTG_FS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CC () ; fn DMA0_CHANNEL7 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn OTG_FS () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBHS_EP1_OUT () ; fn USBHS_EP1_IN () ; fn USBHS_WKUP () ; fn USBHS () ; fn DCI () ; fn TRNG () ; fn FPU () ; fn UART6 () ; fn UART7 () ; fn SPI3 () ; fn SPI4 () ; fn SPI5 () ; fn TLI () ; fn TLI_ER () ; fn IPA () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 91]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : RTC_TAMPER } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : OTG_FS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CC } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : OTG_FS } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBHS_EP1_OUT } , Vector { _handler : USBHS_EP1_IN } , Vector { _handler : USBHS_WKUP } , Vector { _handler : USBHS } , Vector { _handler : DCI } , Vector { _reserved : 0 } , Vector { _handler : TRNG } , Vector { _handler : FPU } , Vector { _handler : UART6 } , Vector { _handler : UART7 } , Vector { _handler : SPI3 } , Vector { _handler : SPI4 } , Vector { _handler : SPI5 } , Vector { _reserved : 0 } , Vector { _handler : TLI } , Vector { _handler : TLI_ER } , Vector { _handler : IPA } ,]
; } pub const TIMER1 : gdtimer1f33d033d :: Timer1 = unsafe { gdtimer1f33d033d :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer1f33d033d :: Timer1 = unsafe { gdtimer1f33d033d :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer1f33d033d :: Timer1 = unsafe { gdtimer1f33d033d :: Timer1 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer1f33d033d :: Timer1 = unsafe { gdtimer1f33d033d :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer519fda6d7 :: Timer5 = unsafe { gdtimer519fda6d7 :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer519fda6d7 :: Timer5 = unsafe { gdtimer519fda6d7 :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer895e47fd0 :: Timer8 = unsafe { gdtimer895e47fd0 :: Timer8 :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer9b6e04d24 :: Timer9 = unsafe { gdtimer9b6e04d24 :: Timer9 :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer9b6e04d24 :: Timer9 = unsafe { gdtimer9b6e04d24 :: Timer9 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc34bd68c7 :: Rtc = unsafe { gdrtc34bd68c7 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtb5a65d35 :: Fwdgt = unsafe { gdfwdgtb5a65d35 :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4000_3c00usize as _) } ; pub const I2S2_ADD : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4000_4000usize as _) } ; pub const USART1 : gdusart06fc75967 :: Usart0 = unsafe { gdusart06fc75967 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart06fc75967 :: Usart0 = unsafe { gdusart06fc75967 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart38ecaf091 :: Uart3 = unsafe { gduart38ecaf091 :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart38ecaf091 :: Uart3 = unsafe { gduart38ecaf091 :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0116537ab :: I2c0 = unsafe { gdi2c0116537ab :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0116537ab :: I2c0 = unsafe { gdi2c0116537ab :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const I2C2 : gdi2c0116537ab :: I2c0 = unsafe { gdi2c0116537ab :: I2c0 :: from_ptr (0x4000_5c00usize as _) } ; pub const CAN0 : gdcan06b36baa3 :: Can0 = unsafe { gdcan06b36baa3 :: Can0 :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan06b36baa3 :: Can0 = unsafe { gdcan06b36baa3 :: Can0 :: from_ptr (0x4000_6800usize as _) } ; pub const CTC : gdctc47444a2c :: Ctc = unsafe { gdctc47444a2c :: Ctc :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu52565809 :: Pmu = unsafe { gdpmu52565809 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac4f1e7381 :: Dac = unsafe { gddac4f1e7381 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gduart38ecaf091 :: Uart3 = unsafe { gduart38ecaf091 :: Uart3 :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gduart38ecaf091 :: Uart3 = unsafe { gduart38ecaf091 :: Uart3 :: from_ptr (0x4000_7c00usize as _) } ; pub const IREF : gdiref361590d6 :: Iref = unsafe { gdiref361590d6 :: Iref :: from_ptr (0x4000_c400usize as _) } ; pub const TIMER0 : gdtimer0ac749699 :: Timer0 = unsafe { gdtimer0ac749699 :: Timer0 :: from_ptr (0x4001_0000usize as _) } ; pub const TIMER7 : gdtimer0ac749699 :: Timer0 = unsafe { gdtimer0ac749699 :: Timer0 :: from_ptr (0x4001_0400usize as _) } ; pub const USART0 : gdusart06fc75967 :: Usart0 = unsafe { gdusart06fc75967 :: Usart0 :: from_ptr (0x4001_1000usize as _) } ; pub const USART5 : gdusart06fc75967 :: Usart0 = unsafe { gdusart06fc75967 :: Usart0 :: from_ptr (0x4001_1400usize as _) } ; pub const ADC0 : gdadc0644c59d8 :: Adc0 = unsafe { gdadc0644c59d8 :: Adc0 :: from_ptr (0x4001_2000usize as _) } ; pub const ADC1 : gdadc0644c59d8 :: Adc0 = unsafe { gdadc0644c59d8 :: Adc0 :: from_ptr (0x4001_2100usize as _) } ; pub const ADC2 : gdadc0644c59d8 :: Adc0 = unsafe { gdadc0644c59d8 :: Adc0 :: from_ptr (0x4001_2200usize as _) } ; pub const ADC_COMMON : gdadccommon6f53c1c8 :: AdcCommon = unsafe { gdadccommon6f53c1c8 :: AdcCommon :: from_ptr (0x4001_2300usize as _) } ; pub const SDIO : gdsdio6b548c0d :: Sdio = unsafe { gdsdio6b548c0d :: Sdio :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const SPI3 : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4001_3400usize as _) } ; pub const SYSCFG : gdsyscfg417a49de :: Syscfg = unsafe { gdsyscfg417a49de :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdexti2861ec2a :: Exti = unsafe { gdexti2861ec2a :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimer895e47fd0 :: Timer8 = unsafe { gdtimer895e47fd0 :: Timer8 :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER9 : gdtimer9b6e04d24 :: Timer9 = unsafe { gdtimer9b6e04d24 :: Timer9 :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER10 : gdtimer9b6e04d24 :: Timer9 = unsafe { gdtimer9b6e04d24 :: Timer9 :: from_ptr (0x4001_4800usize as _) } ; pub const SPI4 : gdspi0a39abaa4 :: Spi0 = unsafe { gdspi0a39abaa4 :: Spi0 :: from_ptr (0x4001_5000usize as _) } ; pub const SPI5 : gdspi528277832 :: Spi5 = unsafe { gdspi528277832 :: Spi5 :: from_ptr (0x4001_5400usize as _) } ; pub const TLI : gdtli3a8126bb :: Tli = unsafe { gdtli3a8126bb :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const GPIOA : gdgpioa54e55541 :: Gpioa = unsafe { gdgpioa54e55541 :: Gpioa :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpiob0a8ce2af :: Gpiob = unsafe { gdgpiob0a8ce2af :: Gpiob :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_0800usize as _) } ; pub const GPIOD : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_0c00usize as _) } ; pub const GPIOE : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_1000usize as _) } ; pub const GPIOF : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_1400usize as _) } ; pub const GPIOG : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_1800usize as _) } ; pub const GPIOH : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_1c00usize as _) } ; pub const GPIOI : gdgpiocc25656a9 :: Gpioc = unsafe { gdgpiocc25656a9 :: Gpioc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc66a4f78d :: Crc = unsafe { gdcrc66a4f78d :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcud76b9bb4 :: Rcu = unsafe { gdrcud76b9bb4 :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const FMC : gdfmcf2343cdc :: Fmc = unsafe { gdfmcf2343cdc :: Fmc :: from_ptr (0x4002_3c00usize as _) } ; pub const DMA0 : gddma024ec4b91 :: Dma0 = unsafe { gddma024ec4b91 :: Dma0 :: from_ptr (0x4002_6000usize as _) } ; pub const DMA1 : gddma024ec4b91 :: Dma0 = unsafe { gddma024ec4b91 :: Dma0 :: from_ptr (0x4002_6400usize as _) } ; pub const ENET_MAC : gdenetmac93552dd1 :: EnetMac = unsafe { gdenetmac93552dd1 :: EnetMac :: from_ptr (0x4002_8000usize as _) } ; pub const ENET_MSC : gdenetmsc10390666 :: EnetMsc = unsafe { gdenetmsc10390666 :: EnetMsc :: from_ptr (0x4002_8100usize as _) } ; pub const ENET_PTP : gdenetptp5c8a2d48 :: EnetPtp = unsafe { gdenetptp5c8a2d48 :: EnetPtp :: from_ptr (0x4002_8700usize as _) } ; pub const ENET_DMA : gdenetdma7fbba2f4 :: EnetDma = unsafe { gdenetdma7fbba2f4 :: EnetDma :: from_ptr (0x4002_9000usize as _) } ; pub const ENET_MAC_FCTH : gdenetmacfcth8ada9e21 :: EnetMacFcth = unsafe { gdenetmacfcth8ada9e21 :: EnetMacFcth :: from_ptr (0x4002_9080usize as _) } ; pub const IPA : gdipae676fed9 :: Ipa = unsafe { gdipae676fed9 :: Ipa :: from_ptr (0x4002_b000usize as _) } ; pub const HS_GLOBAL : gdhsglobalc406147a :: HsGlobal = unsafe { gdhsglobalc406147a :: HsGlobal :: from_ptr (0x4004_0000usize as _) } ; pub const HS_HOST : gdhshostc2377b4a :: HsHost = unsafe { gdhshostc2377b4a :: HsHost :: from_ptr (0x4004_0400usize as _) } ; pub const HS_DEVICE : gdhsdevicec9d69f15 :: HsDevice = unsafe { gdhsdevicec9d69f15 :: HsDevice :: from_ptr (0x4004_0800usize as _) } ; pub const HS_PWRCLK : gdhspwrclk9376d26f :: HsPwrclk = unsafe { gdhspwrclk9376d26f :: HsPwrclk :: from_ptr (0x4004_0e00usize as _) } ; pub const FS_GLOBAL : gdfsglobale74e6f0e :: FsGlobal = unsafe { gdfsglobale74e6f0e :: FsGlobal :: from_ptr (0x5000_0000usize as _) } ; pub const FS_HOST : gdfshost44621b1c :: FsHost = unsafe { gdfshost44621b1c :: FsHost :: from_ptr (0x5000_0400usize as _) } ; pub const FS_DEVICE : gdfsdeviceb377b28b :: FsDevice = unsafe { gdfsdeviceb377b28b :: FsDevice :: from_ptr (0x5000_0800usize as _) } ; pub const FS_PWRCLK : gdfspwrclk87dcd48b :: FsPwrclk = unsafe { gdfspwrclk87dcd48b :: FsPwrclk :: from_ptr (0x5000_0e00usize as _) } ; pub const DCI : gddci704bb188 :: Dci = unsafe { gddci704bb188 :: Dci :: from_ptr (0x5005_0000usize as _) } ; pub const TRNG : gdtrngb48807ab :: Trng = unsafe { gdtrngb48807ab :: Trng :: from_ptr (0x5006_0800usize as _) } ; pub const EXMC : gdexmc55cf46b0 :: Exmc = unsafe { gdexmc55cf46b0 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg50e0203e :: Dbg = unsafe { gddbg50e0203e :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc0644c59d8_v1.rs"] pub mod gdadc0644c59d8;
#[path="../../peripherals/gdadccommon6f53c1c8_v1.rs"] pub mod gdadccommon6f53c1c8;
#[path="../../peripherals/gdcan06b36baa3_v1.rs"] pub mod gdcan06b36baa3;
#[path="../../peripherals/gdcrc66a4f78d_v1.rs"] pub mod gdcrc66a4f78d;
#[path="../../peripherals/gdctc47444a2c_v1.rs"] pub mod gdctc47444a2c;
#[path="../../peripherals/gddac4f1e7381_v1.rs"] pub mod gddac4f1e7381;
#[path="../../peripherals/gddbg50e0203e_v1.rs"] pub mod gddbg50e0203e;
#[path="../../peripherals/gddci704bb188_v1.rs"] pub mod gddci704bb188;
#[path="../../peripherals/gddma024ec4b91_v1.rs"] pub mod gddma024ec4b91;
#[path="../../peripherals/gdenetdma7fbba2f4_v1.rs"] pub mod gdenetdma7fbba2f4;
#[path="../../peripherals/gdenetmac93552dd1_v1.rs"] pub mod gdenetmac93552dd1;
#[path="../../peripherals/gdenetmacfcth8ada9e21_v1.rs"] pub mod gdenetmacfcth8ada9e21;
#[path="../../peripherals/gdenetmsc10390666_v1.rs"] pub mod gdenetmsc10390666;
#[path="../../peripherals/gdenetptp5c8a2d48_v1.rs"] pub mod gdenetptp5c8a2d48;
#[path="../../peripherals/gdexmc55cf46b0_v1.rs"] pub mod gdexmc55cf46b0;
#[path="../../peripherals/gdexti2861ec2a_v1.rs"] pub mod gdexti2861ec2a;
#[path="../../peripherals/gdfmcf2343cdc_v1.rs"] pub mod gdfmcf2343cdc;
#[path="../../peripherals/gdfsdeviceb377b28b_v1.rs"] pub mod gdfsdeviceb377b28b;
#[path="../../peripherals/gdfsglobale74e6f0e_v1.rs"] pub mod gdfsglobale74e6f0e;
#[path="../../peripherals/gdfshost44621b1c_v1.rs"] pub mod gdfshost44621b1c;
#[path="../../peripherals/gdfspwrclk87dcd48b_v1.rs"] pub mod gdfspwrclk87dcd48b;
#[path="../../peripherals/gdfwdgtb5a65d35_v1.rs"] pub mod gdfwdgtb5a65d35;
#[path="../../peripherals/gdgpioa54e55541_v1.rs"] pub mod gdgpioa54e55541;
#[path="../../peripherals/gdgpiob0a8ce2af_v1.rs"] pub mod gdgpiob0a8ce2af;
#[path="../../peripherals/gdgpiocc25656a9_v1.rs"] pub mod gdgpiocc25656a9;
#[path="../../peripherals/gdhsdevicec9d69f15_v1.rs"] pub mod gdhsdevicec9d69f15;
#[path="../../peripherals/gdhsglobalc406147a_v1.rs"] pub mod gdhsglobalc406147a;
#[path="../../peripherals/gdhshostc2377b4a_v1.rs"] pub mod gdhshostc2377b4a;
#[path="../../peripherals/gdhspwrclk9376d26f_v1.rs"] pub mod gdhspwrclk9376d26f;
#[path="../../peripherals/gdi2c0116537ab_v1.rs"] pub mod gdi2c0116537ab;
#[path="../../peripherals/gdipae676fed9_v1.rs"] pub mod gdipae676fed9;
#[path="../../peripherals/gdiref361590d6_v1.rs"] pub mod gdiref361590d6;
#[path="../../peripherals/gdpmu52565809_v1.rs"] pub mod gdpmu52565809;
#[path="../../peripherals/gdrcud76b9bb4_v1.rs"] pub mod gdrcud76b9bb4;
#[path="../../peripherals/gdrtc34bd68c7_v1.rs"] pub mod gdrtc34bd68c7;
#[path="../../peripherals/gdsdio6b548c0d_v1.rs"] pub mod gdsdio6b548c0d;
#[path="../../peripherals/gdspi0a39abaa4_v1.rs"] pub mod gdspi0a39abaa4;
#[path="../../peripherals/gdspi528277832_v1.rs"] pub mod gdspi528277832;
#[path="../../peripherals/gdsyscfg417a49de_v1.rs"] pub mod gdsyscfg417a49de;
#[path="../../peripherals/gdtimer0ac749699_v1.rs"] pub mod gdtimer0ac749699;
#[path="../../peripherals/gdtimer1f33d033d_v1.rs"] pub mod gdtimer1f33d033d;
#[path="../../peripherals/gdtimer519fda6d7_v1.rs"] pub mod gdtimer519fda6d7;
#[path="../../peripherals/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../../peripherals/gdtimer9b6e04d24_v1.rs"] pub mod gdtimer9b6e04d24;
#[path="../../peripherals/gdtli3a8126bb_v1.rs"] pub mod gdtli3a8126bb;
#[path="../../peripherals/gdtrngb48807ab_v1.rs"] pub mod gdtrngb48807ab;
#[path="../../peripherals/gduart38ecaf091_v1.rs"] pub mod gduart38ecaf091;
#[path="../../peripherals/gdusart06fc75967_v1.rs"] pub mod gdusart06fc75967;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
