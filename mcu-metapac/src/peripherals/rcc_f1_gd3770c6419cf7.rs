#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn rccctl(self) -> crate::common::Reg<regs::Rccctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuctl(self) -> crate::common::Reg<regs::Rccrcuctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock configuration register (RCC_CFGR)"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock configuration register 0 (RCU_CFG0)"]
    #[inline(always)]
    pub const fn rcccfg0(self) -> crate::common::Reg<regs::Rcccfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcucfg0(self) -> crate::common::Reg<regs::Rccrcucfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock interrupt register (RCC_CIR)"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock interrupt register (RCU_INT)"]
    #[inline(always)]
    pub const fn rccint(self) -> crate::common::Reg<regs::Rccint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuint(self) -> crate::common::Reg<regs::Rccrcuint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "APB2 reset register (RCU_APB2RST)"]
    #[inline(always)]
    pub const fn rccapb2rst(self) -> crate::common::Reg<regs::Rccapb2rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuapb2rst(self) -> crate::common::Reg<regs::Rccrcuapb2rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "APB1 reset register (RCU_APB1RST)"]
    #[inline(always)]
    pub const fn rccapb1rst(self) -> crate::common::Reg<regs::Rccapb1rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuapb1rst(self) -> crate::common::Reg<regs::Rccrcuapb1rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[inline(always)]
    pub const fn ahbenr(self) -> crate::common::Reg<regs::Ahbenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "AHB enable register"]
    #[inline(always)]
    pub const fn rccahben(self) -> crate::common::Reg<regs::Rccahben, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuahben(self) -> crate::common::Reg<regs::Rccrcuahben, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "APB2 clock enable register (RCU_APB2EN)"]
    #[inline(always)]
    pub const fn rccapb2en(self) -> crate::common::Reg<regs::Rccapb2en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuapb2en(self) -> crate::common::Reg<regs::Rccrcuapb2en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "APB1 clock enable register (RCU_APB1EN)"]
    #[inline(always)]
    pub const fn rccapb1en(self) -> crate::common::Reg<regs::Rccapb1en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuapb1en(self) -> crate::common::Reg<regs::Rccrcuapb1en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Backup domain control register (RCC_BDCR)"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Backup domain control register (RCU_BDCTL)"]
    #[inline(always)]
    pub const fn rccbdctl(self) -> crate::common::Reg<regs::Rccbdctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcubdctl(self) -> crate::common::Reg<regs::Rccrcubdctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Control/status register (RCC_CSR)"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcurstsck(self) -> crate::common::Reg<regs::Rccrcurstsck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Reset source /clock register (RCU_RSTSCK)"]
    #[inline(always)]
    pub const fn rccrstsck(self) -> crate::common::Reg<regs::Rccrstsck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB reset register"]
    #[inline(always)]
    pub const fn rccahbrst(self) -> crate::common::Reg<regs::Rccahbrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuahbrst(self) -> crate::common::Reg<regs::Rccrcuahbrst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Clock Configuration register 1"]
    #[inline(always)]
    pub const fn rcccfg1(self) -> crate::common::Reg<regs::Rcccfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcucfg1(self) -> crate::common::Reg<regs::Rccrcucfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deep sleep mode Voltage register"]
    #[inline(always)]
    pub const fn rccdsv(self) -> crate::common::Reg<regs::Rccdsv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcudsv(self) -> crate::common::Reg<regs::Rccrcudsv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Additional clock control register"]
    #[inline(always)]
    pub const fn rccaddctl(self) -> crate::common::Reg<regs::Rccaddctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuaddctl(self) -> crate::common::Reg<regs::Rccrcuaddctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Additional clock interrupt register"]
    #[inline(always)]
    pub const fn rccaddint(self) -> crate::common::Reg<regs::Rccaddint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuaddint(self) -> crate::common::Reg<regs::Rccrcuaddint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "APB1 additional reset register"]
    #[inline(always)]
    pub const fn rccaddapb1rst(self) -> crate::common::Reg<regs::Rccaddapb1rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuaddapb1rst(self) -> crate::common::Reg<regs::Rccrcuaddapb1rst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "APB1 additional enable register"]
    #[inline(always)]
    pub const fn rccaddapb1en(self) -> crate::common::Reg<regs::Rccaddapb1en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn rccrcuaddapb1en(self) -> crate::common::Reg<regs::Rccrcuaddapb1en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbenr(pub u32);
    impl Ahbenr {
        #[doc = "DMA1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sramen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable"]
        #[inline(always)]
        pub const fn set_sramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FLASH clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable"]
        #[inline(always)]
        pub const fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CRC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FSMC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fsmcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC clock enable"]
        #[inline(always)]
        pub const fn set_fsmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Ethernet MAC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable"]
        #[inline(always)]
        pub const fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC TX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC TX clock enable"]
        #[inline(always)]
        pub const fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet MAC RX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC RX clock enable"]
        #[inline(always)]
        pub const fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ahbenr {
        #[inline(always)]
        fn default() -> Ahbenr {
            Ahbenr(0)
        }
    }
    impl core::fmt::Debug for Ahbenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbenr")
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("sramen", &self.sramen())
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("fsmcen", &self.fsmcen())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahbenr {{ dma1en: {=bool:?}, dma2en: {=bool:?}, sramen: {=bool:?}, flashen: {=bool:?}, crcen: {=bool:?}, fsmcen: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?} }}",
                self.dma1en(),
                self.dma2en(),
                self.sramen(),
                self.flashen(),
                self.crcen(),
                self.fsmcen(),
                self.ethen(),
                self.ethtxen(),
                self.ethrxen()
            )
        }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "Timer 2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 clock enable"]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enable"]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 clock enable"]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 5 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 5 clock enable"]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 6 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable"]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 clock enable"]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer 12 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 12 clock enable"]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer 13 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 13 clock enable"]
        #[inline(always)]
        pub const fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer 14 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 14 clock enable"]
        #[inline(always)]
        pub const fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Window watchdog clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 clock enable"]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI 3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 3 clock enable"]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 clock enable"]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART 4 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART 4 clock enable"]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART 5 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART 5 clock enable"]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C 1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 clock enable"]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C 2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 2 clock enable"]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn canen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN clock enable"]
        #[inline(always)]
        pub const fn set_canen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface clock enable"]
        #[inline(always)]
        pub const fn set_bkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub const fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub const fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1enr {
        #[inline(always)]
        fn default() -> Apb1enr {
            Apb1enr(0)
        }
    }
    impl core::fmt::Debug for Apb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("canen", &self.canen())
                .field("bkpen", &self.bkpen())
                .field("pwren", &self.pwren())
                .field("dacen", &self.dacen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, canen: {=bool:?}, bkpen: {=bool:?}, pwren: {=bool:?}, dacen: {=bool:?} }}",
                self.tim2en(),
                self.tim3en(),
                self.tim4en(),
                self.tim5en(),
                self.tim6en(),
                self.tim7en(),
                self.tim12en(),
                self.tim13en(),
                self.tim14en(),
                self.wwdgen(),
                self.spi2en(),
                self.spi3en(),
                self.usart2en(),
                self.usart3en(),
                self.uart4en(),
                self.uart5en(),
                self.i2c1en(),
                self.i2c2en(),
                self.canen(),
                self.bkpen(),
                self.pwren(),
                self.dacen()
            )
        }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "Timer 2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset"]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 reset"]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 reset"]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 5 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 5 reset"]
        #[inline(always)]
        pub const fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 6 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 reset"]
        #[inline(always)]
        pub const fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub const fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer 12 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 12 reset"]
        #[inline(always)]
        pub const fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer 13 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 13 reset"]
        #[inline(always)]
        pub const fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Timer 14 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 14 reset"]
        #[inline(always)]
        pub const fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Window watchdog reset"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub const fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset"]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 reset"]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 reset"]
        #[inline(always)]
        pub const fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART 5 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART 5 reset"]
        #[inline(always)]
        pub const fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN reset"]
        #[must_use]
        #[inline(always)]
        pub const fn canrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN reset"]
        #[inline(always)]
        pub const fn set_canrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bkprst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface reset"]
        #[inline(always)]
        pub const fn set_bkprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub const fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface reset"]
        #[inline(always)]
        pub const fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1rstr {
        #[inline(always)]
        fn default() -> Apb1rstr {
            Apb1rstr(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("canrst", &self.canrst())
                .field("bkprst", &self.bkprst())
                .field("pwrrst", &self.pwrrst())
                .field("dacrst", &self.dacrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, wwdgrst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, canrst: {=bool:?}, bkprst: {=bool:?}, pwrrst: {=bool:?}, dacrst: {=bool:?} }}",
                self.tim2rst(),
                self.tim3rst(),
                self.tim4rst(),
                self.tim5rst(),
                self.tim6rst(),
                self.tim7rst(),
                self.tim12rst(),
                self.tim13rst(),
                self.tim14rst(),
                self.wwdgrst(),
                self.spi2rst(),
                self.spi3rst(),
                self.usart2rst(),
                self.usart3rst(),
                self.uart4rst(),
                self.uart5rst(),
                self.i2c1rst(),
                self.i2c2rst(),
                self.canrst(),
                self.bkprst(),
                self.pwrrst(),
                self.dacrst()
            )
        }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "Alternate function I/O clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn afioen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O clock enable"]
        #[inline(always)]
        pub const fn set_afioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I/O port A clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A clock enable"]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "I/O port B clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B clock enable"]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O port C clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C clock enable"]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O port D clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable"]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I/O port E clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port E clock enable"]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I/O port F clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port F clock enable"]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "I/O port G clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port G clock enable"]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC 1 interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 interface clock enable"]
        #[inline(always)]
        pub const fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface clock enable"]
        #[inline(always)]
        pub const fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 Timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 Timer clock enable"]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable"]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 Timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 Timer clock enable"]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC3 interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 interface clock enable"]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM9 Timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 Timer clock enable"]
        #[inline(always)]
        pub const fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM10 Timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 Timer clock enable"]
        #[inline(always)]
        pub const fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIM11 Timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 Timer clock enable"]
        #[inline(always)]
        pub const fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    impl core::fmt::Debug for Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2enr")
                .field("afioen", &self.afioen())
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("adc1en", &self.adc1en())
                .field("adc2en", &self.adc2en())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("adc3en", &self.adc3en())
                .field("tim9en", &self.tim9en())
                .field("tim10en", &self.tim10en())
                .field("tim11en", &self.tim11en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ afioen: {=bool:?}, gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, adc1en: {=bool:?}, adc2en: {=bool:?}, tim1en: {=bool:?}, spi1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, adc3en: {=bool:?}, tim9en: {=bool:?}, tim10en: {=bool:?}, tim11en: {=bool:?} }}",
                self.afioen(),
                self.gpioaen(),
                self.gpioben(),
                self.gpiocen(),
                self.gpioden(),
                self.gpioeen(),
                self.gpiofen(),
                self.gpiogen(),
                self.adc1en(),
                self.adc2en(),
                self.tim1en(),
                self.spi1en(),
                self.tim8en(),
                self.usart1en(),
                self.adc3en(),
                self.tim9en(),
                self.tim10en(),
                self.tim11en()
            )
        }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "Alternate function I/O reset"]
        #[must_use]
        #[inline(always)]
        pub const fn afiorst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O reset"]
        #[inline(always)]
        pub const fn set_afiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port A reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port B reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port C reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port D reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port E reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port F reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub const fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port G reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC 1 interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 1 interface reset"]
        #[inline(always)]
        pub const fn set_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 2 interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc2rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 2 interface reset"]
        #[inline(always)]
        pub const fn set_adc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM1 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset"]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset"]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer reset"]
        #[inline(always)]
        pub const fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC 3 interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 3 interface reset"]
        #[inline(always)]
        pub const fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM9 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 timer reset"]
        #[inline(always)]
        pub const fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM10 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 timer reset"]
        #[inline(always)]
        pub const fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIM11 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 timer reset"]
        #[inline(always)]
        pub const fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    impl core::fmt::Debug for Apb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2rstr")
                .field("afiorst", &self.afiorst())
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("adc1rst", &self.adc1rst())
                .field("adc2rst", &self.adc2rst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("adc3rst", &self.adc3rst())
                .field("tim9rst", &self.tim9rst())
                .field("tim10rst", &self.tim10rst())
                .field("tim11rst", &self.tim11rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ afiorst: {=bool:?}, gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, adc1rst: {=bool:?}, adc2rst: {=bool:?}, tim1rst: {=bool:?}, spi1rst: {=bool:?}, tim8rst: {=bool:?}, usart1rst: {=bool:?}, adc3rst: {=bool:?}, tim9rst: {=bool:?}, tim10rst: {=bool:?}, tim11rst: {=bool:?} }}",
                self.afiorst(),
                self.gpioarst(),
                self.gpiobrst(),
                self.gpiocrst(),
                self.gpiodrst(),
                self.gpioerst(),
                self.gpiofrst(),
                self.gpiogrst(),
                self.adc1rst(),
                self.adc2rst(),
                self.tim1rst(),
                self.spi1rst(),
                self.tim8rst(),
                self.usart1rst(),
                self.adc3rst(),
                self.tim9rst(),
                self.tim10rst(),
                self.tim11rst()
            )
        }
    }
    #[doc = "Backup domain control register (RCC_BDCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "External Low Speed oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator enable"]
        #[inline(always)]
        pub const fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Low Speed oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator ready"]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Low Speed oscillator bypass"]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator bypass"]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RTC clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub const fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, bdrst: {=bool:?} }}",
                self.lseon(),
                self.lserdy(),
                self.lsebyp(),
                self.rtcsel(),
                self.rtcen(),
                self.bdrst()
            )
        }
    }
    #[doc = "Clock configuration register (RCC_CFGR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock Switch"]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock Switch"]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System Clock Switch Status"]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System Clock Switch Status"]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB High speed prescaler (APB2)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB High speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "ADC prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn adcpre(&self) -> super::vals::Adcpre {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Adcpre::from_bits(val as u8)
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn set_adcpre(&mut self, val: super::vals::Adcpre) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "PLL entry clock source"]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source"]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE divider for PLL entry"]
        #[must_use]
        #[inline(always)]
        pub const fn pllxtpre(&self) -> super::vals::Pllxtpre {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pllxtpre::from_bits(val as u8)
        }
        #[doc = "HSE divider for PLL entry"]
        #[inline(always)]
        pub const fn set_pllxtpre(&mut self, val: super::vals::Pllxtpre) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL Multiplication Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pllmul(&self) -> super::vals::Pllmul {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Pllmul::from_bits(val as u8)
        }
        #[doc = "PLL Multiplication Factor"]
        #[inline(always)]
        pub const fn set_pllmul(&mut self, val: super::vals::Pllmul) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "USB prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn usbpre(&self) -> super::vals::Usbpre {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Usbpre::from_bits(val as u8)
        }
        #[doc = "USB prescaler"]
        #[inline(always)]
        pub const fn set_usbpre(&mut self, val: super::vals::Usbpre) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Microcontroller clock output"]
        #[must_use]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output"]
        #[inline(always)]
        pub const fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("adcpre", &self.adcpre())
                .field("pllsrc", &self.pllsrc())
                .field("pllxtpre", &self.pllxtpre())
                .field("pllmul", &self.pllmul())
                .field("usbpre", &self.usbpre())
                .field("mcosel", &self.mcosel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ sw: {:?}, sws: {:?}, hpre: {:?}, ppre1: {:?}, ppre2: {:?}, adcpre: {:?}, pllsrc: {:?}, pllxtpre: {:?}, pllmul: {:?}, usbpre: {:?}, mcosel: {:?} }}",
                self.sw(),
                self.sws(),
                self.hpre(),
                self.ppre1(),
                self.ppre2(),
                self.adcpre(),
                self.pllsrc(),
                self.pllxtpre(),
                self.pllmul(),
                self.usbpre(),
                self.mcosel()
            )
        }
    }
    #[doc = "Clock interrupt register (RCC_CIR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cir(pub u32);
    impl Cir {
        #[doc = "LSI Ready Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt flag"]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE Ready Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt flag"]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI Ready Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt flag"]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE Ready Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt flag"]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL Ready Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt flag"]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI Ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI Ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE Ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL Ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LSI Ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE Ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI Ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL Ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cir {
        #[inline(always)]
        fn default() -> Cir {
            Cir(0)
        }
    }
    impl core::fmt::Debug for Cir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cir")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("cssf", &self.cssf())
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("pllrdyie", &self.pllrdyie())
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("cssc", &self.cssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cir {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, cssf: {=bool:?}, lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, cssc: {=bool:?} }}",
                self.lsirdyf(),
                self.lserdyf(),
                self.hsirdyf(),
                self.hserdyf(),
                self.pllrdyf(),
                self.cssf(),
                self.lsirdyie(),
                self.lserdyie(),
                self.hsirdyie(),
                self.hserdyie(),
                self.pllrdyie(),
                self.lsirdyc(),
                self.lserdyc(),
                self.hsirdyc(),
                self.hserdyc(),
                self.pllrdyc(),
                self.cssc()
            )
        }
    }
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal High Speed clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable"]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal High Speed clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock ready flag"]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal High Speed clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal High Speed clock trimming"]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal High Speed clock Calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal High Speed clock Calibration"]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "External High Speed clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable"]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock ready flag"]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External High Speed clock Bypass"]
        #[must_use]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock Bypass"]
        #[inline(always)]
        pub const fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock Security System enable"]
        #[must_use]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System enable"]
        #[inline(always)]
        pub const fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub const fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock ready flag"]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("hsion", &self.hsion())
                .field("hsirdy", &self.hsirdy())
                .field("hsitrim", &self.hsitrim())
                .field("hsical", &self.hsical())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ hsion: {=bool:?}, hsirdy: {=bool:?}, hsitrim: {=u8:?}, hsical: {=u8:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, csson: {=bool:?}, pllon: {=bool:?}, pllrdy: {=bool:?} }}",
                self.hsion(),
                self.hsirdy(),
                self.hsitrim(),
                self.hsical(),
                self.hseon(),
                self.hserdy(),
                self.hsebyp(),
                self.csson(),
                self.pllon(),
                self.pllrdy()
            )
        }
    }
    #[doc = "Control/status register (RCC_CSR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Internal low speed oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator enable"]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low speed oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator ready"]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remove reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PIN reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("rmvf", &self.rmvf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, rmvf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}",
                self.lsion(),
                self.lsirdy(),
                self.rmvf(),
                self.pinrstf(),
                self.porrstf(),
                self.sftrstf(),
                self.iwdgrstf(),
                self.wwdgrstf(),
                self.lpwrrstf()
            )
        }
    }
    #[doc = "APB1 additional enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccaddapb1en(pub u32);
    impl Rccaddapb1en {
        #[doc = "CTC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ctcen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "CTC clock enable"]
        #[inline(always)]
        pub const fn set_ctcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rccaddapb1en {
        #[inline(always)]
        fn default() -> Rccaddapb1en {
            Rccaddapb1en(0)
        }
    }
    impl core::fmt::Debug for Rccaddapb1en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccaddapb1en").field("ctcen", &self.ctcen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccaddapb1en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rccaddapb1en {{ ctcen: {=bool:?} }}", self.ctcen())
        }
    }
    #[doc = "APB1 additional reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccaddapb1rst(pub u32);
    impl Rccaddapb1rst {
        #[doc = "CTC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ctcrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "CTC reset"]
        #[inline(always)]
        pub const fn set_ctcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rccaddapb1rst {
        #[inline(always)]
        fn default() -> Rccaddapb1rst {
            Rccaddapb1rst(0)
        }
    }
    impl core::fmt::Debug for Rccaddapb1rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccaddapb1rst").field("ctcrst", &self.ctcrst()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccaddapb1rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rccaddapb1rst {{ ctcrst: {=bool:?} }}", self.ctcrst())
        }
    }
    #[doc = "Additional clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccaddctl(pub u32);
    impl Rccaddctl {
        #[doc = "48MHz clock selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ck48msel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "48MHz clock selection"]
        #[inline(always)]
        pub const fn set_ck48msel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal 48MHz RC oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48men(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal 48MHz RC oscillator enable"]
        #[inline(always)]
        pub const fn set_irc48men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal 48MHz RC oscillator clock stabilization Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48mstb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal 48MHz RC oscillator clock stabilization Flag"]
        #[inline(always)]
        pub const fn set_irc48mstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal 48MHz RC oscillator calibration value register"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48mcalib(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Internal 48MHz RC oscillator calibration value register"]
        #[inline(always)]
        pub const fn set_irc48mcalib(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rccaddctl {
        #[inline(always)]
        fn default() -> Rccaddctl {
            Rccaddctl(0)
        }
    }
    impl core::fmt::Debug for Rccaddctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccaddctl")
                .field("ck48msel", &self.ck48msel())
                .field("irc48men", &self.irc48men())
                .field("irc48mstb", &self.irc48mstb())
                .field("irc48mcalib", &self.irc48mcalib())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccaddctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccaddctl {{ ck48msel: {=bool:?}, irc48men: {=bool:?}, irc48mstb: {=bool:?}, irc48mcalib: {=u8:?} }}",
                self.ck48msel(),
                self.irc48men(),
                self.irc48mstb(),
                self.irc48mcalib()
            )
        }
    }
    #[doc = "Additional clock interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccaddint(pub u32);
    impl Rccaddint {
        #[doc = "IRC48M stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48mstbif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IRC48M stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_irc48mstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48mstbie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_irc48mstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn irc48mstbic(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal 48 MHz RC oscillator Stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_irc48mstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Rccaddint {
        #[inline(always)]
        fn default() -> Rccaddint {
            Rccaddint(0)
        }
    }
    impl core::fmt::Debug for Rccaddint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccaddint")
                .field("irc48mstbif", &self.irc48mstbif())
                .field("irc48mstbie", &self.irc48mstbie())
                .field("irc48mstbic", &self.irc48mstbic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccaddint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccaddint {{ irc48mstbif: {=bool:?}, irc48mstbie: {=bool:?}, irc48mstbic: {=bool:?} }}",
                self.irc48mstbif(),
                self.irc48mstbie(),
                self.irc48mstbic()
            )
        }
    }
    #[doc = "AHB enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccahben(pub u32);
    impl Rccahben {
        #[doc = "DMA0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA0 clock enable"]
        #[inline(always)]
        pub const fn set_dma0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM interface clock enable when sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sramspen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable when sleep mode"]
        #[inline(always)]
        pub const fn set_sramspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FMC clock enable when sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fmcspen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC clock enable when sleep mode"]
        #[inline(always)]
        pub const fn set_fmcspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CRC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EXMC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn exmcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EXMC clock enable"]
        #[inline(always)]
        pub const fn set_exmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "USBFS clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usbfsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS clock enable"]
        #[inline(always)]
        pub const fn set_usbfsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Ethernet clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eneten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet clock enable"]
        #[inline(always)]
        pub const fn set_eneten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet TX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn enettxen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet TX clock enable"]
        #[inline(always)]
        pub const fn set_enettxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet RX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn enetrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet RX clock enable"]
        #[inline(always)]
        pub const fn set_enetrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rccahben {
        #[inline(always)]
        fn default() -> Rccahben {
            Rccahben(0)
        }
    }
    impl core::fmt::Debug for Rccahben {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccahben")
                .field("dma0en", &self.dma0en())
                .field("dma1en", &self.dma1en())
                .field("sramspen", &self.sramspen())
                .field("fmcspen", &self.fmcspen())
                .field("crcen", &self.crcen())
                .field("exmcen", &self.exmcen())
                .field("usbfsen", &self.usbfsen())
                .field("eneten", &self.eneten())
                .field("enettxen", &self.enettxen())
                .field("enetrxen", &self.enetrxen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccahben {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccahben {{ dma0en: {=bool:?}, dma1en: {=bool:?}, sramspen: {=bool:?}, fmcspen: {=bool:?}, crcen: {=bool:?}, exmcen: {=bool:?}, usbfsen: {=bool:?}, eneten: {=bool:?}, enettxen: {=bool:?}, enetrxen: {=bool:?} }}",
                self.dma0en(),
                self.dma1en(),
                self.sramspen(),
                self.fmcspen(),
                self.crcen(),
                self.exmcen(),
                self.usbfsen(),
                self.eneten(),
                self.enettxen(),
                self.enetrxen()
            )
        }
    }
    #[doc = "AHB reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccahbrst(pub u32);
    impl Rccahbrst {
        #[doc = "USBFS reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usbfsrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USBFS reset"]
        #[inline(always)]
        pub const fn set_usbfsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ENET reset"]
        #[must_use]
        #[inline(always)]
        pub const fn enetrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ENET reset"]
        #[inline(always)]
        pub const fn set_enetrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Rccahbrst {
        #[inline(always)]
        fn default() -> Rccahbrst {
            Rccahbrst(0)
        }
    }
    impl core::fmt::Debug for Rccahbrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccahbrst")
                .field("usbfsrst", &self.usbfsrst())
                .field("enetrst", &self.enetrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccahbrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccahbrst {{ usbfsrst: {=bool:?}, enetrst: {=bool:?} }}",
                self.usbfsrst(),
                self.enetrst()
            )
        }
    }
    #[doc = "APB1 clock enable register (RCU_APB1EN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccapb1en(pub u32);
    impl Rccapb1en {
        #[doc = "TIMER1 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIMER2 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER2 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIMER3 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER3 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIMER4 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer4en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER4 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIMER5 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer5en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER5 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMER6 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER6 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIMER11 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer11en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER11 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIMER12 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer12en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER12 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIMER13 timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer13en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER13 timer clock enable"]
        #[inline(always)]
        pub const fn set_timer13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Window watchdog timer clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog timer clock enable"]
        #[inline(always)]
        pub const fn set_wwdgten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart3en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART3 clock enable"]
        #[inline(always)]
        pub const fn set_uart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART4 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C0 clock enable"]
        #[inline(always)]
        pub const fn set_i2c0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn can0en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN0 clock enable"]
        #[inline(always)]
        pub const fn set_can0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn can1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 clock enable"]
        #[inline(always)]
        pub const fn set_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface clock enable"]
        #[inline(always)]
        pub const fn set_bkpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power control clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pmuen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power control clock enable"]
        #[inline(always)]
        pub const fn set_pmuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC clock enable"]
        #[inline(always)]
        pub const fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccapb1en {
        #[inline(always)]
        fn default() -> Rccapb1en {
            Rccapb1en(0)
        }
    }
    impl core::fmt::Debug for Rccapb1en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccapb1en")
                .field("timer1en", &self.timer1en())
                .field("timer2en", &self.timer2en())
                .field("timer3en", &self.timer3en())
                .field("timer4en", &self.timer4en())
                .field("timer5en", &self.timer5en())
                .field("timer6en", &self.timer6en())
                .field("timer11en", &self.timer11en())
                .field("timer12en", &self.timer12en())
                .field("timer13en", &self.timer13en())
                .field("wwdgten", &self.wwdgten())
                .field("spi1en", &self.spi1en())
                .field("spi2en", &self.spi2en())
                .field("usart1en", &self.usart1en())
                .field("usart2en", &self.usart2en())
                .field("uart3en", &self.uart3en())
                .field("uart4en", &self.uart4en())
                .field("i2c0en", &self.i2c0en())
                .field("i2c1en", &self.i2c1en())
                .field("can0en", &self.can0en())
                .field("can1en", &self.can1en())
                .field("bkpien", &self.bkpien())
                .field("pmuen", &self.pmuen())
                .field("dacen", &self.dacen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccapb1en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccapb1en {{ timer1en: {=bool:?}, timer2en: {=bool:?}, timer3en: {=bool:?}, timer4en: {=bool:?}, timer5en: {=bool:?}, timer6en: {=bool:?}, timer11en: {=bool:?}, timer12en: {=bool:?}, timer13en: {=bool:?}, wwdgten: {=bool:?}, spi1en: {=bool:?}, spi2en: {=bool:?}, usart1en: {=bool:?}, usart2en: {=bool:?}, uart3en: {=bool:?}, uart4en: {=bool:?}, i2c0en: {=bool:?}, i2c1en: {=bool:?}, can0en: {=bool:?}, can1en: {=bool:?}, bkpien: {=bool:?}, pmuen: {=bool:?}, dacen: {=bool:?} }}",
                self.timer1en(),
                self.timer2en(),
                self.timer3en(),
                self.timer4en(),
                self.timer5en(),
                self.timer6en(),
                self.timer11en(),
                self.timer12en(),
                self.timer13en(),
                self.wwdgten(),
                self.spi1en(),
                self.spi2en(),
                self.usart1en(),
                self.usart2en(),
                self.uart3en(),
                self.uart4en(),
                self.i2c0en(),
                self.i2c1en(),
                self.can0en(),
                self.can1en(),
                self.bkpien(),
                self.pmuen(),
                self.dacen()
            )
        }
    }
    #[doc = "APB1 reset register (RCU_APB1RST)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccapb1rst(pub u32);
    impl Rccapb1rst {
        #[doc = "TIMER1 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER1 timer reset"]
        #[inline(always)]
        pub const fn set_timer1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIMER2 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER2 timer reset"]
        #[inline(always)]
        pub const fn set_timer2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIMER3 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer3rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER3 timer reset"]
        #[inline(always)]
        pub const fn set_timer3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIMER4 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer4rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER4 timer reset"]
        #[inline(always)]
        pub const fn set_timer4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIMER5 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer5rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER5 timer reset"]
        #[inline(always)]
        pub const fn set_timer5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIMER6 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER6 timer reset"]
        #[inline(always)]
        pub const fn set_timer6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIMER11 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer11rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER11 timer reset"]
        #[inline(always)]
        pub const fn set_timer11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIMER12 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer12rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER12 timer reset"]
        #[inline(always)]
        pub const fn set_timer12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIMER13 timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer13rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER13 timer reset"]
        #[inline(always)]
        pub const fn set_timer13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Window watchdog timer reset"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgtrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog timer reset"]
        #[inline(always)]
        pub const fn set_wwdgtrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart3rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART3 reset"]
        #[inline(always)]
        pub const fn set_uart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART4 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub const fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C0 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c0rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C0 reset"]
        #[inline(always)]
        pub const fn set_i2c0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CAN0 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn can0rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN0 reset"]
        #[inline(always)]
        pub const fn set_can0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn can1rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub const fn set_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpirst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Backup interface reset"]
        #[inline(always)]
        pub const fn set_bkpirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power control reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pmurst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power control reset"]
        #[inline(always)]
        pub const fn set_pmurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC reset"]
        #[inline(always)]
        pub const fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccapb1rst {
        #[inline(always)]
        fn default() -> Rccapb1rst {
            Rccapb1rst(0)
        }
    }
    impl core::fmt::Debug for Rccapb1rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccapb1rst")
                .field("timer1rst", &self.timer1rst())
                .field("timer2rst", &self.timer2rst())
                .field("timer3rst", &self.timer3rst())
                .field("timer4rst", &self.timer4rst())
                .field("timer5rst", &self.timer5rst())
                .field("timer6rst", &self.timer6rst())
                .field("timer11rst", &self.timer11rst())
                .field("timer12rst", &self.timer12rst())
                .field("timer13rst", &self.timer13rst())
                .field("wwdgtrst", &self.wwdgtrst())
                .field("spi1rst", &self.spi1rst())
                .field("spi2rst", &self.spi2rst())
                .field("usart1rst", &self.usart1rst())
                .field("usart2rst", &self.usart2rst())
                .field("uart3rst", &self.uart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("i2c0rst", &self.i2c0rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("can0rst", &self.can0rst())
                .field("can1rst", &self.can1rst())
                .field("bkpirst", &self.bkpirst())
                .field("pmurst", &self.pmurst())
                .field("dacrst", &self.dacrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccapb1rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccapb1rst {{ timer1rst: {=bool:?}, timer2rst: {=bool:?}, timer3rst: {=bool:?}, timer4rst: {=bool:?}, timer5rst: {=bool:?}, timer6rst: {=bool:?}, timer11rst: {=bool:?}, timer12rst: {=bool:?}, timer13rst: {=bool:?}, wwdgtrst: {=bool:?}, spi1rst: {=bool:?}, spi2rst: {=bool:?}, usart1rst: {=bool:?}, usart2rst: {=bool:?}, uart3rst: {=bool:?}, uart4rst: {=bool:?}, i2c0rst: {=bool:?}, i2c1rst: {=bool:?}, can0rst: {=bool:?}, can1rst: {=bool:?}, bkpirst: {=bool:?}, pmurst: {=bool:?}, dacrst: {=bool:?} }}",
                self.timer1rst(),
                self.timer2rst(),
                self.timer3rst(),
                self.timer4rst(),
                self.timer5rst(),
                self.timer6rst(),
                self.timer11rst(),
                self.timer12rst(),
                self.timer13rst(),
                self.wwdgtrst(),
                self.spi1rst(),
                self.spi2rst(),
                self.usart1rst(),
                self.usart2rst(),
                self.uart3rst(),
                self.uart4rst(),
                self.i2c0rst(),
                self.i2c1rst(),
                self.can0rst(),
                self.can1rst(),
                self.bkpirst(),
                self.pmurst(),
                self.dacrst()
            )
        }
    }
    #[doc = "APB2 clock enable register (RCU_APB2EN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccapb2en(pub u32);
    impl Rccapb2en {
        #[doc = "Alternate function IO clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn afen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function IO clock enable"]
        #[inline(always)]
        pub const fn set_afen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO port A clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn paen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port A clock enable"]
        #[inline(always)]
        pub const fn set_paen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO port B clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port B clock enable"]
        #[inline(always)]
        pub const fn set_pben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO port C clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port C clock enable"]
        #[inline(always)]
        pub const fn set_pcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO port D clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port D clock enable"]
        #[inline(always)]
        pub const fn set_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO port E clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn peen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port E clock enable"]
        #[inline(always)]
        pub const fn set_peen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO port F clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pfen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port F clock enable"]
        #[inline(always)]
        pub const fn set_pfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO port G clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pgen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port G clock enable"]
        #[inline(always)]
        pub const fn set_pgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc0en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC0 clock enable"]
        #[inline(always)]
        pub const fn set_adc0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub const fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIMER0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer0en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER0 clock enable"]
        #[inline(always)]
        pub const fn set_timer0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi0en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI0 clock enable"]
        #[inline(always)]
        pub const fn set_spi0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIMER7 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer7en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER7 clock enable"]
        #[inline(always)]
        pub const fn set_timer7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART0 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart0en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART0 clock enable"]
        #[inline(always)]
        pub const fn set_usart0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 clock enable"]
        #[inline(always)]
        pub const fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIMER8 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer8en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER8 clock enable"]
        #[inline(always)]
        pub const fn set_timer8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIMER9 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer9en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER9 clock enable"]
        #[inline(always)]
        pub const fn set_timer9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TIMER10 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn timer10en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "TIMER10 clock enable"]
        #[inline(always)]
        pub const fn set_timer10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Rccapb2en {
        #[inline(always)]
        fn default() -> Rccapb2en {
            Rccapb2en(0)
        }
    }
    impl core::fmt::Debug for Rccapb2en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccapb2en")
                .field("afen", &self.afen())
                .field("paen", &self.paen())
                .field("pben", &self.pben())
                .field("pcen", &self.pcen())
                .field("pden", &self.pden())
                .field("peen", &self.peen())
                .field("pfen", &self.pfen())
                .field("pgen", &self.pgen())
                .field("adc0en", &self.adc0en())
                .field("adc1en", &self.adc1en())
                .field("timer0en", &self.timer0en())
                .field("spi0en", &self.spi0en())
                .field("timer7en", &self.timer7en())
                .field("usart0en", &self.usart0en())
                .field("adc2en", &self.adc2en())
                .field("timer8en", &self.timer8en())
                .field("timer9en", &self.timer9en())
                .field("timer10en", &self.timer10en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccapb2en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccapb2en {{ afen: {=bool:?}, paen: {=bool:?}, pben: {=bool:?}, pcen: {=bool:?}, pden: {=bool:?}, peen: {=bool:?}, pfen: {=bool:?}, pgen: {=bool:?}, adc0en: {=bool:?}, adc1en: {=bool:?}, timer0en: {=bool:?}, spi0en: {=bool:?}, timer7en: {=bool:?}, usart0en: {=bool:?}, adc2en: {=bool:?}, timer8en: {=bool:?}, timer9en: {=bool:?}, timer10en: {=bool:?} }}",
                self.afen(),
                self.paen(),
                self.pben(),
                self.pcen(),
                self.pden(),
                self.peen(),
                self.pfen(),
                self.pgen(),
                self.adc0en(),
                self.adc1en(),
                self.timer0en(),
                self.spi0en(),
                self.timer7en(),
                self.usart0en(),
                self.adc2en(),
                self.timer8en(),
                self.timer9en(),
                self.timer10en()
            )
        }
    }
    #[doc = "APB2 reset register (RCU_APB2RST)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccapb2rst(pub u32);
    impl Rccapb2rst {
        #[doc = "Alternate function I/O reset"]
        #[must_use]
        #[inline(always)]
        pub const fn afrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate function I/O reset"]
        #[inline(always)]
        pub const fn set_afrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO port A reset"]
        #[must_use]
        #[inline(always)]
        pub const fn parst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port A reset"]
        #[inline(always)]
        pub const fn set_parst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO port B reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pbrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port B reset"]
        #[inline(always)]
        pub const fn set_pbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO port C reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port C reset"]
        #[inline(always)]
        pub const fn set_pcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO port D reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pdrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port D reset"]
        #[inline(always)]
        pub const fn set_pdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO port E reset"]
        #[must_use]
        #[inline(always)]
        pub const fn perst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port E reset"]
        #[inline(always)]
        pub const fn set_perst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO portF reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pfrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO portF reset"]
        #[inline(always)]
        pub const fn set_pfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO port G reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pgrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO port G reset"]
        #[inline(always)]
        pub const fn set_pgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC0 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc0rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC0 reset"]
        #[inline(always)]
        pub const fn set_adc0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 reset"]
        #[inline(always)]
        pub const fn set_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer 0 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer0rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 0 reset"]
        #[inline(always)]
        pub const fn set_timer0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI0 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi0rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI0 reset"]
        #[inline(always)]
        pub const fn set_spi0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer 7 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer7rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub const fn set_timer7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART0 Reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart0rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART0 Reset"]
        #[inline(always)]
        pub const fn set_usart0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ADC2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc2rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 reset"]
        #[inline(always)]
        pub const fn set_adc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Timer 8 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer8rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 8 reset"]
        #[inline(always)]
        pub const fn set_timer8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Timer 9 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer9rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 9 reset"]
        #[inline(always)]
        pub const fn set_timer9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Timer 10 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn timer10rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 10 reset"]
        #[inline(always)]
        pub const fn set_timer10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Rccapb2rst {
        #[inline(always)]
        fn default() -> Rccapb2rst {
            Rccapb2rst(0)
        }
    }
    impl core::fmt::Debug for Rccapb2rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccapb2rst")
                .field("afrst", &self.afrst())
                .field("parst", &self.parst())
                .field("pbrst", &self.pbrst())
                .field("pcrst", &self.pcrst())
                .field("pdrst", &self.pdrst())
                .field("perst", &self.perst())
                .field("pfrst", &self.pfrst())
                .field("pgrst", &self.pgrst())
                .field("adc0rst", &self.adc0rst())
                .field("adc1rst", &self.adc1rst())
                .field("timer0rst", &self.timer0rst())
                .field("spi0rst", &self.spi0rst())
                .field("timer7rst", &self.timer7rst())
                .field("usart0rst", &self.usart0rst())
                .field("adc2rst", &self.adc2rst())
                .field("timer8rst", &self.timer8rst())
                .field("timer9rst", &self.timer9rst())
                .field("timer10rst", &self.timer10rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccapb2rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccapb2rst {{ afrst: {=bool:?}, parst: {=bool:?}, pbrst: {=bool:?}, pcrst: {=bool:?}, pdrst: {=bool:?}, perst: {=bool:?}, pfrst: {=bool:?}, pgrst: {=bool:?}, adc0rst: {=bool:?}, adc1rst: {=bool:?}, timer0rst: {=bool:?}, spi0rst: {=bool:?}, timer7rst: {=bool:?}, usart0rst: {=bool:?}, adc2rst: {=bool:?}, timer8rst: {=bool:?}, timer9rst: {=bool:?}, timer10rst: {=bool:?} }}",
                self.afrst(),
                self.parst(),
                self.pbrst(),
                self.pcrst(),
                self.pdrst(),
                self.perst(),
                self.pfrst(),
                self.pgrst(),
                self.adc0rst(),
                self.adc1rst(),
                self.timer0rst(),
                self.spi0rst(),
                self.timer7rst(),
                self.usart0rst(),
                self.adc2rst(),
                self.timer8rst(),
                self.timer9rst(),
                self.timer10rst()
            )
        }
    }
    #[doc = "Backup domain control register (RCU_BDCTL)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccbdctl(pub u32);
    impl Rccbdctl {
        #[doc = "LXTAL enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LXTAL enable"]
        #[inline(always)]
        pub const fn set_lxtalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External low-speed oscillator stabilization"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator stabilization"]
        #[inline(always)]
        pub const fn set_lxtalstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LXTAL bypass mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalbps(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LXTAL bypass mode enable"]
        #[inline(always)]
        pub const fn set_lxtalbps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LXTAL drive capability"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtaldri(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "LXTAL drive capability"]
        #[inline(always)]
        pub const fn set_lxtaldri(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "RTC clock entry selection"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcsrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "RTC clock entry selection"]
        #[inline(always)]
        pub const fn set_rtcsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bkprst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain reset"]
        #[inline(always)]
        pub const fn set_bkprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rccbdctl {
        #[inline(always)]
        fn default() -> Rccbdctl {
            Rccbdctl(0)
        }
    }
    impl core::fmt::Debug for Rccbdctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccbdctl")
                .field("lxtalen", &self.lxtalen())
                .field("lxtalstb", &self.lxtalstb())
                .field("lxtalbps", &self.lxtalbps())
                .field("lxtaldri", &self.lxtaldri())
                .field("rtcsrc", &self.rtcsrc())
                .field("rtcen", &self.rtcen())
                .field("bkprst", &self.bkprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccbdctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccbdctl {{ lxtalen: {=bool:?}, lxtalstb: {=bool:?}, lxtalbps: {=bool:?}, lxtaldri: {=u8:?}, rtcsrc: {=u8:?}, rtcen: {=bool:?}, bkprst: {=bool:?} }}",
                self.lxtalen(),
                self.lxtalstb(),
                self.lxtalbps(),
                self.lxtaldri(),
                self.rtcsrc(),
                self.rtcen(),
                self.bkprst()
            )
        }
    }
    #[doc = "Clock configuration register 0 (RCU_CFG0)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcccfg0(pub u32);
    impl Rcccfg0 {
        #[doc = "System clock switch"]
        #[must_use]
        #[inline(always)]
        pub const fn scs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn set_scs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[must_use]
        #[inline(always)]
        pub const fn scss(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn set_scss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ahbpsc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "AHB prescaler selection"]
        #[inline(always)]
        pub const fn set_ahbpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB1 prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn apb1psc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "APB1 prescaler selection"]
        #[inline(always)]
        pub const fn set_apb1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "APB2 prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn apb2psc(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "APB2 prescaler selection"]
        #[inline(always)]
        pub const fn set_apb2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[doc = "ADC clock prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn adcpsc_1_0(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "ADC clock prescaler selection"]
        #[inline(always)]
        pub const fn set_adcpsc_1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "PLL Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pllsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Clock Source Selection"]
        #[inline(always)]
        pub const fn set_pllsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The LSB of PREDV0 division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn predv0_lsb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "The LSB of PREDV0 division factor"]
        #[inline(always)]
        pub const fn set_predv0_lsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The PLL clock multiplication factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pllmf_3_0(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "The PLL clock multiplication factor"]
        #[inline(always)]
        pub const fn set_pllmf_3_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "USBFS clock prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usbfspsc_1_0(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "USBFS clock prescaler selection"]
        #[inline(always)]
        pub const fn set_usbfspsc_1_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "CKOUT0 Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ckout0sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "CKOUT0 Clock Source Selection"]
        #[inline(always)]
        pub const fn set_ckout0sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Bit 2 of ADCPSC"]
        #[must_use]
        #[inline(always)]
        pub const fn adcpsc_2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Bit 2 of ADCPSC"]
        #[inline(always)]
        pub const fn set_adcpsc_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Bit 5 and Bit 4 of PLLMF"]
        #[must_use]
        #[inline(always)]
        pub const fn pllmf_5_4(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Bit 5 and Bit 4 of PLLMF"]
        #[inline(always)]
        pub const fn set_pllmf_5_4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "Bit 2 of USBFSPSC"]
        #[must_use]
        #[inline(always)]
        pub const fn usbfspsc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bit 2 of USBFSPSC"]
        #[inline(always)]
        pub const fn set_usbfspsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rcccfg0 {
        #[inline(always)]
        fn default() -> Rcccfg0 {
            Rcccfg0(0)
        }
    }
    impl core::fmt::Debug for Rcccfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcccfg0")
                .field("scs", &self.scs())
                .field("scss", &self.scss())
                .field("ahbpsc", &self.ahbpsc())
                .field("apb1psc", &self.apb1psc())
                .field("apb2psc", &self.apb2psc())
                .field("adcpsc_1_0", &self.adcpsc_1_0())
                .field("pllsel", &self.pllsel())
                .field("predv0_lsb", &self.predv0_lsb())
                .field("pllmf_3_0", &self.pllmf_3_0())
                .field("usbfspsc_1_0", &self.usbfspsc_1_0())
                .field("ckout0sel", &self.ckout0sel())
                .field("adcpsc_2", &self.adcpsc_2())
                .field("pllmf_5_4", &self.pllmf_5_4())
                .field("usbfspsc", &self.usbfspsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcccfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rcccfg0 {{ scs: {=u8:?}, scss: {=u8:?}, ahbpsc: {=u8:?}, apb1psc: {=u8:?}, apb2psc: {=u8:?}, adcpsc_1_0: {=u8:?}, pllsel: {=bool:?}, predv0_lsb: {=bool:?}, pllmf_3_0: {=u8:?}, usbfspsc_1_0: {=u8:?}, ckout0sel: {=u8:?}, adcpsc_2: {=bool:?}, pllmf_5_4: {=u8:?}, usbfspsc: {=bool:?} }}",
                self.scs(),
                self.scss(),
                self.ahbpsc(),
                self.apb1psc(),
                self.apb2psc(),
                self.adcpsc_1_0(),
                self.pllsel(),
                self.predv0_lsb(),
                self.pllmf_3_0(),
                self.usbfspsc_1_0(),
                self.ckout0sel(),
                self.adcpsc_2(),
                self.pllmf_5_4(),
                self.usbfspsc()
            )
        }
    }
    #[doc = "Clock Configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcccfg1(pub u32);
    impl Rcccfg1 {
        #[doc = "PREDV0 division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn predv0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PREDV0 division factor"]
        #[inline(always)]
        pub const fn set_predv0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PREDV1 division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn predv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PREDV1 division factor"]
        #[inline(always)]
        pub const fn set_predv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "The PLL1 clock multiplication factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1mf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "The PLL1 clock multiplication factor"]
        #[inline(always)]
        pub const fn set_pll1mf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "The PLL2 clock multiplication factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2mf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "The PLL2 clock multiplication factor"]
        #[inline(always)]
        pub const fn set_pll2mf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "PREDV0 input Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn predv0sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PREDV0 input Clock Source Selection"]
        #[inline(always)]
        pub const fn set_predv0sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "I2S1 Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2s1sel(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "I2S1 Clock Source Selection"]
        #[inline(always)]
        pub const fn set_i2s1sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I2S2 Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2s2sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "I2S2 Clock Source Selection"]
        #[inline(always)]
        pub const fn set_i2s2sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bit 4 of ADCPSC"]
        #[must_use]
        #[inline(always)]
        pub const fn adcpsc_3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Bit 4 of ADCPSC"]
        #[inline(always)]
        pub const fn set_adcpsc_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PLL Clock Source Selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pllpresel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Clock Source Selection"]
        #[inline(always)]
        pub const fn set_pllpresel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bit 5 of PLL2MF"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2mf_4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bit 5 of PLL2MF"]
        #[inline(always)]
        pub const fn set_pll2mf_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rcccfg1 {
        #[inline(always)]
        fn default() -> Rcccfg1 {
            Rcccfg1(0)
        }
    }
    impl core::fmt::Debug for Rcccfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcccfg1")
                .field("predv0", &self.predv0())
                .field("predv1", &self.predv1())
                .field("pll1mf", &self.pll1mf())
                .field("pll2mf", &self.pll2mf())
                .field("predv0sel", &self.predv0sel())
                .field("i2s1sel", &self.i2s1sel())
                .field("i2s2sel", &self.i2s2sel())
                .field("adcpsc_3", &self.adcpsc_3())
                .field("pllpresel", &self.pllpresel())
                .field("pll2mf_4", &self.pll2mf_4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcccfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rcccfg1 {{ predv0: {=u8:?}, predv1: {=u8:?}, pll1mf: {=u8:?}, pll2mf: {=u8:?}, predv0sel: {=bool:?}, i2s1sel: {=bool:?}, i2s2sel: {=bool:?}, adcpsc_3: {=bool:?}, pllpresel: {=bool:?}, pll2mf_4: {=bool:?} }}",
                self.predv0(),
                self.predv1(),
                self.pll1mf(),
                self.pll2mf(),
                self.predv0sel(),
                self.i2s1sel(),
                self.i2s2sel(),
                self.adcpsc_3(),
                self.pllpresel(),
                self.pll2mf_4()
            )
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccctl(pub u32);
    impl Rccctl {
        #[doc = "Internal 8MHz RC oscillator Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8men(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal 8MHz RC oscillator Enable"]
        #[inline(always)]
        pub const fn set_irc8men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8mstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IRC8M Internal 8MHz RC Oscillator stabilization Flag"]
        #[inline(always)]
        pub const fn set_irc8mstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal 8MHz RC Oscillator clock trim adjust value"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8madj(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal 8MHz RC Oscillator clock trim adjust value"]
        #[inline(always)]
        pub const fn set_irc8madj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal 8MHz RC Oscillator calibration value register"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8mcalib(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal 8MHz RC Oscillator calibration value register"]
        #[inline(always)]
        pub const fn set_irc8mcalib(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "External High Speed oscillator Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed oscillator Enable"]
        #[inline(always)]
        pub const fn set_hxtalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External crystal oscillator (HXTAL) clock stabilization flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalstb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External crystal oscillator (HXTAL) clock stabilization flag"]
        #[inline(always)]
        pub const fn set_hxtalstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External crystal oscillator (HXTAL) clock bypass mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalbps(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External crystal oscillator (HXTAL) clock bypass mode enable"]
        #[inline(always)]
        pub const fn set_hxtalbps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HXTAL Clock Monitor Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ckmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL Clock Monitor Enable"]
        #[inline(always)]
        pub const fn set_ckmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub const fn set_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL Clock Stabilization Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllstb(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Clock Stabilization Flag"]
        #[inline(always)]
        pub const fn set_pllstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PLL1 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub const fn set_pll1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PLL1 Clock Stabilization Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1stb(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 Clock Stabilization Flag"]
        #[inline(always)]
        pub const fn set_pll1stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLL2 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 enable"]
        #[inline(always)]
        pub const fn set_pll2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PLL2 Clock Stabilization Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2stb(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 Clock Stabilization Flag"]
        #[inline(always)]
        pub const fn set_pll2stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccctl {
        #[inline(always)]
        fn default() -> Rccctl {
            Rccctl(0)
        }
    }
    impl core::fmt::Debug for Rccctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccctl")
                .field("irc8men", &self.irc8men())
                .field("irc8mstb", &self.irc8mstb())
                .field("irc8madj", &self.irc8madj())
                .field("irc8mcalib", &self.irc8mcalib())
                .field("hxtalen", &self.hxtalen())
                .field("hxtalstb", &self.hxtalstb())
                .field("hxtalbps", &self.hxtalbps())
                .field("ckmen", &self.ckmen())
                .field("pllen", &self.pllen())
                .field("pllstb", &self.pllstb())
                .field("pll1en", &self.pll1en())
                .field("pll1stb", &self.pll1stb())
                .field("pll2en", &self.pll2en())
                .field("pll2stb", &self.pll2stb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccctl {{ irc8men: {=bool:?}, irc8mstb: {=bool:?}, irc8madj: {=u8:?}, irc8mcalib: {=u8:?}, hxtalen: {=bool:?}, hxtalstb: {=bool:?}, hxtalbps: {=bool:?}, ckmen: {=bool:?}, pllen: {=bool:?}, pllstb: {=bool:?}, pll1en: {=bool:?}, pll1stb: {=bool:?}, pll2en: {=bool:?}, pll2stb: {=bool:?} }}",
                self.irc8men(),
                self.irc8mstb(),
                self.irc8madj(),
                self.irc8mcalib(),
                self.hxtalen(),
                self.hxtalstb(),
                self.hxtalbps(),
                self.ckmen(),
                self.pllen(),
                self.pllstb(),
                self.pll1en(),
                self.pll1stb(),
                self.pll2en(),
                self.pll2stb()
            )
        }
    }
    #[doc = "Deep sleep mode Voltage register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccdsv(pub u32);
    impl Rccdsv {
        #[doc = "Deep-sleep mode voltage select"]
        #[must_use]
        #[inline(always)]
        pub const fn dslpvs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Deep-sleep mode voltage select"]
        #[inline(always)]
        pub const fn set_dslpvs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Rccdsv {
        #[inline(always)]
        fn default() -> Rccdsv {
            Rccdsv(0)
        }
    }
    impl core::fmt::Debug for Rccdsv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccdsv").field("dslpvs", &self.dslpvs()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccdsv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rccdsv {{ dslpvs: {=u8:?} }}", self.dslpvs())
        }
    }
    #[doc = "Clock interrupt register (RCU_INT)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccint(pub u32);
    impl Rccint {
        #[doc = "IRC40K stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn irc40kstbif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IRC40K stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_irc40kstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LXTAL stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalstbif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LXTAL stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_lxtalstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IRC8M stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8mstbif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IRC8M stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_irc8mstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HXTAL stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalstbif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_hxtalstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllstbif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_pllstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1stbif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_pll1stbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL2 stabilization interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2stbif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 stabilization interrupt flag"]
        #[inline(always)]
        pub const fn set_pll2stbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HXTAL Clock Stuck Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ckmif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL Clock Stuck Interrupt Flag"]
        #[inline(always)]
        pub const fn set_ckmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IRC40K Stabilization interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc40kstbie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IRC40K Stabilization interrupt enable"]
        #[inline(always)]
        pub const fn set_irc40kstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LXTAL Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalstbie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LXTAL Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lxtalstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IRC8M Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8mstbie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IRC8M Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_irc8mstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HXTAL Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalstbie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hxtalstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllstbie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_pllstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PLL1 Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1stbie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_pll1stbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PLL2 Stabilization Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2stbie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 Stabilization Interrupt Enable"]
        #[inline(always)]
        pub const fn set_pll2stbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "IRC40K Stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn irc40kstbic(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IRC40K Stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_irc40kstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LXTAL Stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lxtalstbic(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LXTAL Stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lxtalstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IRC8M Stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn irc8mstbic(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IRC8M Stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_irc8mstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HXTAL Stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hxtalstbic(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL Stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hxtalstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pllstbic(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_pllstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pll1stbic(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_pll1stbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PLL2 stabilization Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pll2stbic(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 stabilization Interrupt Clear"]
        #[inline(always)]
        pub const fn set_pll2stbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "HXTAL Clock Stuck Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn ckmic(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "HXTAL Clock Stuck Interrupt Clear"]
        #[inline(always)]
        pub const fn set_ckmic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Rccint {
        #[inline(always)]
        fn default() -> Rccint {
            Rccint(0)
        }
    }
    impl core::fmt::Debug for Rccint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccint")
                .field("irc40kstbif", &self.irc40kstbif())
                .field("lxtalstbif", &self.lxtalstbif())
                .field("irc8mstbif", &self.irc8mstbif())
                .field("hxtalstbif", &self.hxtalstbif())
                .field("pllstbif", &self.pllstbif())
                .field("pll1stbif", &self.pll1stbif())
                .field("pll2stbif", &self.pll2stbif())
                .field("ckmif", &self.ckmif())
                .field("irc40kstbie", &self.irc40kstbie())
                .field("lxtalstbie", &self.lxtalstbie())
                .field("irc8mstbie", &self.irc8mstbie())
                .field("hxtalstbie", &self.hxtalstbie())
                .field("pllstbie", &self.pllstbie())
                .field("pll1stbie", &self.pll1stbie())
                .field("pll2stbie", &self.pll2stbie())
                .field("irc40kstbic", &self.irc40kstbic())
                .field("lxtalstbic", &self.lxtalstbic())
                .field("irc8mstbic", &self.irc8mstbic())
                .field("hxtalstbic", &self.hxtalstbic())
                .field("pllstbic", &self.pllstbic())
                .field("pll1stbic", &self.pll1stbic())
                .field("pll2stbic", &self.pll2stbic())
                .field("ckmic", &self.ckmic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccint {{ irc40kstbif: {=bool:?}, lxtalstbif: {=bool:?}, irc8mstbif: {=bool:?}, hxtalstbif: {=bool:?}, pllstbif: {=bool:?}, pll1stbif: {=bool:?}, pll2stbif: {=bool:?}, ckmif: {=bool:?}, irc40kstbie: {=bool:?}, lxtalstbie: {=bool:?}, irc8mstbie: {=bool:?}, hxtalstbie: {=bool:?}, pllstbie: {=bool:?}, pll1stbie: {=bool:?}, pll2stbie: {=bool:?}, irc40kstbic: {=bool:?}, lxtalstbic: {=bool:?}, irc8mstbic: {=bool:?}, hxtalstbic: {=bool:?}, pllstbic: {=bool:?}, pll1stbic: {=bool:?}, pll2stbic: {=bool:?}, ckmic: {=bool:?} }}",
                self.irc40kstbif(),
                self.lxtalstbif(),
                self.irc8mstbif(),
                self.hxtalstbif(),
                self.pllstbif(),
                self.pll1stbif(),
                self.pll2stbif(),
                self.ckmif(),
                self.irc40kstbie(),
                self.lxtalstbie(),
                self.irc8mstbie(),
                self.hxtalstbie(),
                self.pllstbie(),
                self.pll1stbie(),
                self.pll2stbie(),
                self.irc40kstbic(),
                self.lxtalstbic(),
                self.irc8mstbic(),
                self.hxtalstbic(),
                self.pllstbic(),
                self.pll1stbic(),
                self.pll2stbic(),
                self.ckmic()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuaddapb1en(pub u32);
    impl Rccrcuaddapb1en {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addapb1en_ctcen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addapb1en_ctcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rccrcuaddapb1en {
        #[inline(always)]
        fn default() -> Rccrcuaddapb1en {
            Rccrcuaddapb1en(0)
        }
    }
    impl core::fmt::Debug for Rccrcuaddapb1en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuaddapb1en")
                .field("rcu_addapb1en_ctcen", &self.rcu_addapb1en_ctcen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuaddapb1en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuaddapb1en {{ rcu_addapb1en_ctcen: {=bool:?} }}",
                self.rcu_addapb1en_ctcen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuaddapb1rst(pub u32);
    impl Rccrcuaddapb1rst {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addapb1rst_ctcrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addapb1rst_ctcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rccrcuaddapb1rst {
        #[inline(always)]
        fn default() -> Rccrcuaddapb1rst {
            Rccrcuaddapb1rst(0)
        }
    }
    impl core::fmt::Debug for Rccrcuaddapb1rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuaddapb1rst")
                .field("rcu_addapb1rst_ctcrst", &self.rcu_addapb1rst_ctcrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuaddapb1rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuaddapb1rst {{ rcu_addapb1rst_ctcrst: {=bool:?} }}",
                self.rcu_addapb1rst_ctcrst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuaddctl(pub u32);
    impl Rccrcuaddctl {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addctl_ck48msel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addctl_ck48msel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addctl_irc48men(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addctl_irc48men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addctl_irc48mstb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addctl_irc48mstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addctl_irc48mcal(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_addctl_irc48mcal(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Rccrcuaddctl {
        #[inline(always)]
        fn default() -> Rccrcuaddctl {
            Rccrcuaddctl(0)
        }
    }
    impl core::fmt::Debug for Rccrcuaddctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuaddctl")
                .field("rcu_addctl_ck48msel", &self.rcu_addctl_ck48msel())
                .field("rcu_addctl_irc48men", &self.rcu_addctl_irc48men())
                .field("rcu_addctl_irc48mstb", &self.rcu_addctl_irc48mstb())
                .field("rcu_addctl_irc48mcal", &self.rcu_addctl_irc48mcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuaddctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuaddctl {{ rcu_addctl_ck48msel: {=bool:?}, rcu_addctl_irc48men: {=bool:?}, rcu_addctl_irc48mstb: {=bool:?}, rcu_addctl_irc48mcal: {=u8:?} }}",
                self.rcu_addctl_ck48msel(),
                self.rcu_addctl_irc48men(),
                self.rcu_addctl_irc48mstb(),
                self.rcu_addctl_irc48mcal()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuaddint(pub u32);
    impl Rccrcuaddint {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addint_irc48mstbif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addint_irc48mstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addint_irc48mstbie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addint_irc48mstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_addint_irc48mstbic(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_addint_irc48mstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Rccrcuaddint {
        #[inline(always)]
        fn default() -> Rccrcuaddint {
            Rccrcuaddint(0)
        }
    }
    impl core::fmt::Debug for Rccrcuaddint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuaddint")
                .field("rcu_addint_irc48mstbif", &self.rcu_addint_irc48mstbif())
                .field("rcu_addint_irc48mstbie", &self.rcu_addint_irc48mstbie())
                .field("rcu_addint_irc48mstbic", &self.rcu_addint_irc48mstbic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuaddint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuaddint {{ rcu_addint_irc48mstbif: {=bool:?}, rcu_addint_irc48mstbie: {=bool:?}, rcu_addint_irc48mstbic: {=bool:?} }}",
                self.rcu_addint_irc48mstbif(),
                self.rcu_addint_irc48mstbie(),
                self.rcu_addint_irc48mstbic()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuahben(pub u32);
    impl Rccrcuahben {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_dma0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_dma0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_dma1en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_sramspen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_sramspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_fmcspen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_fmcspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_exmcen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_exmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_usbfsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_usbfsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_eneten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_eneten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_enettxen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_enettxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahben_enetrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahben_enetrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rccrcuahben {
        #[inline(always)]
        fn default() -> Rccrcuahben {
            Rccrcuahben(0)
        }
    }
    impl core::fmt::Debug for Rccrcuahben {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuahben")
                .field("rcu_ahben_dma0en", &self.rcu_ahben_dma0en())
                .field("rcu_ahben_dma1en", &self.rcu_ahben_dma1en())
                .field("rcu_ahben_sramspen", &self.rcu_ahben_sramspen())
                .field("rcu_ahben_fmcspen", &self.rcu_ahben_fmcspen())
                .field("rcu_ahben_crcen", &self.rcu_ahben_crcen())
                .field("rcu_ahben_exmcen", &self.rcu_ahben_exmcen())
                .field("rcu_ahben_usbfsen", &self.rcu_ahben_usbfsen())
                .field("rcu_ahben_eneten", &self.rcu_ahben_eneten())
                .field("rcu_ahben_enettxen", &self.rcu_ahben_enettxen())
                .field("rcu_ahben_enetrxen", &self.rcu_ahben_enetrxen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuahben {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuahben {{ rcu_ahben_dma0en: {=bool:?}, rcu_ahben_dma1en: {=bool:?}, rcu_ahben_sramspen: {=bool:?}, rcu_ahben_fmcspen: {=bool:?}, rcu_ahben_crcen: {=bool:?}, rcu_ahben_exmcen: {=bool:?}, rcu_ahben_usbfsen: {=bool:?}, rcu_ahben_eneten: {=bool:?}, rcu_ahben_enettxen: {=bool:?}, rcu_ahben_enetrxen: {=bool:?} }}",
                self.rcu_ahben_dma0en(),
                self.rcu_ahben_dma1en(),
                self.rcu_ahben_sramspen(),
                self.rcu_ahben_fmcspen(),
                self.rcu_ahben_crcen(),
                self.rcu_ahben_exmcen(),
                self.rcu_ahben_usbfsen(),
                self.rcu_ahben_eneten(),
                self.rcu_ahben_enettxen(),
                self.rcu_ahben_enetrxen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuahbrst(pub u32);
    impl Rccrcuahbrst {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahbrst_usbfsrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahbrst_usbfsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ahbrst_enetrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ahbrst_enetrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Rccrcuahbrst {
        #[inline(always)]
        fn default() -> Rccrcuahbrst {
            Rccrcuahbrst(0)
        }
    }
    impl core::fmt::Debug for Rccrcuahbrst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuahbrst")
                .field("rcu_ahbrst_usbfsrst", &self.rcu_ahbrst_usbfsrst())
                .field("rcu_ahbrst_enetrst", &self.rcu_ahbrst_enetrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuahbrst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuahbrst {{ rcu_ahbrst_usbfsrst: {=bool:?}, rcu_ahbrst_enetrst: {=bool:?} }}",
                self.rcu_ahbrst_usbfsrst(),
                self.rcu_ahbrst_enetrst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuapb1en(pub u32);
    impl Rccrcuapb1en {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer4en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer5en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer11en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer12en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_timer13en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_timer13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_wwdgten(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_wwdgten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_spi1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_spi2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_usart1en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_usart2en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_uart3en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_uart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_uart4en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_i2c0en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_i2c0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_i2c1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_can0en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_can0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_can1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_bkpien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_bkpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_pmuen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_pmuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1en_dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1en_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccrcuapb1en {
        #[inline(always)]
        fn default() -> Rccrcuapb1en {
            Rccrcuapb1en(0)
        }
    }
    impl core::fmt::Debug for Rccrcuapb1en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuapb1en")
                .field("rcu_apb1en_timer1en", &self.rcu_apb1en_timer1en())
                .field("rcu_apb1en_timer2en", &self.rcu_apb1en_timer2en())
                .field("rcu_apb1en_timer3en", &self.rcu_apb1en_timer3en())
                .field("rcu_apb1en_timer4en", &self.rcu_apb1en_timer4en())
                .field("rcu_apb1en_timer5en", &self.rcu_apb1en_timer5en())
                .field("rcu_apb1en_timer6en", &self.rcu_apb1en_timer6en())
                .field("rcu_apb1en_timer11en", &self.rcu_apb1en_timer11en())
                .field("rcu_apb1en_timer12en", &self.rcu_apb1en_timer12en())
                .field("rcu_apb1en_timer13en", &self.rcu_apb1en_timer13en())
                .field("rcu_apb1en_wwdgten", &self.rcu_apb1en_wwdgten())
                .field("rcu_apb1en_spi1en", &self.rcu_apb1en_spi1en())
                .field("rcu_apb1en_spi2en", &self.rcu_apb1en_spi2en())
                .field("rcu_apb1en_usart1en", &self.rcu_apb1en_usart1en())
                .field("rcu_apb1en_usart2en", &self.rcu_apb1en_usart2en())
                .field("rcu_apb1en_uart3en", &self.rcu_apb1en_uart3en())
                .field("rcu_apb1en_uart4en", &self.rcu_apb1en_uart4en())
                .field("rcu_apb1en_i2c0en", &self.rcu_apb1en_i2c0en())
                .field("rcu_apb1en_i2c1en", &self.rcu_apb1en_i2c1en())
                .field("rcu_apb1en_can0en", &self.rcu_apb1en_can0en())
                .field("rcu_apb1en_can1en", &self.rcu_apb1en_can1en())
                .field("rcu_apb1en_bkpien", &self.rcu_apb1en_bkpien())
                .field("rcu_apb1en_pmuen", &self.rcu_apb1en_pmuen())
                .field("rcu_apb1en_dacen", &self.rcu_apb1en_dacen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuapb1en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuapb1en {{ rcu_apb1en_timer1en: {=bool:?}, rcu_apb1en_timer2en: {=bool:?}, rcu_apb1en_timer3en: {=bool:?}, rcu_apb1en_timer4en: {=bool:?}, rcu_apb1en_timer5en: {=bool:?}, rcu_apb1en_timer6en: {=bool:?}, rcu_apb1en_timer11en: {=bool:?}, rcu_apb1en_timer12en: {=bool:?}, rcu_apb1en_timer13en: {=bool:?}, rcu_apb1en_wwdgten: {=bool:?}, rcu_apb1en_spi1en: {=bool:?}, rcu_apb1en_spi2en: {=bool:?}, rcu_apb1en_usart1en: {=bool:?}, rcu_apb1en_usart2en: {=bool:?}, rcu_apb1en_uart3en: {=bool:?}, rcu_apb1en_uart4en: {=bool:?}, rcu_apb1en_i2c0en: {=bool:?}, rcu_apb1en_i2c1en: {=bool:?}, rcu_apb1en_can0en: {=bool:?}, rcu_apb1en_can1en: {=bool:?}, rcu_apb1en_bkpien: {=bool:?}, rcu_apb1en_pmuen: {=bool:?}, rcu_apb1en_dacen: {=bool:?} }}",
                self.rcu_apb1en_timer1en(),
                self.rcu_apb1en_timer2en(),
                self.rcu_apb1en_timer3en(),
                self.rcu_apb1en_timer4en(),
                self.rcu_apb1en_timer5en(),
                self.rcu_apb1en_timer6en(),
                self.rcu_apb1en_timer11en(),
                self.rcu_apb1en_timer12en(),
                self.rcu_apb1en_timer13en(),
                self.rcu_apb1en_wwdgten(),
                self.rcu_apb1en_spi1en(),
                self.rcu_apb1en_spi2en(),
                self.rcu_apb1en_usart1en(),
                self.rcu_apb1en_usart2en(),
                self.rcu_apb1en_uart3en(),
                self.rcu_apb1en_uart4en(),
                self.rcu_apb1en_i2c0en(),
                self.rcu_apb1en_i2c1en(),
                self.rcu_apb1en_can0en(),
                self.rcu_apb1en_can1en(),
                self.rcu_apb1en_bkpien(),
                self.rcu_apb1en_pmuen(),
                self.rcu_apb1en_dacen()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuapb1rst(pub u32);
    impl Rccrcuapb1rst {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer3rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer4rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer5rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer11rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer12rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_timer13rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_timer13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_wwdgtrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_wwdgtrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_spi1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_spi2rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_usart1rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_usart2rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_uart3rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_uart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_uart4rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_i2c0rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_i2c0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_i2c1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_can0rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_can0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_can1rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_bkpirst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_bkpirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_pmurst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_pmurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb1rst_dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb1rst_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccrcuapb1rst {
        #[inline(always)]
        fn default() -> Rccrcuapb1rst {
            Rccrcuapb1rst(0)
        }
    }
    impl core::fmt::Debug for Rccrcuapb1rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuapb1rst")
                .field("rcu_apb1rst_timer1rst", &self.rcu_apb1rst_timer1rst())
                .field("rcu_apb1rst_timer2rst", &self.rcu_apb1rst_timer2rst())
                .field("rcu_apb1rst_timer3rst", &self.rcu_apb1rst_timer3rst())
                .field("rcu_apb1rst_timer4rst", &self.rcu_apb1rst_timer4rst())
                .field("rcu_apb1rst_timer5rst", &self.rcu_apb1rst_timer5rst())
                .field("rcu_apb1rst_timer6rst", &self.rcu_apb1rst_timer6rst())
                .field("rcu_apb1rst_timer11rst", &self.rcu_apb1rst_timer11rst())
                .field("rcu_apb1rst_timer12rst", &self.rcu_apb1rst_timer12rst())
                .field("rcu_apb1rst_timer13rst", &self.rcu_apb1rst_timer13rst())
                .field("rcu_apb1rst_wwdgtrst", &self.rcu_apb1rst_wwdgtrst())
                .field("rcu_apb1rst_spi1rst", &self.rcu_apb1rst_spi1rst())
                .field("rcu_apb1rst_spi2rst", &self.rcu_apb1rst_spi2rst())
                .field("rcu_apb1rst_usart1rst", &self.rcu_apb1rst_usart1rst())
                .field("rcu_apb1rst_usart2rst", &self.rcu_apb1rst_usart2rst())
                .field("rcu_apb1rst_uart3rst", &self.rcu_apb1rst_uart3rst())
                .field("rcu_apb1rst_uart4rst", &self.rcu_apb1rst_uart4rst())
                .field("rcu_apb1rst_i2c0rst", &self.rcu_apb1rst_i2c0rst())
                .field("rcu_apb1rst_i2c1rst", &self.rcu_apb1rst_i2c1rst())
                .field("rcu_apb1rst_can0rst", &self.rcu_apb1rst_can0rst())
                .field("rcu_apb1rst_can1rst", &self.rcu_apb1rst_can1rst())
                .field("rcu_apb1rst_bkpirst", &self.rcu_apb1rst_bkpirst())
                .field("rcu_apb1rst_pmurst", &self.rcu_apb1rst_pmurst())
                .field("rcu_apb1rst_dacrst", &self.rcu_apb1rst_dacrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuapb1rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuapb1rst {{ rcu_apb1rst_timer1rst: {=bool:?}, rcu_apb1rst_timer2rst: {=bool:?}, rcu_apb1rst_timer3rst: {=bool:?}, rcu_apb1rst_timer4rst: {=bool:?}, rcu_apb1rst_timer5rst: {=bool:?}, rcu_apb1rst_timer6rst: {=bool:?}, rcu_apb1rst_timer11rst: {=bool:?}, rcu_apb1rst_timer12rst: {=bool:?}, rcu_apb1rst_timer13rst: {=bool:?}, rcu_apb1rst_wwdgtrst: {=bool:?}, rcu_apb1rst_spi1rst: {=bool:?}, rcu_apb1rst_spi2rst: {=bool:?}, rcu_apb1rst_usart1rst: {=bool:?}, rcu_apb1rst_usart2rst: {=bool:?}, rcu_apb1rst_uart3rst: {=bool:?}, rcu_apb1rst_uart4rst: {=bool:?}, rcu_apb1rst_i2c0rst: {=bool:?}, rcu_apb1rst_i2c1rst: {=bool:?}, rcu_apb1rst_can0rst: {=bool:?}, rcu_apb1rst_can1rst: {=bool:?}, rcu_apb1rst_bkpirst: {=bool:?}, rcu_apb1rst_pmurst: {=bool:?}, rcu_apb1rst_dacrst: {=bool:?} }}",
                self.rcu_apb1rst_timer1rst(),
                self.rcu_apb1rst_timer2rst(),
                self.rcu_apb1rst_timer3rst(),
                self.rcu_apb1rst_timer4rst(),
                self.rcu_apb1rst_timer5rst(),
                self.rcu_apb1rst_timer6rst(),
                self.rcu_apb1rst_timer11rst(),
                self.rcu_apb1rst_timer12rst(),
                self.rcu_apb1rst_timer13rst(),
                self.rcu_apb1rst_wwdgtrst(),
                self.rcu_apb1rst_spi1rst(),
                self.rcu_apb1rst_spi2rst(),
                self.rcu_apb1rst_usart1rst(),
                self.rcu_apb1rst_usart2rst(),
                self.rcu_apb1rst_uart3rst(),
                self.rcu_apb1rst_uart4rst(),
                self.rcu_apb1rst_i2c0rst(),
                self.rcu_apb1rst_i2c1rst(),
                self.rcu_apb1rst_can0rst(),
                self.rcu_apb1rst_can1rst(),
                self.rcu_apb1rst_bkpirst(),
                self.rcu_apb1rst_pmurst(),
                self.rcu_apb1rst_dacrst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuapb2en(pub u32);
    impl Rccrcuapb2en {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_afen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_afen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_paen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_paen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_pben(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_pben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_pcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_pcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_pden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_peen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_peen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_pfen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_pfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_pgen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_pgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_adc0en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_adc0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_adc1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_timer0en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_timer0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_spi0en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_spi0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_timer7en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_timer7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_usart0en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_usart0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_timer8en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_timer8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_timer9en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_timer9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2en_timer10en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2en_timer10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Rccrcuapb2en {
        #[inline(always)]
        fn default() -> Rccrcuapb2en {
            Rccrcuapb2en(0)
        }
    }
    impl core::fmt::Debug for Rccrcuapb2en {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuapb2en")
                .field("rcu_apb2en_afen", &self.rcu_apb2en_afen())
                .field("rcu_apb2en_paen", &self.rcu_apb2en_paen())
                .field("rcu_apb2en_pben", &self.rcu_apb2en_pben())
                .field("rcu_apb2en_pcen", &self.rcu_apb2en_pcen())
                .field("rcu_apb2en_pden", &self.rcu_apb2en_pden())
                .field("rcu_apb2en_peen", &self.rcu_apb2en_peen())
                .field("rcu_apb2en_pfen", &self.rcu_apb2en_pfen())
                .field("rcu_apb2en_pgen", &self.rcu_apb2en_pgen())
                .field("rcu_apb2en_adc0en", &self.rcu_apb2en_adc0en())
                .field("rcu_apb2en_adc1en", &self.rcu_apb2en_adc1en())
                .field("rcu_apb2en_timer0en", &self.rcu_apb2en_timer0en())
                .field("rcu_apb2en_spi0en", &self.rcu_apb2en_spi0en())
                .field("rcu_apb2en_timer7en", &self.rcu_apb2en_timer7en())
                .field("rcu_apb2en_usart0en", &self.rcu_apb2en_usart0en())
                .field("rcu_apb2en_timer8en", &self.rcu_apb2en_timer8en())
                .field("rcu_apb2en_timer9en", &self.rcu_apb2en_timer9en())
                .field("rcu_apb2en_timer10en", &self.rcu_apb2en_timer10en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuapb2en {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuapb2en {{ rcu_apb2en_afen: {=bool:?}, rcu_apb2en_paen: {=bool:?}, rcu_apb2en_pben: {=bool:?}, rcu_apb2en_pcen: {=bool:?}, rcu_apb2en_pden: {=bool:?}, rcu_apb2en_peen: {=bool:?}, rcu_apb2en_pfen: {=bool:?}, rcu_apb2en_pgen: {=bool:?}, rcu_apb2en_adc0en: {=bool:?}, rcu_apb2en_adc1en: {=bool:?}, rcu_apb2en_timer0en: {=bool:?}, rcu_apb2en_spi0en: {=bool:?}, rcu_apb2en_timer7en: {=bool:?}, rcu_apb2en_usart0en: {=bool:?}, rcu_apb2en_timer8en: {=bool:?}, rcu_apb2en_timer9en: {=bool:?}, rcu_apb2en_timer10en: {=bool:?} }}",
                self.rcu_apb2en_afen(),
                self.rcu_apb2en_paen(),
                self.rcu_apb2en_pben(),
                self.rcu_apb2en_pcen(),
                self.rcu_apb2en_pden(),
                self.rcu_apb2en_peen(),
                self.rcu_apb2en_pfen(),
                self.rcu_apb2en_pgen(),
                self.rcu_apb2en_adc0en(),
                self.rcu_apb2en_adc1en(),
                self.rcu_apb2en_timer0en(),
                self.rcu_apb2en_spi0en(),
                self.rcu_apb2en_timer7en(),
                self.rcu_apb2en_usart0en(),
                self.rcu_apb2en_timer8en(),
                self.rcu_apb2en_timer9en(),
                self.rcu_apb2en_timer10en()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuapb2rst(pub u32);
    impl Rccrcuapb2rst {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_afrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_afrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_parst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_parst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_pbrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_pbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_pcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_pcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_pdrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_pdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_perst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_perst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_pfrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_pfrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_pgrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_pgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_adc0rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_adc0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_adc1rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_timer0rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_timer0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_spi0rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_spi0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_timer7rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_timer7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_usart0rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_usart0rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_timer8rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_timer8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_timer9rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_timer9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_apb2rst_timer10rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_apb2rst_timer10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Rccrcuapb2rst {
        #[inline(always)]
        fn default() -> Rccrcuapb2rst {
            Rccrcuapb2rst(0)
        }
    }
    impl core::fmt::Debug for Rccrcuapb2rst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuapb2rst")
                .field("rcu_apb2rst_afrst", &self.rcu_apb2rst_afrst())
                .field("rcu_apb2rst_parst", &self.rcu_apb2rst_parst())
                .field("rcu_apb2rst_pbrst", &self.rcu_apb2rst_pbrst())
                .field("rcu_apb2rst_pcrst", &self.rcu_apb2rst_pcrst())
                .field("rcu_apb2rst_pdrst", &self.rcu_apb2rst_pdrst())
                .field("rcu_apb2rst_perst", &self.rcu_apb2rst_perst())
                .field("rcu_apb2rst_pfrst", &self.rcu_apb2rst_pfrst())
                .field("rcu_apb2rst_pgrst", &self.rcu_apb2rst_pgrst())
                .field("rcu_apb2rst_adc0rst", &self.rcu_apb2rst_adc0rst())
                .field("rcu_apb2rst_adc1rst", &self.rcu_apb2rst_adc1rst())
                .field("rcu_apb2rst_timer0rst", &self.rcu_apb2rst_timer0rst())
                .field("rcu_apb2rst_spi0rst", &self.rcu_apb2rst_spi0rst())
                .field("rcu_apb2rst_timer7rst", &self.rcu_apb2rst_timer7rst())
                .field("rcu_apb2rst_usart0rst", &self.rcu_apb2rst_usart0rst())
                .field("rcu_apb2rst_timer8rst", &self.rcu_apb2rst_timer8rst())
                .field("rcu_apb2rst_timer9rst", &self.rcu_apb2rst_timer9rst())
                .field("rcu_apb2rst_timer10rst", &self.rcu_apb2rst_timer10rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuapb2rst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuapb2rst {{ rcu_apb2rst_afrst: {=bool:?}, rcu_apb2rst_parst: {=bool:?}, rcu_apb2rst_pbrst: {=bool:?}, rcu_apb2rst_pcrst: {=bool:?}, rcu_apb2rst_pdrst: {=bool:?}, rcu_apb2rst_perst: {=bool:?}, rcu_apb2rst_pfrst: {=bool:?}, rcu_apb2rst_pgrst: {=bool:?}, rcu_apb2rst_adc0rst: {=bool:?}, rcu_apb2rst_adc1rst: {=bool:?}, rcu_apb2rst_timer0rst: {=bool:?}, rcu_apb2rst_spi0rst: {=bool:?}, rcu_apb2rst_timer7rst: {=bool:?}, rcu_apb2rst_usart0rst: {=bool:?}, rcu_apb2rst_timer8rst: {=bool:?}, rcu_apb2rst_timer9rst: {=bool:?}, rcu_apb2rst_timer10rst: {=bool:?} }}",
                self.rcu_apb2rst_afrst(),
                self.rcu_apb2rst_parst(),
                self.rcu_apb2rst_pbrst(),
                self.rcu_apb2rst_pcrst(),
                self.rcu_apb2rst_pdrst(),
                self.rcu_apb2rst_perst(),
                self.rcu_apb2rst_pfrst(),
                self.rcu_apb2rst_pgrst(),
                self.rcu_apb2rst_adc0rst(),
                self.rcu_apb2rst_adc1rst(),
                self.rcu_apb2rst_timer0rst(),
                self.rcu_apb2rst_spi0rst(),
                self.rcu_apb2rst_timer7rst(),
                self.rcu_apb2rst_usart0rst(),
                self.rcu_apb2rst_timer8rst(),
                self.rcu_apb2rst_timer9rst(),
                self.rcu_apb2rst_timer10rst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcubdctl(pub u32);
    impl Rccrcubdctl {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_lxtalen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_lxtalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_lxtalstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_lxtalstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_lxtalbps(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_lxtalbps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_lxtaldri(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_lxtaldri(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_rtcsrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_rtcsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_bdctl_bkprst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_bdctl_bkprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rccrcubdctl {
        #[inline(always)]
        fn default() -> Rccrcubdctl {
            Rccrcubdctl(0)
        }
    }
    impl core::fmt::Debug for Rccrcubdctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcubdctl")
                .field("rcu_bdctl_lxtalen", &self.rcu_bdctl_lxtalen())
                .field("rcu_bdctl_lxtalstb", &self.rcu_bdctl_lxtalstb())
                .field("rcu_bdctl_lxtalbps", &self.rcu_bdctl_lxtalbps())
                .field("rcu_bdctl_lxtaldri", &self.rcu_bdctl_lxtaldri())
                .field("rcu_bdctl_rtcsrc", &self.rcu_bdctl_rtcsrc())
                .field("rcu_bdctl_rtcen", &self.rcu_bdctl_rtcen())
                .field("rcu_bdctl_bkprst", &self.rcu_bdctl_bkprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcubdctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcubdctl {{ rcu_bdctl_lxtalen: {=bool:?}, rcu_bdctl_lxtalstb: {=bool:?}, rcu_bdctl_lxtalbps: {=bool:?}, rcu_bdctl_lxtaldri: {=u8:?}, rcu_bdctl_rtcsrc: {=u8:?}, rcu_bdctl_rtcen: {=bool:?}, rcu_bdctl_bkprst: {=bool:?} }}",
                self.rcu_bdctl_lxtalen(),
                self.rcu_bdctl_lxtalstb(),
                self.rcu_bdctl_lxtalbps(),
                self.rcu_bdctl_lxtaldri(),
                self.rcu_bdctl_rtcsrc(),
                self.rcu_bdctl_rtcen(),
                self.rcu_bdctl_bkprst()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcucfg0(pub u32);
    impl Rccrcucfg0 {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_scs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_scs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_scss(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_scss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_ahbpsc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_ahbpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_apb1psc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_apb1psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_apb2psc(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_apb2psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_adcpsc(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_adcpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_pllsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_pllsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_predv0_lsb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_predv0_lsb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_pllmf(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_pllmf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_usbfspsc(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_usbfspsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_ckout0sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_ckout0sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_adcpsc_2(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_adcpsc_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_pllmf_4(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_pllmf_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_pllmf_5(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_pllmf_5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg0_usbfspsc_2(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg0_usbfspsc_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rccrcucfg0 {
        #[inline(always)]
        fn default() -> Rccrcucfg0 {
            Rccrcucfg0(0)
        }
    }
    impl core::fmt::Debug for Rccrcucfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcucfg0")
                .field("rcu_cfg0_scs", &self.rcu_cfg0_scs())
                .field("rcu_cfg0_scss", &self.rcu_cfg0_scss())
                .field("rcu_cfg0_ahbpsc", &self.rcu_cfg0_ahbpsc())
                .field("rcu_cfg0_apb1psc", &self.rcu_cfg0_apb1psc())
                .field("rcu_cfg0_apb2psc", &self.rcu_cfg0_apb2psc())
                .field("rcu_cfg0_adcpsc", &self.rcu_cfg0_adcpsc())
                .field("rcu_cfg0_pllsel", &self.rcu_cfg0_pllsel())
                .field("rcu_cfg0_predv0_lsb", &self.rcu_cfg0_predv0_lsb())
                .field("rcu_cfg0_pllmf", &self.rcu_cfg0_pllmf())
                .field("rcu_cfg0_usbfspsc", &self.rcu_cfg0_usbfspsc())
                .field("rcu_cfg0_ckout0sel", &self.rcu_cfg0_ckout0sel())
                .field("rcu_cfg0_adcpsc_2", &self.rcu_cfg0_adcpsc_2())
                .field("rcu_cfg0_pllmf_4", &self.rcu_cfg0_pllmf_4())
                .field("rcu_cfg0_pllmf_5", &self.rcu_cfg0_pllmf_5())
                .field("rcu_cfg0_usbfspsc_2", &self.rcu_cfg0_usbfspsc_2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcucfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcucfg0 {{ rcu_cfg0_scs: {=u8:?}, rcu_cfg0_scss: {=u8:?}, rcu_cfg0_ahbpsc: {=u8:?}, rcu_cfg0_apb1psc: {=u8:?}, rcu_cfg0_apb2psc: {=u8:?}, rcu_cfg0_adcpsc: {=u8:?}, rcu_cfg0_pllsel: {=bool:?}, rcu_cfg0_predv0_lsb: {=bool:?}, rcu_cfg0_pllmf: {=u8:?}, rcu_cfg0_usbfspsc: {=u8:?}, rcu_cfg0_ckout0sel: {=u8:?}, rcu_cfg0_adcpsc_2: {=bool:?}, rcu_cfg0_pllmf_4: {=bool:?}, rcu_cfg0_pllmf_5: {=bool:?}, rcu_cfg0_usbfspsc_2: {=bool:?} }}",
                self.rcu_cfg0_scs(),
                self.rcu_cfg0_scss(),
                self.rcu_cfg0_ahbpsc(),
                self.rcu_cfg0_apb1psc(),
                self.rcu_cfg0_apb2psc(),
                self.rcu_cfg0_adcpsc(),
                self.rcu_cfg0_pllsel(),
                self.rcu_cfg0_predv0_lsb(),
                self.rcu_cfg0_pllmf(),
                self.rcu_cfg0_usbfspsc(),
                self.rcu_cfg0_ckout0sel(),
                self.rcu_cfg0_adcpsc_2(),
                self.rcu_cfg0_pllmf_4(),
                self.rcu_cfg0_pllmf_5(),
                self.rcu_cfg0_usbfspsc_2()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcucfg1(pub u32);
    impl Rccrcucfg1 {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_predv0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_predv0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_predv1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_predv1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_pll1mf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_pll1mf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_pll2mf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_pll2mf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_predv0sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_predv0sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_i2s1sel(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_i2s1sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_i2s2sel(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_i2s2sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_adcpsc_3(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_adcpsc_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_pllpresel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_pllpresel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_cfg1_pll2mf_4(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_cfg1_pll2mf_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rccrcucfg1 {
        #[inline(always)]
        fn default() -> Rccrcucfg1 {
            Rccrcucfg1(0)
        }
    }
    impl core::fmt::Debug for Rccrcucfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcucfg1")
                .field("rcu_cfg1_predv0", &self.rcu_cfg1_predv0())
                .field("rcu_cfg1_predv1", &self.rcu_cfg1_predv1())
                .field("rcu_cfg1_pll1mf", &self.rcu_cfg1_pll1mf())
                .field("rcu_cfg1_pll2mf", &self.rcu_cfg1_pll2mf())
                .field("rcu_cfg1_predv0sel", &self.rcu_cfg1_predv0sel())
                .field("rcu_cfg1_i2s1sel", &self.rcu_cfg1_i2s1sel())
                .field("rcu_cfg1_i2s2sel", &self.rcu_cfg1_i2s2sel())
                .field("rcu_cfg1_adcpsc_3", &self.rcu_cfg1_adcpsc_3())
                .field("rcu_cfg1_pllpresel", &self.rcu_cfg1_pllpresel())
                .field("rcu_cfg1_pll2mf_4", &self.rcu_cfg1_pll2mf_4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcucfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcucfg1 {{ rcu_cfg1_predv0: {=u8:?}, rcu_cfg1_predv1: {=u8:?}, rcu_cfg1_pll1mf: {=u8:?}, rcu_cfg1_pll2mf: {=u8:?}, rcu_cfg1_predv0sel: {=bool:?}, rcu_cfg1_i2s1sel: {=bool:?}, rcu_cfg1_i2s2sel: {=bool:?}, rcu_cfg1_adcpsc_3: {=bool:?}, rcu_cfg1_pllpresel: {=bool:?}, rcu_cfg1_pll2mf_4: {=bool:?} }}",
                self.rcu_cfg1_predv0(),
                self.rcu_cfg1_predv1(),
                self.rcu_cfg1_pll1mf(),
                self.rcu_cfg1_pll2mf(),
                self.rcu_cfg1_predv0sel(),
                self.rcu_cfg1_i2s1sel(),
                self.rcu_cfg1_i2s2sel(),
                self.rcu_cfg1_adcpsc_3(),
                self.rcu_cfg1_pllpresel(),
                self.rcu_cfg1_pll2mf_4()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuctl(pub u32);
    impl Rccrcuctl {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_irc8men(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_irc8men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_irc8mstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_irc8mstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_irc8madj(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_irc8madj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_irc8mcalib(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_irc8mcalib(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_hxtalen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_hxtalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_hxtalstb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_hxtalstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_hxtalbps(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_hxtalbps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_ckmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_ckmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pllen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pllstb(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pllstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pll1en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pll1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pll1stb(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pll1stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pll2en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pll2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_ctl_pll2stb(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_ctl_pll2stb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Rccrcuctl {
        #[inline(always)]
        fn default() -> Rccrcuctl {
            Rccrcuctl(0)
        }
    }
    impl core::fmt::Debug for Rccrcuctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuctl")
                .field("rcu_ctl_irc8men", &self.rcu_ctl_irc8men())
                .field("rcu_ctl_irc8mstb", &self.rcu_ctl_irc8mstb())
                .field("rcu_ctl_irc8madj", &self.rcu_ctl_irc8madj())
                .field("rcu_ctl_irc8mcalib", &self.rcu_ctl_irc8mcalib())
                .field("rcu_ctl_hxtalen", &self.rcu_ctl_hxtalen())
                .field("rcu_ctl_hxtalstb", &self.rcu_ctl_hxtalstb())
                .field("rcu_ctl_hxtalbps", &self.rcu_ctl_hxtalbps())
                .field("rcu_ctl_ckmen", &self.rcu_ctl_ckmen())
                .field("rcu_ctl_pllen", &self.rcu_ctl_pllen())
                .field("rcu_ctl_pllstb", &self.rcu_ctl_pllstb())
                .field("rcu_ctl_pll1en", &self.rcu_ctl_pll1en())
                .field("rcu_ctl_pll1stb", &self.rcu_ctl_pll1stb())
                .field("rcu_ctl_pll2en", &self.rcu_ctl_pll2en())
                .field("rcu_ctl_pll2stb", &self.rcu_ctl_pll2stb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuctl {{ rcu_ctl_irc8men: {=bool:?}, rcu_ctl_irc8mstb: {=bool:?}, rcu_ctl_irc8madj: {=u8:?}, rcu_ctl_irc8mcalib: {=u8:?}, rcu_ctl_hxtalen: {=bool:?}, rcu_ctl_hxtalstb: {=bool:?}, rcu_ctl_hxtalbps: {=bool:?}, rcu_ctl_ckmen: {=bool:?}, rcu_ctl_pllen: {=bool:?}, rcu_ctl_pllstb: {=bool:?}, rcu_ctl_pll1en: {=bool:?}, rcu_ctl_pll1stb: {=bool:?}, rcu_ctl_pll2en: {=bool:?}, rcu_ctl_pll2stb: {=bool:?} }}",
                self.rcu_ctl_irc8men(),
                self.rcu_ctl_irc8mstb(),
                self.rcu_ctl_irc8madj(),
                self.rcu_ctl_irc8mcalib(),
                self.rcu_ctl_hxtalen(),
                self.rcu_ctl_hxtalstb(),
                self.rcu_ctl_hxtalbps(),
                self.rcu_ctl_ckmen(),
                self.rcu_ctl_pllen(),
                self.rcu_ctl_pllstb(),
                self.rcu_ctl_pll1en(),
                self.rcu_ctl_pll1stb(),
                self.rcu_ctl_pll2en(),
                self.rcu_ctl_pll2stb()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcudsv(pub u32);
    impl Rccrcudsv {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_dsv_dslpvs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_rcu_dsv_dslpvs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Rccrcudsv {
        #[inline(always)]
        fn default() -> Rccrcudsv {
            Rccrcudsv(0)
        }
    }
    impl core::fmt::Debug for Rccrcudsv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcudsv")
                .field("rcu_dsv_dslpvs", &self.rcu_dsv_dslpvs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcudsv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rccrcudsv {{ rcu_dsv_dslpvs: {=u8:?} }}", self.rcu_dsv_dslpvs())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcuint(pub u32);
    impl Rccrcuint {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc40kstbif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc40kstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_lxtalstbif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_lxtalstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc8mstbif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc8mstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_hxtalstbif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_hxtalstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pllstbif(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pllstbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll1stbif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll1stbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll2stbif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll2stbif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_ckmif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_ckmif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc40kstbie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc40kstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_lxtalstbie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_lxtalstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc8mstbie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc8mstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_hxtalstbie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_hxtalstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pllstbie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pllstbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll1stbie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll1stbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll2stbie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll2stbie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc40kstbic(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc40kstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_lxtalstbic(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_lxtalstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_irc8mstbic(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_irc8mstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_hxtalstbic(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_hxtalstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pllstbic(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pllstbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll1stbic(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll1stbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_pll2stbic(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_pll2stbic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_int_ckmic(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_int_ckmic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Rccrcuint {
        #[inline(always)]
        fn default() -> Rccrcuint {
            Rccrcuint(0)
        }
    }
    impl core::fmt::Debug for Rccrcuint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcuint")
                .field("rcu_int_irc40kstbif", &self.rcu_int_irc40kstbif())
                .field("rcu_int_lxtalstbif", &self.rcu_int_lxtalstbif())
                .field("rcu_int_irc8mstbif", &self.rcu_int_irc8mstbif())
                .field("rcu_int_hxtalstbif", &self.rcu_int_hxtalstbif())
                .field("rcu_int_pllstbif", &self.rcu_int_pllstbif())
                .field("rcu_int_pll1stbif", &self.rcu_int_pll1stbif())
                .field("rcu_int_pll2stbif", &self.rcu_int_pll2stbif())
                .field("rcu_int_ckmif", &self.rcu_int_ckmif())
                .field("rcu_int_irc40kstbie", &self.rcu_int_irc40kstbie())
                .field("rcu_int_lxtalstbie", &self.rcu_int_lxtalstbie())
                .field("rcu_int_irc8mstbie", &self.rcu_int_irc8mstbie())
                .field("rcu_int_hxtalstbie", &self.rcu_int_hxtalstbie())
                .field("rcu_int_pllstbie", &self.rcu_int_pllstbie())
                .field("rcu_int_pll1stbie", &self.rcu_int_pll1stbie())
                .field("rcu_int_pll2stbie", &self.rcu_int_pll2stbie())
                .field("rcu_int_irc40kstbic", &self.rcu_int_irc40kstbic())
                .field("rcu_int_lxtalstbic", &self.rcu_int_lxtalstbic())
                .field("rcu_int_irc8mstbic", &self.rcu_int_irc8mstbic())
                .field("rcu_int_hxtalstbic", &self.rcu_int_hxtalstbic())
                .field("rcu_int_pllstbic", &self.rcu_int_pllstbic())
                .field("rcu_int_pll1stbic", &self.rcu_int_pll1stbic())
                .field("rcu_int_pll2stbic", &self.rcu_int_pll2stbic())
                .field("rcu_int_ckmic", &self.rcu_int_ckmic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcuint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcuint {{ rcu_int_irc40kstbif: {=bool:?}, rcu_int_lxtalstbif: {=bool:?}, rcu_int_irc8mstbif: {=bool:?}, rcu_int_hxtalstbif: {=bool:?}, rcu_int_pllstbif: {=bool:?}, rcu_int_pll1stbif: {=bool:?}, rcu_int_pll2stbif: {=bool:?}, rcu_int_ckmif: {=bool:?}, rcu_int_irc40kstbie: {=bool:?}, rcu_int_lxtalstbie: {=bool:?}, rcu_int_irc8mstbie: {=bool:?}, rcu_int_hxtalstbie: {=bool:?}, rcu_int_pllstbie: {=bool:?}, rcu_int_pll1stbie: {=bool:?}, rcu_int_pll2stbie: {=bool:?}, rcu_int_irc40kstbic: {=bool:?}, rcu_int_lxtalstbic: {=bool:?}, rcu_int_irc8mstbic: {=bool:?}, rcu_int_hxtalstbic: {=bool:?}, rcu_int_pllstbic: {=bool:?}, rcu_int_pll1stbic: {=bool:?}, rcu_int_pll2stbic: {=bool:?}, rcu_int_ckmic: {=bool:?} }}",
                self.rcu_int_irc40kstbif(),
                self.rcu_int_lxtalstbif(),
                self.rcu_int_irc8mstbif(),
                self.rcu_int_hxtalstbif(),
                self.rcu_int_pllstbif(),
                self.rcu_int_pll1stbif(),
                self.rcu_int_pll2stbif(),
                self.rcu_int_ckmif(),
                self.rcu_int_irc40kstbie(),
                self.rcu_int_lxtalstbie(),
                self.rcu_int_irc8mstbie(),
                self.rcu_int_hxtalstbie(),
                self.rcu_int_pllstbie(),
                self.rcu_int_pll1stbie(),
                self.rcu_int_pll2stbie(),
                self.rcu_int_irc40kstbic(),
                self.rcu_int_lxtalstbic(),
                self.rcu_int_irc8mstbic(),
                self.rcu_int_hxtalstbic(),
                self.rcu_int_pllstbic(),
                self.rcu_int_pll1stbic(),
                self.rcu_int_pll2stbic(),
                self.rcu_int_ckmic()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrcurstsck(pub u32);
    impl Rccrcurstsck {
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_irc40ken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_irc40ken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_irc40kstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_irc40kstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_rstfc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_rstfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_eprstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_eprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_swrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_swrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_fwdgtrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_fwdgtrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_wwdgtrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_wwdgtrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn rcu_rstsck_lprstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rcu_rstsck_lprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rccrcurstsck {
        #[inline(always)]
        fn default() -> Rccrcurstsck {
            Rccrcurstsck(0)
        }
    }
    impl core::fmt::Debug for Rccrcurstsck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrcurstsck")
                .field("rcu_rstsck_irc40ken", &self.rcu_rstsck_irc40ken())
                .field("rcu_rstsck_irc40kstb", &self.rcu_rstsck_irc40kstb())
                .field("rcu_rstsck_rstfc", &self.rcu_rstsck_rstfc())
                .field("rcu_rstsck_eprstf", &self.rcu_rstsck_eprstf())
                .field("rcu_rstsck_porrstf", &self.rcu_rstsck_porrstf())
                .field("rcu_rstsck_swrstf", &self.rcu_rstsck_swrstf())
                .field("rcu_rstsck_fwdgtrstf", &self.rcu_rstsck_fwdgtrstf())
                .field("rcu_rstsck_wwdgtrstf", &self.rcu_rstsck_wwdgtrstf())
                .field("rcu_rstsck_lprstf", &self.rcu_rstsck_lprstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrcurstsck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrcurstsck {{ rcu_rstsck_irc40ken: {=bool:?}, rcu_rstsck_irc40kstb: {=bool:?}, rcu_rstsck_rstfc: {=bool:?}, rcu_rstsck_eprstf: {=bool:?}, rcu_rstsck_porrstf: {=bool:?}, rcu_rstsck_swrstf: {=bool:?}, rcu_rstsck_fwdgtrstf: {=bool:?}, rcu_rstsck_wwdgtrstf: {=bool:?}, rcu_rstsck_lprstf: {=bool:?} }}",
                self.rcu_rstsck_irc40ken(),
                self.rcu_rstsck_irc40kstb(),
                self.rcu_rstsck_rstfc(),
                self.rcu_rstsck_eprstf(),
                self.rcu_rstsck_porrstf(),
                self.rcu_rstsck_swrstf(),
                self.rcu_rstsck_fwdgtrstf(),
                self.rcu_rstsck_wwdgtrstf(),
                self.rcu_rstsck_lprstf()
            )
        }
    }
    #[doc = "Reset source /clock register (RCU_RSTSCK)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rccrstsck(pub u32);
    impl Rccrstsck {
        #[doc = "IRC40K enable"]
        #[must_use]
        #[inline(always)]
        pub const fn irc40ken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IRC40K enable"]
        #[inline(always)]
        pub const fn set_irc40ken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IRC40K stabilization"]
        #[must_use]
        #[inline(always)]
        pub const fn irc40kstb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IRC40K stabilization"]
        #[inline(always)]
        pub const fn set_irc40kstb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset flag clear"]
        #[must_use]
        #[inline(always)]
        pub const fn rstfc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset flag clear"]
        #[inline(always)]
        pub const fn set_rstfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "External PIN reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn eprstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "External PIN reset flag"]
        #[inline(always)]
        pub const fn set_eprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Power reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Power reset flag"]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn swrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn set_swrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Free Watchdog timer reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn fwdgtrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Free Watchdog timer reset flag"]
        #[inline(always)]
        pub const fn set_fwdgtrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog timer reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgtrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog timer reset flag"]
        #[inline(always)]
        pub const fn set_wwdgtrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lprstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn set_lprstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rccrstsck {
        #[inline(always)]
        fn default() -> Rccrstsck {
            Rccrstsck(0)
        }
    }
    impl core::fmt::Debug for Rccrstsck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rccrstsck")
                .field("irc40ken", &self.irc40ken())
                .field("irc40kstb", &self.irc40kstb())
                .field("rstfc", &self.rstfc())
                .field("eprstf", &self.eprstf())
                .field("porrstf", &self.porrstf())
                .field("swrstf", &self.swrstf())
                .field("fwdgtrstf", &self.fwdgtrstf())
                .field("wwdgtrstf", &self.wwdgtrstf())
                .field("lprstf", &self.lprstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rccrstsck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rccrstsck {{ irc40ken: {=bool:?}, irc40kstb: {=bool:?}, rstfc: {=bool:?}, eprstf: {=bool:?}, porrstf: {=bool:?}, swrstf: {=bool:?}, fwdgtrstf: {=bool:?}, wwdgtrstf: {=bool:?}, lprstf: {=bool:?} }}",
                self.irc40ken(),
                self.irc40kstb(),
                self.rstfc(),
                self.eprstf(),
                self.porrstf(),
                self.swrstf(),
                self.fwdgtrstf(),
                self.wwdgtrstf(),
                self.lprstf()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcpre {
        #[doc = "PCLK2 divided by 2"]
        Div2 = 0x0,
        #[doc = "PCLK2 divided by 4"]
        Div4 = 0x01,
        #[doc = "PCLK2 divided by 6"]
        Div6 = 0x02,
        #[doc = "PCLK2 divided by 8"]
        Div8 = 0x03,
    }
    impl Adcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcpre {
        #[inline(always)]
        fn from(val: u8) -> Adcpre {
            Adcpre::from_bits(val)
        }
    }
    impl From<Adcpre> for u8 {
        #[inline(always)]
        fn from(val: Adcpre) -> u8 {
            Adcpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "SYSCLK not divided"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYSCLK divided by 2"]
        Div2 = 0x08,
        #[doc = "SYSCLK divided by 4"]
        Div4 = 0x09,
        #[doc = "SYSCLK divided by 8"]
        Div8 = 0x0a,
        #[doc = "SYSCLK divided by 16"]
        Div16 = 0x0b,
        #[doc = "SYSCLK divided by 64"]
        Div64 = 0x0c,
        #[doc = "SYSCLK divided by 128"]
        Div128 = 0x0d,
        #[doc = "SYSCLK divided by 256"]
        Div256 = 0x0e,
        #[doc = "SYSCLK divided by 512"]
        Div512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO"]
        Disable = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "System clock selected"]
        Sys = 0x04,
        #[doc = "HSI oscillator clock selected"]
        Hsi = 0x05,
        #[doc = "HSE oscillator clock selected"]
        Hse = 0x06,
        #[doc = "PLL clock divided by 2 selected"]
        Pll = 0x07,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcosel {
        #[inline(always)]
        fn from(val: u8) -> Mcosel {
            Mcosel::from_bits(val)
        }
    }
    impl From<Mcosel> for u8 {
        #[inline(always)]
        fn from(val: Mcosel) -> u8 {
            Mcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllmul {
        #[doc = "PLL input clock x2"]
        Mul2 = 0x0,
        #[doc = "PLL input clock x3"]
        Mul3 = 0x01,
        #[doc = "PLL input clock x4"]
        Mul4 = 0x02,
        #[doc = "PLL input clock x5"]
        Mul5 = 0x03,
        #[doc = "PLL input clock x6"]
        Mul6 = 0x04,
        #[doc = "PLL input clock x7"]
        Mul7 = 0x05,
        #[doc = "PLL input clock x8"]
        Mul8 = 0x06,
        #[doc = "PLL input clock x9"]
        Mul9 = 0x07,
        #[doc = "PLL input clock x10"]
        Mul10 = 0x08,
        #[doc = "PLL input clock x11"]
        Mul11 = 0x09,
        #[doc = "PLL input clock x12"]
        Mul12 = 0x0a,
        #[doc = "PLL input clock x13"]
        Mul13 = 0x0b,
        #[doc = "PLL input clock x14"]
        Mul14 = 0x0c,
        #[doc = "PLL input clock x15"]
        Mul15 = 0x0d,
        #[doc = "PLL input clock x16"]
        Mul16 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pllmul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmul {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmul {
        #[inline(always)]
        fn from(val: u8) -> Pllmul {
            Pllmul::from_bits(val)
        }
    }
    impl From<Pllmul> for u8 {
        #[inline(always)]
        fn from(val: Pllmul) -> u8 {
            Pllmul::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI divided by 2 selected as PLL input clock"]
        HsiDiv2 = 0x0,
        #[doc = "HSE divided by PREDIV selected as PLL input clock"]
        HseDivPrediv = 0x01,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllxtpre {
        #[doc = "HSE clock not divided"]
        Div1 = 0x0,
        #[doc = "HSE clock divided by 2"]
        Div2 = 0x01,
    }
    impl Pllxtpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllxtpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllxtpre {
        #[inline(always)]
        fn from(val: u8) -> Pllxtpre {
            Pllxtpre::from_bits(val)
        }
    }
    impl From<Pllxtpre> for u8 {
        #[inline(always)]
        fn from(val: Pllxtpre) -> u8 {
            Pllxtpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "HCLK not divided"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        Div2 = 0x04,
        #[doc = "HCLK divided by 4"]
        Div4 = 0x05,
        #[doc = "HCLK divided by 8"]
        Div8 = 0x06,
        #[doc = "HCLK divided by 16"]
        Div16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "No clock"]
        Disable = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        Lse = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        Lsi = 0x02,
        #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
        Hse = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "HSI selected as system clock"]
        Hsi = 0x0,
        #[doc = "HSE selected as system clock"]
        Hse = 0x01,
        #[doc = "PLL selected as system clock"]
        Pll1P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbpre {
        #[doc = "PLL clock is divided by 1.5"]
        Div15 = 0x0,
        #[doc = "PLL clock is not divided"]
        Div1 = 0x01,
    }
    impl Usbpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbpre {
        #[inline(always)]
        fn from(val: u8) -> Usbpre {
            Usbpre::from_bits(val)
        }
    }
    impl From<Usbpre> for u8 {
        #[inline(always)]
        fn from(val: Usbpre) -> u8 {
            Usbpre::to_bits(val)
        }
    }
}
