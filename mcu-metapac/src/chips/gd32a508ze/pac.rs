

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "19 - CAN0_TX"]
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
RTC_ALARM = 41 , # [doc = "42 - USBHS_WKUP"]
USBHS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CC"]
TIMER7_CC = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3_DMA1_CHANNEL4"]
DMA1_CHANNEL3_DMA1_CHANNEL4 = 59 , # [doc = "61 - ENET"]
ENET = 61 , # [doc = "62 - ENET_WKUP"]
ENET_WKUP = 62 , # [doc = "63 - CAN1_TX"]
CAN1_TX = 63 , # [doc = "64 - CAN1_RX0"]
CAN1_RX0 = 64 , # [doc = "65 - CAN1_RX1"]
CAN1_RX1 = 65 , # [doc = "66 - CAN1_EWMC"]
CAN1_EWMC = 66 , # [doc = "67 - USBHS"]
USBHS = 67 , # [doc = "69 - SHRTIMER_IRQ2"]
SHRTIMER_IRQ2 = 69 , # [doc = "70 - SHRTIMER_IRQ3"]
SHRTIMER_IRQ3 = 70 , # [doc = "71 - SHRTIMER_IRQ4"]
SHRTIMER_IRQ4 = 71 , # [doc = "72 - SHRTIMER_IRQ5"]
SHRTIMER_IRQ5 = 72 , # [doc = "73 - SHRTIMER_IRQ6"]
SHRTIMER_IRQ6 = 73 , # [doc = "76 - SHRTIMER_IRQ0"]
SHRTIMER_IRQ0 = 76 , # [doc = "77 - SHRTIMER_IRQ1"]
SHRTIMER_IRQ1 = 77 , # [doc = "78 - CAN2_TX"]
CAN2_TX = 78 , # [doc = "79 - CAN2_RX0"]
CAN2_RX0 = 79 , # [doc = "80 - CAN2_RX1"]
CAN2_RX1 = 80 , # [doc = "81 - CAN2_EWMC"]
CAN2_EWMC = 81 , # [doc = "82 - I2C2_EV"]
I2C2_EV = 82 , # [doc = "83 - I2C2_ER"]
I2C2_ER = 83 , # [doc = "84 - USART5"]
USART5 = 84 , # [doc = "85 - I2C2_WKUP"]
I2C2_WKUP = 85 , # [doc = "86 - USART5_WKUP"]
USART5_WKUP = 86 , # [doc = "87 - TMU"]
TMU = 87 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn USBHS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CC () ; fn ADC2 () ; fn EXMC () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3_DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn USBHS () ; fn SHRTIMER_IRQ2 () ; fn SHRTIMER_IRQ3 () ; fn SHRTIMER_IRQ4 () ; fn SHRTIMER_IRQ5 () ; fn SHRTIMER_IRQ6 () ; fn SHRTIMER_IRQ0 () ; fn SHRTIMER_IRQ1 () ; fn CAN2_TX () ; fn CAN2_RX0 () ; fn CAN2_RX1 () ; fn CAN2_EWMC () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USART5 () ; fn I2C2_WKUP () ; fn USART5_WKUP () ; fn TMU () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 88]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBHS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CC } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _reserved : 0 } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3_DMA1_CHANNEL4 } , Vector { _reserved : 0 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : USBHS } , Vector { _reserved : 0 } , Vector { _handler : SHRTIMER_IRQ2 } , Vector { _handler : SHRTIMER_IRQ3 } , Vector { _handler : SHRTIMER_IRQ4 } , Vector { _handler : SHRTIMER_IRQ5 } , Vector { _handler : SHRTIMER_IRQ6 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : SHRTIMER_IRQ0 } , Vector { _handler : SHRTIMER_IRQ1 } , Vector { _handler : CAN2_TX } , Vector { _handler : CAN2_RX0 } , Vector { _handler : CAN2_RX1 } , Vector { _handler : CAN2_EWMC } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USART5 } , Vector { _handler : I2C2_WKUP } , Vector { _handler : USART5_WKUP } , Vector { _handler : TMU } ,]
; } pub const TIMER1 : gdtimer11e77ba65 :: Timer1 = unsafe { gdtimer11e77ba65 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer2868720fc :: Timer2 = unsafe { gdtimer2868720fc :: Timer2 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer3b6308935 :: Timer3 = unsafe { gdtimer3b6308935 :: Timer3 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer456047876 :: Timer4 = unsafe { gdtimer456047876 :: Timer4 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer642c906a4 :: Timer6 = unsafe { gdtimer642c906a4 :: Timer6 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer118878f54b :: Timer11 = unsafe { gdtimer118878f54b :: Timer11 :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer12119fcec0 :: Timer12 = unsafe { gdtimer12119fcec0 :: Timer12 :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer13e079e34e :: Timer13 = unsafe { gdtimer13e079e34e :: Timer13 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc6b0c077c :: Rtc = unsafe { gdrtc6b0c077c :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtdc3d0d7a :: Fwdgt = unsafe { gdfwdgtdc3d0d7a :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi18ad3a9b8 :: Spi1 = unsafe { gdspi18ad3a9b8 :: Spi1 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi2541b1531 :: Spi2 = unsafe { gdspi2541b1531 :: Spi2 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart1c03e4b14 :: Usart1 = unsafe { gdusart1c03e4b14 :: Usart1 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart2719a6075 :: Usart2 = unsafe { gdusart2719a6075 :: Usart2 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart3a8b9d61c :: Uart3 = unsafe { gduart3a8b9d61c :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart421c89746 :: Uart4 = unsafe { gduart421c89746 :: Uart4 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0fc829b2b :: I2c0 = unsafe { gdi2c0fc829b2b :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c1c9f3d155 :: I2c1 = unsafe { gdi2c1c9f3d155 :: I2c1 :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcan0b8705c1f :: Can0 = unsafe { gdcan0b8705c1f :: Can0 :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan1ab0668da :: Can1 = unsafe { gdcan1ab0668da :: Can1 :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkp9347e01b :: Bkp = unsafe { gdbkp9347e01b :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmuce6d9d5f :: Pmu = unsafe { gdpmuce6d9d5f :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddace9519f30 :: Dac = unsafe { gddace9519f30 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const I2C2 : gdi2c2c47a3cd3 :: I2c2 = unsafe { gdi2c2c47a3cd3 :: I2c2 :: from_ptr (0x4000_c000usize as _) } ; pub const CTC : gdctc0cad8643 :: Ctc = unsafe { gdctc0cad8643 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const CAN2 : gdcan2368703f4 :: Can2 = unsafe { gdcan2368703f4 :: Can2 :: from_ptr (0x4000_cc00usize as _) } ; pub const AFIO : gdafio79140b31 :: Afio = unsafe { gdafio79140b31 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti285c938f :: Exti = unsafe { gdexti285c938f :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioac3e5c224 :: Gpioa = unsafe { gdgpioac3e5c224 :: Gpioa :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpiob48ef64a7 :: Gpiob = unsafe { gdgpiob48ef64a7 :: Gpiob :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpiocbac6a6b9 :: Gpioc = unsafe { gdgpiocbac6a6b9 :: Gpioc :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpiod0082ea0a :: Gpiod = unsafe { gdgpiod0082ea0a :: Gpiod :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioebd4c6204 :: Gpioe = unsafe { gdgpioebd4c6204 :: Gpioe :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpiofc46221fa :: Gpiof = unsafe { gdgpiofc46221fa :: Gpiof :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpiogc07baa36 :: Gpiog = unsafe { gdgpiogc07baa36 :: Gpiog :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadc0eac10ba1 :: Adc0 = unsafe { gdadc0eac10ba1 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc1a7ac49f7 :: Adc1 = unsafe { gdadc1a7ac49f7 :: Adc1 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0a0aa2af0 :: Timer0 = unsafe { gdtimer0a0aa2af0 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi03c62ee5d :: Spi0 = unsafe { gdspi03c62ee5d :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer7b78318c5 :: Timer7 = unsafe { gdtimer7b78318c5 :: Timer7 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart016d80f16 :: Usart0 = unsafe { gdusart016d80f16 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadc2fc8b862c :: Adc2 = unsafe { gdadc2fc8b862c :: Adc2 :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimer895e47fd0 :: Timer8 = unsafe { gdtimer895e47fd0 :: Timer8 :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimer94ea426f7 :: Timer9 = unsafe { gdtimer94ea426f7 :: Timer9 :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimer10ba08925f :: Timer10 = unsafe { gdtimer10ba08925f :: Timer10 :: from_ptr (0x4001_5400usize as _) } ; pub const USART5 : gdusart55c097497 :: Usart5 = unsafe { gdusart55c097497 :: Usart5 :: from_ptr (0x4001_7000usize as _) } ; pub const MASTER_TIMER : gdmastertimerc379fdf1 :: MasterTimer = unsafe { gdmastertimerc379fdf1 :: MasterTimer :: from_ptr (0x4001_7400usize as _) } ; pub const SLAVE_TIMER0 : gdslavetimer01ad417fe :: SlaveTimer0 = unsafe { gdslavetimer01ad417fe :: SlaveTimer0 :: from_ptr (0x4001_7480usize as _) } ; pub const SLAVE_TIMER1 : gdslavetimer105c1946a :: SlaveTimer1 = unsafe { gdslavetimer105c1946a :: SlaveTimer1 :: from_ptr (0x4001_7500usize as _) } ; pub const SLAVE_TIMER2 : gdslavetimer249c017c7 :: SlaveTimer2 = unsafe { gdslavetimer249c017c7 :: SlaveTimer2 :: from_ptr (0x4001_7580usize as _) } ; pub const SLAVE_TIMER3 : gdslavetimer3b64bedd3 :: SlaveTimer3 = unsafe { gdslavetimer3b64bedd3 :: SlaveTimer3 :: from_ptr (0x4001_7600usize as _) } ; pub const SLAVE_TIMER4 : gdslavetimer45a69fbbd :: SlaveTimer4 = unsafe { gdslavetimer45a69fbbd :: SlaveTimer4 :: from_ptr (0x4001_7680usize as _) } ; pub const SHRTIMER_COMMON : gdshrtimercommon3523955c :: ShrtimerCommon = unsafe { gdshrtimercommon3523955c :: ShrtimerCommon :: from_ptr (0x4001_7780usize as _) } ; pub const CMP : gdcmp49f0325b :: Cmp = unsafe { gdcmp49f0325b :: Cmp :: from_ptr (0x4001_7c00usize as _) } ; pub const DMA0 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma13e943824 :: Dma1 = unsafe { gddma13e943824 :: Dma1 :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu325d5693 :: Rcu = unsafe { gdrcu325d5693 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmce1c61199 :: Fmc = unsafe { gdfmce1c61199 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc2255b0ef :: Crc = unsafe { gdcrc2255b0ef :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET_MAC : gdenetmac93552dd1 :: EnetMac = unsafe { gdenetmac93552dd1 :: EnetMac :: from_ptr (0x4002_8000usize as _) } ; pub const ENET_MSC : gdenetmsc10390666 :: EnetMsc = unsafe { gdenetmsc10390666 :: EnetMsc :: from_ptr (0x4002_8100usize as _) } ; pub const ENET_PTP : gdenetptp5c8a2d48 :: EnetPtp = unsafe { gdenetptp5c8a2d48 :: EnetPtp :: from_ptr (0x4002_8700usize as _) } ; pub const ENET_DMA : gdenetdma717f16f5 :: EnetDma = unsafe { gdenetdma717f16f5 :: EnetDma :: from_ptr (0x4002_9000usize as _) } ; pub const ENET_MAC_FCTH : gdenetmacfcth8ada9e21 :: EnetMacFcth = unsafe { gdenetmacfcth8ada9e21 :: EnetMacFcth :: from_ptr (0x4002_9080usize as _) } ; pub const TMU : gdtmu528d66a8 :: Tmu = unsafe { gdtmu528d66a8 :: Tmu :: from_ptr (0x4008_0000usize as _) } ; pub const USBHS_GLOBAL : gdusbhsglobalef49f048 :: UsbhsGlobal = unsafe { gdusbhsglobalef49f048 :: UsbhsGlobal :: from_ptr (0x5000_0000usize as _) } ; pub const USBGS_HOST : gdusbgshost2794baaa :: UsbgsHost = unsafe { gdusbgshost2794baaa :: UsbgsHost :: from_ptr (0x5000_0400usize as _) } ; pub const USBHS_DEVICE : gdusbhsdevicea32ae2bb :: UsbhsDevice = unsafe { gdusbhsdevicea32ae2bb :: UsbhsDevice :: from_ptr (0x5000_0800usize as _) } ; pub const USBHS_PWRCLK : gdusbhspwrclk77209260 :: UsbhsPwrclk = unsafe { gdusbhspwrclk77209260 :: UsbhsPwrclk :: from_ptr (0x5000_0e00usize as _) } ; pub const EXMC : gdexmc9f6a36f3 :: Exmc = unsafe { gdexmc9f6a36f3 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const SQPI : gdsqpi2c944cc9 :: Sqpi = unsafe { gdsqpi2c944cc9 :: Sqpi :: from_ptr (0xa000_1000usize as _) } ; pub const DBG : gddbg0aebad37 :: Dbg = unsafe { gddbg0aebad37 :: Dbg :: from_ptr (0xe004_4000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc0eac10ba1_v1.rs"] pub mod gdadc0eac10ba1;
#[path="../../peripherals/gdadc1a7ac49f7_v1.rs"] pub mod gdadc1a7ac49f7;
#[path="../../peripherals/gdadc2fc8b862c_v1.rs"] pub mod gdadc2fc8b862c;
#[path="../../peripherals/gdafio79140b31_v1.rs"] pub mod gdafio79140b31;
#[path="../../peripherals/gdbkp9347e01b_v1.rs"] pub mod gdbkp9347e01b;
#[path="../../peripherals/gdcan0b8705c1f_v1.rs"] pub mod gdcan0b8705c1f;
#[path="../../peripherals/gdcan1ab0668da_v1.rs"] pub mod gdcan1ab0668da;
#[path="../../peripherals/gdcan2368703f4_v1.rs"] pub mod gdcan2368703f4;
#[path="../../peripherals/gdcmp49f0325b_v1.rs"] pub mod gdcmp49f0325b;
#[path="../../peripherals/gdcrc2255b0ef_v1.rs"] pub mod gdcrc2255b0ef;
#[path="../../peripherals/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../../peripherals/gddace9519f30_v1.rs"] pub mod gddace9519f30;
#[path="../../peripherals/gddbg0aebad37_v1.rs"] pub mod gddbg0aebad37;
#[path="../../peripherals/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../../peripherals/gddma13e943824_v1.rs"] pub mod gddma13e943824;
#[path="../../peripherals/gdenetdma717f16f5_v1.rs"] pub mod gdenetdma717f16f5;
#[path="../../peripherals/gdenetmac93552dd1_v1.rs"] pub mod gdenetmac93552dd1;
#[path="../../peripherals/gdenetmacfcth8ada9e21_v1.rs"] pub mod gdenetmacfcth8ada9e21;
#[path="../../peripherals/gdenetmsc10390666_v1.rs"] pub mod gdenetmsc10390666;
#[path="../../peripherals/gdenetptp5c8a2d48_v1.rs"] pub mod gdenetptp5c8a2d48;
#[path="../../peripherals/gdexmc9f6a36f3_v1.rs"] pub mod gdexmc9f6a36f3;
#[path="../../peripherals/gdexti285c938f_v1.rs"] pub mod gdexti285c938f;
#[path="../../peripherals/gdfmce1c61199_v1.rs"] pub mod gdfmce1c61199;
#[path="../../peripherals/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../../peripherals/gdgpioac3e5c224_v1.rs"] pub mod gdgpioac3e5c224;
#[path="../../peripherals/gdgpiob48ef64a7_v1.rs"] pub mod gdgpiob48ef64a7;
#[path="../../peripherals/gdgpiocbac6a6b9_v1.rs"] pub mod gdgpiocbac6a6b9;
#[path="../../peripherals/gdgpiod0082ea0a_v1.rs"] pub mod gdgpiod0082ea0a;
#[path="../../peripherals/gdgpioebd4c6204_v1.rs"] pub mod gdgpioebd4c6204;
#[path="../../peripherals/gdgpiofc46221fa_v1.rs"] pub mod gdgpiofc46221fa;
#[path="../../peripherals/gdgpiogc07baa36_v1.rs"] pub mod gdgpiogc07baa36;
#[path="../../peripherals/gdi2c0fc829b2b_v1.rs"] pub mod gdi2c0fc829b2b;
#[path="../../peripherals/gdi2c1c9f3d155_v1.rs"] pub mod gdi2c1c9f3d155;
#[path="../../peripherals/gdi2c2c47a3cd3_v1.rs"] pub mod gdi2c2c47a3cd3;
#[path="../../peripherals/gdmastertimerc379fdf1_v1.rs"] pub mod gdmastertimerc379fdf1;
#[path="../../peripherals/gdpmuce6d9d5f_v1.rs"] pub mod gdpmuce6d9d5f;
#[path="../../peripherals/gdrcu325d5693_v1.rs"] pub mod gdrcu325d5693;
#[path="../../peripherals/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../../peripherals/gdshrtimercommon3523955c_v1.rs"] pub mod gdshrtimercommon3523955c;
#[path="../../peripherals/gdslavetimer01ad417fe_v1.rs"] pub mod gdslavetimer01ad417fe;
#[path="../../peripherals/gdslavetimer105c1946a_v1.rs"] pub mod gdslavetimer105c1946a;
#[path="../../peripherals/gdslavetimer249c017c7_v1.rs"] pub mod gdslavetimer249c017c7;
#[path="../../peripherals/gdslavetimer3b64bedd3_v1.rs"] pub mod gdslavetimer3b64bedd3;
#[path="../../peripherals/gdslavetimer45a69fbbd_v1.rs"] pub mod gdslavetimer45a69fbbd;
#[path="../../peripherals/gdspi03c62ee5d_v1.rs"] pub mod gdspi03c62ee5d;
#[path="../../peripherals/gdspi18ad3a9b8_v1.rs"] pub mod gdspi18ad3a9b8;
#[path="../../peripherals/gdspi2541b1531_v1.rs"] pub mod gdspi2541b1531;
#[path="../../peripherals/gdsqpi2c944cc9_v1.rs"] pub mod gdsqpi2c944cc9;
#[path="../../peripherals/gdtimer0a0aa2af0_v1.rs"] pub mod gdtimer0a0aa2af0;
#[path="../../peripherals/gdtimer10ba08925f_v1.rs"] pub mod gdtimer10ba08925f;
#[path="../../peripherals/gdtimer118878f54b_v1.rs"] pub mod gdtimer118878f54b;
#[path="../../peripherals/gdtimer11e77ba65_v1.rs"] pub mod gdtimer11e77ba65;
#[path="../../peripherals/gdtimer12119fcec0_v1.rs"] pub mod gdtimer12119fcec0;
#[path="../../peripherals/gdtimer13e079e34e_v1.rs"] pub mod gdtimer13e079e34e;
#[path="../../peripherals/gdtimer2868720fc_v1.rs"] pub mod gdtimer2868720fc;
#[path="../../peripherals/gdtimer3b6308935_v1.rs"] pub mod gdtimer3b6308935;
#[path="../../peripherals/gdtimer456047876_v1.rs"] pub mod gdtimer456047876;
#[path="../../peripherals/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../../peripherals/gdtimer642c906a4_v1.rs"] pub mod gdtimer642c906a4;
#[path="../../peripherals/gdtimer7b78318c5_v1.rs"] pub mod gdtimer7b78318c5;
#[path="../../peripherals/gdtimer895e47fd0_v1.rs"] pub mod gdtimer895e47fd0;
#[path="../../peripherals/gdtimer94ea426f7_v1.rs"] pub mod gdtimer94ea426f7;
#[path="../../peripherals/gdtmu528d66a8_v1.rs"] pub mod gdtmu528d66a8;
#[path="../../peripherals/gduart3a8b9d61c_v1.rs"] pub mod gduart3a8b9d61c;
#[path="../../peripherals/gduart421c89746_v1.rs"] pub mod gduart421c89746;
#[path="../../peripherals/gdusart016d80f16_v1.rs"] pub mod gdusart016d80f16;
#[path="../../peripherals/gdusart1c03e4b14_v1.rs"] pub mod gdusart1c03e4b14;
#[path="../../peripherals/gdusart2719a6075_v1.rs"] pub mod gdusart2719a6075;
#[path="../../peripherals/gdusart55c097497_v1.rs"] pub mod gdusart55c097497;
#[path="../../peripherals/gdusbgshost2794baaa_v1.rs"] pub mod gdusbgshost2794baaa;
#[path="../../peripherals/gdusbhsdevicea32ae2bb_v1.rs"] pub mod gdusbhsdevicea32ae2bb;
#[path="../../peripherals/gdusbhsglobalef49f048_v1.rs"] pub mod gdusbhsglobalef49f048;
#[path="../../peripherals/gdusbhspwrclk77209260_v1.rs"] pub mod gdusbhspwrclk77209260;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
