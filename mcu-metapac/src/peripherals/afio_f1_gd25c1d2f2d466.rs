#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afio {
    ptr: *mut u8,
}
unsafe impl Send for Afio {}
unsafe impl Sync for Afio {}
impl Afio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn afioafioec(self) -> crate::common::Reg<regs::Afioafioec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event control register"]
    #[inline(always)]
    pub const fn afioec(self) -> crate::common::Reg<regs::Afioec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[inline(always)]
    pub const fn evcr(self) -> crate::common::Reg<regs::Evcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn afioafiopcf0(self) -> crate::common::Reg<regs::Afioafiopcf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "AFIO port configuration register 0"]
    #[inline(always)]
    pub const fn afiopcf0(self) -> crate::common::Reg<regs::Afiopcf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[inline(always)]
    pub const fn mapr(self) -> crate::common::Reg<regs::Mapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn afioafioextiss0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "EXTI sources selection register 0"]
    #[inline(always)]
    pub const fn afioextiss0(self) -> crate::common::Reg<regs::Afioextiss0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn afioafioextiss1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "EXTI sources selection register 1"]
    #[inline(always)]
    pub const fn afioextiss1(self) -> crate::common::Reg<regs::Afioextiss1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn afioafioextiss2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "EXTI sources selection register 2"]
    #[inline(always)]
    pub const fn afioextiss2(self) -> crate::common::Reg<regs::Afioextiss2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn afioafioextiss3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "EXTI sources selection register 3"]
    #[inline(always)]
    pub const fn afioextiss3(self) -> crate::common::Reg<regs::Afioextiss3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn afioafiopcf1(self) -> crate::common::Reg<regs::Afioafiopcf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "AFIO port configuration register 1"]
    #[inline(always)]
    pub const fn afiopcf1(self) -> crate::common::Reg<regs::Afiopcf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[inline(always)]
    pub const fn mapr2(self) -> crate::common::Reg<regs::Mapr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn afioafiocpsctl(self) -> crate::common::Reg<regs::Afioafiocpsctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "IO compensation control register"]
    #[inline(always)]
    pub const fn afiocpsctl(self) -> crate::common::Reg<regs::Afiocpsctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioafiocpsctl(pub u32);
    impl Afioafiocpsctl {
        #[must_use]
        #[inline(always)]
        pub const fn afio_cpsctl_cps_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_cpsctl_cps_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_cpsctl_cps_rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_cpsctl_cps_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Afioafiocpsctl {
        #[inline(always)]
        fn default() -> Afioafiocpsctl {
            Afioafiocpsctl(0)
        }
    }
    impl core::fmt::Debug for Afioafiocpsctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioafiocpsctl")
                .field("afio_cpsctl_cps_en", &self.afio_cpsctl_cps_en())
                .field("afio_cpsctl_cps_rdy", &self.afio_cpsctl_cps_rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioafiocpsctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioafiocpsctl {{ afio_cpsctl_cps_en: {=bool:?}, afio_cpsctl_cps_rdy: {=bool:?} }}",
                self.afio_cpsctl_cps_en(),
                self.afio_cpsctl_cps_rdy()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioafioec(pub u32);
    impl Afioafioec {
        #[must_use]
        #[inline(always)]
        pub const fn afio_ec_pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_ec_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_ec_port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_ec_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_ec_eoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_ec_eoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Afioafioec {
        #[inline(always)]
        fn default() -> Afioafioec {
            Afioafioec(0)
        }
    }
    impl core::fmt::Debug for Afioafioec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioafioec")
                .field("afio_ec_pin", &self.afio_ec_pin())
                .field("afio_ec_port", &self.afio_ec_port())
                .field("afio_ec_eoe", &self.afio_ec_eoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioafioec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioafioec {{ afio_ec_pin: {=u8:?}, afio_ec_port: {=u8:?}, afio_ec_eoe: {=bool:?} }}",
                self.afio_ec_pin(),
                self.afio_ec_port(),
                self.afio_ec_eoe()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioafiopcf0(pub u32);
    impl Afioafiopcf0 {
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_spi0_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_spi0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_i2c0_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_i2c0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_usart0_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_usart0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_usart1_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_usart1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_usart2_remap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_usart2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer0_remap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer0_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer1_remap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer2_remap(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer3_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer3_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_can0_remap(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_can0_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_pd01_remap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_pd01_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer4ch3_iremap(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer4ch3_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_enet_remap(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_enet_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_can1_remap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_can1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_enet_phy_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_enet_phy_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_swj_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_swj_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_spi2_remap(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_spi2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_timer1iti1_remap(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_timer1iti1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf0_ptp_pps_remap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf0_ptp_pps_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Afioafiopcf0 {
        #[inline(always)]
        fn default() -> Afioafiopcf0 {
            Afioafiopcf0(0)
        }
    }
    impl core::fmt::Debug for Afioafiopcf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioafiopcf0")
                .field("afio_pcf0_spi0_remap", &self.afio_pcf0_spi0_remap())
                .field("afio_pcf0_i2c0_remap", &self.afio_pcf0_i2c0_remap())
                .field("afio_pcf0_usart0_remap", &self.afio_pcf0_usart0_remap())
                .field("afio_pcf0_usart1_remap", &self.afio_pcf0_usart1_remap())
                .field("afio_pcf0_usart2_remap", &self.afio_pcf0_usart2_remap())
                .field("afio_pcf0_timer0_remap", &self.afio_pcf0_timer0_remap())
                .field("afio_pcf0_timer1_remap", &self.afio_pcf0_timer1_remap())
                .field("afio_pcf0_timer2_remap", &self.afio_pcf0_timer2_remap())
                .field("afio_pcf0_timer3_remap", &self.afio_pcf0_timer3_remap())
                .field("afio_pcf0_can0_remap", &self.afio_pcf0_can0_remap())
                .field("afio_pcf0_pd01_remap", &self.afio_pcf0_pd01_remap())
                .field("afio_pcf0_timer4ch3_iremap", &self.afio_pcf0_timer4ch3_iremap())
                .field("afio_pcf0_enet_remap", &self.afio_pcf0_enet_remap())
                .field("afio_pcf0_can1_remap", &self.afio_pcf0_can1_remap())
                .field("afio_pcf0_enet_phy_sel", &self.afio_pcf0_enet_phy_sel())
                .field("afio_pcf0_swj_cfg", &self.afio_pcf0_swj_cfg())
                .field("afio_pcf0_spi2_remap", &self.afio_pcf0_spi2_remap())
                .field("afio_pcf0_timer1iti1_remap", &self.afio_pcf0_timer1iti1_remap())
                .field("afio_pcf0_ptp_pps_remap", &self.afio_pcf0_ptp_pps_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioafiopcf0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioafiopcf0 {{ afio_pcf0_spi0_remap: {=bool:?}, afio_pcf0_i2c0_remap: {=bool:?}, afio_pcf0_usart0_remap: {=bool:?}, afio_pcf0_usart1_remap: {=bool:?}, afio_pcf0_usart2_remap: {=u8:?}, afio_pcf0_timer0_remap: {=u8:?}, afio_pcf0_timer1_remap: {=u8:?}, afio_pcf0_timer2_remap: {=u8:?}, afio_pcf0_timer3_remap: {=bool:?}, afio_pcf0_can0_remap: {=u8:?}, afio_pcf0_pd01_remap: {=bool:?}, afio_pcf0_timer4ch3_iremap: {=bool:?}, afio_pcf0_enet_remap: {=bool:?}, afio_pcf0_can1_remap: {=bool:?}, afio_pcf0_enet_phy_sel: {=bool:?}, afio_pcf0_swj_cfg: {=u8:?}, afio_pcf0_spi2_remap: {=bool:?}, afio_pcf0_timer1iti1_remap: {=bool:?}, afio_pcf0_ptp_pps_remap: {=bool:?} }}",
                self.afio_pcf0_spi0_remap(),
                self.afio_pcf0_i2c0_remap(),
                self.afio_pcf0_usart0_remap(),
                self.afio_pcf0_usart1_remap(),
                self.afio_pcf0_usart2_remap(),
                self.afio_pcf0_timer0_remap(),
                self.afio_pcf0_timer1_remap(),
                self.afio_pcf0_timer2_remap(),
                self.afio_pcf0_timer3_remap(),
                self.afio_pcf0_can0_remap(),
                self.afio_pcf0_pd01_remap(),
                self.afio_pcf0_timer4ch3_iremap(),
                self.afio_pcf0_enet_remap(),
                self.afio_pcf0_can1_remap(),
                self.afio_pcf0_enet_phy_sel(),
                self.afio_pcf0_swj_cfg(),
                self.afio_pcf0_spi2_remap(),
                self.afio_pcf0_timer1iti1_remap(),
                self.afio_pcf0_ptp_pps_remap()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioafiopcf1(pub u32);
    impl Afioafiopcf1 {
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_timer8_remap(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_timer8_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_timer9_remap(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_timer9_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_timer10_remap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_timer10_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_timer12_remap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_timer12_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_timer13_remap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_timer13_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_exmc_nadv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_exmc_nadv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn afio_pcf1_ctc_remap(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_afio_pcf1_ctc_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
    }
    impl Default for Afioafiopcf1 {
        #[inline(always)]
        fn default() -> Afioafiopcf1 {
            Afioafiopcf1(0)
        }
    }
    impl core::fmt::Debug for Afioafiopcf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioafiopcf1")
                .field("afio_pcf1_timer8_remap", &self.afio_pcf1_timer8_remap())
                .field("afio_pcf1_timer9_remap", &self.afio_pcf1_timer9_remap())
                .field("afio_pcf1_timer10_remap", &self.afio_pcf1_timer10_remap())
                .field("afio_pcf1_timer12_remap", &self.afio_pcf1_timer12_remap())
                .field("afio_pcf1_timer13_remap", &self.afio_pcf1_timer13_remap())
                .field("afio_pcf1_exmc_nadv", &self.afio_pcf1_exmc_nadv())
                .field("afio_pcf1_ctc_remap", &self.afio_pcf1_ctc_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioafiopcf1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioafiopcf1 {{ afio_pcf1_timer8_remap: {=bool:?}, afio_pcf1_timer9_remap: {=bool:?}, afio_pcf1_timer10_remap: {=bool:?}, afio_pcf1_timer12_remap: {=bool:?}, afio_pcf1_timer13_remap: {=bool:?}, afio_pcf1_exmc_nadv: {=bool:?}, afio_pcf1_ctc_remap: {=u8:?} }}",
                self.afio_pcf1_timer8_remap(),
                self.afio_pcf1_timer9_remap(),
                self.afio_pcf1_timer10_remap(),
                self.afio_pcf1_timer12_remap(),
                self.afio_pcf1_timer13_remap(),
                self.afio_pcf1_exmc_nadv(),
                self.afio_pcf1_ctc_remap()
            )
        }
    }
    #[doc = "IO compensation control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afiocpsctl(pub u32);
    impl Afiocpsctl {
        #[doc = "I/O compensation cell enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cps_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I/O compensation cell enable"]
        #[inline(always)]
        pub const fn set_cps_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O compensation cell is really or not"]
        #[must_use]
        #[inline(always)]
        pub const fn cps_rdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O compensation cell is really or not"]
        #[inline(always)]
        pub const fn set_cps_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Afiocpsctl {
        #[inline(always)]
        fn default() -> Afiocpsctl {
            Afiocpsctl(0)
        }
    }
    impl core::fmt::Debug for Afiocpsctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afiocpsctl")
                .field("cps_en", &self.cps_en())
                .field("cps_rdy", &self.cps_rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afiocpsctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afiocpsctl {{ cps_en: {=bool:?}, cps_rdy: {=bool:?} }}",
                self.cps_en(),
                self.cps_rdy()
            )
        }
    }
    #[doc = "Event control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioec(pub u32);
    impl Afioec {
        #[doc = "Event output pin selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Event output pin selection"]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Event output port selection"]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Event output port selection"]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event output enable"]
        #[inline(always)]
        pub const fn set_eoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Afioec {
        #[inline(always)]
        fn default() -> Afioec {
            Afioec(0)
        }
    }
    impl core::fmt::Debug for Afioec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioec")
                .field("pin", &self.pin())
                .field("port", &self.port())
                .field("eoe", &self.eoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioec {{ pin: {=u8:?}, port: {=u8:?}, eoe: {=bool:?} }}",
                self.pin(),
                self.port(),
                self.eoe()
            )
        }
    }
    #[doc = "EXTI sources selection register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioextiss0(pub u32);
    impl Afioextiss0 {
        #[doc = "EXTI 0 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti0_ss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 0 sources selection"]
        #[inline(always)]
        pub const fn set_exti0_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "EXTI 1 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti1_ss(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 1 sources selection"]
        #[inline(always)]
        pub const fn set_exti1_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "EXTI 2 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti2_ss(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 2 sources selection"]
        #[inline(always)]
        pub const fn set_exti2_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "EXTI 3 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti3_ss(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 3 sources selection"]
        #[inline(always)]
        pub const fn set_exti3_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Afioextiss0 {
        #[inline(always)]
        fn default() -> Afioextiss0 {
            Afioextiss0(0)
        }
    }
    impl core::fmt::Debug for Afioextiss0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioextiss0")
                .field("exti0_ss", &self.exti0_ss())
                .field("exti1_ss", &self.exti1_ss())
                .field("exti2_ss", &self.exti2_ss())
                .field("exti3_ss", &self.exti3_ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioextiss0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioextiss0 {{ exti0_ss: {=u8:?}, exti1_ss: {=u8:?}, exti2_ss: {=u8:?}, exti3_ss: {=u8:?} }}",
                self.exti0_ss(),
                self.exti1_ss(),
                self.exti2_ss(),
                self.exti3_ss()
            )
        }
    }
    #[doc = "EXTI sources selection register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioextiss1(pub u32);
    impl Afioextiss1 {
        #[doc = "EXTI 4 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti4_ss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 4 sources selection"]
        #[inline(always)]
        pub const fn set_exti4_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "EXTI 5 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti5_ss(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 5 sources selection"]
        #[inline(always)]
        pub const fn set_exti5_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "EXTI 6 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti6_ss(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 6 sources selection"]
        #[inline(always)]
        pub const fn set_exti6_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "EXTI 7 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti7_ss(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 7 sources selection"]
        #[inline(always)]
        pub const fn set_exti7_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Afioextiss1 {
        #[inline(always)]
        fn default() -> Afioextiss1 {
            Afioextiss1(0)
        }
    }
    impl core::fmt::Debug for Afioextiss1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioextiss1")
                .field("exti4_ss", &self.exti4_ss())
                .field("exti5_ss", &self.exti5_ss())
                .field("exti6_ss", &self.exti6_ss())
                .field("exti7_ss", &self.exti7_ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioextiss1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioextiss1 {{ exti4_ss: {=u8:?}, exti5_ss: {=u8:?}, exti6_ss: {=u8:?}, exti7_ss: {=u8:?} }}",
                self.exti4_ss(),
                self.exti5_ss(),
                self.exti6_ss(),
                self.exti7_ss()
            )
        }
    }
    #[doc = "EXTI sources selection register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioextiss2(pub u32);
    impl Afioextiss2 {
        #[doc = "EXTI 8 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti8_ss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 8 sources selection"]
        #[inline(always)]
        pub const fn set_exti8_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "EXTI 9 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti9_ss(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 9 sources selection"]
        #[inline(always)]
        pub const fn set_exti9_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "EXTI 10 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti10_ss(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 10 sources selection"]
        #[inline(always)]
        pub const fn set_exti10_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "EXTI 11 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti11_ss(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 11 sources selection"]
        #[inline(always)]
        pub const fn set_exti11_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Afioextiss2 {
        #[inline(always)]
        fn default() -> Afioextiss2 {
            Afioextiss2(0)
        }
    }
    impl core::fmt::Debug for Afioextiss2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioextiss2")
                .field("exti8_ss", &self.exti8_ss())
                .field("exti9_ss", &self.exti9_ss())
                .field("exti10_ss", &self.exti10_ss())
                .field("exti11_ss", &self.exti11_ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioextiss2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioextiss2 {{ exti8_ss: {=u8:?}, exti9_ss: {=u8:?}, exti10_ss: {=u8:?}, exti11_ss: {=u8:?} }}",
                self.exti8_ss(),
                self.exti9_ss(),
                self.exti10_ss(),
                self.exti11_ss()
            )
        }
    }
    #[doc = "EXTI sources selection register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afioextiss3(pub u32);
    impl Afioextiss3 {
        #[doc = "EXTI 12 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti12_ss(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 12 sources selection"]
        #[inline(always)]
        pub const fn set_exti12_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "EXTI 13 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti13_ss(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 13 sources selection"]
        #[inline(always)]
        pub const fn set_exti13_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "EXTI 14 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti14_ss(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 14 sources selection"]
        #[inline(always)]
        pub const fn set_exti14_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "EXTI 15 sources selection"]
        #[must_use]
        #[inline(always)]
        pub const fn exti15_ss(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "EXTI 15 sources selection"]
        #[inline(always)]
        pub const fn set_exti15_ss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Afioextiss3 {
        #[inline(always)]
        fn default() -> Afioextiss3 {
            Afioextiss3(0)
        }
    }
    impl core::fmt::Debug for Afioextiss3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afioextiss3")
                .field("exti12_ss", &self.exti12_ss())
                .field("exti13_ss", &self.exti13_ss())
                .field("exti14_ss", &self.exti14_ss())
                .field("exti15_ss", &self.exti15_ss())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afioextiss3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afioextiss3 {{ exti12_ss: {=u8:?}, exti13_ss: {=u8:?}, exti14_ss: {=u8:?}, exti15_ss: {=u8:?} }}",
                self.exti12_ss(),
                self.exti13_ss(),
                self.exti14_ss(),
                self.exti15_ss()
            )
        }
    }
    #[doc = "AFIO port configuration register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afiopcf0(pub u32);
    impl Afiopcf0 {
        #[doc = "SPI0 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn spi0_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI0 remapping"]
        #[inline(always)]
        pub const fn set_spi0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C0 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C0 remapping"]
        #[inline(always)]
        pub const fn set_i2c0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART0 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart0_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USART0 remapping"]
        #[inline(always)]
        pub const fn set_usart0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub const fn set_usart1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART2 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_remap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub const fn set_usart2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "TIMER0 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer0_remap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TIMER0 remapping"]
        #[inline(always)]
        pub const fn set_timer0_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "TIMER1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer1_remap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TIMER1 remapping"]
        #[inline(always)]
        pub const fn set_timer1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TIMER2 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer2_remap(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TIMER2 remapping"]
        #[inline(always)]
        pub const fn set_timer2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIMER3 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer3_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER3 remapping"]
        #[inline(always)]
        pub const fn set_timer3_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN0 alternate interface remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn can0_remap(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CAN0 alternate interface remapping"]
        #[inline(always)]
        pub const fn set_can0_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
        #[must_use]
        #[inline(always)]
        pub const fn pd01_remap(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
        #[inline(always)]
        pub const fn set_pd01_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIMER4 channel3 internal remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer4ch3_iremap(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER4 channel3 internal remapping"]
        #[inline(always)]
        pub const fn set_timer4ch3_iremap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn enet_remap(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC I/O remapping"]
        #[inline(always)]
        pub const fn set_enet_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CAN1 I/O remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn can1_remap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 I/O remapping"]
        #[inline(always)]
        pub const fn set_can1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Ethernet MII or RMII PHY selection"]
        #[must_use]
        #[inline(always)]
        pub const fn enet_phy_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MII or RMII PHY selection"]
        #[inline(always)]
        pub const fn set_enet_phy_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Serial wire JTAG configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn swj_cfg(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Serial wire JTAG configuration"]
        #[inline(always)]
        pub const fn set_swj_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "SPI2/I2S2 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2_remap(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2/I2S2 remapping"]
        #[inline(always)]
        pub const fn set_spi2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "TIMER1 internal trigger 0 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer1itr0_remap(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 internal trigger 0 remapping"]
        #[inline(always)]
        pub const fn set_timer1itr0_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn ptp_pps_remap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub const fn set_ptp_pps_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Afiopcf0 {
        #[inline(always)]
        fn default() -> Afiopcf0 {
            Afiopcf0(0)
        }
    }
    impl core::fmt::Debug for Afiopcf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afiopcf0")
                .field("spi0_remap", &self.spi0_remap())
                .field("i2c0_remap", &self.i2c0_remap())
                .field("usart0_remap", &self.usart0_remap())
                .field("usart1_remap", &self.usart1_remap())
                .field("usart2_remap", &self.usart2_remap())
                .field("timer0_remap", &self.timer0_remap())
                .field("timer1_remap", &self.timer1_remap())
                .field("timer2_remap", &self.timer2_remap())
                .field("timer3_remap", &self.timer3_remap())
                .field("can0_remap", &self.can0_remap())
                .field("pd01_remap", &self.pd01_remap())
                .field("timer4ch3_iremap", &self.timer4ch3_iremap())
                .field("enet_remap", &self.enet_remap())
                .field("can1_remap", &self.can1_remap())
                .field("enet_phy_sel", &self.enet_phy_sel())
                .field("swj_cfg", &self.swj_cfg())
                .field("spi2_remap", &self.spi2_remap())
                .field("timer1itr0_remap", &self.timer1itr0_remap())
                .field("ptp_pps_remap", &self.ptp_pps_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afiopcf0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afiopcf0 {{ spi0_remap: {=bool:?}, i2c0_remap: {=bool:?}, usart0_remap: {=bool:?}, usart1_remap: {=bool:?}, usart2_remap: {=u8:?}, timer0_remap: {=u8:?}, timer1_remap: {=u8:?}, timer2_remap: {=u8:?}, timer3_remap: {=bool:?}, can0_remap: {=u8:?}, pd01_remap: {=bool:?}, timer4ch3_iremap: {=bool:?}, enet_remap: {=bool:?}, can1_remap: {=bool:?}, enet_phy_sel: {=bool:?}, swj_cfg: {=u8:?}, spi2_remap: {=bool:?}, timer1itr0_remap: {=bool:?}, ptp_pps_remap: {=bool:?} }}",
                self.spi0_remap(),
                self.i2c0_remap(),
                self.usart0_remap(),
                self.usart1_remap(),
                self.usart2_remap(),
                self.timer0_remap(),
                self.timer1_remap(),
                self.timer2_remap(),
                self.timer3_remap(),
                self.can0_remap(),
                self.pd01_remap(),
                self.timer4ch3_iremap(),
                self.enet_remap(),
                self.can1_remap(),
                self.enet_phy_sel(),
                self.swj_cfg(),
                self.spi2_remap(),
                self.timer1itr0_remap(),
                self.ptp_pps_remap()
            )
        }
    }
    #[doc = "AFIO port configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afiopcf1(pub u32);
    impl Afiopcf1 {
        #[doc = "TIMER8 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer8_remap(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER8 remapping"]
        #[inline(always)]
        pub const fn set_timer8_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIMER9 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer9_remap(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER9 remapping"]
        #[inline(always)]
        pub const fn set_timer9_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIMER10 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer10_remap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER10 remapping"]
        #[inline(always)]
        pub const fn set_timer10_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIMER12 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer12_remap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER12 remapping"]
        #[inline(always)]
        pub const fn set_timer12_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIMER13 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn timer13_remap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER13 remapping"]
        #[inline(always)]
        pub const fn set_timer13_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FSMC_NADV connect/disconnect"]
        #[must_use]
        #[inline(always)]
        pub const fn fsmc_nadv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC_NADV connect/disconnect"]
        #[inline(always)]
        pub const fn set_fsmc_nadv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CTC remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn ctc_remap(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "CTC remapping"]
        #[inline(always)]
        pub const fn set_ctc_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
    }
    impl Default for Afiopcf1 {
        #[inline(always)]
        fn default() -> Afiopcf1 {
            Afiopcf1(0)
        }
    }
    impl core::fmt::Debug for Afiopcf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afiopcf1")
                .field("timer8_remap", &self.timer8_remap())
                .field("timer9_remap", &self.timer9_remap())
                .field("timer10_remap", &self.timer10_remap())
                .field("timer12_remap", &self.timer12_remap())
                .field("timer13_remap", &self.timer13_remap())
                .field("fsmc_nadv", &self.fsmc_nadv())
                .field("ctc_remap", &self.ctc_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afiopcf1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Afiopcf1 {{ timer8_remap: {=bool:?}, timer9_remap: {=bool:?}, timer10_remap: {=bool:?}, timer12_remap: {=bool:?}, timer13_remap: {=bool:?}, fsmc_nadv: {=bool:?}, ctc_remap: {=u8:?} }}",
                self.timer8_remap(),
                self.timer9_remap(),
                self.timer10_remap(),
                self.timer12_remap(),
                self.timer13_remap(),
                self.fsmc_nadv(),
                self.ctc_remap()
            )
        }
    }
    #[doc = "Event Control Register (AFIO_EVCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evcr(pub u32);
    impl Evcr {
        #[doc = "Pin selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pin selection"]
        #[inline(always)]
        pub const fn set_pin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Port selection"]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Port selection"]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Event Output Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn evoe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Event Output Enable"]
        #[inline(always)]
        pub const fn set_evoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Evcr {
        #[inline(always)]
        fn default() -> Evcr {
            Evcr(0)
        }
    }
    impl core::fmt::Debug for Evcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evcr")
                .field("pin", &self.pin())
                .field("port", &self.port())
                .field("evoe", &self.evoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Evcr {{ pin: {=u8:?}, port: {=u8:?}, evoe: {=bool:?} }}",
                self.pin(),
                self.port(),
                self.evoe()
            )
        }
    }
    #[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI12 configuration"]
        #[inline(always)]
        pub const fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "AF remap and debug I/O configuration register (AFIO_MAPR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr(pub u32);
    impl Mapr {
        #[doc = "SPI1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1_remap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 remapping"]
        #[inline(always)]
        pub const fn set_spi1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_remap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 remapping"]
        #[inline(always)]
        pub const fn set_i2c1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1_remap(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 remapping"]
        #[inline(always)]
        pub const fn set_usart1_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART2 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2_remap(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 remapping"]
        #[inline(always)]
        pub const fn set_usart2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "USART3 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3_remap(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 remapping"]
        #[inline(always)]
        pub const fn set_usart3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "TIM1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_remap(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 remapping"]
        #[inline(always)]
        pub const fn set_tim1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "TIM2 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_remap(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 remapping"]
        #[inline(always)]
        pub const fn set_tim2_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "TIM3 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_remap(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "TIM3 remapping"]
        #[inline(always)]
        pub const fn set_tim3_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "TIM4 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4_remap(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 remapping"]
        #[inline(always)]
        pub const fn set_tim4_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CAN1 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn can1_remap(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "CAN1 remapping"]
        #[inline(always)]
        pub const fn set_can1_remap(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "CAN2 I/O remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn can2_remap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 I/O remapping"]
        #[inline(always)]
        pub const fn set_can2_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "MII or RMII selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "MII or RMII selection"]
        #[inline(always)]
        pub const fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Serial wire JTAG configuration (must be set to NoOp to leave it unchanged!)"]
        #[must_use]
        #[inline(always)]
        pub const fn swj_cfg(&self) -> super::vals::SwjCfg {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::SwjCfg::from_bits(val as u8)
        }
        #[doc = "Serial wire JTAG configuration (must be set to NoOp to leave it unchanged!)"]
        #[inline(always)]
        pub const fn set_swj_cfg(&mut self, val: super::vals::SwjCfg) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3_remap(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3/I2S3 remapping"]
        #[inline(always)]
        pub const fn set_spi3_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn ptp_pps_remap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP PPS remapping"]
        #[inline(always)]
        pub const fn set_ptp_pps_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Mapr {
        #[inline(always)]
        fn default() -> Mapr {
            Mapr(0)
        }
    }
    impl core::fmt::Debug for Mapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mapr")
                .field("spi1_remap", &self.spi1_remap())
                .field("i2c1_remap", &self.i2c1_remap())
                .field("usart1_remap", &self.usart1_remap())
                .field("usart2_remap", &self.usart2_remap())
                .field("usart3_remap", &self.usart3_remap())
                .field("tim1_remap", &self.tim1_remap())
                .field("tim2_remap", &self.tim2_remap())
                .field("tim3_remap", &self.tim3_remap())
                .field("tim4_remap", &self.tim4_remap())
                .field("can1_remap", &self.can1_remap())
                .field("can2_remap", &self.can2_remap())
                .field("mii_rmii_sel", &self.mii_rmii_sel())
                .field("swj_cfg", &self.swj_cfg())
                .field("spi3_remap", &self.spi3_remap())
                .field("ptp_pps_remap", &self.ptp_pps_remap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mapr {{ spi1_remap: {=bool:?}, i2c1_remap: {=bool:?}, usart1_remap: {=bool:?}, usart2_remap: {=bool:?}, usart3_remap: {=u8:?}, tim1_remap: {=u8:?}, tim2_remap: {=u8:?}, tim3_remap: {=u8:?}, tim4_remap: {=bool:?}, can1_remap: {=u8:?}, can2_remap: {=bool:?}, mii_rmii_sel: {=bool:?}, swj_cfg: {:?}, spi3_remap: {=bool:?}, ptp_pps_remap: {=bool:?} }}",
                self.spi1_remap(),
                self.i2c1_remap(),
                self.usart1_remap(),
                self.usart2_remap(),
                self.usart3_remap(),
                self.tim1_remap(),
                self.tim2_remap(),
                self.tim3_remap(),
                self.tim4_remap(),
                self.can1_remap(),
                self.can2_remap(),
                self.mii_rmii_sel(),
                self.swj_cfg(),
                self.spi3_remap(),
                self.ptp_pps_remap()
            )
        }
    }
    #[doc = "AF remap and debug I/O configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mapr2(pub u32);
    impl Mapr2 {
        #[doc = "TIM9 remapping"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9_remap(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 remapping"]
        #[inline(always)]
        pub const fn set_tim9_remap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "NADV connect/disconnect"]
        #[must_use]
        #[inline(always)]
        pub const fn fsmc_nadv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "NADV connect/disconnect"]
        #[inline(always)]
        pub const fn set_fsmc_nadv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Mapr2 {
        #[inline(always)]
        fn default() -> Mapr2 {
            Mapr2(0)
        }
    }
    impl core::fmt::Debug for Mapr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mapr2")
                .field("tim9_remap", &self.tim9_remap())
                .field("fsmc_nadv", &self.fsmc_nadv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mapr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mapr2 {{ tim9_remap: {=bool:?}, fsmc_nadv: {=bool:?} }}",
                self.tim9_remap(),
                self.fsmc_nadv()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SwjCfg {
        #[doc = "Full SWJ (JTAG-DP + SW-DP) (Reset state)"]
        Reset = 0x0,
        #[doc = "Full SWJ (JTAG-DP + SW-DP) but without NJTRST"]
        NoJntRst = 0x01,
        #[doc = "JTAG-DP Disabled and SW-DP Enabled"]
        JtagDisable = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "JTAG-DP Disabled and SW-DP Disabled"]
        Disable = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "Sets all bits to 1, indicating that the configuration should remain unchanged"]
        NoOp = 0x07,
    }
    impl SwjCfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SwjCfg {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SwjCfg {
        #[inline(always)]
        fn from(val: u8) -> SwjCfg {
            SwjCfg::from_bits(val)
        }
    }
    impl From<SwjCfg> for u8 {
        #[inline(always)]
        fn from(val: SwjCfg) -> u8 {
            SwjCfg::to_bits(val)
        }
    }
}
