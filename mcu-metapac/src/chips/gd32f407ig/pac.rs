




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD"]
LVD = 1 , # [doc = "2 - TAMPER_STAMP"]
TAMPER_STAMP = 2 , # [doc = "3 - RTC_WKUP"]
RTC_WKUP = 3 , # [doc = "4 - FMC"]
FMC = 4 , # [doc = "5 - RCU_CTC"]
RCU_CTC = 5 , # [doc = "6 - EXTI0"]
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
ADC = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI5_9"]
EXTI5_9 = 23 , # [doc = "24 - TIMER0_BRK_TIMER8"]
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
USART2 = 39 , # [doc = "40 - EXTI10_15"]
EXTI10_15 = 40 , # [doc = "41 - RTC_ALARM"]
RTC_ALARM = 41 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - DMA0_CHANNEL7"]
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
CAN1_EWMC = 66 , # [doc = "67 - USBFS"]
USBFS = 67 , # [doc = "68 - DMA1_CHANNEL5"]
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
FPU = 81 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD () ; fn TAMPER_STAMP () ; fn RTC_WKUP () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK_TIMER8 () ; fn TIMER0_UP_TIMER9 () ; fn TIMER0_TRG_CMT_TIMER10 () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CHANNEL () ; fn DMA0_CHANNEL7 () ; fn EXMC () ; fn SDIO () ; fn TIMER4 () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5_DAC () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn ENET () ; fn ENET_WKUP () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn USBFS () ; fn DMA1_CHANNEL5 () ; fn DMA1_CHANNEL6 () ; fn DMA1_CHANNEL7 () ; fn USART5 () ; fn I2C2_EV () ; fn I2C2_ER () ; fn USBHS_EP1_OUT () ; fn USBHS_EP1_IN () ; fn USBHS_WKUP () ; fn USBHS () ; fn DCI () ; fn TRNG () ; fn FPU () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 82]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD } , Vector { _handler : TAMPER_STAMP } , Vector { _handler : RTC_WKUP } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK_TIMER8 } , Vector { _handler : TIMER0_UP_TIMER9 } , Vector { _handler : TIMER0_TRG_CMT_TIMER10 } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : DMA0_CHANNEL7 } , Vector { _handler : EXMC } , Vector { _handler : SDIO } , Vector { _handler : TIMER4 } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5_DAC } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : ENET } , Vector { _handler : ENET_WKUP } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : USBFS } , Vector { _handler : DMA1_CHANNEL5 } , Vector { _handler : DMA1_CHANNEL6 } , Vector { _handler : DMA1_CHANNEL7 } , Vector { _handler : USART5 } , Vector { _handler : I2C2_EV } , Vector { _handler : I2C2_ER } , Vector { _handler : USBHS_EP1_OUT } , Vector { _handler : USBHS_EP1_IN } , Vector { _handler : USBHS_WKUP } , Vector { _handler : USBHS } , Vector { _handler : DCI } , Vector { _reserved : 0 } , Vector { _handler : TRNG } , Vector { _handler : FPU } ,]
; } pub const OB : gdob16bd26c7 :: Ob = unsafe { gdob16bd26c7 :: Ob :: from_ptr (0x1ffe_c000usize as _) } ; pub const TIMER1 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER11 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const TIMER12 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_1c00usize as _) } ; pub const TIMER13 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4000_2000usize as _) } ; pub const RTC : gdrtcea88a5d1 :: Rtc = unsafe { gdrtcea88a5d1 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt30374593 :: Wwdgt = unsafe { gdwwdgt30374593 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const I2S1_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_3400usize as _) } ; pub const SPI1 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const I2S2_ADD : gdi2s32f828a0 :: I2s = unsafe { gdi2s32f828a0 :: I2s :: from_ptr (0x4000_4000usize as _) } ; pub const USART1 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2ce15a3570 :: I2c = unsafe { gdi2ce15a3570 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2ce15a3570 :: I2c = unsafe { gdi2ce15a3570 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const I2C2 : gdi2ce15a3570 :: I2c = unsafe { gdi2ce15a3570 :: I2c :: from_ptr (0x4000_5c00usize as _) } ; pub const CAN0 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6400usize as _) } ; pub const CAN1 : gdcan486a8ac4 :: Can = unsafe { gdcan486a8ac4 :: Can :: from_ptr (0x4000_6800usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu36bfb084 :: Pmu = unsafe { gdpmu36bfb084 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC0 : gddac4ad47e29 :: Dac = unsafe { gddac4ad47e29 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const UART6 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_7800usize as _) } ; pub const UART7 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4000_7c00usize as _) } ; pub const IREF : gdiref4b25e655 :: Iref = unsafe { gdiref4b25e655 :: Iref :: from_ptr (0x4000_c400usize as _) } ; pub const TIMER0 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4001_0000usize as _) } ; pub const TIMER7 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4001_0400usize as _) } ; pub const USART0 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4001_1000usize as _) } ; pub const USART5 : gdusartd5126b39 :: Usart = unsafe { gdusartd5126b39 :: Usart :: from_ptr (0x4001_1400usize as _) } ; pub const ADC0 : gdadc4e1ce4e3 :: Adc = unsafe { gdadc4e1ce4e3 :: Adc :: from_ptr (0x4001_2000usize as _) } ; pub const ADC_BASE : gdadcbasec6505c26 :: AdcBase = unsafe { gdadcbasec6505c26 :: AdcBase :: from_ptr (0x4001_2000usize as _) } ; pub const ADC1 : gdadc4e1ce4e3 :: Adc = unsafe { gdadc4e1ce4e3 :: Adc :: from_ptr (0x4001_2100usize as _) } ; pub const ADC2 : gdadc4e1ce4e3 :: Adc = unsafe { gdadc4e1ce4e3 :: Adc :: from_ptr (0x4001_2200usize as _) } ; pub const SDIO : gdsdioa16a5588 :: Sdio = unsafe { gdsdioa16a5588 :: Sdio :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const SPI3 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4001_3400usize as _) } ; pub const SYSCFG : gdsyscfg66a0d661 :: Syscfg = unsafe { gdsyscfg66a0d661 :: Syscfg :: from_ptr (0x4001_3800usize as _) } ; pub const EXTI : gdexti43b21b0c :: Exti = unsafe { gdexti43b21b0c :: Exti :: from_ptr (0x4001_3c00usize as _) } ; pub const TIMER8 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4001_4000usize as _) } ; pub const TIMER9 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER10 : gdtimer7fc294ac :: Timer = unsafe { gdtimer7fc294ac :: Timer :: from_ptr (0x4001_4800usize as _) } ; pub const SPI4 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4001_5000usize as _) } ; pub const SPI5 : gdspi84bb0d40 :: Spi = unsafe { gdspi84bb0d40 :: Spi :: from_ptr (0x4001_5400usize as _) } ; pub const LAYER0 : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const TLI : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6800usize as _) } ; pub const LAYER1 : gdtli94583ddb :: Tli = unsafe { gdtli94583ddb :: Tli :: from_ptr (0x4001_6880usize as _) } ; pub const GPIOA : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_0000usize as _) } ; pub const GPIOB : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_0400usize as _) } ; pub const GPIOC : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_0800usize as _) } ; pub const GPIOD : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_0c00usize as _) } ; pub const GPIOE : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_1000usize as _) } ; pub const GPIOF : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_1400usize as _) } ; pub const GPIOG : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_1800usize as _) } ; pub const GPIOH : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_1c00usize as _) } ; pub const GPIOI : gdgpio45754e8d :: Gpio = unsafe { gdgpio45754e8d :: Gpio :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc3d3f2740 :: Crc = unsafe { gdcrc3d3f2740 :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const RCU : gdrcu1692fdfa :: Rcu = unsafe { gdrcu1692fdfa :: Rcu :: from_ptr (0x4002_3800usize as _) } ; pub const FMC : gdfmc229649d7 :: Fmc = unsafe { gdfmc229649d7 :: Fmc :: from_ptr (0x4002_3c00usize as _) } ; pub const DMA0 : gddma35406330 :: Dma = unsafe { gddma35406330 :: Dma :: from_ptr (0x4002_6000usize as _) } ; pub const DMA1 : gddma35406330 :: Dma = unsafe { gddma35406330 :: Dma :: from_ptr (0x4002_6400usize as _) } ; pub const ENET : gdenetba6af7a7 :: Enet = unsafe { gdenetba6af7a7 :: Enet :: from_ptr (0x4002_8000usize as _) } ; pub const IPA : gdipad2e9ee25 :: Ipa = unsafe { gdipad2e9ee25 :: Ipa :: from_ptr (0x4002_b000usize as _) } ; pub const DCI : gddci6728f4f7 :: Dci = unsafe { gddci6728f4f7 :: Dci :: from_ptr (0x5005_0000usize as _) } ; pub const TRNG : gdtrng13872700 :: Trng = unsafe { gdtrng13872700 :: Trng :: from_ptr (0x5006_0800usize as _) } ; pub const EXMC_NOR_PSRAM : gdexmcec740ad5 :: Exmc = unsafe { gdexmcec740ad5 :: Exmc :: from_ptr (0x6000_0000usize as _) } ; pub const EXMC_NAND : gdexmcec740ad5 :: Exmc = unsafe { gdexmcec740ad5 :: Exmc :: from_ptr (0x7000_0000usize as _) } ; pub const EXMC_PCCARD : gdexmcec740ad5 :: Exmc = unsafe { gdexmcec740ad5 :: Exmc :: from_ptr (0x9000_0000usize as _) } ; pub const EXMC : gdexmcec740ad5 :: Exmc = unsafe { gdexmcec740ad5 :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const EXMC_SDRAM : gdexmcec740ad5 :: Exmc = unsafe { gdexmcec740ad5 :: Exmc :: from_ptr (0xc000_0000usize as _) } ; pub const DBG : gddbge84f01a0 :: Dbg = unsafe { gddbge84f01a0 :: Dbg :: from_ptr (0xe004_2000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc4e1ce4e3_v1.rs"] pub mod gdadc4e1ce4e3;
#[path="../../peripherals/gdadcbasec6505c26_v1.rs"] pub mod gdadcbasec6505c26;
#[path="../../peripherals/gdcan486a8ac4_v1.rs"] pub mod gdcan486a8ac4;
#[path="../../peripherals/gdcrc3d3f2740_v1.rs"] pub mod gdcrc3d3f2740;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddac4ad47e29_v1.rs"] pub mod gddac4ad47e29;
#[path="../../peripherals/gddbge84f01a0_v1.rs"] pub mod gddbge84f01a0;
#[path="../../peripherals/gddci6728f4f7_v1.rs"] pub mod gddci6728f4f7;
#[path="../../peripherals/gddma35406330_v1.rs"] pub mod gddma35406330;
#[path="../../peripherals/gdenetba6af7a7_v1.rs"] pub mod gdenetba6af7a7;
#[path="../../peripherals/gdexmcec740ad5_v1.rs"] pub mod gdexmcec740ad5;
#[path="../../peripherals/gdexti43b21b0c_v1.rs"] pub mod gdexti43b21b0c;
#[path="../../peripherals/gdfmc229649d7_v1.rs"] pub mod gdfmc229649d7;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpio45754e8d_v1.rs"] pub mod gdgpio45754e8d;
#[path="../../peripherals/gdi2ce15a3570_v1.rs"] pub mod gdi2ce15a3570;
#[path="../../peripherals/gdi2s32f828a0_v1.rs"] pub mod gdi2s32f828a0;
#[path="../../peripherals/gdipad2e9ee25_v1.rs"] pub mod gdipad2e9ee25;
#[path="../../peripherals/gdiref4b25e655_v1.rs"] pub mod gdiref4b25e655;
#[path="../../peripherals/gdob16bd26c7_v1.rs"] pub mod gdob16bd26c7;
#[path="../../peripherals/gdpmu36bfb084_v1.rs"] pub mod gdpmu36bfb084;
#[path="../../peripherals/gdrcu1692fdfa_v1.rs"] pub mod gdrcu1692fdfa;
#[path="../../peripherals/gdrtcea88a5d1_v1.rs"] pub mod gdrtcea88a5d1;
#[path="../../peripherals/gdsdioa16a5588_v1.rs"] pub mod gdsdioa16a5588;
#[path="../../peripherals/gdspi84bb0d40_v1.rs"] pub mod gdspi84bb0d40;
#[path="../../peripherals/gdsyscfg66a0d661_v1.rs"] pub mod gdsyscfg66a0d661;
#[path="../../peripherals/gdtimer7fc294ac_v1.rs"] pub mod gdtimer7fc294ac;
#[path="../../peripherals/gdtli94583ddb_v1.rs"] pub mod gdtli94583ddb;
#[path="../../peripherals/gdtrng13872700_v1.rs"] pub mod gdtrng13872700;
#[path="../../peripherals/gdusartd5126b39_v1.rs"] pub mod gdusartd5126b39;
#[path="../../peripherals/gdwwdgt30374593_v1.rs"] pub mod gdwwdgt30374593;
