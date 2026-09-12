#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdMergedFc02e261ca5a {
    ptr: *mut u8,
}
unsafe impl Send for GdMergedFc02e261ca5a {}
unsafe impl Sync for GdMergedFc02e261ca5a {}
impl GdMergedFc02e261ca5a {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ctc_ctl0(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aFwCtcCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control register 0"]
    #[inline(always)]
    pub const fn ctl0(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aSvdCtl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ctc_ctl1(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aFwCtcCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn ctl1(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aSvdCtl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ctc_stat(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aFwCtcStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aSvdStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ctc_intc(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aFwCtcIntc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt clear register"]
    #[inline(always)]
    pub const fn intc(self) -> crate::common::Reg<regs::GdMergedFc02e261ca5aSvdIntc, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aFwCtcCtl0(pub u32);
    impl GdMergedFc02e261ca5aFwCtcCtl0 {
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_ckokie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_ckokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_ckwarnie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_ckwarnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_erefie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_erefie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_cnten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_cnten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_autotrim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_autotrim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_swrefpul(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_swrefpul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl0_trimvalue(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ctc_ctl0_trimvalue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aFwCtcCtl0 {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aFwCtcCtl0 {
            GdMergedFc02e261ca5aFwCtcCtl0(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aFwCtcCtl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aFwCtcCtl0")
                .field("ctc_ctl0_ckokie", &self.ctc_ctl0_ckokie())
                .field("ctc_ctl0_ckwarnie", &self.ctc_ctl0_ckwarnie())
                .field("ctc_ctl0_errie", &self.ctc_ctl0_errie())
                .field("ctc_ctl0_erefie", &self.ctc_ctl0_erefie())
                .field("ctc_ctl0_cnten", &self.ctc_ctl0_cnten())
                .field("ctc_ctl0_autotrim", &self.ctc_ctl0_autotrim())
                .field("ctc_ctl0_swrefpul", &self.ctc_ctl0_swrefpul())
                .field("ctc_ctl0_trimvalue", &self.ctc_ctl0_trimvalue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aFwCtcCtl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aFwCtcCtl0 {{ ctc_ctl0_ckokie: {=bool:?}, ctc_ctl0_ckwarnie: {=bool:?}, ctc_ctl0_errie: {=bool:?}, ctc_ctl0_erefie: {=bool:?}, ctc_ctl0_cnten: {=bool:?}, ctc_ctl0_autotrim: {=bool:?}, ctc_ctl0_swrefpul: {=bool:?}, ctc_ctl0_trimvalue: {=u8:?} }}",
                self.ctc_ctl0_ckokie(),
                self.ctc_ctl0_ckwarnie(),
                self.ctc_ctl0_errie(),
                self.ctc_ctl0_erefie(),
                self.ctc_ctl0_cnten(),
                self.ctc_ctl0_autotrim(),
                self.ctc_ctl0_swrefpul(),
                self.ctc_ctl0_trimvalue()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aFwCtcCtl1(pub u32);
    impl GdMergedFc02e261ca5aFwCtcCtl1 {
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl1_rlvalue(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ctc_ctl1_rlvalue(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl1_cklim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ctc_ctl1_cklim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl1_refpsc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ctc_ctl1_refpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl1_refsel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ctc_ctl1_refsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_ctl1_refpol(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_ctl1_refpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aFwCtcCtl1 {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aFwCtcCtl1 {
            GdMergedFc02e261ca5aFwCtcCtl1(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aFwCtcCtl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aFwCtcCtl1")
                .field("ctc_ctl1_rlvalue", &self.ctc_ctl1_rlvalue())
                .field("ctc_ctl1_cklim", &self.ctc_ctl1_cklim())
                .field("ctc_ctl1_refpsc", &self.ctc_ctl1_refpsc())
                .field("ctc_ctl1_refsel", &self.ctc_ctl1_refsel())
                .field("ctc_ctl1_refpol", &self.ctc_ctl1_refpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aFwCtcCtl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aFwCtcCtl1 {{ ctc_ctl1_rlvalue: {=u16:?}, ctc_ctl1_cklim: {=u8:?}, ctc_ctl1_refpsc: {=u8:?}, ctc_ctl1_refsel: {=u8:?}, ctc_ctl1_refpol: {=bool:?} }}",
                self.ctc_ctl1_rlvalue(),
                self.ctc_ctl1_cklim(),
                self.ctc_ctl1_refpsc(),
                self.ctc_ctl1_refsel(),
                self.ctc_ctl1_refpol()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aFwCtcIntc(pub u32);
    impl GdMergedFc02e261ca5aFwCtcIntc {
        #[must_use]
        #[inline(always)]
        pub const fn ctc_intc_ckokic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_intc_ckokic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_intc_ckwarnic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_intc_ckwarnic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_intc_erric(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_intc_erric(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_intc_erefic(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_intc_erefic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aFwCtcIntc {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aFwCtcIntc {
            GdMergedFc02e261ca5aFwCtcIntc(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aFwCtcIntc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aFwCtcIntc")
                .field("ctc_intc_ckokic", &self.ctc_intc_ckokic())
                .field("ctc_intc_ckwarnic", &self.ctc_intc_ckwarnic())
                .field("ctc_intc_erric", &self.ctc_intc_erric())
                .field("ctc_intc_erefic", &self.ctc_intc_erefic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aFwCtcIntc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aFwCtcIntc {{ ctc_intc_ckokic: {=bool:?}, ctc_intc_ckwarnic: {=bool:?}, ctc_intc_erric: {=bool:?}, ctc_intc_erefic: {=bool:?} }}",
                self.ctc_intc_ckokic(),
                self.ctc_intc_ckwarnic(),
                self.ctc_intc_erric(),
                self.ctc_intc_erefic()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aFwCtcStat(pub u32);
    impl GdMergedFc02e261ca5aFwCtcStat {
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_ckokif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_ckokif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_ckwarnif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_ckwarnif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_errif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_errif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_erefif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_erefif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_ckerr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_ckerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_refmiss(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_refmiss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_trimerr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_trimerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_refdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ctc_stat_refdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ctc_stat_refcap(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ctc_stat_refcap(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aFwCtcStat {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aFwCtcStat {
            GdMergedFc02e261ca5aFwCtcStat(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aFwCtcStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aFwCtcStat")
                .field("ctc_stat_ckokif", &self.ctc_stat_ckokif())
                .field("ctc_stat_ckwarnif", &self.ctc_stat_ckwarnif())
                .field("ctc_stat_errif", &self.ctc_stat_errif())
                .field("ctc_stat_erefif", &self.ctc_stat_erefif())
                .field("ctc_stat_ckerr", &self.ctc_stat_ckerr())
                .field("ctc_stat_refmiss", &self.ctc_stat_refmiss())
                .field("ctc_stat_trimerr", &self.ctc_stat_trimerr())
                .field("ctc_stat_refdir", &self.ctc_stat_refdir())
                .field("ctc_stat_refcap", &self.ctc_stat_refcap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aFwCtcStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aFwCtcStat {{ ctc_stat_ckokif: {=bool:?}, ctc_stat_ckwarnif: {=bool:?}, ctc_stat_errif: {=bool:?}, ctc_stat_erefif: {=bool:?}, ctc_stat_ckerr: {=bool:?}, ctc_stat_refmiss: {=bool:?}, ctc_stat_trimerr: {=bool:?}, ctc_stat_refdir: {=bool:?}, ctc_stat_refcap: {=u16:?} }}",
                self.ctc_stat_ckokif(),
                self.ctc_stat_ckwarnif(),
                self.ctc_stat_errif(),
                self.ctc_stat_erefif(),
                self.ctc_stat_ckerr(),
                self.ctc_stat_refmiss(),
                self.ctc_stat_trimerr(),
                self.ctc_stat_refdir(),
                self.ctc_stat_refcap()
            )
        }
    }
    #[doc = "Control register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aSvdCtl0(pub u32);
    impl GdMergedFc02e261ca5aSvdCtl0 {
        #[doc = "Clock trim ok interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ckokie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock trim ok interrupt enable"]
        #[inline(always)]
        pub const fn set_ckokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock trim warning interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ckwarnie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock trim warning interrupt enable"]
        #[inline(always)]
        pub const fn set_ckwarnie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EREFIF interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn erefie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EREFIF interrupt enable"]
        #[inline(always)]
        pub const fn set_erefie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CTC counter enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cnten(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CTC counter enable"]
        #[inline(always)]
        pub const fn set_cnten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hardware automatically trim mode"]
        #[must_use]
        #[inline(always)]
        pub const fn autotrim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware automatically trim mode"]
        #[inline(always)]
        pub const fn set_autotrim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Software reference source sync pulse"]
        #[must_use]
        #[inline(always)]
        pub const fn swrefpul(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Software reference source sync pulse"]
        #[inline(always)]
        pub const fn set_swrefpul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IRC48M trim value"]
        #[must_use]
        #[inline(always)]
        pub const fn trimvalue(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "IRC48M trim value"]
        #[inline(always)]
        pub const fn set_trimvalue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aSvdCtl0 {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aSvdCtl0 {
            GdMergedFc02e261ca5aSvdCtl0(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aSvdCtl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aSvdCtl0")
                .field("ckokie", &self.ckokie())
                .field("ckwarnie", &self.ckwarnie())
                .field("errie", &self.errie())
                .field("erefie", &self.erefie())
                .field("cnten", &self.cnten())
                .field("autotrim", &self.autotrim())
                .field("swrefpul", &self.swrefpul())
                .field("trimvalue", &self.trimvalue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aSvdCtl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aSvdCtl0 {{ ckokie: {=bool:?}, ckwarnie: {=bool:?}, errie: {=bool:?}, erefie: {=bool:?}, cnten: {=bool:?}, autotrim: {=bool:?}, swrefpul: {=bool:?}, trimvalue: {=u8:?} }}",
                self.ckokie(),
                self.ckwarnie(),
                self.errie(),
                self.erefie(),
                self.cnten(),
                self.autotrim(),
                self.swrefpul(),
                self.trimvalue()
            )
        }
    }
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aSvdCtl1(pub u32);
    impl GdMergedFc02e261ca5aSvdCtl1 {
        #[doc = "CTC counter reload value"]
        #[must_use]
        #[inline(always)]
        pub const fn rlvalue(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CTC counter reload value"]
        #[inline(always)]
        pub const fn set_rlvalue(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Clock trim base limit value"]
        #[must_use]
        #[inline(always)]
        pub const fn cklim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clock trim base limit value"]
        #[inline(always)]
        pub const fn set_cklim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Reference signal source prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn refpsc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Reference signal source prescaler"]
        #[inline(always)]
        pub const fn set_refpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Reference signal source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn refsel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "Reference signal source selection"]
        #[inline(always)]
        pub const fn set_refsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "Reference signal source polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn refpol(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reference signal source polarity"]
        #[inline(always)]
        pub const fn set_refpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aSvdCtl1 {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aSvdCtl1 {
            GdMergedFc02e261ca5aSvdCtl1(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aSvdCtl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aSvdCtl1")
                .field("rlvalue", &self.rlvalue())
                .field("cklim", &self.cklim())
                .field("refpsc", &self.refpsc())
                .field("refsel", &self.refsel())
                .field("refpol", &self.refpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aSvdCtl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aSvdCtl1 {{ rlvalue: {=u16:?}, cklim: {=u8:?}, refpsc: {=u8:?}, refsel: {=u8:?}, refpol: {=bool:?} }}",
                self.rlvalue(),
                self.cklim(),
                self.refpsc(),
                self.refsel(),
                self.refpol()
            )
        }
    }
    #[doc = "Interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aSvdIntc(pub u32);
    impl GdMergedFc02e261ca5aSvdIntc {
        #[doc = "CKOKIF interrupt clear bit"]
        #[must_use]
        #[inline(always)]
        pub const fn ckokic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CKOKIF interrupt clear bit"]
        #[inline(always)]
        pub const fn set_ckokic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CKWARNIF interrupt clear bit"]
        #[must_use]
        #[inline(always)]
        pub const fn ckwarnic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CKWARNIF interrupt clear bit"]
        #[inline(always)]
        pub const fn set_ckwarnic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ERRIF interrupt clear bit"]
        #[must_use]
        #[inline(always)]
        pub const fn erric(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERRIF interrupt clear bit"]
        #[inline(always)]
        pub const fn set_erric(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EREFIF interrupt clear bit"]
        #[must_use]
        #[inline(always)]
        pub const fn erefic(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EREFIF interrupt clear bit"]
        #[inline(always)]
        pub const fn set_erefic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aSvdIntc {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aSvdIntc {
            GdMergedFc02e261ca5aSvdIntc(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aSvdIntc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aSvdIntc")
                .field("ckokic", &self.ckokic())
                .field("ckwarnic", &self.ckwarnic())
                .field("erric", &self.erric())
                .field("erefic", &self.erefic())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aSvdIntc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aSvdIntc {{ ckokic: {=bool:?}, ckwarnic: {=bool:?}, erric: {=bool:?}, erefic: {=bool:?} }}",
                self.ckokic(),
                self.ckwarnic(),
                self.erric(),
                self.erefic()
            )
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMergedFc02e261ca5aSvdStat(pub u32);
    impl GdMergedFc02e261ca5aSvdStat {
        #[doc = "Clock trim OK interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ckokif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock trim OK interrupt flag"]
        #[inline(always)]
        pub const fn set_ckokif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock trim warning interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ckwarnif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock trim warning interrupt flag"]
        #[inline(always)]
        pub const fn set_ckwarnif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Error interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn errif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt flag"]
        #[inline(always)]
        pub const fn set_errif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Expect reference interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn erefif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Expect reference interrupt flag"]
        #[inline(always)]
        pub const fn set_erefif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock trim error bit"]
        #[must_use]
        #[inline(always)]
        pub const fn ckerr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock trim error bit"]
        #[inline(always)]
        pub const fn set_ckerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Reference sync pulse miss"]
        #[must_use]
        #[inline(always)]
        pub const fn refmiss(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Reference sync pulse miss"]
        #[inline(always)]
        pub const fn set_refmiss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Trim value error bit"]
        #[must_use]
        #[inline(always)]
        pub const fn trimerr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Trim value error bit"]
        #[inline(always)]
        pub const fn set_trimerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CTC trim counter direction when reference sync pulse"]
        #[must_use]
        #[inline(always)]
        pub const fn refdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CTC trim counter direction when reference sync pulse"]
        #[inline(always)]
        pub const fn set_refdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CTC counter capture when reference sync pulse"]
        #[must_use]
        #[inline(always)]
        pub const fn refcap(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "CTC counter capture when reference sync pulse"]
        #[inline(always)]
        pub const fn set_refcap(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for GdMergedFc02e261ca5aSvdStat {
        #[inline(always)]
        fn default() -> GdMergedFc02e261ca5aSvdStat {
            GdMergedFc02e261ca5aSvdStat(0)
        }
    }
    impl core::fmt::Debug for GdMergedFc02e261ca5aSvdStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMergedFc02e261ca5aSvdStat")
                .field("ckokif", &self.ckokif())
                .field("ckwarnif", &self.ckwarnif())
                .field("errif", &self.errif())
                .field("erefif", &self.erefif())
                .field("ckerr", &self.ckerr())
                .field("refmiss", &self.refmiss())
                .field("trimerr", &self.trimerr())
                .field("refdir", &self.refdir())
                .field("refcap", &self.refcap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMergedFc02e261ca5aSvdStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMergedFc02e261ca5aSvdStat {{ ckokif: {=bool:?}, ckwarnif: {=bool:?}, errif: {=bool:?}, erefif: {=bool:?}, ckerr: {=bool:?}, refmiss: {=bool:?}, trimerr: {=bool:?}, refdir: {=bool:?}, refcap: {=u16:?} }}",
                self.ckokif(),
                self.ckwarnif(),
                self.errif(),
                self.erefif(),
                self.ckerr(),
                self.refmiss(),
                self.trimerr(),
                self.refdir(),
                self.refcap()
            )
        }
    }
}
