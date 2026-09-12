#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn exti_inten(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiInten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt enable register (EXTI_INTEN)"]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Interrupt enable register (EXTI_INTEN)"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdInten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event enable register (EXTI_EVEN)"]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "Event enable register (EXTI_EVEN)"]
    #[inline(always)]
    pub const fn even(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdEven, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn exti_even(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiEven, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn exti_rten(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiRten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
    #[inline(always)]
    pub const fn rten(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdRten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
    #[inline(always)]
    pub const fn rtsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn exti_ften(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiFten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
    #[inline(always)]
    pub const fn ften(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdFten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
    #[inline(always)]
    pub const fn ftsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn exti_swiev(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiSwiev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEV)"]
    #[inline(always)]
    pub const fn swier(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 32usize) as _) }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEV)"]
    #[inline(always)]
    pub const fn swiev(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdSwiev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn exti_pd(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfFwExtiPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pending register (EXTI_PD)"]
    #[inline(always)]
    pub const fn pd(self) -> crate::common::Reg<regs::GdMergedEcb613bbf6cfSvdPd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pending register (EXTI_PD)"]
    #[inline(always)]
    pub const fn pr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 32usize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiEven(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiEven {
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_even_even19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_even_even19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiEven {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiEven {
            GdMergedEcb613bbf6cfFwExtiEven(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiEven {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiEven")
                .field("exti_even_even0", &self.exti_even_even0())
                .field("exti_even_even1", &self.exti_even_even1())
                .field("exti_even_even2", &self.exti_even_even2())
                .field("exti_even_even3", &self.exti_even_even3())
                .field("exti_even_even4", &self.exti_even_even4())
                .field("exti_even_even5", &self.exti_even_even5())
                .field("exti_even_even6", &self.exti_even_even6())
                .field("exti_even_even7", &self.exti_even_even7())
                .field("exti_even_even8", &self.exti_even_even8())
                .field("exti_even_even9", &self.exti_even_even9())
                .field("exti_even_even10", &self.exti_even_even10())
                .field("exti_even_even11", &self.exti_even_even11())
                .field("exti_even_even12", &self.exti_even_even12())
                .field("exti_even_even13", &self.exti_even_even13())
                .field("exti_even_even14", &self.exti_even_even14())
                .field("exti_even_even15", &self.exti_even_even15())
                .field("exti_even_even16", &self.exti_even_even16())
                .field("exti_even_even17", &self.exti_even_even17())
                .field("exti_even_even18", &self.exti_even_even18())
                .field("exti_even_even19", &self.exti_even_even19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiEven {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiEven {{ exti_even_even0: {=bool:?}, exti_even_even1: {=bool:?}, exti_even_even2: {=bool:?}, exti_even_even3: {=bool:?}, exti_even_even4: {=bool:?}, exti_even_even5: {=bool:?}, exti_even_even6: {=bool:?}, exti_even_even7: {=bool:?}, exti_even_even8: {=bool:?}, exti_even_even9: {=bool:?}, exti_even_even10: {=bool:?}, exti_even_even11: {=bool:?}, exti_even_even12: {=bool:?}, exti_even_even13: {=bool:?}, exti_even_even14: {=bool:?}, exti_even_even15: {=bool:?}, exti_even_even16: {=bool:?}, exti_even_even17: {=bool:?}, exti_even_even18: {=bool:?}, exti_even_even19: {=bool:?} }}",
                self.exti_even_even0(),
                self.exti_even_even1(),
                self.exti_even_even2(),
                self.exti_even_even3(),
                self.exti_even_even4(),
                self.exti_even_even5(),
                self.exti_even_even6(),
                self.exti_even_even7(),
                self.exti_even_even8(),
                self.exti_even_even9(),
                self.exti_even_even10(),
                self.exti_even_even11(),
                self.exti_even_even12(),
                self.exti_even_even13(),
                self.exti_even_even14(),
                self.exti_even_even15(),
                self.exti_even_even16(),
                self.exti_even_even17(),
                self.exti_even_even18(),
                self.exti_even_even19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiFten(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiFten {
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_ften_ften19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_ften_ften19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiFten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiFten {
            GdMergedEcb613bbf6cfFwExtiFten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiFten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiFten")
                .field("exti_ften_ften0", &self.exti_ften_ften0())
                .field("exti_ften_ften1", &self.exti_ften_ften1())
                .field("exti_ften_ften2", &self.exti_ften_ften2())
                .field("exti_ften_ften3", &self.exti_ften_ften3())
                .field("exti_ften_ften4", &self.exti_ften_ften4())
                .field("exti_ften_ften5", &self.exti_ften_ften5())
                .field("exti_ften_ften6", &self.exti_ften_ften6())
                .field("exti_ften_ften7", &self.exti_ften_ften7())
                .field("exti_ften_ften8", &self.exti_ften_ften8())
                .field("exti_ften_ften9", &self.exti_ften_ften9())
                .field("exti_ften_ften10", &self.exti_ften_ften10())
                .field("exti_ften_ften11", &self.exti_ften_ften11())
                .field("exti_ften_ften12", &self.exti_ften_ften12())
                .field("exti_ften_ften13", &self.exti_ften_ften13())
                .field("exti_ften_ften14", &self.exti_ften_ften14())
                .field("exti_ften_ften15", &self.exti_ften_ften15())
                .field("exti_ften_ften16", &self.exti_ften_ften16())
                .field("exti_ften_ften17", &self.exti_ften_ften17())
                .field("exti_ften_ften18", &self.exti_ften_ften18())
                .field("exti_ften_ften19", &self.exti_ften_ften19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiFten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiFten {{ exti_ften_ften0: {=bool:?}, exti_ften_ften1: {=bool:?}, exti_ften_ften2: {=bool:?}, exti_ften_ften3: {=bool:?}, exti_ften_ften4: {=bool:?}, exti_ften_ften5: {=bool:?}, exti_ften_ften6: {=bool:?}, exti_ften_ften7: {=bool:?}, exti_ften_ften8: {=bool:?}, exti_ften_ften9: {=bool:?}, exti_ften_ften10: {=bool:?}, exti_ften_ften11: {=bool:?}, exti_ften_ften12: {=bool:?}, exti_ften_ften13: {=bool:?}, exti_ften_ften14: {=bool:?}, exti_ften_ften15: {=bool:?}, exti_ften_ften16: {=bool:?}, exti_ften_ften17: {=bool:?}, exti_ften_ften18: {=bool:?}, exti_ften_ften19: {=bool:?} }}",
                self.exti_ften_ften0(),
                self.exti_ften_ften1(),
                self.exti_ften_ften2(),
                self.exti_ften_ften3(),
                self.exti_ften_ften4(),
                self.exti_ften_ften5(),
                self.exti_ften_ften6(),
                self.exti_ften_ften7(),
                self.exti_ften_ften8(),
                self.exti_ften_ften9(),
                self.exti_ften_ften10(),
                self.exti_ften_ften11(),
                self.exti_ften_ften12(),
                self.exti_ften_ften13(),
                self.exti_ften_ften14(),
                self.exti_ften_ften15(),
                self.exti_ften_ften16(),
                self.exti_ften_ften17(),
                self.exti_ften_ften18(),
                self.exti_ften_ften19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiInten(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiInten {
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_inten_inten19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_inten_inten19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiInten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiInten {
            GdMergedEcb613bbf6cfFwExtiInten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiInten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiInten")
                .field("exti_inten_inten0", &self.exti_inten_inten0())
                .field("exti_inten_inten1", &self.exti_inten_inten1())
                .field("exti_inten_inten2", &self.exti_inten_inten2())
                .field("exti_inten_inten3", &self.exti_inten_inten3())
                .field("exti_inten_inten4", &self.exti_inten_inten4())
                .field("exti_inten_inten5", &self.exti_inten_inten5())
                .field("exti_inten_inten6", &self.exti_inten_inten6())
                .field("exti_inten_inten7", &self.exti_inten_inten7())
                .field("exti_inten_inten8", &self.exti_inten_inten8())
                .field("exti_inten_inten9", &self.exti_inten_inten9())
                .field("exti_inten_inten10", &self.exti_inten_inten10())
                .field("exti_inten_inten11", &self.exti_inten_inten11())
                .field("exti_inten_inten12", &self.exti_inten_inten12())
                .field("exti_inten_inten13", &self.exti_inten_inten13())
                .field("exti_inten_inten14", &self.exti_inten_inten14())
                .field("exti_inten_inten15", &self.exti_inten_inten15())
                .field("exti_inten_inten16", &self.exti_inten_inten16())
                .field("exti_inten_inten17", &self.exti_inten_inten17())
                .field("exti_inten_inten18", &self.exti_inten_inten18())
                .field("exti_inten_inten19", &self.exti_inten_inten19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiInten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiInten {{ exti_inten_inten0: {=bool:?}, exti_inten_inten1: {=bool:?}, exti_inten_inten2: {=bool:?}, exti_inten_inten3: {=bool:?}, exti_inten_inten4: {=bool:?}, exti_inten_inten5: {=bool:?}, exti_inten_inten6: {=bool:?}, exti_inten_inten7: {=bool:?}, exti_inten_inten8: {=bool:?}, exti_inten_inten9: {=bool:?}, exti_inten_inten10: {=bool:?}, exti_inten_inten11: {=bool:?}, exti_inten_inten12: {=bool:?}, exti_inten_inten13: {=bool:?}, exti_inten_inten14: {=bool:?}, exti_inten_inten15: {=bool:?}, exti_inten_inten16: {=bool:?}, exti_inten_inten17: {=bool:?}, exti_inten_inten18: {=bool:?}, exti_inten_inten19: {=bool:?} }}",
                self.exti_inten_inten0(),
                self.exti_inten_inten1(),
                self.exti_inten_inten2(),
                self.exti_inten_inten3(),
                self.exti_inten_inten4(),
                self.exti_inten_inten5(),
                self.exti_inten_inten6(),
                self.exti_inten_inten7(),
                self.exti_inten_inten8(),
                self.exti_inten_inten9(),
                self.exti_inten_inten10(),
                self.exti_inten_inten11(),
                self.exti_inten_inten12(),
                self.exti_inten_inten13(),
                self.exti_inten_inten14(),
                self.exti_inten_inten15(),
                self.exti_inten_inten16(),
                self.exti_inten_inten17(),
                self.exti_inten_inten18(),
                self.exti_inten_inten19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiPd(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiPd {
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_pd_pd19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_pd_pd19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiPd {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiPd {
            GdMergedEcb613bbf6cfFwExtiPd(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiPd")
                .field("exti_pd_pd0", &self.exti_pd_pd0())
                .field("exti_pd_pd1", &self.exti_pd_pd1())
                .field("exti_pd_pd2", &self.exti_pd_pd2())
                .field("exti_pd_pd3", &self.exti_pd_pd3())
                .field("exti_pd_pd4", &self.exti_pd_pd4())
                .field("exti_pd_pd5", &self.exti_pd_pd5())
                .field("exti_pd_pd6", &self.exti_pd_pd6())
                .field("exti_pd_pd7", &self.exti_pd_pd7())
                .field("exti_pd_pd8", &self.exti_pd_pd8())
                .field("exti_pd_pd9", &self.exti_pd_pd9())
                .field("exti_pd_pd10", &self.exti_pd_pd10())
                .field("exti_pd_pd11", &self.exti_pd_pd11())
                .field("exti_pd_pd12", &self.exti_pd_pd12())
                .field("exti_pd_pd13", &self.exti_pd_pd13())
                .field("exti_pd_pd14", &self.exti_pd_pd14())
                .field("exti_pd_pd15", &self.exti_pd_pd15())
                .field("exti_pd_pd16", &self.exti_pd_pd16())
                .field("exti_pd_pd17", &self.exti_pd_pd17())
                .field("exti_pd_pd18", &self.exti_pd_pd18())
                .field("exti_pd_pd19", &self.exti_pd_pd19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiPd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiPd {{ exti_pd_pd0: {=bool:?}, exti_pd_pd1: {=bool:?}, exti_pd_pd2: {=bool:?}, exti_pd_pd3: {=bool:?}, exti_pd_pd4: {=bool:?}, exti_pd_pd5: {=bool:?}, exti_pd_pd6: {=bool:?}, exti_pd_pd7: {=bool:?}, exti_pd_pd8: {=bool:?}, exti_pd_pd9: {=bool:?}, exti_pd_pd10: {=bool:?}, exti_pd_pd11: {=bool:?}, exti_pd_pd12: {=bool:?}, exti_pd_pd13: {=bool:?}, exti_pd_pd14: {=bool:?}, exti_pd_pd15: {=bool:?}, exti_pd_pd16: {=bool:?}, exti_pd_pd17: {=bool:?}, exti_pd_pd18: {=bool:?}, exti_pd_pd19: {=bool:?} }}",
                self.exti_pd_pd0(),
                self.exti_pd_pd1(),
                self.exti_pd_pd2(),
                self.exti_pd_pd3(),
                self.exti_pd_pd4(),
                self.exti_pd_pd5(),
                self.exti_pd_pd6(),
                self.exti_pd_pd7(),
                self.exti_pd_pd8(),
                self.exti_pd_pd9(),
                self.exti_pd_pd10(),
                self.exti_pd_pd11(),
                self.exti_pd_pd12(),
                self.exti_pd_pd13(),
                self.exti_pd_pd14(),
                self.exti_pd_pd15(),
                self.exti_pd_pd16(),
                self.exti_pd_pd17(),
                self.exti_pd_pd18(),
                self.exti_pd_pd19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiRten(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiRten {
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_rten_rten19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_rten_rten19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiRten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiRten {
            GdMergedEcb613bbf6cfFwExtiRten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiRten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiRten")
                .field("exti_rten_rten0", &self.exti_rten_rten0())
                .field("exti_rten_rten1", &self.exti_rten_rten1())
                .field("exti_rten_rten2", &self.exti_rten_rten2())
                .field("exti_rten_rten3", &self.exti_rten_rten3())
                .field("exti_rten_rten4", &self.exti_rten_rten4())
                .field("exti_rten_rten5", &self.exti_rten_rten5())
                .field("exti_rten_rten6", &self.exti_rten_rten6())
                .field("exti_rten_rten7", &self.exti_rten_rten7())
                .field("exti_rten_rten8", &self.exti_rten_rten8())
                .field("exti_rten_rten9", &self.exti_rten_rten9())
                .field("exti_rten_rten10", &self.exti_rten_rten10())
                .field("exti_rten_rten11", &self.exti_rten_rten11())
                .field("exti_rten_rten12", &self.exti_rten_rten12())
                .field("exti_rten_rten13", &self.exti_rten_rten13())
                .field("exti_rten_rten14", &self.exti_rten_rten14())
                .field("exti_rten_rten15", &self.exti_rten_rten15())
                .field("exti_rten_rten16", &self.exti_rten_rten16())
                .field("exti_rten_rten17", &self.exti_rten_rten17())
                .field("exti_rten_rten18", &self.exti_rten_rten18())
                .field("exti_rten_rten19", &self.exti_rten_rten19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiRten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiRten {{ exti_rten_rten0: {=bool:?}, exti_rten_rten1: {=bool:?}, exti_rten_rten2: {=bool:?}, exti_rten_rten3: {=bool:?}, exti_rten_rten4: {=bool:?}, exti_rten_rten5: {=bool:?}, exti_rten_rten6: {=bool:?}, exti_rten_rten7: {=bool:?}, exti_rten_rten8: {=bool:?}, exti_rten_rten9: {=bool:?}, exti_rten_rten10: {=bool:?}, exti_rten_rten11: {=bool:?}, exti_rten_rten12: {=bool:?}, exti_rten_rten13: {=bool:?}, exti_rten_rten14: {=bool:?}, exti_rten_rten15: {=bool:?}, exti_rten_rten16: {=bool:?}, exti_rten_rten17: {=bool:?}, exti_rten_rten18: {=bool:?}, exti_rten_rten19: {=bool:?} }}",
                self.exti_rten_rten0(),
                self.exti_rten_rten1(),
                self.exti_rten_rten2(),
                self.exti_rten_rten3(),
                self.exti_rten_rten4(),
                self.exti_rten_rten5(),
                self.exti_rten_rten6(),
                self.exti_rten_rten7(),
                self.exti_rten_rten8(),
                self.exti_rten_rten9(),
                self.exti_rten_rten10(),
                self.exti_rten_rten11(),
                self.exti_rten_rten12(),
                self.exti_rten_rten13(),
                self.exti_rten_rten14(),
                self.exti_rten_rten15(),
                self.exti_rten_rten16(),
                self.exti_rten_rten17(),
                self.exti_rten_rten18(),
                self.exti_rten_rten19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfFwExtiSwiev(pub u32);
    impl GdMergedEcb613bbf6cfFwExtiSwiev {
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn exti_swiev_swiev19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_exti_swiev_swiev19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfFwExtiSwiev {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfFwExtiSwiev {
            GdMergedEcb613bbf6cfFwExtiSwiev(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfFwExtiSwiev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfFwExtiSwiev")
                .field("exti_swiev_swiev0", &self.exti_swiev_swiev0())
                .field("exti_swiev_swiev1", &self.exti_swiev_swiev1())
                .field("exti_swiev_swiev2", &self.exti_swiev_swiev2())
                .field("exti_swiev_swiev3", &self.exti_swiev_swiev3())
                .field("exti_swiev_swiev4", &self.exti_swiev_swiev4())
                .field("exti_swiev_swiev5", &self.exti_swiev_swiev5())
                .field("exti_swiev_swiev6", &self.exti_swiev_swiev6())
                .field("exti_swiev_swiev7", &self.exti_swiev_swiev7())
                .field("exti_swiev_swiev8", &self.exti_swiev_swiev8())
                .field("exti_swiev_swiev9", &self.exti_swiev_swiev9())
                .field("exti_swiev_swiev10", &self.exti_swiev_swiev10())
                .field("exti_swiev_swiev11", &self.exti_swiev_swiev11())
                .field("exti_swiev_swiev12", &self.exti_swiev_swiev12())
                .field("exti_swiev_swiev13", &self.exti_swiev_swiev13())
                .field("exti_swiev_swiev14", &self.exti_swiev_swiev14())
                .field("exti_swiev_swiev15", &self.exti_swiev_swiev15())
                .field("exti_swiev_swiev16", &self.exti_swiev_swiev16())
                .field("exti_swiev_swiev17", &self.exti_swiev_swiev17())
                .field("exti_swiev_swiev18", &self.exti_swiev_swiev18())
                .field("exti_swiev_swiev19", &self.exti_swiev_swiev19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfFwExtiSwiev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfFwExtiSwiev {{ exti_swiev_swiev0: {=bool:?}, exti_swiev_swiev1: {=bool:?}, exti_swiev_swiev2: {=bool:?}, exti_swiev_swiev3: {=bool:?}, exti_swiev_swiev4: {=bool:?}, exti_swiev_swiev5: {=bool:?}, exti_swiev_swiev6: {=bool:?}, exti_swiev_swiev7: {=bool:?}, exti_swiev_swiev8: {=bool:?}, exti_swiev_swiev9: {=bool:?}, exti_swiev_swiev10: {=bool:?}, exti_swiev_swiev11: {=bool:?}, exti_swiev_swiev12: {=bool:?}, exti_swiev_swiev13: {=bool:?}, exti_swiev_swiev14: {=bool:?}, exti_swiev_swiev15: {=bool:?}, exti_swiev_swiev16: {=bool:?}, exti_swiev_swiev17: {=bool:?}, exti_swiev_swiev18: {=bool:?}, exti_swiev_swiev19: {=bool:?} }}",
                self.exti_swiev_swiev0(),
                self.exti_swiev_swiev1(),
                self.exti_swiev_swiev2(),
                self.exti_swiev_swiev3(),
                self.exti_swiev_swiev4(),
                self.exti_swiev_swiev5(),
                self.exti_swiev_swiev6(),
                self.exti_swiev_swiev7(),
                self.exti_swiev_swiev8(),
                self.exti_swiev_swiev9(),
                self.exti_swiev_swiev10(),
                self.exti_swiev_swiev11(),
                self.exti_swiev_swiev12(),
                self.exti_swiev_swiev13(),
                self.exti_swiev_swiev14(),
                self.exti_swiev_swiev15(),
                self.exti_swiev_swiev16(),
                self.exti_swiev_swiev17(),
                self.exti_swiev_swiev18(),
                self.exti_swiev_swiev19()
            )
        }
    }
    #[doc = "Event enable register (EXTI_EVEN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdEven(pub u32);
    impl GdMergedEcb613bbf6cfSvdEven {
        #[doc = "Enable Event on line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn even0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 0"]
        #[inline(always)]
        pub const fn set_even0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Event on line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn even1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 1"]
        #[inline(always)]
        pub const fn set_even1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Event on line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn even2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 2"]
        #[inline(always)]
        pub const fn set_even2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Event on line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn even3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 3"]
        #[inline(always)]
        pub const fn set_even3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Event on line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn even4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 4"]
        #[inline(always)]
        pub const fn set_even4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Event on line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn even5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 5"]
        #[inline(always)]
        pub const fn set_even5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Event on line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn even6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 6"]
        #[inline(always)]
        pub const fn set_even6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Event on line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn even7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 7"]
        #[inline(always)]
        pub const fn set_even7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Event on line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn even8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 8"]
        #[inline(always)]
        pub const fn set_even8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Event on line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn even9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 9"]
        #[inline(always)]
        pub const fn set_even9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Event on line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn even10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 10"]
        #[inline(always)]
        pub const fn set_even10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Event on line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn even11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 11"]
        #[inline(always)]
        pub const fn set_even11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Event on line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn even12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 12"]
        #[inline(always)]
        pub const fn set_even12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Event on line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn even13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 13"]
        #[inline(always)]
        pub const fn set_even13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Event on line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn even14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 14"]
        #[inline(always)]
        pub const fn set_even14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Event on line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn even15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 15"]
        #[inline(always)]
        pub const fn set_even15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Enable Event on line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn even16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 16"]
        #[inline(always)]
        pub const fn set_even16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable Event on line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn even17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 17"]
        #[inline(always)]
        pub const fn set_even17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable Event on line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn even18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 18"]
        #[inline(always)]
        pub const fn set_even18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Event on line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn even19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Event on line 19"]
        #[inline(always)]
        pub const fn set_even19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdEven {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdEven {
            GdMergedEcb613bbf6cfSvdEven(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdEven {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdEven")
                .field("even0", &self.even0())
                .field("even1", &self.even1())
                .field("even2", &self.even2())
                .field("even3", &self.even3())
                .field("even4", &self.even4())
                .field("even5", &self.even5())
                .field("even6", &self.even6())
                .field("even7", &self.even7())
                .field("even8", &self.even8())
                .field("even9", &self.even9())
                .field("even10", &self.even10())
                .field("even11", &self.even11())
                .field("even12", &self.even12())
                .field("even13", &self.even13())
                .field("even14", &self.even14())
                .field("even15", &self.even15())
                .field("even16", &self.even16())
                .field("even17", &self.even17())
                .field("even18", &self.even18())
                .field("even19", &self.even19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdEven {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdEven {{ even0: {=bool:?}, even1: {=bool:?}, even2: {=bool:?}, even3: {=bool:?}, even4: {=bool:?}, even5: {=bool:?}, even6: {=bool:?}, even7: {=bool:?}, even8: {=bool:?}, even9: {=bool:?}, even10: {=bool:?}, even11: {=bool:?}, even12: {=bool:?}, even13: {=bool:?}, even14: {=bool:?}, even15: {=bool:?}, even16: {=bool:?}, even17: {=bool:?}, even18: {=bool:?}, even19: {=bool:?} }}",
                self.even0(),
                self.even1(),
                self.even2(),
                self.even3(),
                self.even4(),
                self.even5(),
                self.even6(),
                self.even7(),
                self.even8(),
                self.even9(),
                self.even10(),
                self.even11(),
                self.even12(),
                self.even13(),
                self.even14(),
                self.even15(),
                self.even16(),
                self.even17(),
                self.even18(),
                self.even19()
            )
        }
    }
    #[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdFten(pub u32);
    impl GdMergedEcb613bbf6cfSvdFten {
        #[doc = "Falling edge trigger enable of line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn ften0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 0"]
        #[inline(always)]
        pub const fn set_ften0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Falling edge trigger enable of line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn ften1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 1"]
        #[inline(always)]
        pub const fn set_ften1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Falling edge trigger enable of line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn ften2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 2"]
        #[inline(always)]
        pub const fn set_ften2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Falling edge trigger enable of line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn ften3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 3"]
        #[inline(always)]
        pub const fn set_ften3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Falling edge trigger enable of line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn ften4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 4"]
        #[inline(always)]
        pub const fn set_ften4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Falling edge trigger enable of line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn ften5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 5"]
        #[inline(always)]
        pub const fn set_ften5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Falling edge trigger enable of line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn ften6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 6"]
        #[inline(always)]
        pub const fn set_ften6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Falling edge trigger enable of line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn ften7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 7"]
        #[inline(always)]
        pub const fn set_ften7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Falling edge trigger enable of line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn ften8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 8"]
        #[inline(always)]
        pub const fn set_ften8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Falling edge trigger enable of line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn ften9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 9"]
        #[inline(always)]
        pub const fn set_ften9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Falling edge trigger enable of line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn ften10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 10"]
        #[inline(always)]
        pub const fn set_ften10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Falling edge trigger enable of line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn ften11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 11"]
        #[inline(always)]
        pub const fn set_ften11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Falling edge trigger enable of line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn ften12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 12"]
        #[inline(always)]
        pub const fn set_ften12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Falling edge trigger enable of line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn ften13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 13"]
        #[inline(always)]
        pub const fn set_ften13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Falling edge trigger enable of line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn ften14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 14"]
        #[inline(always)]
        pub const fn set_ften14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Falling edge trigger enable of line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn ften15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 15"]
        #[inline(always)]
        pub const fn set_ften15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Falling edge trigger enable of line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn ften16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 16"]
        #[inline(always)]
        pub const fn set_ften16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Falling edge trigger enable of line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn ften17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 17"]
        #[inline(always)]
        pub const fn set_ften17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Falling edge trigger enable of line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn ften18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 18"]
        #[inline(always)]
        pub const fn set_ften18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Falling edge trigger enable of line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn ften19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Falling edge trigger enable of line 19"]
        #[inline(always)]
        pub const fn set_ften19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdFten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdFten {
            GdMergedEcb613bbf6cfSvdFten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdFten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdFten")
                .field("ften0", &self.ften0())
                .field("ften1", &self.ften1())
                .field("ften2", &self.ften2())
                .field("ften3", &self.ften3())
                .field("ften4", &self.ften4())
                .field("ften5", &self.ften5())
                .field("ften6", &self.ften6())
                .field("ften7", &self.ften7())
                .field("ften8", &self.ften8())
                .field("ften9", &self.ften9())
                .field("ften10", &self.ften10())
                .field("ften11", &self.ften11())
                .field("ften12", &self.ften12())
                .field("ften13", &self.ften13())
                .field("ften14", &self.ften14())
                .field("ften15", &self.ften15())
                .field("ften16", &self.ften16())
                .field("ften17", &self.ften17())
                .field("ften18", &self.ften18())
                .field("ften19", &self.ften19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdFten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdFten {{ ften0: {=bool:?}, ften1: {=bool:?}, ften2: {=bool:?}, ften3: {=bool:?}, ften4: {=bool:?}, ften5: {=bool:?}, ften6: {=bool:?}, ften7: {=bool:?}, ften8: {=bool:?}, ften9: {=bool:?}, ften10: {=bool:?}, ften11: {=bool:?}, ften12: {=bool:?}, ften13: {=bool:?}, ften14: {=bool:?}, ften15: {=bool:?}, ften16: {=bool:?}, ften17: {=bool:?}, ften18: {=bool:?}, ften19: {=bool:?} }}",
                self.ften0(),
                self.ften1(),
                self.ften2(),
                self.ften3(),
                self.ften4(),
                self.ften5(),
                self.ften6(),
                self.ften7(),
                self.ften8(),
                self.ften9(),
                self.ften10(),
                self.ften11(),
                self.ften12(),
                self.ften13(),
                self.ften14(),
                self.ften15(),
                self.ften16(),
                self.ften17(),
                self.ften18(),
                self.ften19()
            )
        }
    }
    #[doc = "Interrupt enable register (EXTI_INTEN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdInten(pub u32);
    impl GdMergedEcb613bbf6cfSvdInten {
        #[doc = "Enable Interrupt on line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn inten0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 0"]
        #[inline(always)]
        pub const fn set_inten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Interrupt on line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn inten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 1"]
        #[inline(always)]
        pub const fn set_inten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Interrupt on line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn inten2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 2"]
        #[inline(always)]
        pub const fn set_inten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Interrupt on line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn inten3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 3"]
        #[inline(always)]
        pub const fn set_inten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Interrupt on line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn inten4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 4"]
        #[inline(always)]
        pub const fn set_inten4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Interrupt on line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn inten5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 5"]
        #[inline(always)]
        pub const fn set_inten5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable Interrupt on line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn inten6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 6"]
        #[inline(always)]
        pub const fn set_inten6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable Interrupt on line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn inten7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 7"]
        #[inline(always)]
        pub const fn set_inten7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable Interrupt on line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn inten8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 8"]
        #[inline(always)]
        pub const fn set_inten8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable Interrupt on line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn inten9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 9"]
        #[inline(always)]
        pub const fn set_inten9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Interrupt on line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn inten10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 10"]
        #[inline(always)]
        pub const fn set_inten10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Interrupt on line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn inten11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 11"]
        #[inline(always)]
        pub const fn set_inten11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable Interrupt on line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn inten12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 12"]
        #[inline(always)]
        pub const fn set_inten12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable Interrupt on line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn inten13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 13"]
        #[inline(always)]
        pub const fn set_inten13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable Interrupt on line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn inten14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 14"]
        #[inline(always)]
        pub const fn set_inten14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Interrupt on line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn inten15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 15"]
        #[inline(always)]
        pub const fn set_inten15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Enable Interrupt on line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn inten16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 16"]
        #[inline(always)]
        pub const fn set_inten16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable Interrupt on line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn inten17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 17"]
        #[inline(always)]
        pub const fn set_inten17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable Interrupt on line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn inten18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 18"]
        #[inline(always)]
        pub const fn set_inten18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable Interrupt on line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn inten19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Interrupt on line 19"]
        #[inline(always)]
        pub const fn set_inten19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdInten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdInten {
            GdMergedEcb613bbf6cfSvdInten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdInten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdInten")
                .field("inten0", &self.inten0())
                .field("inten1", &self.inten1())
                .field("inten2", &self.inten2())
                .field("inten3", &self.inten3())
                .field("inten4", &self.inten4())
                .field("inten5", &self.inten5())
                .field("inten6", &self.inten6())
                .field("inten7", &self.inten7())
                .field("inten8", &self.inten8())
                .field("inten9", &self.inten9())
                .field("inten10", &self.inten10())
                .field("inten11", &self.inten11())
                .field("inten12", &self.inten12())
                .field("inten13", &self.inten13())
                .field("inten14", &self.inten14())
                .field("inten15", &self.inten15())
                .field("inten16", &self.inten16())
                .field("inten17", &self.inten17())
                .field("inten18", &self.inten18())
                .field("inten19", &self.inten19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdInten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdInten {{ inten0: {=bool:?}, inten1: {=bool:?}, inten2: {=bool:?}, inten3: {=bool:?}, inten4: {=bool:?}, inten5: {=bool:?}, inten6: {=bool:?}, inten7: {=bool:?}, inten8: {=bool:?}, inten9: {=bool:?}, inten10: {=bool:?}, inten11: {=bool:?}, inten12: {=bool:?}, inten13: {=bool:?}, inten14: {=bool:?}, inten15: {=bool:?}, inten16: {=bool:?}, inten17: {=bool:?}, inten18: {=bool:?}, inten19: {=bool:?} }}",
                self.inten0(),
                self.inten1(),
                self.inten2(),
                self.inten3(),
                self.inten4(),
                self.inten5(),
                self.inten6(),
                self.inten7(),
                self.inten8(),
                self.inten9(),
                self.inten10(),
                self.inten11(),
                self.inten12(),
                self.inten13(),
                self.inten14(),
                self.inten15(),
                self.inten16(),
                self.inten17(),
                self.inten18(),
                self.inten19()
            )
        }
    }
    #[doc = "Pending register (EXTI_PD)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdPd(pub u32);
    impl GdMergedEcb613bbf6cfSvdPd {
        #[doc = "Interrupt pending status of line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 0"]
        #[inline(always)]
        pub const fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt pending status of line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 1"]
        #[inline(always)]
        pub const fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt pending status of line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 2"]
        #[inline(always)]
        pub const fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt pending status of line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 3"]
        #[inline(always)]
        pub const fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt pending status of line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 4"]
        #[inline(always)]
        pub const fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt pending status of line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 5"]
        #[inline(always)]
        pub const fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interrupt pending status of line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 6"]
        #[inline(always)]
        pub const fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt pending status of line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 7"]
        #[inline(always)]
        pub const fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Interrupt pending status of line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 8"]
        #[inline(always)]
        pub const fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt pending status of line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 9"]
        #[inline(always)]
        pub const fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Interrupt pending status of line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 10"]
        #[inline(always)]
        pub const fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Interrupt pending status of line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 11"]
        #[inline(always)]
        pub const fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Interrupt pending status of line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 12"]
        #[inline(always)]
        pub const fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Interrupt pending status of line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 13"]
        #[inline(always)]
        pub const fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Interrupt pending status of line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 14"]
        #[inline(always)]
        pub const fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Interrupt pending status of line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 15"]
        #[inline(always)]
        pub const fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Interrupt pending status of line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn pd16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 16"]
        #[inline(always)]
        pub const fn set_pd16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Interrupt pending status of line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn pd17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 17"]
        #[inline(always)]
        pub const fn set_pd17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Interrupt pending status of line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn pd18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 18"]
        #[inline(always)]
        pub const fn set_pd18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Interrupt pending status of line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn pd19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt pending status of line 19"]
        #[inline(always)]
        pub const fn set_pd19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdPd {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdPd {
            GdMergedEcb613bbf6cfSvdPd(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdPd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdPd")
                .field("pd0", &self.pd0())
                .field("pd1", &self.pd1())
                .field("pd2", &self.pd2())
                .field("pd3", &self.pd3())
                .field("pd4", &self.pd4())
                .field("pd5", &self.pd5())
                .field("pd6", &self.pd6())
                .field("pd7", &self.pd7())
                .field("pd8", &self.pd8())
                .field("pd9", &self.pd9())
                .field("pd10", &self.pd10())
                .field("pd11", &self.pd11())
                .field("pd12", &self.pd12())
                .field("pd13", &self.pd13())
                .field("pd14", &self.pd14())
                .field("pd15", &self.pd15())
                .field("pd16", &self.pd16())
                .field("pd17", &self.pd17())
                .field("pd18", &self.pd18())
                .field("pd19", &self.pd19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdPd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdPd {{ pd0: {=bool:?}, pd1: {=bool:?}, pd2: {=bool:?}, pd3: {=bool:?}, pd4: {=bool:?}, pd5: {=bool:?}, pd6: {=bool:?}, pd7: {=bool:?}, pd8: {=bool:?}, pd9: {=bool:?}, pd10: {=bool:?}, pd11: {=bool:?}, pd12: {=bool:?}, pd13: {=bool:?}, pd14: {=bool:?}, pd15: {=bool:?}, pd16: {=bool:?}, pd17: {=bool:?}, pd18: {=bool:?}, pd19: {=bool:?} }}",
                self.pd0(),
                self.pd1(),
                self.pd2(),
                self.pd3(),
                self.pd4(),
                self.pd5(),
                self.pd6(),
                self.pd7(),
                self.pd8(),
                self.pd9(),
                self.pd10(),
                self.pd11(),
                self.pd12(),
                self.pd13(),
                self.pd14(),
                self.pd15(),
                self.pd16(),
                self.pd17(),
                self.pd18(),
                self.pd19()
            )
        }
    }
    #[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdRten(pub u32);
    impl GdMergedEcb613bbf6cfSvdRten {
        #[doc = "Rising edge trigger enable of line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn rten0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 0"]
        #[inline(always)]
        pub const fn set_rten0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rising edge trigger enable of line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn rten1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 1"]
        #[inline(always)]
        pub const fn set_rten1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Rising edge trigger enable of line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn rten2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 2"]
        #[inline(always)]
        pub const fn set_rten2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Rising edge trigger enable of line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn rten3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 3"]
        #[inline(always)]
        pub const fn set_rten3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Rising edge trigger enable of line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn rten4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 4"]
        #[inline(always)]
        pub const fn set_rten4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rising edge trigger enable of line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn rten5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 5"]
        #[inline(always)]
        pub const fn set_rten5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Rising edge trigger enable of line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn rten6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 6"]
        #[inline(always)]
        pub const fn set_rten6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Rising edge trigger enable of line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn rten7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 7"]
        #[inline(always)]
        pub const fn set_rten7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Rising edge trigger enable of line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn rten8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 8"]
        #[inline(always)]
        pub const fn set_rten8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Rising edge trigger enable of line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn rten9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 9"]
        #[inline(always)]
        pub const fn set_rten9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Rising edge trigger enable of line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn rten10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 10"]
        #[inline(always)]
        pub const fn set_rten10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Rising edge trigger enable of line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn rten11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 11"]
        #[inline(always)]
        pub const fn set_rten11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Rising edge trigger enable of line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn rten12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 12"]
        #[inline(always)]
        pub const fn set_rten12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Rising edge trigger enable of line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn rten13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 13"]
        #[inline(always)]
        pub const fn set_rten13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Rising edge trigger enable of line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn rten14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 14"]
        #[inline(always)]
        pub const fn set_rten14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rising edge trigger enable of line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn rten15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 15"]
        #[inline(always)]
        pub const fn set_rten15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Rising edge trigger enable of line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn rten16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 16"]
        #[inline(always)]
        pub const fn set_rten16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Rising edge trigger enable of line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn rten17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 17"]
        #[inline(always)]
        pub const fn set_rten17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Rising edge trigger enable of line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn rten18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 18"]
        #[inline(always)]
        pub const fn set_rten18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Rising edge trigger enable of line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn rten19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Rising edge trigger enable of line 19"]
        #[inline(always)]
        pub const fn set_rten19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdRten {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdRten {
            GdMergedEcb613bbf6cfSvdRten(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdRten {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdRten")
                .field("rten0", &self.rten0())
                .field("rten1", &self.rten1())
                .field("rten2", &self.rten2())
                .field("rten3", &self.rten3())
                .field("rten4", &self.rten4())
                .field("rten5", &self.rten5())
                .field("rten6", &self.rten6())
                .field("rten7", &self.rten7())
                .field("rten8", &self.rten8())
                .field("rten9", &self.rten9())
                .field("rten10", &self.rten10())
                .field("rten11", &self.rten11())
                .field("rten12", &self.rten12())
                .field("rten13", &self.rten13())
                .field("rten14", &self.rten14())
                .field("rten15", &self.rten15())
                .field("rten16", &self.rten16())
                .field("rten17", &self.rten17())
                .field("rten18", &self.rten18())
                .field("rten19", &self.rten19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdRten {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdRten {{ rten0: {=bool:?}, rten1: {=bool:?}, rten2: {=bool:?}, rten3: {=bool:?}, rten4: {=bool:?}, rten5: {=bool:?}, rten6: {=bool:?}, rten7: {=bool:?}, rten8: {=bool:?}, rten9: {=bool:?}, rten10: {=bool:?}, rten11: {=bool:?}, rten12: {=bool:?}, rten13: {=bool:?}, rten14: {=bool:?}, rten15: {=bool:?}, rten16: {=bool:?}, rten17: {=bool:?}, rten18: {=bool:?}, rten19: {=bool:?} }}",
                self.rten0(),
                self.rten1(),
                self.rten2(),
                self.rten3(),
                self.rten4(),
                self.rten5(),
                self.rten6(),
                self.rten7(),
                self.rten8(),
                self.rten9(),
                self.rten10(),
                self.rten11(),
                self.rten12(),
                self.rten13(),
                self.rten14(),
                self.rten15(),
                self.rten16(),
                self.rten17(),
                self.rten18(),
                self.rten19()
            )
        }
    }
    #[doc = "Software interrupt event register (EXTI_SWIEV)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedEcb613bbf6cfSvdSwiev(pub u32);
    impl GdMergedEcb613bbf6cfSvdSwiev {
        #[doc = "Interrupt/Event software trigger on line 0"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 0"]
        #[inline(always)]
        pub const fn set_swiev0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt/Event software trigger on line 1"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 1"]
        #[inline(always)]
        pub const fn set_swiev1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt/Event software trigger on line 2"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 2"]
        #[inline(always)]
        pub const fn set_swiev2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt/Event software trigger on line 3"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 3"]
        #[inline(always)]
        pub const fn set_swiev3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt/Event software trigger on line 4"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 4"]
        #[inline(always)]
        pub const fn set_swiev4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt/Event software trigger on line 5"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 5"]
        #[inline(always)]
        pub const fn set_swiev5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Interrupt/Event software trigger on line 6"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 6"]
        #[inline(always)]
        pub const fn set_swiev6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt/Event software trigger on line 7"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 7"]
        #[inline(always)]
        pub const fn set_swiev7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Interrupt/Event software trigger on line 8"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 8"]
        #[inline(always)]
        pub const fn set_swiev8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt/Event software trigger on line 9"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 9"]
        #[inline(always)]
        pub const fn set_swiev9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Interrupt/Event software trigger on line 10"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 10"]
        #[inline(always)]
        pub const fn set_swiev10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Interrupt/Event software trigger on line 11"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 11"]
        #[inline(always)]
        pub const fn set_swiev11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Interrupt/Event software trigger on line 12"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 12"]
        #[inline(always)]
        pub const fn set_swiev12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Interrupt/Event software trigger on line 13"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 13"]
        #[inline(always)]
        pub const fn set_swiev13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Interrupt/Event software trigger on line 14"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 14"]
        #[inline(always)]
        pub const fn set_swiev14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Interrupt/Event software trigger on line 15"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 15"]
        #[inline(always)]
        pub const fn set_swiev15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Interrupt/Event software trigger on line 16"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 16"]
        #[inline(always)]
        pub const fn set_swiev16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Interrupt/Event software trigger on line 17"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 17"]
        #[inline(always)]
        pub const fn set_swiev17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Interrupt/Event software trigger on line 18"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 18"]
        #[inline(always)]
        pub const fn set_swiev18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Interrupt/Event software trigger on line 19"]
        #[must_use]
        #[inline(always)]
        pub const fn swiev19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/Event software trigger on line 19"]
        #[inline(always)]
        pub const fn set_swiev19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for GdMergedEcb613bbf6cfSvdSwiev {
        #[inline(always)]
        fn default() -> GdMergedEcb613bbf6cfSvdSwiev {
            GdMergedEcb613bbf6cfSvdSwiev(0)
        }
    }
    impl core::fmt::Debug for GdMergedEcb613bbf6cfSvdSwiev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedEcb613bbf6cfSvdSwiev")
                .field("swiev0", &self.swiev0())
                .field("swiev1", &self.swiev1())
                .field("swiev2", &self.swiev2())
                .field("swiev3", &self.swiev3())
                .field("swiev4", &self.swiev4())
                .field("swiev5", &self.swiev5())
                .field("swiev6", &self.swiev6())
                .field("swiev7", &self.swiev7())
                .field("swiev8", &self.swiev8())
                .field("swiev9", &self.swiev9())
                .field("swiev10", &self.swiev10())
                .field("swiev11", &self.swiev11())
                .field("swiev12", &self.swiev12())
                .field("swiev13", &self.swiev13())
                .field("swiev14", &self.swiev14())
                .field("swiev15", &self.swiev15())
                .field("swiev16", &self.swiev16())
                .field("swiev17", &self.swiev17())
                .field("swiev18", &self.swiev18())
                .field("swiev19", &self.swiev19())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedEcb613bbf6cfSvdSwiev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedEcb613bbf6cfSvdSwiev {{ swiev0: {=bool:?}, swiev1: {=bool:?}, swiev2: {=bool:?}, swiev3: {=bool:?}, swiev4: {=bool:?}, swiev5: {=bool:?}, swiev6: {=bool:?}, swiev7: {=bool:?}, swiev8: {=bool:?}, swiev9: {=bool:?}, swiev10: {=bool:?}, swiev11: {=bool:?}, swiev12: {=bool:?}, swiev13: {=bool:?}, swiev14: {=bool:?}, swiev15: {=bool:?}, swiev16: {=bool:?}, swiev17: {=bool:?}, swiev18: {=bool:?}, swiev19: {=bool:?} }}",
                self.swiev0(),
                self.swiev1(),
                self.swiev2(),
                self.swiev3(),
                self.swiev4(),
                self.swiev5(),
                self.swiev6(),
                self.swiev7(),
                self.swiev8(),
                self.swiev9(),
                self.swiev10(),
                self.swiev11(),
                self.swiev12(),
                self.swiev13(),
                self.swiev14(),
                self.swiev15(),
                self.swiev16(),
                self.swiev17(),
                self.swiev18(),
                self.swiev19()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lines(pub u32);
    impl Lines {
        #[doc = "EXTI line"]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI line"]
        #[inline(always)]
        pub const fn set_line(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Lines {
        #[inline(always)]
        fn default() -> Lines {
            Lines(0)
        }
    }
    impl core::fmt::Debug for Lines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lines")
                .field("line[0]", &self.line(0usize))
                .field("line[1]", &self.line(1usize))
                .field("line[2]", &self.line(2usize))
                .field("line[3]", &self.line(3usize))
                .field("line[4]", &self.line(4usize))
                .field("line[5]", &self.line(5usize))
                .field("line[6]", &self.line(6usize))
                .field("line[7]", &self.line(7usize))
                .field("line[8]", &self.line(8usize))
                .field("line[9]", &self.line(9usize))
                .field("line[10]", &self.line(10usize))
                .field("line[11]", &self.line(11usize))
                .field("line[12]", &self.line(12usize))
                .field("line[13]", &self.line(13usize))
                .field("line[14]", &self.line(14usize))
                .field("line[15]", &self.line(15usize))
                .field("line[16]", &self.line(16usize))
                .field("line[17]", &self.line(17usize))
                .field("line[18]", &self.line(18usize))
                .field("line[19]", &self.line(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lines {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lines {{ line[0]: {=bool:?}, line[1]: {=bool:?}, line[2]: {=bool:?}, line[3]: {=bool:?}, line[4]: {=bool:?}, line[5]: {=bool:?}, line[6]: {=bool:?}, line[7]: {=bool:?}, line[8]: {=bool:?}, line[9]: {=bool:?}, line[10]: {=bool:?}, line[11]: {=bool:?}, line[12]: {=bool:?}, line[13]: {=bool:?}, line[14]: {=bool:?}, line[15]: {=bool:?}, line[16]: {=bool:?}, line[17]: {=bool:?}, line[18]: {=bool:?}, line[19]: {=bool:?} }}",
                self.line(0usize),
                self.line(1usize),
                self.line(2usize),
                self.line(3usize),
                self.line(4usize),
                self.line(5usize),
                self.line(6usize),
                self.line(7usize),
                self.line(8usize),
                self.line(9usize),
                self.line(10usize),
                self.line(11usize),
                self.line(12usize),
                self.line(13usize),
                self.line(14usize),
                self.line(15usize),
                self.line(16usize),
                self.line(17usize),
                self.line(18usize),
                self.line(19usize)
            )
        }
    }
}
