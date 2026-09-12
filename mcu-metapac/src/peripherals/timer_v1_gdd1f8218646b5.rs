#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim1ch {}
unsafe impl Sync for Tim1ch {}
impl Tim1ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr1ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer1ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1chCmp {
    ptr: *mut u8,
}
unsafe impl Send for Tim1chCmp {}
unsafe impl Sync for Tim1chCmp {}
impl Tim1chCmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr21chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr1chCmp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::DmarGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2ch {
    ptr: *mut u8,
}
unsafe impl Send for Tim2ch {}
unsafe impl Sync for Tim2ch {}
impl Tim2ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr22ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr2ch, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput2ch, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2chCmp {
    ptr: *mut u8,
}
unsafe impl Send for Tim2chCmp {}
unsafe impl Sync for Tim2chCmp {}
impl Tim2chCmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr11ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr22chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::Dier2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::Egr2chCmp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self) -> crate::common::Reg<regs::CcmrOutput2ch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::Ccer2chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-2)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::DmarGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimAdv {
    ptr: *mut u8,
}
unsafe impl Send for TimAdv {}
unsafe impl Sync for TimAdv {}
impl TimAdv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrAdv, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::RcrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::BdtrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasic {
    ptr: *mut u8,
}
unsafe impl Send for TimBasic {}
unsafe impl Sync for TimBasic {}
impl TimBasic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Basic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasicNoCr2 {
    ptr: *mut u8,
}
unsafe impl Send for TimBasicNoCr2 {}
unsafe impl Sync for TimBasicNoCr2 {}
impl TimBasicNoCr2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasicNoCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimCore {
    ptr: *mut u8,
}
unsafe impl Send for TimCore {}
unsafe impl Sync for TimCore {}
impl TimCore {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Core, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrCore, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp16 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp16 {}
unsafe impl Sync for TimGp16 {}
impl TimGp16 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::SmcrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput2ch, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1-2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutputGp16, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::CntCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> PscRegister { PscRegister(unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }) }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::ArrCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "capture/compare register x (x=1-4)"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr1ch, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr1chCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::DmarGp16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "auto-reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ArrCore(pub u32);
    impl ArrCore {
        #[doc = "Auto-reload value"]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ArrCore {
        #[inline(always)]
        fn default() -> ArrCore {
            ArrCore(0)
        }
    }
    impl core::fmt::Debug for ArrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ArrCore").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ArrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ArrCore {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "break and dead-time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr1chCmp(pub u32);
    impl Bdtr1chCmp {
        #[doc = "Dead-time generator setup"]
        #[must_use]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub const fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::Lock {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lock::from_bits(val as u8)
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: super::vals::Lock) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ossi(&self) -> super::vals::Ossi {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ossi::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub const fn set_ossi(&mut self, val: super::vals::Ossi) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ossr(&self) -> super::vals::Ossr {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ossr::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub const fn set_ossr(&mut self, val: super::vals::Ossr) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Break x (x=1) enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bke(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) enable"]
        #[inline(always)]
        pub const fn set_bke(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1) polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self, n: usize) -> super::vals::Bkp {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkp::from_bits(val as u8)
        }
        #[doc = "Break x (x=1) polarity"]
        #[inline(always)]
        pub const fn set_bkp(&mut self, n: usize, val: super::vals::Bkp) {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Automatic output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub const fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Bdtr1chCmp {
        #[inline(always)]
        fn default() -> Bdtr1chCmp {
            Bdtr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Bdtr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdtr1chCmp")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke[0]", &self.bke(0usize))
                .field("bkp[0]", &self.bkp(0usize))
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdtr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdtr1chCmp {{ dtg: {=u8:?}, lock: {:?}, ossi: {:?}, ossr: {:?}, bke[0]: {=bool:?}, bkp[0]: {:?}, aoe: {=bool:?}, moe: {=bool:?} }}",
                self.dtg(),
                self.lock(),
                self.ossi(),
                self.ossr(),
                self.bke(0usize),
                self.bkp(0usize),
                self.aoe(),
                self.moe()
            )
        }
    }
    #[doc = "break and dead-time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BdtrAdv(pub u32);
    impl BdtrAdv {
        #[doc = "Dead-time generator setup"]
        #[must_use]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub const fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::Lock {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lock::from_bits(val as u8)
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: super::vals::Lock) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ossi(&self) -> super::vals::Ossi {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ossi::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub const fn set_ossi(&mut self, val: super::vals::Ossi) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ossr(&self) -> super::vals::Ossr {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ossr::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub const fn set_ossr(&mut self, val: super::vals::Ossr) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Break x (x=1,2) enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bke(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1,2) enable"]
        #[inline(always)]
        pub const fn set_bke(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 12usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Break x (x=1,2) polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn bkp(&self, n: usize) -> super::vals::Bkp {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Bkp::from_bits(val as u8)
        }
        #[doc = "Break x (x=1,2) polarity"]
        #[inline(always)]
        pub const fn set_bkp(&mut self, n: usize, val: super::vals::Bkp) {
            assert!(n < 1usize);
            let offs = 13usize + n * 12usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Automatic output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub const fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub const fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for BdtrAdv {
        #[inline(always)]
        fn default() -> BdtrAdv {
            BdtrAdv(0)
        }
    }
    impl core::fmt::Debug for BdtrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BdtrAdv")
                .field("dtg", &self.dtg())
                .field("lock", &self.lock())
                .field("ossi", &self.ossi())
                .field("ossr", &self.ossr())
                .field("bke[0]", &self.bke(0usize))
                .field("bkp[0]", &self.bkp(0usize))
                .field("aoe", &self.aoe())
                .field("moe", &self.moe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BdtrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BdtrAdv {{ dtg: {=u8:?}, lock: {:?}, ossi: {:?}, ossr: {:?}, bke[0]: {=bool:?}, bkp[0]: {:?}, aoe: {=bool:?}, moe: {=bool:?} }}",
                self.dtg(),
                self.lock(),
                self.ossi(),
                self.ossr(),
                self.bke(0usize),
                self.bkp(0usize),
                self.aoe(),
                self.moe()
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer1ch(pub u32);
    impl Ccer1ch {
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer1ch {
        #[inline(always)]
        fn default() -> Ccer1ch {
            Ccer1ch(0)
        }
    }
    impl core::fmt::Debug for Ccer1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer1ch")
                .field("cce[0]", &self.cce(0usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer1ch {{ cce[0]: {=bool:?}, ccp[0]: {=bool:?}, ccnp[0]: {=bool:?} }}",
                self.cce(0usize),
                self.ccp(0usize),
                self.ccnp(0usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer1chCmp(pub u32);
    impl Ccer1chCmp {
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub const fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer1chCmp {
        #[inline(always)]
        fn default() -> Ccer1chCmp {
            Ccer1chCmp(0)
        }
    }
    impl core::fmt::Debug for Ccer1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer1chCmp")
                .field("cce[0]", &self.cce(0usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer1chCmp {{ cce[0]: {=bool:?}, ccp[0]: {=bool:?}, ccne[0]: {=bool:?}, ccnp[0]: {=bool:?} }}",
                self.cce(0usize),
                self.ccp(0usize),
                self.ccne(0usize),
                self.ccnp(0usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer2ch(pub u32);
    impl Ccer2ch {
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer2ch {
        #[inline(always)]
        fn default() -> Ccer2ch {
            Ccer2ch(0)
        }
    }
    impl core::fmt::Debug for Ccer2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer2ch")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer2ch {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?} }}",
                self.cce(0usize),
                self.cce(1usize),
                self.ccp(0usize),
                self.ccp(1usize),
                self.ccnp(0usize),
                self.ccnp(1usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccer2chCmp(pub u32);
    impl Ccer2chCmp {
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) complementary output enable"]
        #[inline(always)]
        pub const fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ccer2chCmp {
        #[inline(always)]
        fn default() -> Ccer2chCmp {
            Ccer2chCmp(0)
        }
    }
    impl core::fmt::Debug for Ccer2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccer2chCmp")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccer2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccer2chCmp {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccne[0]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?} }}",
                self.cce(0usize),
                self.cce(1usize),
                self.ccp(0usize),
                self.ccp(1usize),
                self.ccne(0usize),
                self.ccnp(0usize),
                self.ccnp(1usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerAdv(pub u32);
    impl CcerAdv {
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-6) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-6) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-3) complementary output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-3) complementary output enable"]
        #[inline(always)]
        pub const fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerAdv {
        #[inline(always)]
        fn default() -> CcerAdv {
            CcerAdv(0)
        }
    }
    impl core::fmt::Debug for CcerAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcerAdv")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("cce[2]", &self.cce(2usize))
                .field("cce[3]", &self.cce(3usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccp[2]", &self.ccp(2usize))
                .field("ccp[3]", &self.ccp(3usize))
                .field("ccne[0]", &self.ccne(0usize))
                .field("ccne[1]", &self.ccne(1usize))
                .field("ccne[2]", &self.ccne(2usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .field("ccnp[2]", &self.ccnp(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcerAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcerAdv {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, cce[2]: {=bool:?}, cce[3]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccp[2]: {=bool:?}, ccp[3]: {=bool:?}, ccne[0]: {=bool:?}, ccne[1]: {=bool:?}, ccne[2]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?}, ccnp[2]: {=bool:?} }}",
                self.cce(0usize),
                self.cce(1usize),
                self.cce(2usize),
                self.cce(3usize),
                self.ccp(0usize),
                self.ccp(1usize),
                self.ccp(2usize),
                self.ccp(3usize),
                self.ccne(0usize),
                self.ccne(1usize),
                self.ccne(2usize),
                self.ccnp(0usize),
                self.ccnp(1usize),
                self.ccnp(2usize)
            )
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerGp16(pub u32);
    impl CcerGp16 {
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output enable"]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) output Polarity"]
        #[inline(always)]
        pub const fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerGp16 {
        #[inline(always)]
        fn default() -> CcerGp16 {
            CcerGp16(0)
        }
    }
    impl core::fmt::Debug for CcerGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcerGp16")
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("cce[2]", &self.cce(2usize))
                .field("cce[3]", &self.cce(3usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("ccp[2]", &self.ccp(2usize))
                .field("ccp[3]", &self.ccp(3usize))
                .field("ccnp[0]", &self.ccnp(0usize))
                .field("ccnp[1]", &self.ccnp(1usize))
                .field("ccnp[2]", &self.ccnp(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcerGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcerGp16 {{ cce[0]: {=bool:?}, cce[1]: {=bool:?}, cce[2]: {=bool:?}, cce[3]: {=bool:?}, ccp[0]: {=bool:?}, ccp[1]: {=bool:?}, ccp[2]: {=bool:?}, ccp[3]: {=bool:?}, ccnp[0]: {=bool:?}, ccnp[1]: {=bool:?}, ccnp[2]: {=bool:?} }}",
                self.cce(0usize),
                self.cce(1usize),
                self.cce(2usize),
                self.cce(3usize),
                self.ccp(0usize),
                self.ccp(1usize),
                self.ccp(2usize),
                self.ccp(3usize),
                self.ccnp(0usize),
                self.ccnp(1usize),
                self.ccnp(2usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput1ch(pub u32);
    impl CcmrInput1ch {
        #[doc = "Capture/Compare y selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[must_use]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput1ch {
        #[inline(always)]
        fn default() -> CcmrInput1ch {
            CcmrInput1ch(0)
        }
    }
    impl core::fmt::Debug for CcmrInput1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrInput1ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("icpsc[0]", &self.icpsc(0usize))
                .field("icf[0]", &self.icf(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrInput1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrInput1ch {{ ccs[0]: {:?}, icpsc[0]: {=u8:?}, icf[0]: {:?} }}",
                self.ccs(0usize),
                self.icpsc(0usize),
                self.icf(0usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput2ch(pub u32);
    impl CcmrInput2ch {
        #[doc = "Capture/Compare y selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture y prescaler"]
        #[inline(always)]
        pub const fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture y filter"]
        #[must_use]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::FilterValue {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "Input capture y filter"]
        #[inline(always)]
        pub const fn set_icf(&mut self, n: usize, val: super::vals::FilterValue) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput2ch {
        #[inline(always)]
        fn default() -> CcmrInput2ch {
            CcmrInput2ch(0)
        }
    }
    impl core::fmt::Debug for CcmrInput2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrInput2ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("icpsc[0]", &self.icpsc(0usize))
                .field("icpsc[1]", &self.icpsc(1usize))
                .field("icf[0]", &self.icf(0usize))
                .field("icf[1]", &self.icf(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrInput2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrInput2ch {{ ccs[0]: {:?}, ccs[1]: {:?}, icpsc[0]: {=u8:?}, icpsc[1]: {=u8:?}, icf[0]: {:?}, icf[1]: {:?} }}",
                self.ccs(0usize),
                self.ccs(1usize),
                self.icpsc(0usize),
                self.icpsc(1usize),
                self.icf(0usize),
                self.icf(1usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput1ch(pub u32);
    impl CcmrOutput1ch {
        #[doc = "Capture/Compare y selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 1usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 1usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for CcmrOutput1ch {
        #[inline(always)]
        fn default() -> CcmrOutput1ch {
            CcmrOutput1ch(0)
        }
    }
    impl core::fmt::Debug for CcmrOutput1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutput1ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocm[0]", &self.ocm(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutput1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrOutput1ch {{ ccs[0]: {:?}, ocfe[0]: {=bool:?}, ocpe[0]: {=bool:?}, ocm[0]: {:?} }}",
                self.ccs(0usize),
                self.ocfe(0usize),
                self.ocpe(0usize),
                self.ocm(0usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput2ch(pub u32);
    impl CcmrOutput2ch {
        #[doc = "Capture/Compare y selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for CcmrOutput2ch {
        #[inline(always)]
        fn default() -> CcmrOutput2ch {
            CcmrOutput2ch(0)
        }
    }
    impl core::fmt::Debug for CcmrOutput2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutput2ch")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocfe[1]", &self.ocfe(1usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocpe[1]", &self.ocpe(1usize))
                .field("ocm[0]", &self.ocm(0usize))
                .field("ocm[1]", &self.ocm(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutput2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrOutput2ch {{ ccs[0]: {:?}, ccs[1]: {:?}, ocfe[0]: {=bool:?}, ocfe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {:?}, ocm[1]: {:?} }}",
                self.ccs(0usize),
                self.ccs(1usize),
                self.ocfe(0usize),
                self.ocfe(1usize),
                self.ocpe(0usize),
                self.ocpe(1usize),
                self.ocm(0usize),
                self.ocm(1usize)
            )
        }
    }
    #[doc = "capture/compare mode register x (x=1-2) (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutputGp16(pub u32);
    impl CcmrOutputGp16 {
        #[doc = "Capture/Compare y selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare y selection"]
        #[inline(always)]
        pub const fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare y fast enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y fast enable"]
        #[inline(always)]
        pub const fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y preload enable"]
        #[inline(always)]
        pub const fn set_ocpe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare y mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare y mode"]
        #[inline(always)]
        pub const fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Output compare y clear enable"]
        #[must_use]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare y clear enable"]
        #[inline(always)]
        pub const fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutputGp16 {
        #[inline(always)]
        fn default() -> CcmrOutputGp16 {
            CcmrOutputGp16(0)
        }
    }
    impl core::fmt::Debug for CcmrOutputGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CcmrOutputGp16")
                .field("ccs[0]", &self.ccs(0usize))
                .field("ccs[1]", &self.ccs(1usize))
                .field("ocfe[0]", &self.ocfe(0usize))
                .field("ocfe[1]", &self.ocfe(1usize))
                .field("ocpe[0]", &self.ocpe(0usize))
                .field("ocpe[1]", &self.ocpe(1usize))
                .field("ocm[0]", &self.ocm(0usize))
                .field("ocm[1]", &self.ocm(1usize))
                .field("occe[0]", &self.occe(0usize))
                .field("occe[1]", &self.occe(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CcmrOutputGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CcmrOutputGp16 {{ ccs[0]: {:?}, ccs[1]: {:?}, ocfe[0]: {=bool:?}, ocfe[1]: {=bool:?}, ocpe[0]: {=bool:?}, ocpe[1]: {=bool:?}, ocm[0]: {:?}, ocm[1]: {:?}, occe[0]: {=bool:?}, occe[1]: {=bool:?} }}",
                self.ccs(0usize),
                self.ccs(1usize),
                self.ocfe(0usize),
                self.ocfe(1usize),
                self.ocpe(0usize),
                self.ocpe(1usize),
                self.ocm(0usize),
                self.ocm(1usize),
                self.occe(0usize),
                self.occe(1usize)
            )
        }
    }
    #[doc = "capture/compare register x (x=1-4,6)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr1ch(pub u32);
    impl Ccr1ch {
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "capture/compare x (x=1-4,6) value"]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr1ch {
        #[inline(always)]
        fn default() -> Ccr1ch {
            Ccr1ch(0)
        }
    }
    impl core::fmt::Debug for Ccr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr1ch").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr1ch {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntCore(pub u32);
    impl CntCore {
        #[doc = "counter value"]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CntCore {
        #[inline(always)]
        fn default() -> CntCore {
            CntCore(0)
        }
    }
    impl core::fmt::Debug for CntCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CntCore").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CntCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CntCore {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr11ch(pub u32);
    impl Cr11ch {
        #[doc = "Counter enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Cr11ch {
        #[inline(always)]
        fn default() -> Cr11ch {
            Cr11ch(0)
        }
    }
    impl core::fmt::Debug for Cr11ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr11ch")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr11ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr11ch {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, arpe: {=bool:?}, ckd: {:?} }}",
                self.cen(),
                self.udis(),
                self.urs(),
                self.opm(),
                self.arpe(),
                self.ckd()
            )
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Core(pub u32);
    impl Cr1Core {
        #[doc = "Counter enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr1Core {
        #[inline(always)]
        fn default() -> Cr1Core {
            Cr1Core(0)
        }
    }
    impl core::fmt::Debug for Cr1Core {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1Core")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("arpe", &self.arpe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1Core {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1Core {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, arpe: {=bool:?} }}",
                self.cen(),
                self.udis(),
                self.urs(),
                self.opm(),
                self.arpe()
            )
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Gp16(pub u32);
    impl Cr1Gp16 {
        #[doc = "Counter enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[must_use]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[must_use]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode enbaled"]
        #[must_use]
        #[inline(always)]
        pub const fn opm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "One-pulse mode enbaled"]
        #[inline(always)]
        pub const fn set_opm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Direction"]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn cms(&self) -> super::vals::Cms {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cms::from_bits(val as u8)
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub const fn set_cms(&mut self, val: super::vals::Cms) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[must_use]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[must_use]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Cr1Gp16 {
        #[inline(always)]
        fn default() -> Cr1Gp16 {
            Cr1Gp16(0)
        }
    }
    impl core::fmt::Debug for Cr1Gp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1Gp16")
                .field("cen", &self.cen())
                .field("udis", &self.udis())
                .field("urs", &self.urs())
                .field("opm", &self.opm())
                .field("dir", &self.dir())
                .field("cms", &self.cms())
                .field("arpe", &self.arpe())
                .field("ckd", &self.ckd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1Gp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1Gp16 {{ cen: {=bool:?}, udis: {=bool:?}, urs: {:?}, opm: {=bool:?}, dir: {:?}, cms: {:?}, arpe: {=bool:?}, ckd: {:?} }}",
                self.cen(),
                self.udis(),
                self.urs(),
                self.opm(),
                self.dir(),
                self.cms(),
                self.arpe(),
                self.ckd()
            )
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr21chCmp(pub u32);
    impl Cr21chCmp {
        #[doc = "Capture/compare preloaded control"]
        #[must_use]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[must_use]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[must_use]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr21chCmp {
        #[inline(always)]
        fn default() -> Cr21chCmp {
            Cr21chCmp(0)
        }
    }
    impl core::fmt::Debug for Cr21chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr21chCmp")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("ois[0]", &self.ois(0usize))
                .field("oisn[0]", &self.oisn(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr21chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr21chCmp {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, ois[0]: {=bool:?}, oisn[0]: {=bool:?} }}",
                self.ccpc(),
                self.ccus(),
                self.ccds(),
                self.ois(0usize),
                self.oisn(0usize)
            )
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr22ch(pub u32);
    impl Cr22ch {
        #[doc = "Master mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr22ch {
        #[inline(always)]
        fn default() -> Cr22ch {
            Cr22ch(0)
        }
    }
    impl core::fmt::Debug for Cr22ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr22ch")
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr22ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr22ch {{ mms: {:?}, ti1s: {:?} }}", self.mms(), self.ti1s())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr22chCmp(pub u32);
    impl Cr22chCmp {
        #[doc = "Capture/compare preloaded control"]
        #[must_use]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state x (x=1,2)"]
        #[must_use]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1,2)"]
        #[inline(always)]
        pub const fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x (x=1)"]
        #[must_use]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1)"]
        #[inline(always)]
        pub const fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr22chCmp {
        #[inline(always)]
        fn default() -> Cr22chCmp {
            Cr22chCmp(0)
        }
    }
    impl core::fmt::Debug for Cr22chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr22chCmp")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois[0]", &self.ois(0usize))
                .field("ois[1]", &self.ois(1usize))
                .field("oisn[0]", &self.oisn(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr22chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr22chCmp {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, mms: {:?}, ti1s: {:?}, ois[0]: {=bool:?}, ois[1]: {=bool:?}, oisn[0]: {=bool:?} }}",
                self.ccpc(),
                self.ccus(),
                self.ccds(),
                self.mms(),
                self.ti1s(),
                self.ois(0usize),
                self.ois(1usize),
                self.oisn(0usize)
            )
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Adv(pub u32);
    impl Cr2Adv {
        #[doc = "Capture/compare preloaded control"]
        #[must_use]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state x (x=1-6)"]
        #[must_use]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x (x=1-6)"]
        #[inline(always)]
        pub const fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state x N x (x=1-4)"]
        #[must_use]
        #[inline(always)]
        pub const fn oisn(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 9usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state x N x (x=1-4)"]
        #[inline(always)]
        pub const fn set_oisn(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 9usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr2Adv {
        #[inline(always)]
        fn default() -> Cr2Adv {
            Cr2Adv(0)
        }
    }
    impl core::fmt::Debug for Cr2Adv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Adv")
                .field("ccpc", &self.ccpc())
                .field("ccus", &self.ccus())
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .field("ois[0]", &self.ois(0usize))
                .field("ois[1]", &self.ois(1usize))
                .field("ois[2]", &self.ois(2usize))
                .field("ois[3]", &self.ois(3usize))
                .field("oisn[0]", &self.oisn(0usize))
                .field("oisn[1]", &self.oisn(1usize))
                .field("oisn[2]", &self.oisn(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Adv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2Adv {{ ccpc: {=bool:?}, ccus: {=bool:?}, ccds: {:?}, mms: {:?}, ti1s: {:?}, ois[0]: {=bool:?}, ois[1]: {=bool:?}, ois[2]: {=bool:?}, ois[3]: {=bool:?}, oisn[0]: {=bool:?}, oisn[1]: {=bool:?}, oisn[2]: {=bool:?} }}",
                self.ccpc(),
                self.ccus(),
                self.ccds(),
                self.mms(),
                self.ti1s(),
                self.ois(0usize),
                self.ois(1usize),
                self.ois(2usize),
                self.ois(3usize),
                self.oisn(0usize),
                self.oisn(1usize),
                self.oisn(2usize)
            )
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Basic(pub u32);
    impl Cr2Basic {
        #[doc = "Master mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Cr2Basic {
        #[inline(always)]
        fn default() -> Cr2Basic {
            Cr2Basic(0)
        }
    }
    impl core::fmt::Debug for Cr2Basic {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Basic").field("mms", &self.mms()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Basic {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr2Basic {{ mms: {:?} }}", self.mms())
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Gp16(pub u32);
    impl Cr2Gp16 {
        #[doc = "Capture/compare DMA selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Ti1s {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Ti1s::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn set_ti1s(&mut self, val: super::vals::Ti1s) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr2Gp16 {
        #[inline(always)]
        fn default() -> Cr2Gp16 {
            Cr2Gp16(0)
        }
    }
    impl core::fmt::Debug for Cr2Gp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Gp16")
                .field("ccds", &self.ccds())
                .field("mms", &self.mms())
                .field("ti1s", &self.ti1s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Gp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2Gp16 {{ ccds: {:?}, mms: {:?}, ti1s: {:?} }}",
                self.ccds(),
                self.mms(),
                self.ti1s()
            )
        }
    }
    #[doc = "DMA control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr1chCmp(pub u32);
    impl Dcr1chCmp {
        #[doc = "DMA base address"]
        #[must_use]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address"]
        #[inline(always)]
        pub const fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length"]
        #[must_use]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub const fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Dcr1chCmp {
        #[inline(always)]
        fn default() -> Dcr1chCmp {
            Dcr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Dcr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr1chCmp")
                .field("dba", &self.dba())
                .field("dbl", &self.dbl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcr1chCmp {{ dba: {=u8:?}, dbl: {=u8:?} }}", self.dba(), self.dbl())
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier1ch(pub u32);
    impl Dier1ch {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier1ch {
        #[inline(always)]
        fn default() -> Dier1ch {
            Dier1ch(0)
        }
    }
    impl core::fmt::Debug for Dier1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier1ch")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier1ch {{ uie: {=bool:?}, ccie[0]: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize)
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier1chCmp(pub u32);
    impl Dier1chCmp {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub const fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Dier1chCmp {
        #[inline(always)]
        fn default() -> Dier1chCmp {
            Dier1chCmp(0)
        }
    }
    impl core::fmt::Debug for Dier1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier1chCmp")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("comie", &self.comie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier1chCmp {{ uie: {=bool:?}, ccie[0]: {=bool:?}, comie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.comie(),
                self.bie(),
                self.ude(),
                self.ccde(0usize)
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier2ch(pub u32);
    impl Dier2ch {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Dier2ch {
        #[inline(always)]
        fn default() -> Dier2ch {
            Dier2ch(0)
        }
    }
    impl core::fmt::Debug for Dier2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier2ch")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("tie", &self.tie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier2ch {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, tie: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.ccie(1usize),
                self.tie()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dier2chCmp(pub u32);
    impl Dier2chCmp {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) DMA request enable"]
        #[inline(always)]
        pub const fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub const fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Dier2chCmp {
        #[inline(always)]
        fn default() -> Dier2chCmp {
            Dier2chCmp(0)
        }
    }
    impl core::fmt::Debug for Dier2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dier2chCmp")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("comde", &self.comde())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dier2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dier2chCmp {{ uie: {=bool:?}, ccie[0]: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, comde: {=bool:?}, tde: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.comie(),
                self.tie(),
                self.bie(),
                self.ude(),
                self.ccde(0usize),
                self.comde(),
                self.tde()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierAdv(pub u32);
    impl DierAdv {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub const fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub const fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for DierAdv {
        #[inline(always)]
        fn default() -> DierAdv {
            DierAdv(0)
        }
    }
    impl core::fmt::Debug for DierAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierAdv")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("ccie[2]", &self.ccie(2usize))
                .field("ccie[3]", &self.ccie(3usize))
                .field("comie", &self.comie())
                .field("tie", &self.tie())
                .field("bie", &self.bie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("ccde[1]", &self.ccde(1usize))
                .field("ccde[2]", &self.ccde(2usize))
                .field("ccde[3]", &self.ccde(3usize))
                .field("comde", &self.comde())
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierAdv {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, ccie[2]: {=bool:?}, ccie[3]: {=bool:?}, comie: {=bool:?}, tie: {=bool:?}, bie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, ccde[2]: {=bool:?}, ccde[3]: {=bool:?}, comde: {=bool:?}, tde: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.ccie(1usize),
                self.ccie(2usize),
                self.ccie(3usize),
                self.comie(),
                self.tie(),
                self.bie(),
                self.ude(),
                self.ccde(0usize),
                self.ccde(1usize),
                self.ccde(2usize),
                self.ccde(3usize),
                self.comde(),
                self.tde()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierBasicNoCr2(pub u32);
    impl DierBasicNoCr2 {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DierBasicNoCr2 {
        #[inline(always)]
        fn default() -> DierBasicNoCr2 {
            DierBasicNoCr2(0)
        }
    }
    impl core::fmt::Debug for DierBasicNoCr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierBasicNoCr2")
                .field("uie", &self.uie())
                .field("ude", &self.ude())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierBasicNoCr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierBasicNoCr2 {{ uie: {=bool:?}, ude: {=bool:?} }}",
                self.uie(),
                self.ude()
            )
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierCore(pub u32);
    impl DierCore {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DierCore {
        #[inline(always)]
        fn default() -> DierCore {
            DierCore(0)
        }
    }
    impl core::fmt::Debug for DierCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierCore").field("uie", &self.uie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DierCore {{ uie: {=bool:?} }}", self.uie())
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierGp16(pub u32);
    impl DierGp16 {
        #[doc = "Update interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) interrupt enable"]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) DMA request enable"]
        #[inline(always)]
        pub const fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger DMA request enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for DierGp16 {
        #[inline(always)]
        fn default() -> DierGp16 {
            DierGp16(0)
        }
    }
    impl core::fmt::Debug for DierGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierGp16")
                .field("uie", &self.uie())
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("ccie[2]", &self.ccie(2usize))
                .field("ccie[3]", &self.ccie(3usize))
                .field("tie", &self.tie())
                .field("ude", &self.ude())
                .field("ccde[0]", &self.ccde(0usize))
                .field("ccde[1]", &self.ccde(1usize))
                .field("ccde[2]", &self.ccde(2usize))
                .field("ccde[3]", &self.ccde(3usize))
                .field("tde", &self.tde())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierGp16 {{ uie: {=bool:?}, ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, ccie[2]: {=bool:?}, ccie[3]: {=bool:?}, tie: {=bool:?}, ude: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, ccde[2]: {=bool:?}, ccde[3]: {=bool:?}, tde: {=bool:?} }}",
                self.uie(),
                self.ccie(0usize),
                self.ccie(1usize),
                self.ccie(2usize),
                self.ccie(3usize),
                self.tie(),
                self.ude(),
                self.ccde(0usize),
                self.ccde(1usize),
                self.ccde(2usize),
                self.ccde(3usize),
                self.tde()
            )
        }
    }
    #[doc = "DMA address for full transfer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmarGp16(pub u32);
    impl DmarGp16 {
        #[doc = "DMA register for burst accesses"]
        #[must_use]
        #[inline(always)]
        pub const fn dmab(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA register for burst accesses"]
        #[inline(always)]
        pub const fn set_dmab(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for DmarGp16 {
        #[inline(always)]
        fn default() -> DmarGp16 {
            DmarGp16(0)
        }
    }
    impl core::fmt::Debug for DmarGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmarGp16").field("dmab", &self.dmab()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DmarGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DmarGp16 {{ dmab: {=u16:?} }}", self.dmab())
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr1ch(pub u32);
    impl Egr1ch {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr1ch {
        #[inline(always)]
        fn default() -> Egr1ch {
            Egr1ch(0)
        }
    }
    impl core::fmt::Debug for Egr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr1ch")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr1ch {{ ug: {=bool:?}, ccg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr1chCmp(pub u32);
    impl Egr1chCmp {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break x (x=1) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub const fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr1chCmp {
        #[inline(always)]
        fn default() -> Egr1chCmp {
            Egr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Egr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr1chCmp")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("comg", &self.comg())
                .field("bg[0]", &self.bg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr1chCmp {{ ug: {=bool:?}, ccg[0]: {=bool:?}, comg: {=bool:?}, bg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.comg(),
                self.bg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr2ch(pub u32);
    impl Egr2ch {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Egr2ch {
        #[inline(always)]
        fn default() -> Egr2ch {
            Egr2ch(0)
        }
    }
    impl core::fmt::Debug for Egr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr2ch")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr2ch {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, tg: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.ccg(1usize),
                self.tg()
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egr2chCmp(pub u32);
    impl Egr2chCmp {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1,2) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1,2) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) generation"]
        #[inline(always)]
        pub const fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Egr2chCmp {
        #[inline(always)]
        fn default() -> Egr2chCmp {
            Egr2chCmp(0)
        }
    }
    impl core::fmt::Debug for Egr2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egr2chCmp")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg[0]", &self.bg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egr2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Egr2chCmp {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.ccg(1usize),
                self.comg(),
                self.tg(),
                self.bg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrAdv(pub u32);
    impl EgrAdv {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1-2) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn bg(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1-2) generation"]
        #[inline(always)]
        pub const fn set_bg(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for EgrAdv {
        #[inline(always)]
        fn default() -> EgrAdv {
            EgrAdv(0)
        }
    }
    impl core::fmt::Debug for EgrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrAdv")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("ccg[2]", &self.ccg(2usize))
                .field("ccg[3]", &self.ccg(3usize))
                .field("comg", &self.comg())
                .field("tg", &self.tg())
                .field("bg[0]", &self.bg(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EgrAdv {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, ccg[2]: {=bool:?}, ccg[3]: {=bool:?}, comg: {=bool:?}, tg: {=bool:?}, bg[0]: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.ccg(1usize),
                self.ccg(2usize),
                self.ccg(3usize),
                self.comg(),
                self.tg(),
                self.bg(0usize)
            )
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrCore(pub u32);
    impl EgrCore {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EgrCore {
        #[inline(always)]
        fn default() -> EgrCore {
            EgrCore(0)
        }
    }
    impl core::fmt::Debug for EgrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrCore").field("ug", &self.ug()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EgrCore {{ ug: {=bool:?} }}", self.ug())
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrGp16(pub u32);
    impl EgrGp16 {
        #[doc = "Update generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) generation"]
        #[inline(always)]
        pub const fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger generation"]
        #[must_use]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for EgrGp16 {
        #[inline(always)]
        fn default() -> EgrGp16 {
            EgrGp16(0)
        }
    }
    impl core::fmt::Debug for EgrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EgrGp16")
                .field("ug", &self.ug())
                .field("ccg[0]", &self.ccg(0usize))
                .field("ccg[1]", &self.ccg(1usize))
                .field("ccg[2]", &self.ccg(2usize))
                .field("ccg[3]", &self.ccg(3usize))
                .field("tg", &self.tg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EgrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EgrGp16 {{ ug: {=bool:?}, ccg[0]: {=bool:?}, ccg[1]: {=bool:?}, ccg[2]: {=bool:?}, ccg[3]: {=bool:?}, tg: {=bool:?} }}",
                self.ug(),
                self.ccg(0usize),
                self.ccg(1usize),
                self.ccg(2usize),
                self.ccg(3usize),
                self.tg()
            )
        }
    }
    #[doc = "repetition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr1chCmp(pub u32);
    impl Rcr1chCmp {
        #[doc = "Repetition counter value"]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u16) {
assert!(val <= u8::MAX as u16, "重复计数超过真实 8 位范围");
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcr1chCmp {
        #[inline(always)]
        fn default() -> Rcr1chCmp {
            Rcr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Rcr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr1chCmp").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rcr1chCmp {{ rep: {=u8:?} }}", self.rep())
        }
    }
    #[doc = "repetition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcrAdv(pub u32);
    impl RcrAdv {
        #[doc = "Repetition counter value"]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u16) {
assert!(val <= u8::MAX as u16, "重复计数超过真实 8 位范围");
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for RcrAdv {
        #[inline(always)]
        fn default() -> RcrAdv {
            RcrAdv(0)
        }
    }
    impl core::fmt::Debug for RcrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcrAdv").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RcrAdv {{ rep: {=u8:?} }}", self.rep())
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr2ch(pub u32);
    impl Smcr2ch {
        #[doc = "Slave mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode"]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Smcr2ch {
        #[inline(always)]
        fn default() -> Smcr2ch {
            Smcr2ch(0)
        }
    }
    impl core::fmt::Debug for Smcr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr2ch")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smcr2ch {{ sms: {:?}, ts: {:?}, msm: {:?} }}",
                self.sms(),
                self.ts(),
                self.msm()
            )
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmcrGp16(pub u32);
    impl SmcrGp16 {
        #[doc = "Slave mode selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode"]
        #[must_use]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[must_use]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::FilterValue {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::FilterValue::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn set_etf(&mut self, val: super::vals::FilterValue) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock mode 2 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ece(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "External clock mode 2 enable"]
        #[inline(always)]
        pub const fn set_ece(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SmcrGp16 {
        #[inline(always)]
        fn default() -> SmcrGp16 {
            SmcrGp16(0)
        }
    }
    impl core::fmt::Debug for SmcrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SmcrGp16")
                .field("sms", &self.sms())
                .field("ts", &self.ts())
                .field("msm", &self.msm())
                .field("etf", &self.etf())
                .field("etps", &self.etps())
                .field("ece", &self.ece())
                .field("etp", &self.etp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SmcrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SmcrGp16 {{ sms: {:?}, ts: {:?}, msm: {:?}, etf: {:?}, etps: {:?}, ece: {=bool:?}, etp: {:?} }}",
                self.sms(),
                self.ts(),
                self.msm(),
                self.etf(),
                self.etps(),
                self.ece(),
                self.etp()
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1ch(pub u32);
    impl Sr1ch {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr1ch {
        #[inline(always)]
        fn default() -> Sr1ch {
            Sr1ch(0)
        }
    }
    impl core::fmt::Debug for Sr1ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1ch")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1ch {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccof[0]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccof(0usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1chCmp(pub u32);
    impl Sr1chCmp {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr1chCmp {
        #[inline(always)]
        fn default() -> Sr1chCmp {
            Sr1chCmp(0)
        }
    }
    impl core::fmt::Debug for Sr1chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1chCmp")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("comif", &self.comif())
                .field("bif[0]", &self.bif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1chCmp {{ uif: {=bool:?}, ccif[0]: {=bool:?}, comif: {=bool:?}, bif[0]: {=bool:?}, ccof[0]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.comif(),
                self.bif(0usize),
                self.ccof(0usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2ch(pub u32);
    impl Sr2ch {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-2) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-2) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr2ch {
        #[inline(always)]
        fn default() -> Sr2ch {
            Sr2ch(0)
        }
    }
    impl core::fmt::Debug for Sr2ch {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2ch")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("tif", &self.tif())
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2ch {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr2ch {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, tif: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccif(1usize),
                self.tif(),
                self.ccof(0usize),
                self.ccof(1usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2chCmp(pub u32);
    impl Sr2chCmp {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1) interrupt flag"]
        #[inline(always)]
        pub const fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1,2) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr2chCmp {
        #[inline(always)]
        fn default() -> Sr2chCmp {
            Sr2chCmp(0)
        }
    }
    impl core::fmt::Debug for Sr2chCmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2chCmp")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif[0]", &self.bif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2chCmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr2chCmp {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif[0]: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccif(1usize),
                self.comif(),
                self.tif(),
                self.bif(0usize),
                self.ccof(0usize),
                self.ccof(1usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrAdv(pub u32);
    impl SrAdv {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break x (x=1,2) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn bif(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Break x (x=1,2) interrupt flag"]
        #[inline(always)]
        pub const fn set_bif(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrAdv {
        #[inline(always)]
        fn default() -> SrAdv {
            SrAdv(0)
        }
    }
    impl core::fmt::Debug for SrAdv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrAdv")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("ccif[2]", &self.ccif(2usize))
                .field("ccif[3]", &self.ccif(3usize))
                .field("comif", &self.comif())
                .field("tif", &self.tif())
                .field("bif[0]", &self.bif(0usize))
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .field("ccof[2]", &self.ccof(2usize))
                .field("ccof[3]", &self.ccof(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrAdv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SrAdv {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, ccif[2]: {=bool:?}, ccif[3]: {=bool:?}, comif: {=bool:?}, tif: {=bool:?}, bif[0]: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, ccof[2]: {=bool:?}, ccof[3]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccif(1usize),
                self.ccif(2usize),
                self.ccif(3usize),
                self.comif(),
                self.tif(),
                self.bif(0usize),
                self.ccof(0usize),
                self.ccof(1usize),
                self.ccof(2usize),
                self.ccof(3usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrCore(pub u32);
    impl SrCore {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SrCore {
        #[inline(always)]
        fn default() -> SrCore {
            SrCore(0)
        }
    }
    impl core::fmt::Debug for SrCore {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrCore").field("uif", &self.uif()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrCore {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SrCore {{ uif: {=bool:?} }}", self.uif())
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrGp16(pub u32);
    impl SrGp16 {
        #[doc = "Update interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare x (x=1-4) interrupt flag"]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare x (x=1-4) overcapture flag"]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrGp16 {
        #[inline(always)]
        fn default() -> SrGp16 {
            SrGp16(0)
        }
    }
    impl core::fmt::Debug for SrGp16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrGp16")
                .field("uif", &self.uif())
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("ccif[2]", &self.ccif(2usize))
                .field("ccif[3]", &self.ccif(3usize))
                .field("tif", &self.tif())
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .field("ccof[2]", &self.ccof(2usize))
                .field("ccof[3]", &self.ccof(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrGp16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SrGp16 {{ uif: {=bool:?}, ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, ccif[2]: {=bool:?}, ccif[3]: {=bool:?}, tif: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, ccof[2]: {=bool:?}, ccof[3]: {=bool:?} }}",
                self.uif(),
                self.ccif(0usize),
                self.ccif(1usize),
                self.ccif(2usize),
                self.ccif(3usize),
                self.tif(),
                self.ccof(0usize),
                self.ccof(1usize),
                self.ccof(2usize),
                self.ccof(3usize)
            )
        }
    }
}
pub mod vals {
/// 仅供不可用 API 签名使用，不代表当前芯片的字段编码。
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Bkinp {
        #[doc = "input polarity is not inverted (active low if BKxP = 0, active high if BKxP = 1)"]
        NotInverted = 0x0,
        #[doc = "input polarity is inverted (active high if BKxP = 0, active low if BKxP = 1)"]
        Inverted = 0x01,
    }

/// 仅供不可用 API 签名使用，不代表当前芯片的字段编码。
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Mms2 {
        #[doc = "The UG bit from the TIMx_EGR register is used as TRGO2"]
        Reset = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as TRGO2"]
        Enable = 0x01,
        #[doc = "The update event is selected as TRGO2"]
        Update = 0x02,
        #[doc = "TRGO2 send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        ComparePulse = 0x03,
        #[doc = "OC1REF signal is used as TRGO2"]
        CompareOc1 = 0x04,
        #[doc = "OC2REF signal is used as TRGO2"]
        CompareOc2 = 0x05,
        #[doc = "OC3REF signal is used as TRGO2"]
        CompareOc3 = 0x06,
        #[doc = "OC4REF signal is used as TRGO2"]
        CompareOc4 = 0x07,
        #[doc = "OC5REF signal is used as TRGO2"]
        CompareOc5 = 0x08,
        #[doc = "OC6REF signal is used as TRGO2"]
        CompareOc6 = 0x09,
        #[doc = "OC4REF rising or falling edges generate pulses on TRGO2"]
        ComparePulseOc4 = 0x0a,
        #[doc = "OC6REF rising or falling edges generate pulses on TRGO2"]
        ComparePulseOc6 = 0x0b,
        #[doc = "OC4REF or OC6REF rising edges generate pulses on TRGO2"]
        ComparePulseOc4OrOc6Rising = 0x0c,
        #[doc = "OC4REF rising or OC6REF falling edges generate pulses on TRGO2"]
        ComparePulseOc4RisingOrOc6Falling = 0x0d,
        #[doc = "OC5REF or OC6REF rising edges generate pulses on TRGO2"]
        ComparePulseOc5OrOc6Rising = 0x0e,
        #[doc = "OC5REF rising or OC6REF falling edges generate pulses on TRGO2"]
        ComparePulseOc5RisingOrOc6Falling = 0x0f,
    }

    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bkp {
        #[doc = "Break input tim_brk is active low"]
        ActiveLow = 0x0,
        #[doc = "Break input tim_brk is active high"]
        ActiveHigh = 0x01,
    }
    impl Bkp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bkp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bkp {
        #[inline(always)]
        fn from(val: u8) -> Bkp {
            Bkp::from_bits(val)
        }
    }
    impl From<Bkp> for u8 {
        #[inline(always)]
        fn from(val: Bkp) -> u8 {
            Bkp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        OnCompare = 0x0,
        #[doc = "CCx DMA request sent when update event occurs"]
        OnUpdate = 0x01,
    }
    impl Ccds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccds {
        #[inline(always)]
        fn from(val: u8) -> Ccds {
            Ccds::from_bits(val)
        }
    }
    impl From<Ccds> for u8 {
        #[inline(always)]
        fn from(val: Ccds) -> u8 {
            Ccds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcmrInputCcs {
        _RESERVED_0 = 0x0,
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        Ti4 = 0x01,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        Ti3 = 0x02,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        Trc = 0x03,
    }
    impl CcmrInputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrInputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrInputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrInputCcs {
            CcmrInputCcs::from_bits(val)
        }
    }
    impl From<CcmrInputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrInputCcs) -> u8 {
            CcmrInputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        Output = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcmrOutputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrOutputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrOutputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrOutputCcs {
            CcmrOutputCcs::from_bits(val)
        }
    }
    impl From<CcmrOutputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrOutputCcs) -> u8 {
            CcmrOutputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckd {
        #[doc = "t_DTS = t_CK_INT"]
        Div1 = 0x0,
        #[doc = "t_DTS = 2 × t_CK_INT"]
        Div2 = 0x01,
        #[doc = "t_DTS = 4 × t_CK_INT"]
        Div4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckd {
        #[inline(always)]
        fn from(val: u8) -> Ckd {
            Ckd::from_bits(val)
        }
    }
    impl From<Ckd> for u8 {
        #[inline(always)]
        fn from(val: Ckd) -> u8 {
            Ckd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EdgeAligned = 0x0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CenterAligned1 = 0x01,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CenterAligned2 = 0x02,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CenterAligned3 = 0x03,
    }
    impl Cms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cms {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cms {
        #[inline(always)]
        fn from(val: u8) -> Cms {
            Cms::from_bits(val)
        }
    }
    impl From<Cms> for u8 {
        #[inline(always)]
        fn from(val: Cms) -> u8 {
            Cms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        Up = 0x0,
        #[doc = "Counter used as downcounter"]
        Down = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Etp {
        #[doc = "ETR is noninverted, active at high level or rising edge"]
        NotInverted = 0x0,
        #[doc = "ETR is inverted, active at low level or falling edge"]
        Inverted = 0x01,
    }
    impl Etp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etp {
        #[inline(always)]
        fn from(val: u8) -> Etp {
            Etp::from_bits(val)
        }
    }
    impl From<Etp> for u8 {
        #[inline(always)]
        fn from(val: Etp) -> u8 {
            Etp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Etps {
        #[doc = "Prescaler OFF"]
        Div1 = 0x0,
        #[doc = "ETRP frequency divided by 2"]
        Div2 = 0x01,
        #[doc = "ETRP frequency divided by 4"]
        Div4 = 0x02,
        #[doc = "ETRP frequency divided by 8"]
        Div8 = 0x03,
    }
    impl Etps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etps {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etps {
        #[inline(always)]
        fn from(val: u8) -> Etps {
            Etps::from_bits(val)
        }
    }
    impl From<Etps> for u8 {
        #[inline(always)]
        fn from(val: Etps) -> u8 {
            Etps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FilterValue {
        #[doc = "No filter, sampling is done at fDTS"]
        NoFilter = 0x0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FckIntN2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FckIntN4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FckIntN8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FdtsDiv2N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FdtsDiv2N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FdtsDiv4N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FdtsDiv4N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FdtsDiv8N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FdtsDiv8N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FdtsDiv16N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FdtsDiv16N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FdtsDiv16N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FdtsDiv32N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FdtsDiv32N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FdtsDiv32N8 = 0x0f,
    }
    impl FilterValue {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterValue {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterValue {
        #[inline(always)]
        fn from(val: u8) -> FilterValue {
            FilterValue::from_bits(val)
        }
    }
    impl From<FilterValue> for u8 {
        #[inline(always)]
        fn from(val: FilterValue) -> u8 {
            FilterValue::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lock {
        #[doc = "No bit is write protected"]
        Disabled = 0x0,
        #[doc = "DTG bits in TIMx_BDTR register, OISx and OISxN bits in TIMx_CR2 register and BKBID/BKE/BKP/AOE bits in TIMx_BDTR register can no longer be written"]
        Level1 = 0x01,
        #[doc = "LOCK Level 1 + CC Polarity bits (CCxP/CCxNP bits in TIMx_CCER register, as long as the related channel is configured in output through the CCxS bits) as well as OSSR and OSSI bits can no longer be written."]
        Level2 = 0x02,
        #[doc = "LOCK Level 2 + CC Control bits (OCxM and OCxPE bits in TIMx_CCMRx registers, as long as the related channel is configured in output through the CCxS bits) can no longer be written."]
        Level3 = 0x03,
    }
    impl Lock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lock {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lock {
        #[inline(always)]
        fn from(val: u8) -> Lock {
            Lock::from_bits(val)
        }
    }
    impl From<Lock> for u8 {
        #[inline(always)]
        fn from(val: Lock) -> u8 {
            Lock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mms {
        #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
        Reset = 0x0,
        #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
        Enable = 0x01,
        #[doc = "The update event is selected as trigger output"]
        Update = 0x02,
        #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        ComparePulse = 0x03,
        #[doc = "OC1REF signal is used as trigger output"]
        CompareOc1 = 0x04,
        #[doc = "OC2REF signal is used as trigger output"]
        CompareOc2 = 0x05,
        #[doc = "OC3REF signal is used as trigger output"]
        CompareOc3 = 0x06,
        #[doc = "OC4REF signal is used as trigger output"]
        CompareOc4 = 0x07,
    }
    impl Mms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms {
        #[inline(always)]
        fn from(val: u8) -> Mms {
            Mms::from_bits(val)
        }
    }
    impl From<Mms> for u8 {
        #[inline(always)]
        fn from(val: Mms) -> u8 {
            Mms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msm {
        #[doc = "No action"]
        NoSync = 0x0,
        #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
        Sync = 0x01,
    }
    impl Msm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msm {
        #[inline(always)]
        fn from(val: u8) -> Msm {
            Msm::from_bits(val)
        }
    }
    impl From<Msm> for u8 {
        #[inline(always)]
        fn from(val: Msm) -> u8 {
            Msm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        Frozen = 0x0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ActiveOnMatch = 0x01,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        InactiveOnMatch = 0x02,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        Toggle = 0x03,
        #[doc = "OCyREF is forced low"]
        ForceInactive = 0x04,
        #[doc = "OCyREF is forced high"]
        ForceActive = 0x05,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PwmMode1 = 0x06,
        #[doc = "Inversely to PwmMode1"]
        PwmMode2 = 0x07,
    }
    impl Ocm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocm {
        #[inline(always)]
        fn from(val: u8) -> Ocm {
            Ocm::from_bits(val)
        }
    }
    impl From<Ocm> for u8 {
        #[inline(always)]
        fn from(val: Ocm) -> u8 {
            Ocm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ossi {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        Disabled = 0x0,
        #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
        IdleLevel = 0x01,
    }
    impl Ossi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossi {
        #[inline(always)]
        fn from(val: u8) -> Ossi {
            Ossi::from_bits(val)
        }
    }
    impl From<Ossi> for u8 {
        #[inline(always)]
        fn from(val: Ossi) -> u8 {
            Ossi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ossr {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        Disabled = 0x0,
        #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
        IdleLevel = 0x01,
    }
    impl Ossr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossr {
        #[inline(always)]
        fn from(val: u8) -> Ossr {
            Ossr::from_bits(val)
        }
    }
    impl From<Ossr> for u8 {
        #[inline(always)]
        fn from(val: Ossr) -> u8 {
            Ossr::to_bits(val)
        }
    }
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Sms(Option<u8>);
    impl Sms {
        pub const Disabled: Self = Self(Some(0));
        pub const EncoderMode2: Self = Self(Some(1));
        pub const EncoderMode1: Self = Self(Some(2));
        pub const EncoderMode3: Self = Self(Some(3));
        pub const ResetMode: Self = Self(Some(4));
        pub const GatedMode: Self = Self(Some(5));
        pub const TriggerMode: Self = Self(Some(6));
        pub const ExtClockMode: Self = Self(Some(7));
        /// 仅软件请求标记，没有寄存器编码；尝试转换为硬件值时拒绝。
        pub const COMBINED_RESET_TRIGGER: Self = Self(None);
        #[allow(non_upper_case_globals)]
        pub const CombinedResetTrigger: Self = Self::COMBINED_RESET_TRIGGER;
        pub const fn from_bits(value: u8) -> Self { Self(Some(value & 7)) }
        pub const fn to_bits(self) -> u8 {
            match self.0 {
                Some(value) => value,
                None => panic!("GD32F30x 没有组合复位触发模式，未写入寄存器"),
            }
        }
    }
    impl From<u8> for Sms {
        #[inline(always)]
        fn from(val: u8) -> Sms {
            Sms::from_bits(val)
        }
    }
    impl From<Sms> for u8 {
        #[inline(always)]
        fn from(val: Sms) -> u8 {
            Sms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ti1s {
        #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
        Normal = 0x0,
        #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
        Xor = 0x01,
    }
    impl Ti1s {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ti1s {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ti1s {
        #[inline(always)]
        fn from(val: u8) -> Ti1s {
            Ti1s::from_bits(val)
        }
    }
    impl From<Ti1s> for u8 {
        #[inline(always)]
        fn from(val: Ti1s) -> u8 {
            Ti1s::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ts {
        #[doc = "Internal Trigger 0"]
        Itr0 = 0x0,
        #[doc = "Internal Trigger 1"]
        Itr1 = 0x01,
        #[doc = "Internal Trigger 2"]
        Itr2 = 0x02,
        #[doc = "Internal Trigger 3"]
        Itr3 = 0x03,
        #[doc = "TI1 Edge Detector"]
        Ti1fEd = 0x04,
        #[doc = "Filtered Timer Input 1"]
        Ti1fp1 = 0x05,
        #[doc = "Filtered Timer Input 2"]
        Ti2fp2 = 0x06,
        #[doc = "External Trigger input"]
        Etrf = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Ts {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ts {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ts {
        #[inline(always)]
        fn from(val: u8) -> Ts {
            Ts::from_bits(val)
        }
    }
    impl From<Ts> for u8 {
        #[inline(always)]
        fn from(val: Ts) -> u8 {
            Ts::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        AnyEvent = 0x0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        CounterOnly = 0x01,
    }
    impl Urs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Urs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Urs {
        #[inline(always)]
        fn from(val: u8) -> Urs {
            Urs::from_bits(val)
        }
    }
    impl From<Urs> for u8 {
        #[inline(always)]
        fn from(val: Urs) -> u8 {
            Urs::to_bits(val)
        }
    }
}

/// 16 位预分频值，保留原生 32 位寄存器总线访问。
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PscRegister(crate::common::Reg<u32, crate::common::RW>);
impl PscRegister {
    pub fn write_value(&self, value: u16) { self.0.write_value(u32::from(value)); }
}
impl core::ops::Deref for PscRegister {
    type Target = crate::common::Reg<u32, crate::common::RW>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// 仅 API 访问视图；实际计数能力仍由设备 metadata 的 GP16/ADV 等类型决定。
#[derive(Copy, Clone)]
pub struct TimGp32(TimGp16);
impl TimGp32 {
    /// # 安全性
    /// 调用者必须保证地址指向有效的、采用该模块布局的定时器，并避免并发写入。
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        // 安全性：将相同映射前提交给既有 GP16 视图，不扩大寄存器布局。
        Self(unsafe { TimGp16::from_ptr(ptr) })
    }
    pub const fn as_ptr(&self) -> *mut () { self.0.as_ptr() }
    pub fn cnt(self) -> NarrowTimerRegister {
        // 安全性：原字段是 repr(transparent) u32，地址及总线访问宽度保持不变。
        NarrowTimerRegister(unsafe { crate::common::Reg::from_ptr(self.0.cnt().as_ptr().cast()) })
    }
    pub fn arr(self) -> NarrowTimerRegister {
        // 安全性：沿用真实 ARR 字段的 32 位对齐地址。
        NarrowTimerRegister(unsafe { crate::common::Reg::from_ptr(self.0.arr().as_ptr().cast()) })
    }
    pub fn ccr(self, n: usize) -> NarrowTimerRegister {
        // 安全性：原访问器检查通道索引；字段底层存储为 u32。
        NarrowTimerRegister(unsafe { crate::common::Reg::from_ptr(self.0.ccr(n).as_ptr().cast()) })
    }
}
impl core::ops::Deref for TimGp32 {
    type Target = TimGp16;
    fn deref(&self) -> &Self::Target { &self.0 }
}
#[derive(Copy, Clone)]
pub struct NarrowTimerRegister(crate::common::Reg<u32, crate::common::RW>);
impl NarrowTimerRegister {
    pub fn read(&self) -> u32 { self.0.read() & u16::MAX as u32 }
    pub fn write_value(&self, value: u32) {
        assert!(value <= u16::MAX as u32, "写入值超过真实 16 位计数器范围");
        self.0.write_value(value);
    }
    pub fn modify<R>(&self, f: impl FnOnce(&mut u32) -> R) -> R {
        let mut value = self.read(); let result = f(&mut value); self.write_value(value); result
    }
    pub fn write<R>(&self, f: impl FnOnce(&mut u32) -> R) -> R {
        let mut value = 0; let result = f(&mut value); self.write_value(value); result
    }
}

impl regs::Bdtr1chCmp {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn bkf(&self, n: usize) -> vals::FilterValue {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_Bdtr1chCmp_bkf() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_Bdtr1chCmp_bkf() }
    }
}

impl regs::Bdtr1chCmp {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn set_bkf(&mut self, n: usize, val: vals::FilterValue) {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_Bdtr1chCmp_set_bkf() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_Bdtr1chCmp_set_bkf() }
    }
}

impl regs::BdtrAdv {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn bkf(&self, n: usize) -> vals::FilterValue {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_BdtrAdv_bkf() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_BdtrAdv_bkf() }
    }
}

impl regs::BdtrAdv {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn set_bkf(&mut self, n: usize, val: vals::FilterValue) {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_BdtrAdv_set_bkf() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_BdtrAdv_set_bkf() }
    }
}

pub struct UnavailableTimGp16Af1 { _private: () }
impl TimGp16 {
pub fn af1(self) -> UnavailableTimGp16Af1 { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_af1() } }
}
impl UnavailableTimGp16Af1 {
pub fn read(&self) -> Self { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_af1() } }
pub fn modify<R>(&self, _f: impl FnOnce(&mut Self) -> R) -> R { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_af1() } }
#[allow(unused_variables)]
pub fn etrsel(&self) -> u8 { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_af1() } }
#[allow(unused_variables)]
pub fn set_etrsel(&mut self, val: u8) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_af1() } }
}

pub struct UnavailableTim1chCmpAf1 { _private: () }
impl Tim1chCmp {
pub fn af1(self) -> UnavailableTim1chCmpAf1 { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
}
impl UnavailableTim1chCmpAf1 {
pub fn read(&self) -> Self { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
pub fn modify<R>(&self, _f: impl FnOnce(&mut Self) -> R) -> R { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn bkine(&self) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn set_bkine(&mut self, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn bkcmpe(&self, n: usize) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn set_bkcmpe(&mut self, n: usize, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn bkdf1bke(&self) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn set_bkdf1bke(&mut self, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn bkinp(&self) -> vals::Bkinp { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn set_bkinp(&mut self, val: vals::Bkinp) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn bkcmpp(&self, n: usize) -> vals::Bkinp { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
#[allow(unused_variables)]
pub fn set_bkcmpp(&mut self, n: usize, val: vals::Bkinp) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_Tim1chCmp_af1() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_Tim1chCmp_af1() } }
}

pub struct UnavailableTimAdvAf2 { _private: () }
impl TimAdv {
pub fn af2(self) -> UnavailableTimAdvAf2 { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
}
impl UnavailableTimAdvAf2 {
pub fn read(&self) -> Self { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
pub fn modify<R>(&self, _f: impl FnOnce(&mut Self) -> R) -> R { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn bk2ine(&self) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn set_bk2ine(&mut self, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn bk2cmpe(&self, n: usize) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn set_bk2cmpe(&mut self, n: usize, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn bk2df1bk1e(&self) -> bool { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn set_bk2df1bk1e(&mut self, val: bool) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn bk2inp(&self) -> vals::Bkinp { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn set_bk2inp(&mut self, val: vals::Bkinp) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn bk2cmpp(&self, n: usize) -> vals::Bkinp { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
#[allow(unused_variables)]
pub fn set_bk2cmpp(&mut self, n: usize, val: vals::Bkinp) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimAdv_af2() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimAdv_af2() } }
}

impl regs::Cr2Adv {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn set_mms2(&mut self, val: vals::Mms2) {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_Cr2Adv_set_mms2() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_Cr2Adv_set_mms2() }
    }
}

impl regs::Cr1Core {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn uifremap(&self) -> bool {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_Cr1Core_uifremap() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_Cr1Core_uifremap() }
    }
}

impl regs::Cr1Core {
    /// 当前硬件无此字段，使用该接口将拒绝链接。
    #[allow(unused_variables)]
    pub fn set_uifremap(&mut self, val: bool) {
        unsafe extern "C" { fn embassy_mcu_compat_unsupported_Cr1Core_set_uifremap() -> !; }
        // 安全性：保留符号无定义，正常链接拒绝该调用，不产生硬件访问。
        unsafe { embassy_mcu_compat_unsupported_Cr1Core_set_uifremap() }
    }
}

pub struct UnavailableTimGp16Tisel { _private: () }
impl TimGp16 {
pub fn tisel(self) -> UnavailableTimGp16Tisel { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_tisel() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_tisel() } }
}
impl UnavailableTimGp16Tisel {
pub fn read(&self) -> Self { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_tisel() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_tisel() } }
pub fn modify<R>(&self, _f: impl FnOnce(&mut Self) -> R) -> R { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_tisel() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_tisel() } }
#[allow(unused_variables)]
pub fn tisel(&self, n: usize) -> u8 { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_tisel() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_tisel() } }
#[allow(unused_variables)]
pub fn set_tisel(&mut self, n: usize, val: u8) { unsafe extern "C" { fn embassy_mcu_compat_unsupported_TimGp16_tisel() -> !; }
// 安全性：保留符号没有定义，阻止链接，不执行寄存器读写。
unsafe { embassy_mcu_compat_unsupported_TimGp16_tisel() } }
}
