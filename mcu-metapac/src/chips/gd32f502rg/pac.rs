

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
# [cfg_attr (feature = "defmt" , derive (defmt :: Format))]
pub enum Interrupt { # [doc = "0 - WWDGT"]
WWDGT = 0 , # [doc = "1 - LVD_VAVD"]
LVD_VAVD = 1 , # [doc = "2 - TAMPER"]
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
DMA0_CHANNEL6 = 17 , # [doc = "18 - ADC1"]
ADC1 = 18 , # [doc = "19 - CAN0_TX"]
CAN0_TX = 19 , # [doc = "20 - CAN0_RX0"]
CAN0_RX0 = 20 , # [doc = "21 - CAN0_RX1"]
CAN0_RX1 = 21 , # [doc = "22 - CAN0_EWMC"]
CAN0_EWMC = 22 , # [doc = "23 - EXTI_LINE9_5"]
EXTI_LINE9_5 = 23 , # [doc = "24 - TIMER0_BRK"]
TIMER0_BRK = 24 , # [doc = "25 - TIMER0_UP"]
TIMER0_UP = 25 , # [doc = "26 - TIMER0_TRG_CMT"]
TIMER0_TRG_CMT = 26 , # [doc = "27 - TIMER0_CC"]
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
RTC_ALARM = 41 , # [doc = "42 - USBFS_WKUP"]
USBFS_WKUP = 42 , # [doc = "43 - TIMER7_BRK_TIMER11"]
TIMER7_BRK_TIMER11 = 43 , # [doc = "44 - TIMER7_UP_TIMER12"]
TIMER7_UP_TIMER12 = 44 , # [doc = "45 - TIMER7_TRG_CMT_TIMER13"]
TIMER7_TRG_CMT_TIMER13 = 45 , # [doc = "46 - TIMER7_CC"]
TIMER7_CC = 46 , # [doc = "47 - ADC2"]
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
DAC = 61 , # [doc = "62 - VUVD_VOVD"]
VUVD_VOVD = 62 , # [doc = "63 - CAN1_TX"]
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
mod _vectors { unsafe extern "C" { fn WWDGT () ; fn LVD_VAVD () ; fn TAMPER () ; fn RTC () ; fn FMC () ; fn RCU_CTC () ; fn EXTI_LINE0 () ; fn EXTI_LINE1 () ; fn EXTI_LINE2 () ; fn EXTI_LINE3 () ; fn EXTI_LINE4 () ; fn DMA0_CHANNEL0 () ; fn DMA0_CHANNEL1 () ; fn DMA0_CHANNEL2 () ; fn DMA0_CHANNEL3 () ; fn DMA0_CHANNEL4 () ; fn DMA0_CHANNEL5 () ; fn DMA0_CHANNEL6 () ; fn ADC1 () ; fn CAN0_TX () ; fn CAN0_RX0 () ; fn CAN0_RX1 () ; fn CAN0_EWMC () ; fn EXTI_LINE9_5 () ; fn TIMER0_BRK () ; fn TIMER0_UP () ; fn TIMER0_TRG_CMT () ; fn TIMER0_CC () ; fn TIMER1 () ; fn TIMER2 () ; fn TIMER3 () ; fn I2C0_EV () ; fn I2C0_ER () ; fn I2C1_EV () ; fn I2C1_ER () ; fn SPI0 () ; fn SPI1 () ; fn USART0 () ; fn USART1 () ; fn USART2 () ; fn EXTI_LINE15_10 () ; fn RTC_ALARM () ; fn USBFS_WKUP () ; fn TIMER7_BRK_TIMER11 () ; fn TIMER7_UP_TIMER12 () ; fn TIMER7_TRG_CMT_TIMER13 () ; fn TIMER7_CC () ; fn ADC2 () ; fn RCU_CKFM () ; fn CMP_WAKEUP () ; fn SPI2 () ; fn UART3 () ; fn UART4 () ; fn TIMER5 () ; fn TIMER6 () ; fn DMA1_CHANNEL0 () ; fn DMA1_CHANNEL1 () ; fn DMA1_CHANNEL2 () ; fn DMA1_CHANNEL3 () ; fn DMA1_CHANNEL4 () ; fn DAC () ; fn VUVD_VOVD () ; fn CAN1_TX () ; fn CAN1_RX0 () ; fn CAN1_RX1 () ; fn CAN1_EWMC () ; fn SRAM_ECC () ; fn FPU () ; fn CMP () ; fn DMAMUX () ; fn CAU () ; fn HAU () ; fn TRNG () ; fn USBFS () ; fn TIMER4 () ; fn TIMER15 () ; fn TIMER16 () ; fn TIMER0_BRK_CHANNEL () ; fn TIMER7_BRK_CHANNEL () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [unsafe(link_section = ".vector_table.interrupts")]
# [unsafe(no_mangle)]
pub static __INTERRUPTS : [Vector ; 80]
= [Vector { _handler : WWDGT } , Vector { _handler : LVD_VAVD } , Vector { _handler : TAMPER } , Vector { _handler : RTC } , Vector { _handler : FMC } , Vector { _handler : RCU_CTC } , Vector { _handler : EXTI_LINE0 } , Vector { _handler : EXTI_LINE1 } , Vector { _handler : EXTI_LINE2 } , Vector { _handler : EXTI_LINE3 } , Vector { _handler : EXTI_LINE4 } , Vector { _handler : DMA0_CHANNEL0 } , Vector { _handler : DMA0_CHANNEL1 } , Vector { _handler : DMA0_CHANNEL2 } , Vector { _handler : DMA0_CHANNEL3 } , Vector { _handler : DMA0_CHANNEL4 } , Vector { _handler : DMA0_CHANNEL5 } , Vector { _handler : DMA0_CHANNEL6 } , Vector { _handler : ADC1 } , Vector { _handler : CAN0_TX } , Vector { _handler : CAN0_RX0 } , Vector { _handler : CAN0_RX1 } , Vector { _handler : CAN0_EWMC } , Vector { _handler : EXTI_LINE9_5 } , Vector { _handler : TIMER0_BRK } , Vector { _handler : TIMER0_UP } , Vector { _handler : TIMER0_TRG_CMT } , Vector { _handler : TIMER0_CC } , Vector { _handler : TIMER1 } , Vector { _handler : TIMER2 } , Vector { _handler : TIMER3 } , Vector { _handler : I2C0_EV } , Vector { _handler : I2C0_ER } , Vector { _handler : I2C1_EV } , Vector { _handler : I2C1_ER } , Vector { _handler : SPI0 } , Vector { _handler : SPI1 } , Vector { _handler : USART0 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _handler : EXTI_LINE15_10 } , Vector { _handler : RTC_ALARM } , Vector { _handler : USBFS_WKUP } , Vector { _handler : TIMER7_BRK_TIMER11 } , Vector { _handler : TIMER7_UP_TIMER12 } , Vector { _handler : TIMER7_TRG_CMT_TIMER13 } , Vector { _handler : TIMER7_CC } , Vector { _handler : ADC2 } , Vector { _reserved : 0 } , Vector { _handler : RCU_CKFM } , Vector { _handler : CMP_WAKEUP } , Vector { _handler : SPI2 } , Vector { _handler : UART3 } , Vector { _handler : UART4 } , Vector { _handler : TIMER5 } , Vector { _handler : TIMER6 } , Vector { _handler : DMA1_CHANNEL0 } , Vector { _handler : DMA1_CHANNEL1 } , Vector { _handler : DMA1_CHANNEL2 } , Vector { _handler : DMA1_CHANNEL3 } , Vector { _handler : DMA1_CHANNEL4 } , Vector { _handler : DAC } , Vector { _handler : VUVD_VOVD } , Vector { _handler : CAN1_TX } , Vector { _handler : CAN1_RX0 } , Vector { _handler : CAN1_RX1 } , Vector { _handler : CAN1_EWMC } , Vector { _handler : SRAM_ECC } , Vector { _handler : FPU } , Vector { _handler : CMP } , Vector { _handler : DMAMUX } , Vector { _handler : CAU } , Vector { _handler : HAU } , Vector { _handler : TRNG } , Vector { _handler : USBFS } , Vector { _handler : TIMER4 } , Vector { _handler : TIMER15 } , Vector { _handler : TIMER16 } , Vector { _handler : TIMER0_BRK_CHANNEL } , Vector { _handler : TIMER7_BRK_CHANNEL } ,]
; } pub const TIMER1 : gdtimer12e5cd301 :: Timer1 = unsafe { gdtimer12e5cd301 :: Timer1 :: from_ptr (0x4000_0000usize as _) } ; pub const TIMER2 : gdtimer12e5cd301 :: Timer1 = unsafe { gdtimer12e5cd301 :: Timer1 :: from_ptr (0x4000_0400usize as _) } ; pub const TIMER3 : gdtimer12e5cd301 :: Timer1 = unsafe { gdtimer12e5cd301 :: Timer1 :: from_ptr (0x4000_0800usize as _) } ; pub const TIMER4 : gdtimer12e5cd301 :: Timer1 = unsafe { gdtimer12e5cd301 :: Timer1 :: from_ptr (0x4000_0c00usize as _) } ; pub const TIMER5 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1000usize as _) } ; pub const TIMER6 : gdtimer54b5e73ec :: Timer5 = unsafe { gdtimer54b5e73ec :: Timer5 :: from_ptr (0x4000_1400usize as _) } ; pub const TIMER16 : gdtimer15296d56bf :: Timer15 = unsafe { gdtimer15296d56bf :: Timer15 :: from_ptr (0x4000_1800usize as _) } ; pub const RTC : gdrtc6b0c077c :: Rtc = unsafe { gdrtc6b0c077c :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDGT : gdwwdgt5c5bacde :: Wwdgt = unsafe { gdwwdgt5c5bacde :: Wwdgt :: from_ptr (0x4000_2c00usize as _) } ; pub const FWDGT : gdfwdgtdc3d0d7a :: Fwdgt = unsafe { gdfwdgtdc3d0d7a :: Fwdgt :: from_ptr (0x4000_3000usize as _) } ; pub const SPI1 : gdspi0ad5e2dff :: Spi0 = unsafe { gdspi0ad5e2dff :: Spi0 :: from_ptr (0x4000_3800usize as _) } ; pub const SPI2 : gdspi0ad5e2dff :: Spi0 = unsafe { gdspi0ad5e2dff :: Spi0 :: from_ptr (0x4000_3c00usize as _) } ; pub const USART1 : gdusart0b635b392 :: Usart0 = unsafe { gdusart0b635b392 :: Usart0 :: from_ptr (0x4000_4400usize as _) } ; pub const USART2 : gdusart0b635b392 :: Usart0 = unsafe { gdusart0b635b392 :: Usart0 :: from_ptr (0x4000_4800usize as _) } ; pub const UART3 : gduart31f2ca0ee :: Uart3 = unsafe { gduart31f2ca0ee :: Uart3 :: from_ptr (0x4000_4c00usize as _) } ; pub const UART4 : gduart31f2ca0ee :: Uart3 = unsafe { gduart31f2ca0ee :: Uart3 :: from_ptr (0x4000_5000usize as _) } ; pub const I2C0 : gdi2c0932dc70f :: I2c0 = unsafe { gdi2c0932dc70f :: I2c0 :: from_ptr (0x4000_5400usize as _) } ; pub const I2C1 : gdi2c0932dc70f :: I2c0 = unsafe { gdi2c0932dc70f :: I2c0 :: from_ptr (0x4000_5800usize as _) } ; pub const BKP : gdbkp16a620e0 :: Bkp = unsafe { gdbkp16a620e0 :: Bkp :: from_ptr (0x4000_6c00usize as _) } ; pub const PMU : gdpmu82ca625b :: Pmu = unsafe { gdpmu82ca625b :: Pmu :: from_ptr (0x4000_7000usize as _) } ; pub const DAC : gddac4d974090 :: Dac = unsafe { gddac4d974090 :: Dac :: from_ptr (0x4000_7400usize as _) } ; pub const CMP : gdcmpfc67f344 :: Cmp = unsafe { gdcmpfc67f344 :: Cmp :: from_ptr (0x4000_7800usize as _) } ; pub const CTC : gdctc0cad8643 :: Ctc = unsafe { gdctc0cad8643 :: Ctc :: from_ptr (0x4000_c800usize as _) } ; pub const AFIO : gdafio220b0195 :: Afio = unsafe { gdafio220b0195 :: Afio :: from_ptr (0x4001_0000usize as _) } ; pub const EXTI : gdexti11a1be47 :: Exti = unsafe { gdexti11a1be47 :: Exti :: from_ptr (0x4001_0400usize as _) } ; pub const GPIOA : gdgpioa519aa242 :: Gpioa = unsafe { gdgpioa519aa242 :: Gpioa :: from_ptr (0x4001_0800usize as _) } ; pub const GPIOB : gdgpioa519aa242 :: Gpioa = unsafe { gdgpioa519aa242 :: Gpioa :: from_ptr (0x4001_0c00usize as _) } ; pub const GPIOC : gdgpioa519aa242 :: Gpioa = unsafe { gdgpioa519aa242 :: Gpioa :: from_ptr (0x4001_1000usize as _) } ; pub const GPIOD : gdgpioa519aa242 :: Gpioa = unsafe { gdgpioa519aa242 :: Gpioa :: from_ptr (0x4001_1400usize as _) } ; pub const GPIOE : gdgpioa519aa242 :: Gpioa = unsafe { gdgpioa519aa242 :: Gpioa :: from_ptr (0x4001_1800usize as _) } ; pub const ADC0 : gdadc05f794981 :: Adc0 = unsafe { gdadc05f794981 :: Adc0 :: from_ptr (0x4001_2400usize as _) } ; pub const ADC1 : gdadc05f794981 :: Adc0 = unsafe { gdadc05f794981 :: Adc0 :: from_ptr (0x4001_2800usize as _) } ; pub const TIMER0 : gdtimer0debe3394 :: Timer0 = unsafe { gdtimer0debe3394 :: Timer0 :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI0 : gdspi0ad5e2dff :: Spi0 = unsafe { gdspi0ad5e2dff :: Spi0 :: from_ptr (0x4001_3000usize as _) } ; pub const TIMER7 : gdtimer0debe3394 :: Timer0 = unsafe { gdtimer0debe3394 :: Timer0 :: from_ptr (0x4001_3400usize as _) } ; pub const USART0 : gdusart0b635b392 :: Usart0 = unsafe { gdusart0b635b392 :: Usart0 :: from_ptr (0x4001_3800usize as _) } ; pub const ADC2 : gdadc05f794981 :: Adc0 = unsafe { gdadc05f794981 :: Adc0 :: from_ptr (0x4001_3c00usize as _) } ; pub const SYSCFG : gdsyscfg14885d82 :: Syscfg = unsafe { gdsyscfg14885d82 :: Syscfg :: from_ptr (0x4001_4000usize as _) } ; pub const TRIGSEL : gdtrigsel7c239a51 :: Trigsel = unsafe { gdtrigsel7c239a51 :: Trigsel :: from_ptr (0x4001_4400usize as _) } ; pub const TIMER15 : gdtimer15296d56bf :: Timer15 = unsafe { gdtimer15296d56bf :: Timer15 :: from_ptr (0x4001_5000usize as _) } ; pub const CAN0 : gdcan0f4404570 :: Can0 = unsafe { gdcan0f4404570 :: Can0 :: from_ptr (0x4001_5800usize as _) } ; pub const CAN1 : gdcan0f4404570 :: Can0 = unsafe { gdcan0f4404570 :: Can0 :: from_ptr (0x4001_5c00usize as _) } ; pub const DMA0 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0000usize as _) } ; pub const DMA1 : gddma011392832 :: Dma0 = unsafe { gddma011392832 :: Dma0 :: from_ptr (0x4002_0400usize as _) } ; pub const DMAMUX : gddmamux84b48a9d :: Dmamux = unsafe { gddmamux84b48a9d :: Dmamux :: from_ptr (0x4002_0800usize as _) } ; pub const RCU : gdrcudff54106 :: Rcu = unsafe { gdrcudff54106 :: Rcu :: from_ptr (0x4002_1000usize as _) } ; pub const FMC : gdfmc32a1a410 :: Fmc = unsafe { gdfmc32a1a410 :: Fmc :: from_ptr (0x4002_2000usize as _) } ; pub const CRC : gdcrc2255b0ef :: Crc = unsafe { gdcrc2255b0ef :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const CAU : gdcaue3998bcd :: Cau = unsafe { gdcaue3998bcd :: Cau :: from_ptr (0x4002_3400usize as _) } ; pub const HAU : gdhau0e6a9f22 :: Hau = unsafe { gdhau0e6a9f22 :: Hau :: from_ptr (0x4002_3800usize as _) } ; pub const TRNG : gdtrng8cefada9 :: Trng = unsafe { gdtrng8cefada9 :: Trng :: from_ptr (0x4002_3c00usize as _) } ; pub const USBFS_GLOBAL : gdusbfsglobald97a6bbd :: UsbfsGlobal = unsafe { gdusbfsglobald97a6bbd :: UsbfsGlobal :: from_ptr (0x5000_0000usize as _) } ; pub const USBFS_HOST : gdusbfshost5f42a79e :: UsbfsHost = unsafe { gdusbfshost5f42a79e :: UsbfsHost :: from_ptr (0x5000_0400usize as _) } ; pub const USBFS_DEVICE : gdusbfsdevicec9f07fda :: UsbfsDevice = unsafe { gdusbfsdevicec9f07fda :: UsbfsDevice :: from_ptr (0x5000_0800usize as _) } ; pub const USBFS_PWRCLK : gdusbfspwrclk2ac667f0 :: UsbfsPwrclk = unsafe { gdusbfspwrclk2ac667f0 :: UsbfsPwrclk :: from_ptr (0x5000_0e00usize as _) } ; pub const EXMC : gdexmce9c487de :: Exmc = unsafe { gdexmce9c487de :: Exmc :: from_ptr (0xa000_0000usize as _) } ; pub const DBG : gddbg56ea3fe0 :: Dbg = unsafe { gddbg56ea3fe0 :: Dbg :: from_ptr (0xe004_5000usize as _) } ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;#[path="../../peripherals/gdadc05f794981_v1.rs"] pub mod gdadc05f794981;
#[path="../../peripherals/gdafio220b0195_v1.rs"] pub mod gdafio220b0195;
#[path="../../peripherals/gdbkp16a620e0_v1.rs"] pub mod gdbkp16a620e0;
#[path="../../peripherals/gdcan0f4404570_v1.rs"] pub mod gdcan0f4404570;
#[path="../../peripherals/gdcaue3998bcd_v1.rs"] pub mod gdcaue3998bcd;
#[path="../../peripherals/gdcmpfc67f344_v1.rs"] pub mod gdcmpfc67f344;
#[path="../../peripherals/gdcrc2255b0ef_v1.rs"] pub mod gdcrc2255b0ef;
#[path="../../peripherals/gdctc0cad8643_v1.rs"] pub mod gdctc0cad8643;
#[path="../../peripherals/gddac4d974090_v1.rs"] pub mod gddac4d974090;
#[path="../../peripherals/gddbg56ea3fe0_v1.rs"] pub mod gddbg56ea3fe0;
#[path="../../peripherals/gddma011392832_v1.rs"] pub mod gddma011392832;
#[path="../../peripherals/gddmamux84b48a9d_v1.rs"] pub mod gddmamux84b48a9d;
#[path="../../peripherals/gdexmce9c487de_v1.rs"] pub mod gdexmce9c487de;
#[path="../../peripherals/gdexti11a1be47_v1.rs"] pub mod gdexti11a1be47;
#[path="../../peripherals/gdfmc32a1a410_v1.rs"] pub mod gdfmc32a1a410;
#[path="../../peripherals/gdfwdgtdc3d0d7a_v1.rs"] pub mod gdfwdgtdc3d0d7a;
#[path="../../peripherals/gdgpioa519aa242_v1.rs"] pub mod gdgpioa519aa242;
#[path="../../peripherals/gdhau0e6a9f22_v1.rs"] pub mod gdhau0e6a9f22;
#[path="../../peripherals/gdi2c0932dc70f_v1.rs"] pub mod gdi2c0932dc70f;
#[path="../../peripherals/gdpmu82ca625b_v1.rs"] pub mod gdpmu82ca625b;
#[path="../../peripherals/gdrcudff54106_v1.rs"] pub mod gdrcudff54106;
#[path="../../peripherals/gdrtc6b0c077c_v1.rs"] pub mod gdrtc6b0c077c;
#[path="../../peripherals/gdspi0ad5e2dff_v1.rs"] pub mod gdspi0ad5e2dff;
#[path="../../peripherals/gdsyscfg14885d82_v1.rs"] pub mod gdsyscfg14885d82;
#[path="../../peripherals/gdtimer0debe3394_v1.rs"] pub mod gdtimer0debe3394;
#[path="../../peripherals/gdtimer12e5cd301_v1.rs"] pub mod gdtimer12e5cd301;
#[path="../../peripherals/gdtimer15296d56bf_v1.rs"] pub mod gdtimer15296d56bf;
#[path="../../peripherals/gdtimer54b5e73ec_v1.rs"] pub mod gdtimer54b5e73ec;
#[path="../../peripherals/gdtrigsel7c239a51_v1.rs"] pub mod gdtrigsel7c239a51;
#[path="../../peripherals/gdtrng8cefada9_v1.rs"] pub mod gdtrng8cefada9;
#[path="../../peripherals/gduart31f2ca0ee_v1.rs"] pub mod gduart31f2ca0ee;
#[path="../../peripherals/gdusart0b635b392_v1.rs"] pub mod gdusart0b635b392;
#[path="../../peripherals/gdusbfsdevicec9f07fda_v1.rs"] pub mod gdusbfsdevicec9f07fda;
#[path="../../peripherals/gdusbfsglobald97a6bbd_v1.rs"] pub mod gdusbfsglobald97a6bbd;
#[path="../../peripherals/gdusbfshost5f42a79e_v1.rs"] pub mod gdusbfshost5f42a79e;
#[path="../../peripherals/gdusbfspwrclk2ac667f0_v1.rs"] pub mod gdusbfspwrclk2ac667f0;
#[path="../../peripherals/gdwwdgt5c5bacde_v1.rs"] pub mod gdwwdgt5c5bacde;
