




# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD_VAVD"]
LVD_VAVD = 1 , # [doc = "2 - TAMPER"]
TAMPER = 2 , # [doc = "3 - RTC"]
RTC = 3 , # [doc = "4 - FMC"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC0_1"]
ADC0_1 = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI5_9"]
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
RTC_ALARM = 41 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "43 - TIMER7_BRK"]
TIMER7_BRK = 43 , # [doc = "44 - TIMER7_UP"]
TIMER7_UP = 44 , # [doc = "45 - TIMER7_TRG_CMT"]
TIMER7_TRG_CMT = 45 , # [doc = "46 - TIMER7_CHANNEL"]
TIMER7_CHANNEL = 46 , # [doc = "47 - ADC2"]
ADC2 = 47 , # [doc = "49 - RCU_CKFM"]
RCU_CKFM = 49 , # [doc = "50 - CMP_WAKEUP"]
CMP_WAKEUP = 50 , # [doc = "51 - SPI2"]
SPI2 = 51 , # [doc = "52 - UART3"]
UART3 = 52 , # [doc = "53 - UART4"]
UART4 = 53 , # [doc = "54 - TIMER5"]
TIMER5 = 54 , # [doc = "55 - TIMER6"]
TIMER6 = 55 , # [doc = "56 - DMA1_CHANNEL0"]
DMA1_CHANNEL0 = 56 , # [doc = "57 - DMA1_CHANNEL1"]
DMA1_CHANNEL1 = 57 , # [doc = "58 - DMA1_CHANNEL2"]
DMA1_CHANNEL2 = 58 , # [doc = "59 - DMA1_CHANNEL3"]
DMA1_CHANNEL3 = 59 , # [doc = "60 - DMA1_CHANNEL4"]
DMA1_CHANNEL4 = 60 , # [doc = "61 - DAC"]
DAC = 61 , # [doc = "62 - PMU_VUVD_VOVD"]
PMU_VUVD_VOVD = 62 , # [doc = "63 - CAN1_TX"]
CAN1_TX = 63 , # [doc = "64 - CAN1_RX0"]
CAN1_RX0 = 64 , # [doc = "65 - CAN1_RX1"]
CAN1_RX1 = 65 , # [doc = "66 - CAN1_EWMC"]
CAN1_EWMC = 66 , # [doc = "67 - SRAM_ECC"]
SRAM_ECC = 67 , # [doc = "68 - FPU"]
FPU = 68 , # [doc = "69 - CMP"]
CMP = 69 , # [doc = "70 - DMAMUX"]
DMAMUX = 70 , # [doc = "71 - CAU"]
CAU = 71 , # [doc = "72 - HAU"]
HAU = 72 , # [doc = "73 - TRNG"]
TRNG = 73 , # [doc = "74 - USBFS"]
USBFS = 74 , # [doc = "75 - TIMER4"]
TIMER4 = 75 , # [doc = "76 - TIMER15"]
TIMER15 = 76 , # [doc = "77 - TIMER16"]
TIMER16 = 77 , # [doc = "78 - TIMER0_BRK_CHANNEL"]
TIMER0_BRK_CHANNEL = 78 , # [doc = "79 - TIMER7_BRK_CHANNEL"]
TIMER7_BRK_CHANNEL = 79 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD_VAVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI0 () ; fn EXTI1 () ; fn EXTI2 () ; fn EXTI3 () ; fn EXTI4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC0_1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI5_9 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CHANNEL () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI10_15 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK () ; fn TIMER7_UP () ; fn TIMER7_TRG_CMT () ; fn TIMER7_CHANNEL () ; fn ADC2 () ; fn RCU_CKFM () ; fn CMP_WAKEUP () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DAC () ; fn PMU_VUVD_VOVD () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn SRAM_ECC () ; fn FPU () ; fn CMP () ; fn DMAMUX () ; fn CAU () ; fn HAU () ; fn TRNG () ; fn USBFS () ; fn TIMER4 () ; fn TIMER15 () ; fn TIMER16 () ; fn TIMER0_BRK_CHANNEL () ; fn TIMER7_BRK_CHANNEL () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe (link_section = ".vector_table.interrupts")]
# [unsafe (no_mangle)]
pub static __INTERRUPTS : [Vector ; 80]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD_VAVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI0 } , Vector { _handler : EXTI1 } , Vector { _handler : EXTI2 } , Vector { _handler : EXTI3 } , Vector { _handler : EXTI4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC0_1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI5_9 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CHANNEL } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI10_15 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK } , Vector { _handler : TIMER7_UP } , Vector { _handler : TIMER7_TRG_CMT } , Vector { _handler : TIMER7_CHANNEL } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _handler : RCU_CKFM } , Vector { _handler : CMP_WAKEUP } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DAC } , Vector { _handler : PMU_VUVD_VOVD } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : SRAM_ECC } , Vector { _handler : FPU } , Vector { _handler : CMP } , Vector { _handler : DMAMUX } , Vector { _handler : CAU } , Vector { _handler : HAU } , Vector { _handler : TRNG } , Vector { _handler : USBFS } , Vector { _handler : TIMER4 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : TIMER0_BRK_CHANNEL } , Vector { _handler : TIMER7_BRK_CHANNEL } ,]
; } pub const OB : gdob138fbadf :: Ob = unsafe { gdob138fbadf :: Ob :: from_ptr (0x1fff_f800usize as _) } ; pub const TIMER1 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER16 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4000_1800usize as _) } ; pub const RTC : gdrtc250e9b91 :: Rtc = unsafe { gdrtc250e9b91 :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt59a14ef4 :: Wwdgt = unsafe { gdwwdgt59a14ef4 :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgt77bb718d :: Fwdgt = unsafe { gdfwdgt77bb718d :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspidfb3af1d :: Spi = unsafe { gdspidfb3af1d :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspidfb3af1d :: Spi = unsafe { gdspidfb3af1d :: Spi :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusartd892c3f9 :: Usart = unsafe { gdusartd892c3f9 :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusartd892c3f9 :: Usart = unsafe { gdusartd892c3f9 :: Usart :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gdusartd892c3f9 :: Usart = unsafe { gdusartd892c3f9 :: Usart :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gdusartd892c3f9 :: Usart = unsafe { gdusartd892c3f9 :: Usart :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2cecd631a5 :: I2c = unsafe { gdi2cecd631a5 :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2cecd631a5 :: I2c = unsafe { gdi2cecd631a5 :: I2c :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkpddaa24e5 :: Bkp = unsafe { gdbkpddaa24e5 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmuf6fd2d81 :: Pmu = unsafe { gdpmuf6fd2d81 :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const CMP : gdcmp70d4db57 :: Cmp = unsafe { gdcmp70d4db57 :: Cmp :: from_ptr (0x4000_7800usize as _) } ; pub const CTC : gdctc6d9ce461 :: Ctc = unsafe { gdctc6d9ce461 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const AFIO : gdafiocfb569a7 :: Afio = unsafe { gdafiocfb569a7 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdextia39263ea :: Exti = unsafe { gdextia39263ea :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpiob9f95038 :: Gpio = unsafe { gdgpiob9f95038 :: Gpio :: from_ptr (0x4001_1800usize as _) } ; pub const ADC0 : gdadcc334f3cc :: Adc = unsafe { gdadcc334f3cc :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadcc334f3cc :: Adc = unsafe { gdadcc334f3cc :: Adc :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspidfb3af1d :: Spi = unsafe { gdspidfb3af1d :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusartd892c3f9 :: Usart = unsafe { gdusartd892c3f9 :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadcc334f3cc :: Adc = unsafe { gdadcc334f3cc :: Adc :: from_ptr (0x4001_3c00usize as _) } ; pub const SYSCFG : gdsyscfga124fcf6 :: Syscfg = unsafe { gdsyscfga124fcf6 :: Syscfg :: from_ptr (0x4001_4000usize as _) } ; pub const TRIGSEL : gdtrigsel280e29a3 :: Trigsel = unsafe { gdtrigsel280e29a3 :: Trigsel :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER15 : gdtimer0a6eba78 :: Timer = unsafe { gdtimer0a6eba78 :: Timer :: from_ptr (0x4001_5000usize as _) } ; pub const CAN0 : gdcan09590032f :: Can0 = unsafe { gdcan09590032f :: Can0 :: from_ptr (0x4001_5800usize as _) } ; pub const CAN1 : gdcan8ce81596 :: Can = unsafe { gdcan8ce81596 :: Can :: from_ptr (0x4001_5c00usize as _) } ; pub const DMA0 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddmae208530b :: Dma = unsafe { gddmae208530b :: Dma :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamuxcd4c69ea :: Dmamux = unsafe { gddmamuxcd4c69ea :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RM_CHXCFG_BASE : gddmamuxrmchxcfgbasef37c083c :: DmamuxRmChxcfgBase = unsafe { gddmamuxrmchxcfgbasef37c083c :: DmamuxRmChxcfgBase :: from_ptr (0x4002_0800usize as _) } ; pub const DMAMUX_RG_CHXCFG_BASE : gddmamuxrgchxcfgbased84fcfa6 :: DmamuxRgChxcfgBase = unsafe { gddmamuxrgchxcfgbased84fcfa6 :: DmamuxRgChxcfgBase :: from_ptr (0x4002_0900usize as _) } ; pub const RCU : gdrcu97a76383 :: Rcu = unsafe { gdrcu97a76383 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc71a20e5f :: Fmc = unsafe { gdfmc71a20e5f :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CAU : gdcauc12c326c :: Cau = unsafe { gdcauc12c326c :: Cau :: from_ptr (0x4002_3400usize as _) } ; pub const HAU : gdhau6f90c013 :: Hau = unsafe { gdhau6f90c013 :: Hau :: from_ptr (0x4002_3800usize as _) } ; pub const TRNG : gdtrng4a6beb42 :: Trng = unsafe { gdtrng4a6beb42 :: Trng :: from_ptr (0x4002_3c00usize as _) } ; pub const EXMC : gdexmcf139886b :: Exmc = unsafe { gdexmcf139886b :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbgc321d162 :: Dbg = unsafe { gddbgc321d162 :: Dbg :: from_ptr (0xe004_5000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadcc334f3cc_v1.rs"] pub mod gdadcc334f3cc;
#[path="../../peripherals/gdafiocfb569a7_v1.rs"] pub mod gdafiocfb569a7;
#[path="../../peripherals/gdbkpddaa24e5_v1.rs"] pub mod gdbkpddaa24e5;
#[path="../../peripherals/gdcan09590032f_v1.rs"] pub mod gdcan09590032f;
#[path="../../peripherals/gdcan8ce81596_v1.rs"] pub mod gdcan8ce81596;
#[path="../../peripherals/gdcauc12c326c_v1.rs"] pub mod gdcauc12c326c;
#[path="../../peripherals/gdcmp70d4db57_v1.rs"] pub mod gdcmp70d4db57;
#[path="../../peripherals/gdctc6d9ce461_v1.rs"] pub mod gdctc6d9ce461;
#[path="../../peripherals/gddbgc321d162_v1.rs"] pub mod gddbgc321d162;
#[path="../../peripherals/gddmae208530b_v1.rs"] pub mod gddmae208530b;
#[path="../../peripherals/gddmamuxcd4c69ea_v1.rs"] pub mod gddmamuxcd4c69ea;
#[path="../../peripherals/gddmamuxrgchxcfgbased84fcfa6_v1.rs"] pub mod gddmamuxrgchxcfgbased84fcfa6;
#[path="../../peripherals/gddmamuxrmchxcfgbasef37c083c_v1.rs"] pub mod gddmamuxrmchxcfgbasef37c083c;
#[path="../../peripherals/gdexmcf139886b_v1.rs"] pub mod gdexmcf139886b;
#[path="../../peripherals/gdextia39263ea_v1.rs"] pub mod gdextia39263ea;
#[path="../../peripherals/gdfmc71a20e5f_v1.rs"] pub mod gdfmc71a20e5f;
#[path="../../peripherals/gdfwdgt77bb718d_v1.rs"] pub mod gdfwdgt77bb718d;
#[path="../../peripherals/gdgpiob9f95038_v1.rs"] pub mod gdgpiob9f95038;
#[path="../../peripherals/gdhau6f90c013_v1.rs"] pub mod gdhau6f90c013;
#[path="../../peripherals/gdi2cecd631a5_v1.rs"] pub mod gdi2cecd631a5;
#[path="../../peripherals/gdob138fbadf_v1.rs"] pub mod gdob138fbadf;
#[path="../../peripherals/gdpmuf6fd2d81_v1.rs"] pub mod gdpmuf6fd2d81;
#[path="../../peripherals/gdrcu97a76383_v1.rs"] pub mod gdrcu97a76383;
#[path="../../peripherals/gdrtc250e9b91_v1.rs"] pub mod gdrtc250e9b91;
#[path="../../peripherals/gdspidfb3af1d_v1.rs"] pub mod gdspidfb3af1d;
#[path="../../peripherals/gdsyscfga124fcf6_v1.rs"] pub mod gdsyscfga124fcf6;
#[path="../../peripherals/gdtimer0a6eba78_v1.rs"] pub mod gdtimer0a6eba78;
#[path="../../peripherals/gdtrigsel280e29a3_v1.rs"] pub mod gdtrigsel280e29a3;
#[path="../../peripherals/gdtrng4a6beb42_v1.rs"] pub mod gdtrng4a6beb42;
#[path="../../peripherals/gdusartd892c3f9_v1.rs"] pub mod gdusartd892c3f9;
#[path="../../peripherals/gdwwdgt59a14ef4_v1.rs"] pub mod gdwwdgt59a14ef4;
