

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
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
ADC0_1 = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI_LINE5_9"]
EXTI_LINE5_9 = 23 , # [doc = "24 - TIMER0_BRK_TIMER8"]
TIMER0_BRK_TIMER8 = 24 , # [doc = "25 - TIMER0_UP_TIMER9"]
TIMER0_UP_TIMER9 = 25 , # [doc = "26 - TIMER0_TRG_CMT_TIMER10"]
TIMER0_TRG_CMT_TIMER10 = 26 , # [doc = "27 - TIMER0_CHANNEL"]
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
USART2 = 39 , # [doc = "40 - EXTI_LINE10_15"]
EXTI_LINE10_15 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "48 - EXMC"]
EXMC = 48 , # [doc = "49 - SDIO"]
SDIO = 49 , # [doc = "50 - TIMER4"]
TIMER4 = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "55 - TIMER6"]
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
CAN1_EWMC = 66 , # [doc = "67 - USBFS"]
USBFS = 67 , # [doc = "71 - USART5"]
USART5 = 71 , # [doc = "72 - I2C2_EV"]
I2C2_EV = 72 , # [doc = "73 - I2C2_ER"]
I2C2_ER = 73 , # [doc = "78 - DCI"]
DCI = 78 , # [doc = "79 - CAU"]
CAU = 79 , # [doc = "80 - HAU_TRNG"]
HAU_TRNG = 80 , # [doc = "82 - UART6"]
UART6 = 82 , # [doc = "83 - UART7"]
UART7 = 83 , # [doc = "88 - TLI"]
TLI = 88 , # [doc = "89 - TLI_ER"]
TLI_ER = 89 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE10_15 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn USBFS () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn DCI () ; fn CAU () ; fn HAU_TRNG () ; fn UART6 () ; fn UART7 () ; fn TLI () ; fn TLI_ER () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 90]
= [Vector { _handler : WWDGT } , Vector { _reserved : 0 } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : USBFS } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : DCI } , Vector { _handler : CAU } , Vector { _handler : HAU_TRNG } , Vector { _reserved : 0 } , Vector { _handler : UART6 } , Vector { _handler : UART7 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TLI } , Vector { _handler : TLI_ER } ,]
; } pub const TIMER1 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer1974d22f3 :: Timer1 = unsafe { gdtimer1974d22f3 :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer55eec4d84 :: Timer5 = unsafe { gdtimer55eec4d84 :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer55eec4d84 :: Timer5 = unsafe { gdtimer55eec4d84 :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer865e4b21d :: Timer8 = unsafe { gdtimer865e4b21d :: Timer8 :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer9c6ee1d55 :: Timer9 = unsafe { gdtimer9c6ee1d55 :: Timer9 :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer9c6ee1d55 :: Timer9 = unsafe { gdtimer9c6ee1d55 :: Timer9 :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtc6b0c077c :: Rtc = unsafe { gdrtc6b0c077c :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt50884229 :: Wwdgt = unsafe { gdwwdgt50884229 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtdc3d0d7a :: Fwdgt = unsafe { gdfwdgtdc3d0d7a :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi09ee016b2 :: Spi0 = unsafe { gdspi09ee016b2 :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi09ee016b2 :: Spi0 = unsafe { gdspi09ee016b2 :: Spi0 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart0677bab67 :: Usart0 = unsafe { gdusart0677bab67 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart0677bab67 :: Usart0 = unsafe { gdusart0677bab67 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart36dbe0a8a :: Uart3 = unsafe { gduart36dbe0a8a :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart36dbe0a8a :: Uart3 = unsafe { gduart36dbe0a8a :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0700b93ad :: I2c0 = unsafe { gdi2c0700b93ad :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0700b93ad :: I2c0 = unsafe { gdi2c0700b93ad :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const CAN0 : gdcan01d9ed235 :: Can0 = unsafe { gdcan01d9ed235 :: Can0 :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan01d9ed235 :: Can0 = unsafe { gdcan01d9ed235 :: Can0 :: from_ptr (0x4000_6800usize as _) } ; pub const BKP : gdbkpb0abe2e5 :: Bkp = unsafe { gdbkpb0abe2e5 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu0a98243f :: Pmu = unsafe { gdpmu0a98243f :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac9551a4ef :: Dac = unsafe { gddac9551a4ef :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gduart36dbe0a8a :: Uart3 = unsafe { gduart36dbe0a8a :: Uart3 :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gduart36dbe0a8a :: Uart3 = unsafe { gduart36dbe0a8a :: Uart3 :: from_ptr (0x4000_7c00usize as _) } ; pub const I2C2 : gdi2c0700b93ad :: I2c0 = unsafe { gdi2c0700b93ad :: I2c0 :: from_ptr (0x4000_c000usize as _) } ; pub const AFIO : gdafio0d83dbd7 :: Afio = unsafe { gdafio0d83dbd7 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti11a1be47 :: Exti = unsafe { gdexti11a1be47 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1800usize as _) } ; pub const GPIOF : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_1c00usize as _) } ; pub const GPIOG : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_2000usize as _) } ; pub const ADC0 : gdadc059fb2391 :: Adc0 = unsafe { gdadc059fb2391 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc059fb2391 :: Adc0 = unsafe { gdadc059fb2391 :: Adc0 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0f10fcbf6 :: Timer0 = unsafe { gdtimer0f10fcbf6 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi09ee016b2 :: Spi0 = unsafe { gdspi09ee016b2 :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer0f10fcbf6 :: Timer0 = unsafe { gdtimer0f10fcbf6 :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart0677bab67 :: Usart0 = unsafe { gdusart0677bab67 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadc059fb2391 :: Adc0 = unsafe { gdadc059fb2391 :: Adc0 :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimer865e4b21d :: Timer8 = unsafe { gdtimer865e4b21d :: Timer8 :: from_ptr (0x4001_4c00usize as _) } ; pub const TIMER9 : gdtimer9c6ee1d55 :: Timer9 = unsafe { gdtimer9c6ee1d55 :: Timer9 :: from_ptr (0x4001_5000usize as _) } ; pub const TIMER10 : gdtimer9c6ee1d55 :: Timer9 = unsafe { gdtimer9c6ee1d55 :: Timer9 :: from_ptr (0x4001_5400usize as _) } ; pub const TLI : gdtli89ae65d5 :: Tli = unsafe { gdtli89ae65d5 :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const USART5 : gdusart0677bab67 :: Usart0 = unsafe { gdusart0677bab67 :: Usart0 :: from_ptr (0x4001_7000usize as _) } ; pub const GPIOH : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_7400usize as _) } ; pub const GPIOI : gdgpioa979b0f67 :: Gpioa = unsafe { gdgpioa979b0f67 :: Gpioa :: from_ptr (0x4001_7800usize as _) } ; pub const SDIO : gdsdio299e3279 :: Sdio = unsafe { gdsdio299e3279 :: Sdio :: from_ptr (0x4001_8000usize as _) } ; pub const DMA0 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma1517575c6 :: Dma1 = unsafe { gddma1517575c6 :: Dma1 :: from_ptr (0x4002_0400usize as _) } ; pub const RCU : gdrcu29040b39 :: Rcu = unsafe { gdrcu29040b39 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmcd52b6db1 :: Fmc = unsafe { gdfmcd52b6db1 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc5c562b59 :: Crc = unsafe { gdcrc5c562b59 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const ENET_MAC : gdenetmac09c072f4 :: EnetMac = unsafe { gdenetmac09c072f4 :: EnetMac :: from_ptr (0x4002_8000usize as _) } ; pub const ENET_MSC : gdenetmsc9217fdbd :: EnetMsc = unsafe { gdenetmsc9217fdbd :: EnetMsc :: from_ptr (0x4002_8100usize as _) } ; pub const ENET_PTP : gdenetptpf491bb9d :: EnetPtp = unsafe { gdenetptpf491bb9d :: EnetPtp :: from_ptr (0x4002_8700usize as _) } ; pub const ENET_DMA : gdenetdmacba9250b :: EnetDma = unsafe { gdenetdmacba9250b :: EnetDma :: from_ptr (0x4002_9000usize as _) } ; pub const USBFS_GLOBAL : gdusbfsglobal019fa48e :: UsbfsGlobal = unsafe { gdusbfsglobal019fa48e :: UsbfsGlobal :: from_ptr (0x5000_0000usize as _) } ; pub const USBFS_HOST : gdusbfshost5f42a79e :: UsbfsHost = unsafe { gdusbfshost5f42a79e :: UsbfsHost :: from_ptr (0x5000_0400usize as _) } ; pub const USBFS_DEVICE : gdusbfsdevicea4903788 :: UsbfsDevice = unsafe { gdusbfsdevicea4903788 :: UsbfsDevice :: from_ptr (0x5000_0800usize as _) } ; pub const USBFS_PWRCLK : gdusbfspwrclk2ac667f0 :: UsbfsPwrclk = unsafe { gdusbfspwrclk2ac667f0 :: UsbfsPwrclk :: from_ptr (0x5000_0e00usize as _) } ; pub const DCI : gddcia70582ff :: Dci = unsafe { gddcia70582ff :: Dci :: from_ptr (0x5005_0000usize as _) } ; pub const CAU : gdcau1d48f570 :: Cau = unsafe { gdcau1d48f570 :: Cau :: from_ptr (0x5006_0000usize as _) } ; pub const HAU : gdhau67f15641 :: Hau = unsafe { gdhau67f15641 :: Hau :: from_ptr (0x5006_0400usize as _) } ; pub const TRNG : gdtrngbf61c352 :: Trng = unsafe { gdtrngbf61c352 :: Trng :: from_ptr (0x5006_0800usize as _) } ; pub const EXMC : gdexmcb6dcdf27 :: Exmc = unsafe { gdexmcb6dcdf27 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbgb21f1063 :: Dbg = unsafe { gddbgb21f1063 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc059fb2391_v1.rs"] pub mod gdadc059fb2391;
#[path="../../peripherals/gdafio0d83dbd7_v1.rs"] pub mod gdafio0d83dbd7;
#[path="../../peripherals/gdbkpb0abe2e5_v1.rs"] pub mod gdbkpb0abe2e5;
#[path="../../peripherals/gdcan01d9ed235_v1.rs"] pub mod gdcan01d9ed235;
#[path="../../peripherals/gdcau1d48f570_v1.rs"] pub mod gdcau1d48f570;
#[path="../../peripherals/gdcrc5c562b59_v1.rs"] pub mod gdcrc5c562b59;
#[path="../../peripherals/gddac9551a4ef_v1.rs"] pub mod gddac9551a4ef;
#[path="../../peripherals/gddbgb21f1063_v1.rs"] pub mod gddbgb21f1063;
#[path="../../peripherals/gddcia70582ff_v1.rs"] pub mod gddcia70582ff;
#[path="../../peripherals/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../../peripherals/gddma1517575c6_v1.rs"] pub mod gddma1517575c6;
#[path="../../peripherals/gdenetdmacba9250b_v1.rs"] pub mod gdenetdmacba9250b;
#[path="../../peripherals/gdenetmac09c072f4_v1.rs"] pub mod gdenetmac09c072f4;
#[path="../../peripherals/gdenetmsc9217fdbd_v1.rs"] pub mod gdenetmsc9217fdbd;
#[path="../../peripherals/gdenetptpf491bb9d_v1.rs"] pub mod gdenetptpf491bb9d;
#[path="../../peripherals/gdexmcb6dcdf27_v1.rs"] pub mod gdexmcb6dcdf27;
#[path="../../peripherals/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../../peripherals/gdfmcd52b6db1_v1.rs"] pub mod gdfmcd52b6db1;
#[path="../../peripherals/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../../peripherals/gdgpioa979b0f67_v1.rs"] pub mod gdgpioa979b0f67;
#[path="../../peripherals/gdhau67f15641_v1.rs"] pub mod gdhau67f15641;
#[path="../../peripherals/gdi2c0700b93ad_v1.rs"] pub mod gdi2c0700b93ad;
#[path="../../peripherals/gdpmu0a98243f_v1.rs"] pub mod gdpmu0a98243f;
#[path="../../peripherals/gdrcu29040b39_v1.rs"] pub mod gdrcu29040b39;
#[path="../../peripherals/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../../peripherals/gdsdio299e3279_v1.rs"] pub mod gdsdio299e3279;
#[path="../../peripherals/gdspi09ee016b2_v1.rs"] pub mod gdspi09ee016b2;
#[path="../../peripherals/gdtimer0f10fcbf6_v1.rs"] pub mod gdtimer0f10fcbf6;
#[path="../../peripherals/gdtimer1974d22f3_v1.rs"] pub mod gdtimer1974d22f3;
#[path="../../peripherals/gdtimer55eec4d84_v1.rs"] pub mod gdtimer55eec4d84;
#[path="../../peripherals/gdtimer865e4b21d_v1.rs"] pub mod gdtimer865e4b21d;
#[path="../../peripherals/gdtimer9c6ee1d55_v1.rs"] pub mod gdtimer9c6ee1d55;
#[path="../../peripherals/gdtli89ae65d5_v1.rs"] pub mod gdtli89ae65d5;
#[path="../../peripherals/gdtrngbf61c352_v1.rs"] pub mod gdtrngbf61c352;
#[path="../../peripherals/gduart36dbe0a8a_v1.rs"] pub mod gduart36dbe0a8a;
#[path="../../peripherals/gdusart0677bab67_v1.rs"] pub mod gdusart0677bab67;
#[path="../../peripherals/gdusbfsdevicea4903788_v1.rs"] pub mod gdusbfsdevicea4903788;
#[path="../../peripherals/gdusbfsglobal019fa48e_v1.rs"] pub mod gdusbfsglobal019fa48e;
#[path="../../peripherals/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../../peripherals/gdwwdgt50884229_v1.rs"] pub mod gdwwdgt50884229;
