#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - VAVD_LVD_VOVD"]
    VAVD_LVD_VOVD = 1,
    #[doc = "2 - TAMPER_STAMP_LXTAL"]
    TAMPER_STAMP_LXTAL = 2,
    #[doc = "3 - RTC_WKUP"]
    RTC_WKUP = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_CHANNEL0"]
    DMA1_CHANNEL0 = 11,
    #[doc = "12 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 12,
    #[doc = "13 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 13,
    #[doc = "14 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 14,
    #[doc = "15 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 15,
    #[doc = "16 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 16,
    #[doc = "17 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 17,
    #[doc = "18 - ADC1_2"]
    ADC1_2 = 18,
    #[doc = "23 - EXTI5_9"]
    EXTI5_9 = 23,
    #[doc = "24 - TIM1_BRK"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1_UP"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI10_15"]
    EXTI10_15 = 40,
    #[doc = "41 - RTC_ALARM"]
    RTC_ALARM = 41,
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 47,
    #[doc = "48 - FMC"]
    FMC = 48,
    #[doc = "49 - SDIO0"]
    SDIO0 = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - UART4"]
    UART4 = 52,
    #[doc = "53 - UART5"]
    UART5 = 53,
    #[doc = "54 - TIM6_DAC1_UDR"]
    TIM6_DAC1_UDR = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_CHANNEL0"]
    DMA2_CHANNEL0 = 56,
    #[doc = "57 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 57,
    #[doc = "58 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 58,
    #[doc = "59 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 59,
    #[doc = "60 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 60,
    #[doc = "61 - ENET0"]
    ENET0 = 61,
    #[doc = "62 - ENET0_WKUP"]
    ENET0_WKUP = 62,
    #[doc = "68 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 68,
    #[doc = "69 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 69,
    #[doc = "70 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - I2C3_EV"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3_ER"]
    I2C3_ER = 73,
    #[doc = "74 - USBHS0_EP1_OUT"]
    USBHS0_EP1_OUT = 74,
    #[doc = "75 - USBHS0_EP1_IN"]
    USBHS0_EP1_IN = 75,
    #[doc = "76 - USBHS0_WKUP"]
    USBHS0_WKUP = 76,
    #[doc = "77 - USBHS0"]
    USBHS0 = 77,
    #[doc = "78 - DCMI"]
    DCMI = 78,
    #[doc = "79 - CAU"]
    CAU = 79,
    #[doc = "80 - HAU_RNG"]
    HAU_RNG = 80,
    #[doc = "81 - FPU"]
    FPU = 81,
    #[doc = "82 - UART7"]
    UART7 = 82,
    #[doc = "83 - UART8"]
    UART8 = 83,
    #[doc = "84 - SPI4"]
    SPI4 = 84,
    #[doc = "85 - SPI5"]
    SPI5 = 85,
    #[doc = "86 - SPI6"]
    SPI6 = 86,
    #[doc = "87 - SAI1"]
    SAI1 = 87,
    #[doc = "88 - LTDC"]
    LTDC = 88,
    #[doc = "89 - LTDC_ER"]
    LTDC_ER = 89,
    #[doc = "90 - IPA"]
    IPA = 90,
    #[doc = "91 - SAI2"]
    SAI2 = 91,
    #[doc = "92 - OSPI0"]
    OSPI0 = 92,
    #[doc = "95 - I2C4_EV"]
    I2C4_EV = 95,
    #[doc = "96 - I2C4_ER"]
    I2C4_ER = 96,
    #[doc = "97 - RSPDIF"]
    RSPDIF = 97,
    #[doc = "102 - DMAMUX1_OVR"]
    DMAMUX1_OVR = 102,
    #[doc = "110 - HPDF_INT0"]
    HPDF_INT0 = 110,
    #[doc = "111 - HPDF_INT1"]
    HPDF_INT1 = 111,
    #[doc = "112 - HPDF_INT2"]
    HPDF_INT2 = 112,
    #[doc = "113 - HPDF_INT3"]
    HPDF_INT3 = 113,
    #[doc = "114 - SAI3"]
    SAI3 = 114,
    #[doc = "116 - TIM15"]
    TIM15 = 116,
    #[doc = "117 - TIM16"]
    TIM16 = 117,
    #[doc = "118 - TIM17"]
    TIM17 = 118,
    #[doc = "120 - MDIO"]
    MDIO = 120,
    #[doc = "122 - MDMA"]
    MDMA = 122,
    #[doc = "124 - SDIO1"]
    SDIO1 = 124,
    #[doc = "125 - HWSEM"]
    HWSEM = 125,
    #[doc = "127 - ADC3"]
    ADC3 = 127,
    #[doc = "137 - CMP0_1"]
    CMP0_1 = 137,
    #[doc = "144 - CRS"]
    CRS = 144,
    #[doc = "145 - RAMECCMU"]
    RAMECCMU = 145,
    #[doc = "150 - OSPI1"]
    OSPI1 = 150,
    #[doc = "151 - RTDEC0"]
    RTDEC0 = 151,
    #[doc = "152 - RTDEC1"]
    RTDEC1 = 152,
    #[doc = "153 - FAC"]
    FAC = 153,
    #[doc = "154 - TMU"]
    TMU = 154,
    #[doc = "161 - TIM23"]
    TIM23 = 161,
    #[doc = "162 - TIM24"]
    TIM24 = 162,
    #[doc = "163 - TIM31"]
    TIM31 = 163,
    #[doc = "164 - TIM32"]
    TIM32 = 164,
    #[doc = "165 - TIM41"]
    TIM41 = 165,
    #[doc = "166 - TIM42"]
    TIM42 = 166,
    #[doc = "167 - TIM43"]
    TIM43 = 167,
    #[doc = "168 - TIM44"]
    TIM44 = 168,
    #[doc = "169 - TIM45"]
    TIM45 = 169,
    #[doc = "170 - TIM51"]
    TIM51 = 170,
    #[doc = "171 - TIM52"]
    TIM52 = 171,
    #[doc = "172 - USBHS1_EP1_OUT"]
    USBHS1_EP1_OUT = 172,
    #[doc = "173 - USBHS1_EP1_IN"]
    USBHS1_EP1_IN = 173,
    #[doc = "174 - USBHS1_WKUP"]
    USBHS1_WKUP = 174,
    #[doc = "175 - USBHS1"]
    USBHS1 = 175,
    #[doc = "176 - ENET1"]
    ENET1 = 176,
    #[doc = "177 - ENET1_WKUP"]
    ENET1_WKUP = 177,
    #[doc = "179 - CAN1_WKUP"]
    CAN1_WKUP = 179,
    #[doc = "180 - CAN1_MESSAGE"]
    CAN1_MESSAGE = 180,
    #[doc = "181 - CAN1_BUSOFF"]
    CAN1_BUSOFF = 181,
    #[doc = "182 - CAN1_ERROR"]
    CAN1_ERROR = 182,
    #[doc = "183 - CAN1_FASTERROR"]
    CAN1_FASTERROR = 183,
    #[doc = "184 - CAN1_TEC"]
    CAN1_TEC = 184,
    #[doc = "185 - CAN1_REC"]
    CAN1_REC = 185,
    #[doc = "186 - CAN2_WKUP"]
    CAN2_WKUP = 186,
    #[doc = "187 - CAN2_MESSAGE"]
    CAN2_MESSAGE = 187,
    #[doc = "188 - CAN2_BUSOFF"]
    CAN2_BUSOFF = 188,
    #[doc = "189 - CAN2_ERROR"]
    CAN2_ERROR = 189,
    #[doc = "190 - CAN2_FASTERROR"]
    CAN2_FASTERROR = 190,
    #[doc = "191 - CAN2_TEC"]
    CAN2_TEC = 191,
    #[doc = "192 - CAN2_REC"]
    CAN2_REC = 192,
    #[doc = "193 - CAN3_WKUP"]
    CAN3_WKUP = 193,
    #[doc = "194 - CAN3_MESSAGE"]
    CAN3_MESSAGE = 194,
    #[doc = "195 - CAN3_BUSOFF"]
    CAN3_BUSOFF = 195,
    #[doc = "196 - CAN3_ERROR"]
    CAN3_ERROR = 196,
    #[doc = "197 - CAN3_FASTERROR"]
    CAN3_FASTERROR = 197,
    #[doc = "198 - CAN3_TEC"]
    CAN3_TEC = 198,
    #[doc = "199 - CAN3_REC"]
    CAN3_REC = 199,
    #[doc = "200 - EFUSE"]
    EFUSE = 200,
    #[doc = "201 - I2C1_WKUP"]
    I2C1_WKUP = 201,
    #[doc = "202 - I2C2_WKUP"]
    I2C2_WKUP = 202,
    #[doc = "203 - I2C3_WKUP"]
    I2C3_WKUP = 203,
    #[doc = "204 - I2C4_WKUP"]
    I2C4_WKUP = 204,
    #[doc = "205 - LPDTS"]
    LPDTS = 205,
    #[doc = "206 - LPDTS_WKUP"]
    LPDTS_WKUP = 206,
    #[doc = "207 - TIM1_DEC"]
    TIM1_DEC = 207,
    #[doc = "208 - TIM8_DEC"]
    TIM8_DEC = 208,
    #[doc = "209 - TIM2_DEC"]
    TIM2_DEC = 209,
    #[doc = "210 - TIM3_DEC"]
    TIM3_DEC = 210,
    #[doc = "211 - TIM4_DEC"]
    TIM4_DEC = 211,
    #[doc = "212 - TIM5_DEC"]
    TIM5_DEC = 212,
    #[doc = "213 - TIM23_DEC"]
    TIM23_DEC = 213,
    #[doc = "214 - TIM24_DEC"]
    TIM24_DEC = 214,
    #[doc = "215 - TIM31_DEC"]
    TIM31_DEC = 215,
    #[doc = "216 - TIM32_DEC"]
    TIM32_DEC = 216,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn VAVD_LVD_VOVD();
        fn TAMPER_STAMP_LXTAL();
        fn RTC_WKUP();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL0();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn ADC1_2();
        fn EXTI5_9();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI10_15();
        fn RTC_ALARM();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn DMA1_CHANNEL7();
        fn FMC();
        fn SDIO0();
        fn TIM5();
        fn SPI3();
        fn UART4();
        fn UART5();
        fn TIM6_DAC1_UDR();
        fn TIM7();
        fn DMA2_CHANNEL0();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn ENET0();
        fn ENET0_WKUP();
        fn DMA2_CHANNEL5();
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn USART6();
        fn I2C3_EV();
        fn I2C3_ER();
        fn USBHS0_EP1_OUT();
        fn USBHS0_EP1_IN();
        fn USBHS0_WKUP();
        fn USBHS0();
        fn DCMI();
        fn CAU();
        fn HAU_RNG();
        fn FPU();
        fn UART7();
        fn UART8();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn SAI1();
        fn LTDC();
        fn LTDC_ER();
        fn IPA();
        fn SAI2();
        fn OSPI0();
        fn I2C4_EV();
        fn I2C4_ER();
        fn RSPDIF();
        fn DMAMUX1_OVR();
        fn HPDF_INT0();
        fn HPDF_INT1();
        fn HPDF_INT2();
        fn HPDF_INT3();
        fn SAI3();
        fn TIM15();
        fn TIM16();
        fn TIM17();
        fn MDIO();
        fn MDMA();
        fn SDIO1();
        fn HWSEM();
        fn ADC3();
        fn CMP0_1();
        fn CRS();
        fn RAMECCMU();
        fn OSPI1();
        fn RTDEC0();
        fn RTDEC1();
        fn FAC();
        fn TMU();
        fn TIM23();
        fn TIM24();
        fn TIM31();
        fn TIM32();
        fn TIM41();
        fn TIM42();
        fn TIM43();
        fn TIM44();
        fn TIM45();
        fn TIM51();
        fn TIM52();
        fn USBHS1_EP1_OUT();
        fn USBHS1_EP1_IN();
        fn USBHS1_WKUP();
        fn USBHS1();
        fn ENET1();
        fn ENET1_WKUP();
        fn CAN1_WKUP();
        fn CAN1_MESSAGE();
        fn CAN1_BUSOFF();
        fn CAN1_ERROR();
        fn CAN1_FASTERROR();
        fn CAN1_TEC();
        fn CAN1_REC();
        fn CAN2_WKUP();
        fn CAN2_MESSAGE();
        fn CAN2_BUSOFF();
        fn CAN2_ERROR();
        fn CAN2_FASTERROR();
        fn CAN2_TEC();
        fn CAN2_REC();
        fn CAN3_WKUP();
        fn CAN3_MESSAGE();
        fn CAN3_BUSOFF();
        fn CAN3_ERROR();
        fn CAN3_FASTERROR();
        fn CAN3_TEC();
        fn CAN3_REC();
        fn EFUSE();
        fn I2C1_WKUP();
        fn I2C2_WKUP();
        fn I2C3_WKUP();
        fn I2C4_WKUP();
        fn LPDTS();
        fn LPDTS_WKUP();
        fn TIM1_DEC();
        fn TIM8_DEC();
        fn TIM2_DEC();
        fn TIM3_DEC();
        fn TIM4_DEC();
        fn TIM5_DEC();
        fn TIM23_DEC();
        fn TIM24_DEC();
        fn TIM31_DEC();
        fn TIM32_DEC();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 217] = [
        Vector { _handler: WWDG },
        Vector {
            _handler: VAVD_LVD_VOVD,
        },
        Vector {
            _handler: TAMPER_STAMP_LXTAL,
        },
        Vector { _handler: RTC_WKUP },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
        Vector {
            _handler: DMA1_CHANNEL0,
        },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector { _handler: ADC1_2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: EXTI5_9 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_TRG_COM },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector { _handler: EXTI10_15 },
        Vector { _handler: RTC_ALARM },
        Vector { _reserved: 0 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector { _handler: TIM8_TRG_COM },
        Vector { _handler: TIM8_CC },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: FMC },
        Vector { _handler: SDIO0 },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector {
            _handler: TIM6_DAC1_UDR,
        },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_CHANNEL0,
        },
        Vector {
            _handler: DMA2_CHANNEL1,
        },
        Vector {
            _handler: DMA2_CHANNEL2,
        },
        Vector {
            _handler: DMA2_CHANNEL3,
        },
        Vector {
            _handler: DMA2_CHANNEL4,
        },
        Vector { _handler: ENET0 },
        Vector { _handler: ENET0_WKUP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector { _handler: USART6 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector {
            _handler: USBHS0_EP1_OUT,
        },
        Vector {
            _handler: USBHS0_EP1_IN,
        },
        Vector { _handler: USBHS0_WKUP },
        Vector { _handler: USBHS0 },
        Vector { _handler: DCMI },
        Vector { _handler: CAU },
        Vector { _handler: HAU_RNG },
        Vector { _handler: FPU },
        Vector { _handler: UART7 },
        Vector { _handler: UART8 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: SAI1 },
        Vector { _handler: LTDC },
        Vector { _handler: LTDC_ER },
        Vector { _handler: IPA },
        Vector { _handler: SAI2 },
        Vector { _handler: OSPI0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: RSPDIF },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: DMAMUX1_OVR },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HPDF_INT0 },
        Vector { _handler: HPDF_INT1 },
        Vector { _handler: HPDF_INT2 },
        Vector { _handler: HPDF_INT3 },
        Vector { _handler: SAI3 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM15 },
        Vector { _handler: TIM16 },
        Vector { _handler: TIM17 },
        Vector { _reserved: 0 },
        Vector { _handler: MDIO },
        Vector { _reserved: 0 },
        Vector { _handler: MDMA },
        Vector { _reserved: 0 },
        Vector { _handler: SDIO1 },
        Vector { _handler: HWSEM },
        Vector { _reserved: 0 },
        Vector { _handler: ADC3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CMP0_1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CRS },
        Vector { _handler: RAMECCMU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: OSPI1 },
        Vector { _handler: RTDEC0 },
        Vector { _handler: RTDEC1 },
        Vector { _handler: FAC },
        Vector { _handler: TMU },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TIM23 },
        Vector { _handler: TIM24 },
        Vector { _handler: TIM31 },
        Vector { _handler: TIM32 },
        Vector { _handler: TIM41 },
        Vector { _handler: TIM42 },
        Vector { _handler: TIM43 },
        Vector { _handler: TIM44 },
        Vector { _handler: TIM45 },
        Vector { _handler: TIM51 },
        Vector { _handler: TIM52 },
        Vector {
            _handler: USBHS1_EP1_OUT,
        },
        Vector {
            _handler: USBHS1_EP1_IN,
        },
        Vector { _handler: USBHS1_WKUP },
        Vector { _handler: USBHS1 },
        Vector { _handler: ENET1 },
        Vector { _handler: ENET1_WKUP },
        Vector { _reserved: 0 },
        Vector { _handler: CAN1_WKUP },
        Vector { _handler: CAN1_MESSAGE },
        Vector { _handler: CAN1_BUSOFF },
        Vector { _handler: CAN1_ERROR },
        Vector {
            _handler: CAN1_FASTERROR,
        },
        Vector { _handler: CAN1_TEC },
        Vector { _handler: CAN1_REC },
        Vector { _handler: CAN2_WKUP },
        Vector { _handler: CAN2_MESSAGE },
        Vector { _handler: CAN2_BUSOFF },
        Vector { _handler: CAN2_ERROR },
        Vector {
            _handler: CAN2_FASTERROR,
        },
        Vector { _handler: CAN2_TEC },
        Vector { _handler: CAN2_REC },
        Vector { _handler: CAN3_WKUP },
        Vector { _handler: CAN3_MESSAGE },
        Vector { _handler: CAN3_BUSOFF },
        Vector { _handler: CAN3_ERROR },
        Vector {
            _handler: CAN3_FASTERROR,
        },
        Vector { _handler: CAN3_TEC },
        Vector { _handler: CAN3_REC },
        Vector { _handler: EFUSE },
        Vector { _handler: I2C1_WKUP },
        Vector { _handler: I2C2_WKUP },
        Vector { _handler: I2C3_WKUP },
        Vector { _handler: I2C4_WKUP },
        Vector { _handler: LPDTS },
        Vector { _handler: LPDTS_WKUP },
        Vector { _handler: TIM1_DEC },
        Vector { _handler: TIM8_DEC },
        Vector { _handler: TIM2_DEC },
        Vector { _handler: TIM3_DEC },
        Vector { _handler: TIM4_DEC },
        Vector { _handler: TIM5_DEC },
        Vector { _handler: TIM23_DEC },
        Vector { _handler: TIM24_DEC },
        Vector { _handler: TIM31_DEC },
        Vector { _handler: TIM32_DEC },
    ];
}
pub const TIM2: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::TimGp32 = unsafe { timer::TimGp32::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: gdtimer5330a987e::Timer5 = unsafe { gdtimer5330a987e::Timer5::from_ptr(0x4000_1000usize as _) };
pub const TIM7: gdtimer5330a987e::Timer5 = unsafe { gdtimer5330a987e::Timer5::from_ptr(0x4000_1400usize as _) };
pub const SPI2: gdspi1356222e3::Spi1 = unsafe { gdspi1356222e3::Spi1::from_ptr(0x4000_3800usize as _) };
pub const SPI3: gdspi255cb8c1f::Spi2 = unsafe { gdspi255cb8c1f::Spi2::from_ptr(0x4000_3c00usize as _) };
pub const RSPDIF: gdrspdif9ce23832::Rspdif = unsafe { gdrspdif9ce23832::Rspdif::from_ptr(0x4000_4000usize as _) };
pub const USART2: gdusart0626fb765::Usart0 = unsafe { gdusart0626fb765::Usart0::from_ptr(0x4000_4400usize as _) };
pub const USART3: gdusart0626fb765::Usart0 = unsafe { gdusart0626fb765::Usart0::from_ptr(0x4000_4800usize as _) };
pub const UART4: gduart330e38640::Uart3 = unsafe { gduart330e38640::Uart3::from_ptr(0x4000_4c00usize as _) };
pub const UART5: gduart330e38640::Uart3 = unsafe { gduart330e38640::Uart3::from_ptr(0x4000_5000usize as _) };
pub const I2C1: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_5400usize as _) };
pub const I2C2: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_5800usize as _) };
pub const I2C4: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_5c00usize as _) };
pub const DAC1: gddac555b6194::Dac = unsafe { gddac555b6194::Dac::from_ptr(0x4000_7400usize as _) };
pub const UART7: gduart330e38640::Uart3 = unsafe { gduart330e38640::Uart3::from_ptr(0x4000_7800usize as _) };
pub const UART8: gduart330e38640::Uart3 = unsafe { gduart330e38640::Uart3::from_ptr(0x4000_7c00usize as _) };
pub const CRS: gdctcdb80f1ce::Ctc = unsafe { gdctcdb80f1ce::Ctc::from_ptr(0x4000_8400usize as _) };
pub const MDIO: gdmdio2685003f::Mdio = unsafe { gdmdio2685003f::Mdio::from_ptr(0x4000_9400usize as _) };
pub const I2C3: gdi2c0cd973dc4::I2c0 = unsafe { gdi2c0cd973dc4::I2c0::from_ptr(0x4000_c000usize as _) };
pub const TIM23: gdtimer1da3bc56a::Timer1 = unsafe { gdtimer1da3bc56a::Timer1::from_ptr(0x4000_e000usize as _) };
pub const TIM24: gdtimer1da3bc56a::Timer1 = unsafe { gdtimer1da3bc56a::Timer1::from_ptr(0x4000_e400usize as _) };
pub const TIM31: gdtimer27201f8c9::Timer2 = unsafe { gdtimer27201f8c9::Timer2::from_ptr(0x4000_e800usize as _) };
pub const TIM32: gdtimer27201f8c9::Timer2 = unsafe { gdtimer27201f8c9::Timer2::from_ptr(0x4000_ec00usize as _) };
pub const TIM51: gdtimer5071732508::Timer50 = unsafe { gdtimer5071732508::Timer50::from_ptr(0x4000_f000usize as _) };
pub const TIM52: gdtimer5071732508::Timer50 = unsafe { gdtimer5071732508::Timer50::from_ptr(0x4000_f400usize as _) };
pub const TIM1: timer::TimAdv = unsafe { timer::TimAdv::from_ptr(0x4001_0000usize as _) };
pub const TIM8: gdtimer03afad14d::Timer0 = unsafe { gdtimer03afad14d::Timer0::from_ptr(0x4001_0400usize as _) };
pub const USART1: gdusart0626fb765::Usart0 = unsafe { gdusart0626fb765::Usart0::from_ptr(0x4001_1000usize as _) };
pub const USART6: gdusart0626fb765::Usart0 = unsafe { gdusart0626fb765::Usart0::from_ptr(0x4001_1400usize as _) };
pub const ADC1: gdadc06d279556::Adc0 = unsafe { gdadc06d279556::Adc0::from_ptr(0x4001_2400usize as _) };
pub const ADC2: gdadc1425a4aff::Adc1 = unsafe { gdadc1425a4aff::Adc1::from_ptr(0x4001_2800usize as _) };
pub const ADC3: gdadc2efea3dc8::Adc2 = unsafe { gdadc2efea3dc8::Adc2::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: gdspi0a7377dd5::Spi0 = unsafe { gdspi0a7377dd5::Spi0::from_ptr(0x4001_3000usize as _) };
pub const SPI4: gdspi3e9b78823::Spi3 = unsafe { gdspi3e9b78823::Spi3::from_ptr(0x4001_3400usize as _) };
pub const SPI6: gdspi5c82f56e6::Spi5 = unsafe { gdspi5c82f56e6::Spi5::from_ptr(0x4001_3800usize as _) };
pub const TIM15: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_4000usize as _) };
pub const TIM16: gdtimer155d5134ba::Timer15 = unsafe { gdtimer155d5134ba::Timer15::from_ptr(0x4001_4400usize as _) };
pub const TIM17: gdtimer155d5134ba::Timer15 = unsafe { gdtimer155d5134ba::Timer15::from_ptr(0x4001_4800usize as _) };
pub const SPI5: gdspi4af049e38::Spi4 = unsafe { gdspi4af049e38::Spi4::from_ptr(0x4001_5000usize as _) };
pub const SAI1: gdsai06e25733b::Sai0 = unsafe { gdsai06e25733b::Sai0::from_ptr(0x4001_5800usize as _) };
pub const SAI2: gdsai06e25733b::Sai0 = unsafe { gdsai06e25733b::Sai0::from_ptr(0x4001_5c00usize as _) };
pub const SAI3: gdsai06e25733b::Sai0 = unsafe { gdsai06e25733b::Sai0::from_ptr(0x4001_6000usize as _) };
pub const HPDF: gdhpdffd9de252::Hpdf = unsafe { gdhpdffd9de252::Hpdf::from_ptr(0x4001_7000usize as _) };
pub const TRIGSEL: gdtrigseldfb10546::Trigsel = unsafe { gdtrigseldfb10546::Trigsel::from_ptr(0x4001_8400usize as _) };
pub const EDOUT: gdedoutfebca4f4::Edout = unsafe { gdedoutfebca4f4::Edout::from_ptr(0x4001_8800usize as _) };
pub const CAN1: gdcan0ab6ea0b5::Can0 = unsafe { gdcan0ab6ea0b5::Can0::from_ptr(0x4001_a000usize as _) };
pub const CAN2: gdcan0ab6ea0b5::Can0 = unsafe { gdcan0ab6ea0b5::Can0::from_ptr(0x4001_b000usize as _) };
pub const CAN3: gdcan0ab6ea0b5::Can0 = unsafe { gdcan0ab6ea0b5::Can0::from_ptr(0x4001_c000usize as _) };
pub const TIM41: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_d000usize as _) };
pub const TIM42: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_d400usize as _) };
pub const TIM43: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_d800usize as _) };
pub const TIM44: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_dc00usize as _) };
pub const TIM45: gdtimer1457881844::Timer14 = unsafe { gdtimer1457881844::Timer14::from_ptr(0x4001_f000usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0400usize as _) };
pub const DMAMUX1: gddmamuxeaace10d::Dmamux = unsafe { gddmamuxeaace10d::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const EFUSE: gdefuse25c60075::Efuse = unsafe { gdefuse25c60075::Efuse::from_ptr(0x4002_2800usize as _) };
pub const ENET0_MAC: gdenet0macd2855220::Enet0Mac =
    unsafe { gdenet0macd2855220::Enet0Mac::from_ptr(0x4002_8000usize as _) };
pub const ENET0_MSC: gdenet0msc2451d465::Enet0Msc =
    unsafe { gdenet0msc2451d465::Enet0Msc::from_ptr(0x4002_8100usize as _) };
pub const ENET0_PTP: gdenet0ptpc700182c::Enet0Ptp =
    unsafe { gdenet0ptpc700182c::Enet0Ptp::from_ptr(0x4002_8700usize as _) };
pub const ENET0_DMA: gdenet0dma7d3e05fd::Enet0Dma =
    unsafe { gdenet0dma7d3e05fd::Enet0Dma::from_ptr(0x4002_9000usize as _) };
pub const ENET0_MAC_FCTH: gdenet0macfcthffd74812::Enet0MacFcth =
    unsafe { gdenet0macfcthffd74812::Enet0MacFcth::from_ptr(0x4002_9080usize as _) };
pub const ENET1_MAC: gdenet1maceef08a3b::Enet1Mac =
    unsafe { gdenet1maceef08a3b::Enet1Mac::from_ptr(0x4002_a000usize as _) };
pub const ENET1_MSC: gdenet1msc4852d4b8::Enet1Msc =
    unsafe { gdenet1msc4852d4b8::Enet1Msc::from_ptr(0x4002_a100usize as _) };
pub const ENET1_PTP: gdenet1ptpedbe1f92::Enet1Ptp =
    unsafe { gdenet1ptpedbe1f92::Enet1Ptp::from_ptr(0x4002_a700usize as _) };
pub const ENET1_DMA: gdenet1dmabfdb3976::Enet1Dma =
    unsafe { gdenet1dmabfdb3976::Enet1Dma::from_ptr(0x4002_b000usize as _) };
pub const ENET1_MAC_FCTH: gdenet1macfcthacf2ccdd::Enet1MacFcth =
    unsafe { gdenet1macfcthacf2ccdd::Enet1MacFcth::from_ptr(0x4002_b080usize as _) };
pub const USBHS0_GLOBAL: gdusbhs0globalbee3a389::Usbhs0Global =
    unsafe { gdusbhs0globalbee3a389::Usbhs0Global::from_ptr(0x4004_0000usize as _) };
pub const USBHS0_HOST: gdusbhs0host663109ac::Usbhs0Host =
    unsafe { gdusbhs0host663109ac::Usbhs0Host::from_ptr(0x4004_0400usize as _) };
pub const USBHS0_DEVICE: gdusbhs0deviced0449d15::Usbhs0Device =
    unsafe { gdusbhs0deviced0449d15::Usbhs0Device::from_ptr(0x4004_0800usize as _) };
pub const USBHS0_PWRCLK: gdusbhs0pwrclk0f97dd8b::Usbhs0Pwrclk =
    unsafe { gdusbhs0pwrclk0f97dd8b::Usbhs0Pwrclk::from_ptr(0x4004_0e00usize as _) };
pub const USBHS1_GLOBAL: gdusbhs1globalb3d6824e::Usbhs1Global =
    unsafe { gdusbhs1globalb3d6824e::Usbhs1Global::from_ptr(0x4008_0000usize as _) };
pub const USBHS1_HOST: gdusbhs1host14113081::Usbhs1Host =
    unsafe { gdusbhs1host14113081::Usbhs1Host::from_ptr(0x4008_0400usize as _) };
pub const USBHS1_DEVICE: gdusbhs1device9d406887::Usbhs1Device =
    unsafe { gdusbhs1device9d406887::Usbhs1Device::from_ptr(0x4008_0800usize as _) };
pub const USBHS1_PWRCLK: gdusbhs1pwrclk29fc276e::Usbhs1Pwrclk =
    unsafe { gdusbhs1pwrclk29fc276e::Usbhs1Pwrclk::from_ptr(0x4008_0e00usize as _) };
pub const DCMI: gddci5ae31085::Dci = unsafe { gddci5ae31085::Dci::from_ptr(0x4802_0000usize as _) };
pub const CAU: gdcau3fafd38d::Cau = unsafe { gdcau3fafd38d::Cau::from_ptr(0x4802_1000usize as _) };
pub const HAU: gdhaub8125197::Hau = unsafe { gdhaub8125197::Hau::from_ptr(0x4802_1400usize as _) };
pub const RNG: gdtrng6bc6a907::Trng = unsafe { gdtrng6bc6a907::Trng::from_ptr(0x4802_1800usize as _) };
pub const SDIO1: gdsdio042a58275::Sdio0 = unsafe { gdsdio042a58275::Sdio0::from_ptr(0x4802_2400usize as _) };
pub const CPDM_SDIO1: gdcpdmsdio04a9ee533::CpdmSdio0 =
    unsafe { gdcpdmsdio04a9ee533::CpdmSdio0::from_ptr(0x4802_2800usize as _) };
pub const RAMECCMU1: gdrameccmu1ba654536::Rameccmu1 =
    unsafe { gdrameccmu1ba654536::Rameccmu1::from_ptr(0x4802_3000usize as _) };
pub const TMU: gdtmucbc214df::Tmu = unsafe { gdtmucbc214df::Tmu::from_ptr(0x4802_4400usize as _) };
pub const FAC: gdfac96d60f19::Fac = unsafe { gdfac96d60f19::Fac::from_ptr(0x4802_4800usize as _) };
pub const LTDC: gdtli3a8126bb::Tli = unsafe { gdtli3a8126bb::Tli::from_ptr(0x5000_1000usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x5000_3000usize as _) };
pub const AXI: gdaxi7000de15::Axi = unsafe { gdaxi7000de15::Axi::from_ptr(0x5100_0000usize as _) };
pub const MDMA: gdmdmab9a60aaf::Mdma = unsafe { gdmdmab9a60aaf::Mdma::from_ptr(0x5200_0000usize as _) };
pub const IPA: gdipae01bd374::Ipa = unsafe { gdipae01bd374::Ipa::from_ptr(0x5200_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x5200_2000usize as _) };
pub const FMC: gdexmc293e9145::Exmc = unsafe { gdexmc293e9145::Exmc::from_ptr(0x5200_4000usize as _) };
pub const OSPI0: gdospi0439e0312::Ospi0 = unsafe { gdospi0439e0312::Ospi0::from_ptr(0x5200_5000usize as _) };
pub const SDIO0: gdsdio042a58275::Sdio0 = unsafe { gdsdio042a58275::Sdio0::from_ptr(0x5200_7000usize as _) };
pub const CPDM_SDIO0: gdcpdmsdio04a9ee533::CpdmSdio0 =
    unsafe { gdcpdmsdio04a9ee533::CpdmSdio0::from_ptr(0x5200_8000usize as _) };
pub const RAMECCMU0: gdrameccmu0d260ef4c::Rameccmu0 =
    unsafe { gdrameccmu0d260ef4c::Rameccmu0::from_ptr(0x5200_9000usize as _) };
pub const OSPI1: gdospi0439e0312::Ospi0 = unsafe { gdospi0439e0312::Ospi0::from_ptr(0x5200_a000usize as _) };
pub const OSPIM: gdospim63e4b4c4::Ospim = unsafe { gdospim63e4b4c4::Ospim::from_ptr(0x5200_b400usize as _) };
pub const RTDEC0: gdrtdec0fa1e67ae::Rtdec0 = unsafe { gdrtdec0fa1e67ae::Rtdec0::from_ptr(0x5200_b800usize as _) };
pub const RTDEC1: gdrtdec1b5caa4c1::Rtdec1 = unsafe { gdrtdec1b5caa4c1::Rtdec1::from_ptr(0x5200_bc00usize as _) };
pub const EXTI: gdextic827d627::Exti = unsafe { gdextic827d627::Exti::from_ptr(0x5800_0000usize as _) };
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x5800_0400usize as _) };
pub const CMP: gdcmp65ef540c::Cmp = unsafe { gdcmp65ef540c::Cmp::from_ptr(0x5800_3800usize as _) };
pub const VREF: gdvref193fa1c3::Vref = unsafe { gdvref193fa1c3::Vref::from_ptr(0x5800_3c00usize as _) };
pub const RTC: gdrtcc8139290::Rtc = unsafe { gdrtcc8139290::Rtc::from_ptr(0x5800_4000usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x5800_4800usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x5800_5800usize as _) };
pub const LPDTS: gdlpdtsa3b40577::Lpdts = unsafe { gdlpdtsa3b40577::Lpdts::from_ptr(0x5800_6800usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0000usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0400usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0800usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_0c00usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1000usize as _) };
pub const GPIOF: gdgpioc18dfc69f::Gpioc = unsafe { gdgpioc18dfc69f::Gpioc::from_ptr(0x5802_1400usize as _) };
pub const GPIOG: gdgpioc18dfc69f::Gpioc = unsafe { gdgpioc18dfc69f::Gpioc::from_ptr(0x5802_1800usize as _) };
pub const GPIOH: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x5802_1c00usize as _) };
pub const GPIOJ: gdgpioc18dfc69f::Gpioc = unsafe { gdgpioc18dfc69f::Gpioc::from_ptr(0x5802_2400usize as _) };
pub const GPIOK: gdgpioc18dfc69f::Gpioc = unsafe { gdgpioc18dfc69f::Gpioc::from_ptr(0x5802_2800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x5802_4400usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x5802_4c00usize as _) };
pub const HWSEM: gdhwsem5325a440::Hwsem = unsafe { gdhwsem5325a440::Hwsem::from_ptr(0x5802_6400usize as _) };
pub const DBGMCU: gddbgefa81966::Dbg = unsafe { gddbgefa81966::Dbg::from_ptr(0xe00e_1000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[path = "../../peripherals/crc_v1.rs"]
pub mod crc;
#[path = "../../peripherals/dma_v2.rs"]
pub mod dma;
#[path = "../../peripherals/flash_f4.rs"]
pub mod flash;
#[path = "../../peripherals/gdadc06d279556_v1.rs"]
pub mod gdadc06d279556;
#[path = "../../peripherals/gdadc1425a4aff_v1.rs"]
pub mod gdadc1425a4aff;
#[path = "../../peripherals/gdadc2efea3dc8_v1.rs"]
pub mod gdadc2efea3dc8;
#[path = "../../peripherals/gdaxi7000de15_v1.rs"]
pub mod gdaxi7000de15;
#[path = "../../peripherals/gdcan0ab6ea0b5_v1.rs"]
pub mod gdcan0ab6ea0b5;
#[path = "../../peripherals/gdcau3fafd38d_v1.rs"]
pub mod gdcau3fafd38d;
#[path = "../../peripherals/gdcmp65ef540c_v1.rs"]
pub mod gdcmp65ef540c;
#[path = "../../peripherals/gdcpdmsdio04a9ee533_v1.rs"]
pub mod gdcpdmsdio04a9ee533;
#[path = "../../peripherals/gdctcdb80f1ce_v1.rs"]
pub mod gdctcdb80f1ce;
#[path = "../../peripherals/gddac555b6194_v1.rs"]
pub mod gddac555b6194;
#[path = "../../peripherals/gddbgefa81966_v1.rs"]
pub mod gddbgefa81966;
#[path = "../../peripherals/gddci5ae31085_v1.rs"]
pub mod gddci5ae31085;
#[path = "../../peripherals/gddmamuxeaace10d_v1.rs"]
pub mod gddmamuxeaace10d;
#[path = "../../peripherals/gdedoutfebca4f4_v1.rs"]
pub mod gdedoutfebca4f4;
#[path = "../../peripherals/gdefuse25c60075_v1.rs"]
pub mod gdefuse25c60075;
#[path = "../../peripherals/gdenet0dma7d3e05fd_v1.rs"]
pub mod gdenet0dma7d3e05fd;
#[path = "../../peripherals/gdenet0macd2855220_v1.rs"]
pub mod gdenet0macd2855220;
#[path = "../../peripherals/gdenet0macfcthffd74812_v1.rs"]
pub mod gdenet0macfcthffd74812;
#[path = "../../peripherals/gdenet0msc2451d465_v1.rs"]
pub mod gdenet0msc2451d465;
#[path = "../../peripherals/gdenet0ptpc700182c_v1.rs"]
pub mod gdenet0ptpc700182c;
#[path = "../../peripherals/gdenet1dmabfdb3976_v1.rs"]
pub mod gdenet1dmabfdb3976;
#[path = "../../peripherals/gdenet1maceef08a3b_v1.rs"]
pub mod gdenet1maceef08a3b;
#[path = "../../peripherals/gdenet1macfcthacf2ccdd_v1.rs"]
pub mod gdenet1macfcthacf2ccdd;
#[path = "../../peripherals/gdenet1msc4852d4b8_v1.rs"]
pub mod gdenet1msc4852d4b8;
#[path = "../../peripherals/gdenet1ptpedbe1f92_v1.rs"]
pub mod gdenet1ptpedbe1f92;
#[path = "../../peripherals/gdexmc293e9145_v1.rs"]
pub mod gdexmc293e9145;
#[path = "../../peripherals/gdextic827d627_v1.rs"]
pub mod gdextic827d627;
#[path = "../../peripherals/gdfac96d60f19_v1.rs"]
pub mod gdfac96d60f19;
#[path = "../../peripherals/gdgpioc18dfc69f_v1.rs"]
pub mod gdgpioc18dfc69f;
#[path = "../../peripherals/gdhaub8125197_v1.rs"]
pub mod gdhaub8125197;
#[path = "../../peripherals/gdhpdffd9de252_v1.rs"]
pub mod gdhpdffd9de252;
#[path = "../../peripherals/gdhwsem5325a440_v1.rs"]
pub mod gdhwsem5325a440;
#[path = "../../peripherals/gdi2c0cd973dc4_v1.rs"]
pub mod gdi2c0cd973dc4;
#[path = "../../peripherals/gdipae01bd374_v1.rs"]
pub mod gdipae01bd374;
#[path = "../../peripherals/gdlpdtsa3b40577_v1.rs"]
pub mod gdlpdtsa3b40577;
#[path = "../../peripherals/gdmdio2685003f_v1.rs"]
pub mod gdmdio2685003f;
#[path = "../../peripherals/gdmdmab9a60aaf_v1.rs"]
pub mod gdmdmab9a60aaf;
#[path = "../../peripherals/gdospi0439e0312_v1.rs"]
pub mod gdospi0439e0312;
#[path = "../../peripherals/gdospim63e4b4c4_v1.rs"]
pub mod gdospim63e4b4c4;
#[path = "../../peripherals/gdrameccmu0d260ef4c_v1.rs"]
pub mod gdrameccmu0d260ef4c;
#[path = "../../peripherals/gdrameccmu1ba654536_v1.rs"]
pub mod gdrameccmu1ba654536;
#[path = "../../peripherals/gdrspdif9ce23832_v1.rs"]
pub mod gdrspdif9ce23832;
#[path = "../../peripherals/gdrtcc8139290_v1.rs"]
pub mod gdrtcc8139290;
#[path = "../../peripherals/gdrtdec0fa1e67ae_v1.rs"]
pub mod gdrtdec0fa1e67ae;
#[path = "../../peripherals/gdrtdec1b5caa4c1_v1.rs"]
pub mod gdrtdec1b5caa4c1;
#[path = "../../peripherals/gdsai06e25733b_v1.rs"]
pub mod gdsai06e25733b;
#[path = "../../peripherals/gdsdio042a58275_v1.rs"]
pub mod gdsdio042a58275;
#[path = "../../peripherals/gdspi0a7377dd5_v1.rs"]
pub mod gdspi0a7377dd5;
#[path = "../../peripherals/gdspi1356222e3_v1.rs"]
pub mod gdspi1356222e3;
#[path = "../../peripherals/gdspi255cb8c1f_v1.rs"]
pub mod gdspi255cb8c1f;
#[path = "../../peripherals/gdspi3e9b78823_v1.rs"]
pub mod gdspi3e9b78823;
#[path = "../../peripherals/gdspi4af049e38_v1.rs"]
pub mod gdspi4af049e38;
#[path = "../../peripherals/gdspi5c82f56e6_v1.rs"]
pub mod gdspi5c82f56e6;
#[path = "../../peripherals/gdtimer03afad14d_v1.rs"]
pub mod gdtimer03afad14d;
#[path = "../../peripherals/gdtimer1457881844_v1.rs"]
pub mod gdtimer1457881844;
#[path = "../../peripherals/gdtimer155d5134ba_v1.rs"]
pub mod gdtimer155d5134ba;
#[path = "../../peripherals/gdtimer1da3bc56a_v1.rs"]
pub mod gdtimer1da3bc56a;
#[path = "../../peripherals/gdtimer27201f8c9_v1.rs"]
pub mod gdtimer27201f8c9;
#[path = "../../peripherals/gdtimer5071732508_v1.rs"]
pub mod gdtimer5071732508;
#[path = "../../peripherals/gdtimer5330a987e_v1.rs"]
pub mod gdtimer5330a987e;
#[path = "../../peripherals/gdtli3a8126bb_v1.rs"]
pub mod gdtli3a8126bb;
#[path = "../../peripherals/gdtmucbc214df_v1.rs"]
pub mod gdtmucbc214df;
#[path = "../../peripherals/gdtrigseldfb10546_v1.rs"]
pub mod gdtrigseldfb10546;
#[path = "../../peripherals/gdtrng6bc6a907_v1.rs"]
pub mod gdtrng6bc6a907;
#[path = "../../peripherals/gduart330e38640_v1.rs"]
pub mod gduart330e38640;
#[path = "../../peripherals/gdusart0626fb765_v1.rs"]
pub mod gdusart0626fb765;
#[path = "../../peripherals/gdusbhs0deviced0449d15_v1.rs"]
pub mod gdusbhs0deviced0449d15;
#[path = "../../peripherals/gdusbhs0globalbee3a389_v1.rs"]
pub mod gdusbhs0globalbee3a389;
#[path = "../../peripherals/gdusbhs0host663109ac_v1.rs"]
pub mod gdusbhs0host663109ac;
#[path = "../../peripherals/gdusbhs0pwrclk0f97dd8b_v1.rs"]
pub mod gdusbhs0pwrclk0f97dd8b;
#[path = "../../peripherals/gdusbhs1device9d406887_v1.rs"]
pub mod gdusbhs1device9d406887;
#[path = "../../peripherals/gdusbhs1globalb3d6824e_v1.rs"]
pub mod gdusbhs1globalb3d6824e;
#[path = "../../peripherals/gdusbhs1host14113081_v1.rs"]
pub mod gdusbhs1host14113081;
#[path = "../../peripherals/gdusbhs1pwrclk29fc276e_v1.rs"]
pub mod gdusbhs1pwrclk29fc276e;
#[path = "../../peripherals/gdvref193fa1c3_v1.rs"]
pub mod gdvref193fa1c3;
#[path = "../../peripherals/gpio_v2.rs"]
pub mod gpio;
#[path = "../../peripherals/iwdg_v1.rs"]
pub mod iwdg;
#[path = "../../peripherals/pwr_f4.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_f4.rs"]
pub mod rcc;
#[path = "../../peripherals/syscfg_f4.rs"]
pub mod syscfg;
#[path = "../../peripherals/timer_v1.rs"]
pub mod timer;
#[path = "../../peripherals/wwdg_v1.rs"]
pub mod wwdg;
