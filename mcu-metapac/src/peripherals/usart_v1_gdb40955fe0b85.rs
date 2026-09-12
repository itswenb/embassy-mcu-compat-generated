#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(self) -> crate::common::Reg<regs::Gtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart {
    ptr: *mut u8,
}
unsafe impl Send for Usart {}
unsafe impl Sync for Usart {}
impl Usart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Usart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3Usart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(self) -> crate::common::Reg<regs::Gtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn ctl3(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dSvdCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn usart_ctl3(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dFwUsartCtl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Receiver timeout register"]
    #[inline(always)]
    pub const fn rt(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dSvdRt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn usart_rt(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dFwUsartRt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Status register 1"]
    #[inline(always)]
    pub const fn stat1(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dSvdStat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn usart_stat1(self) -> crate::common::Reg<regs::GdMerged7c2777bcc34dFwUsartStat1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
}
pub mod regs {
    #[doc = "Baud rate register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "USARTDIV"]
        #[must_use]
        #[inline(always)]
        pub const fn brr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "USARTDIV"]
        #[inline(always)]
        pub const fn set_brr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr").field("brr", &self.brr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Brr {{ brr: {=u16:?} }}", self.brr())
        }
    }
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Send break"]
        #[must_use]
        #[inline(always)]
        pub const fn sbk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send break"]
        #[inline(always)]
        pub const fn set_sbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver wakeup"]
        #[must_use]
        #[inline(always)]
        pub const fn rwu(&self) -> super::vals::Rwu {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Rwu::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup"]
        #[inline(always)]
        pub const fn set_rwu(&mut self, val: super::vals::Rwu) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable"]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable"]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn idleie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE interrupt enable"]
        #[inline(always)]
        pub const fn set_idleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXNE interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete interrupt enable"]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TXE interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TXE interrupt enable"]
        #[inline(always)]
        pub const fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PE interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn peie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PE interrupt enable"]
        #[inline(always)]
        pub const fn set_peie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Parity selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> super::vals::Ps {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Ps::from_bits(val as u8)
        }
        #[doc = "Parity selection"]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: super::vals::Ps) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Parity control enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pce(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Parity control enable"]
        #[inline(always)]
        pub const fn set_pce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver wakeup method"]
        #[must_use]
        #[inline(always)]
        pub const fn wake(&self) -> super::vals::Wake {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wake::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup method"]
        #[inline(always)]
        pub const fn set_wake(&mut self, val: super::vals::Wake) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Word length"]
        #[must_use]
        #[inline(always)]
        pub const fn m0(&self) -> super::vals::M0 {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::M0::from_bits(val as u8)
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub const fn set_m0(&mut self, val: super::vals::M0) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "USART enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable"]
        #[inline(always)]
        pub const fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("sbk", &self.sbk())
                .field("rwu", &self.rwu())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("idleie", &self.idleie())
                .field("rxneie", &self.rxneie())
                .field("tcie", &self.tcie())
                .field("txeie", &self.txeie())
                .field("peie", &self.peie())
                .field("ps", &self.ps())
                .field("pce", &self.pce())
                .field("wake", &self.wake())
                .field("m0", &self.m0())
                .field("ue", &self.ue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ sbk: {=bool:?}, rwu: {:?}, re: {=bool:?}, te: {=bool:?}, idleie: {=bool:?}, rxneie: {=bool:?}, tcie: {=bool:?}, txeie: {=bool:?}, peie: {=bool:?}, ps: {:?}, pce: {=bool:?}, wake: {:?}, m0: {:?}, ue: {=bool:?} }}",
                self.sbk(),
                self.rwu(),
                self.re(),
                self.te(),
                self.idleie(),
                self.rxneie(),
                self.tcie(),
                self.txeie(),
                self.peie(),
                self.ps(),
                self.pce(),
                self.wake(),
                self.m0(),
                self.ue()
            )
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Address of the USART node"]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Line break detection length"]
        #[must_use]
        #[inline(always)]
        pub const fn lbdl(&self) -> super::vals::Lbdl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lbdl::from_bits(val as u8)
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub const fn set_lbdl(&mut self, val: super::vals::Lbdl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub const fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "STOP bits"]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub const fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("add", &self.add())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ add: {=u8:?}, lbdl: {:?}, lbdie: {=bool:?}, stop: {:?}, linen: {=bool:?} }}",
                self.add(),
                self.lbdl(),
                self.lbdie(),
                self.stop(),
                self.linen()
            )
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Usart(pub u32);
    impl Cr2Usart {
        #[doc = "Address of the USART node"]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Line break detection length"]
        #[must_use]
        #[inline(always)]
        pub const fn lbdl(&self) -> super::vals::Lbdl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lbdl::from_bits(val as u8)
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub const fn set_lbdl(&mut self, val: super::vals::Lbdl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub const fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last bit clock pulse"]
        #[must_use]
        #[inline(always)]
        pub const fn lbcl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last bit clock pulse"]
        #[inline(always)]
        pub const fn set_lbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock phase"]
        #[must_use]
        #[inline(always)]
        pub const fn cpha(&self) -> super::vals::Cpha {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Cpha::from_bits(val as u8)
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock polarity"]
        #[must_use]
        #[inline(always)]
        pub const fn cpol(&self) -> super::vals::Cpol {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Cpol::from_bits(val as u8)
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable"]
        #[inline(always)]
        pub const fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP bits"]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub const fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr2Usart {
        #[inline(always)]
        fn default() -> Cr2Usart {
            Cr2Usart(0)
        }
    }
    impl core::fmt::Debug for Cr2Usart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Usart")
                .field("add", &self.add())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("lbcl", &self.lbcl())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("clken", &self.clken())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Usart {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2Usart {{ add: {=u8:?}, lbdl: {:?}, lbdie: {=bool:?}, lbcl: {=bool:?}, cpha: {:?}, cpol: {:?}, clken: {=bool:?}, stop: {:?}, linen: {=bool:?} }}",
                self.add(),
                self.lbdl(),
                self.lbdie(),
                self.lbcl(),
                self.cpha(),
                self.cpol(),
                self.clken(),
                self.stop(),
                self.linen()
            )
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Error interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub const fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power"]
        #[must_use]
        #[inline(always)]
        pub const fn irlp(&self) -> super::vals::Irlp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Irlp::from_bits(val as u8)
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub const fn set_irlp(&mut self, val: super::vals::Irlp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection"]
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA enable receiver"]
        #[must_use]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub const fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter"]
        #[must_use]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub const fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ eie: {=bool:?}, iren: {=bool:?}, irlp: {:?}, hdsel: {=bool:?}, dmar: {=bool:?}, dmat: {=bool:?} }}",
                self.eie(),
                self.iren(),
                self.irlp(),
                self.hdsel(),
                self.dmar(),
                self.dmat()
            )
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3Usart(pub u32);
    impl Cr3Usart {
        #[doc = "Error interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub const fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power"]
        #[must_use]
        #[inline(always)]
        pub const fn irlp(&self) -> super::vals::Irlp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Irlp::from_bits(val as u8)
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub const fn set_irlp(&mut self, val: super::vals::Irlp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection"]
        #[must_use]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub const fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Smartcard NACK enable"]
        #[must_use]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard NACK enable"]
        #[inline(always)]
        pub const fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Smartcard mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn scen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard mode enable"]
        #[inline(always)]
        pub const fn set_scen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA enable receiver"]
        #[must_use]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub const fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter"]
        #[must_use]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub const fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTS enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTS enable"]
        #[inline(always)]
        pub const fn set_rtse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ctse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS enable"]
        #[inline(always)]
        pub const fn set_ctse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ctsie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt enable"]
        #[inline(always)]
        pub const fn set_ctsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr3Usart {
        #[inline(always)]
        fn default() -> Cr3Usart {
            Cr3Usart(0)
        }
    }
    impl core::fmt::Debug for Cr3Usart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3Usart")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("nack", &self.nack())
                .field("scen", &self.scen())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .field("rtse", &self.rtse())
                .field("ctse", &self.ctse())
                .field("ctsie", &self.ctsie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3Usart {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3Usart {{ eie: {=bool:?}, iren: {=bool:?}, irlp: {:?}, hdsel: {=bool:?}, nack: {=bool:?}, scen: {=bool:?}, dmar: {=bool:?}, dmat: {=bool:?}, rtse: {=bool:?}, ctse: {=bool:?}, ctsie: {=bool:?} }}",
                self.eie(),
                self.iren(),
                self.irlp(),
                self.hdsel(),
                self.nack(),
                self.scen(),
                self.dmar(),
                self.dmat(),
                self.rtse(),
                self.ctse(),
                self.ctsie()
            )
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data value"]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data value"]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ dr: {=u16:?} }}", self.dr())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dFwUsartCtl3(pub u32);
    impl GdMerged7c2777bcc34dFwUsartCtl3 {
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_rten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_rten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_scrtnum(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_scrtnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_rtie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_rtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_ebie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_ebie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_rinv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_rinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_tinv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_tinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_dinv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_dinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_ctl3_msbf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_ctl3_msbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dFwUsartCtl3 {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dFwUsartCtl3 {
            GdMerged7c2777bcc34dFwUsartCtl3(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dFwUsartCtl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dFwUsartCtl3")
                .field("usart_ctl3_rten", &self.usart_ctl3_rten())
                .field("usart_ctl3_scrtnum", &self.usart_ctl3_scrtnum())
                .field("usart_ctl3_rtie", &self.usart_ctl3_rtie())
                .field("usart_ctl3_ebie", &self.usart_ctl3_ebie())
                .field("usart_ctl3_rinv", &self.usart_ctl3_rinv())
                .field("usart_ctl3_tinv", &self.usart_ctl3_tinv())
                .field("usart_ctl3_dinv", &self.usart_ctl3_dinv())
                .field("usart_ctl3_msbf", &self.usart_ctl3_msbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dFwUsartCtl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dFwUsartCtl3 {{ usart_ctl3_rten: {=bool:?}, usart_ctl3_scrtnum: {=u8:?}, usart_ctl3_rtie: {=bool:?}, usart_ctl3_ebie: {=bool:?}, usart_ctl3_rinv: {=bool:?}, usart_ctl3_tinv: {=bool:?}, usart_ctl3_dinv: {=bool:?}, usart_ctl3_msbf: {=bool:?} }}",
                self.usart_ctl3_rten(),
                self.usart_ctl3_scrtnum(),
                self.usart_ctl3_rtie(),
                self.usart_ctl3_ebie(),
                self.usart_ctl3_rinv(),
                self.usart_ctl3_tinv(),
                self.usart_ctl3_dinv(),
                self.usart_ctl3_msbf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dFwUsartRt(pub u32);
    impl GdMerged7c2777bcc34dFwUsartRt {
        #[must_use]
        #[inline(always)]
        pub const fn usart_rt_rt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_usart_rt_rt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_rt_bl(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_usart_rt_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dFwUsartRt {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dFwUsartRt {
            GdMerged7c2777bcc34dFwUsartRt(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dFwUsartRt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dFwUsartRt")
                .field("usart_rt_rt", &self.usart_rt_rt())
                .field("usart_rt_bl", &self.usart_rt_bl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dFwUsartRt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dFwUsartRt {{ usart_rt_rt: {=u32:?}, usart_rt_bl: {=u8:?} }}",
                self.usart_rt_rt(),
                self.usart_rt_bl()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dFwUsartStat1(pub u32);
    impl GdMerged7c2777bcc34dFwUsartStat1 {
        #[must_use]
        #[inline(always)]
        pub const fn usart_stat1_rtf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_stat1_rtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_stat1_ebf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_stat1_ebf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn usart_stat1_bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_usart_stat1_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dFwUsartStat1 {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dFwUsartStat1 {
            GdMerged7c2777bcc34dFwUsartStat1(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dFwUsartStat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dFwUsartStat1")
                .field("usart_stat1_rtf", &self.usart_stat1_rtf())
                .field("usart_stat1_ebf", &self.usart_stat1_ebf())
                .field("usart_stat1_bsy", &self.usart_stat1_bsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dFwUsartStat1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dFwUsartStat1 {{ usart_stat1_rtf: {=bool:?}, usart_stat1_ebf: {=bool:?}, usart_stat1_bsy: {=bool:?} }}",
                self.usart_stat1_rtf(),
                self.usart_stat1_ebf(),
                self.usart_stat1_bsy()
            )
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dSvdCtl3(pub u32);
    impl GdMerged7c2777bcc34dSvdCtl3 {
        #[doc = "Receiver timeout enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout enable"]
        #[inline(always)]
        pub const fn set_rten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Smartcard auto-retry number"]
        #[must_use]
        #[inline(always)]
        pub const fn scrtnum(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Smartcard auto-retry number"]
        #[inline(always)]
        pub const fn set_scrtnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "Interrupt enable bit of receive timeout event"]
        #[must_use]
        #[inline(always)]
        pub const fn rtie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable bit of receive timeout event"]
        #[inline(always)]
        pub const fn set_rtie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt enable bit of end of block event"]
        #[must_use]
        #[inline(always)]
        pub const fn ebie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable bit of end of block event"]
        #[inline(always)]
        pub const fn set_ebie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX pin level inversion"]
        #[must_use]
        #[inline(always)]
        pub const fn rinv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RX pin level inversion"]
        #[inline(always)]
        pub const fn set_rinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TX pin level inversion"]
        #[must_use]
        #[inline(always)]
        pub const fn tinv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TX pin level inversion"]
        #[inline(always)]
        pub const fn set_tinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data bit level inversion"]
        #[must_use]
        #[inline(always)]
        pub const fn dinv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data bit level inversion"]
        #[inline(always)]
        pub const fn set_dinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Most significant bit first"]
        #[must_use]
        #[inline(always)]
        pub const fn msbf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Most significant bit first"]
        #[inline(always)]
        pub const fn set_msbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dSvdCtl3 {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dSvdCtl3 {
            GdMerged7c2777bcc34dSvdCtl3(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dSvdCtl3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dSvdCtl3")
                .field("rten", &self.rten())
                .field("scrtnum", &self.scrtnum())
                .field("rtie", &self.rtie())
                .field("ebie", &self.ebie())
                .field("rinv", &self.rinv())
                .field("tinv", &self.tinv())
                .field("dinv", &self.dinv())
                .field("msbf", &self.msbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dSvdCtl3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dSvdCtl3 {{ rten: {=bool:?}, scrtnum: {=u8:?}, rtie: {=bool:?}, ebie: {=bool:?}, rinv: {=bool:?}, tinv: {=bool:?}, dinv: {=bool:?}, msbf: {=bool:?} }}",
                self.rten(),
                self.scrtnum(),
                self.rtie(),
                self.ebie(),
                self.rinv(),
                self.tinv(),
                self.dinv(),
                self.msbf()
            )
        }
    }
    #[doc = "Receiver timeout register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dSvdRt(pub u32);
    impl GdMerged7c2777bcc34dSvdRt {
        #[doc = "Receiver timeout threshold"]
        #[must_use]
        #[inline(always)]
        pub const fn rt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Receiver timeout threshold"]
        #[inline(always)]
        pub const fn set_rt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Block Length"]
        #[must_use]
        #[inline(always)]
        pub const fn bl(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Block Length"]
        #[inline(always)]
        pub const fn set_bl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dSvdRt {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dSvdRt {
            GdMerged7c2777bcc34dSvdRt(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dSvdRt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dSvdRt")
                .field("rt", &self.rt())
                .field("bl", &self.bl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dSvdRt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dSvdRt {{ rt: {=u32:?}, bl: {=u8:?} }}",
                self.rt(),
                self.bl()
            )
        }
    }
    #[doc = "Status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GdMerged7c2777bcc34dSvdStat1(pub u32);
    impl GdMerged7c2777bcc34dSvdStat1 {
        #[doc = "Receiver timeout flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rtf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver timeout flag"]
        #[inline(always)]
        pub const fn set_rtf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "End of block flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ebf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "End of block flag"]
        #[inline(always)]
        pub const fn set_ebf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Busy flag"]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for GdMerged7c2777bcc34dSvdStat1 {
        #[inline(always)]
        fn default() -> GdMerged7c2777bcc34dSvdStat1 {
            GdMerged7c2777bcc34dSvdStat1(0)
        }
    }
    impl core::fmt::Debug for GdMerged7c2777bcc34dSvdStat1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GdMerged7c2777bcc34dSvdStat1")
                .field("rtf", &self.rtf())
                .field("ebf", &self.ebf())
                .field("bsy", &self.bsy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GdMerged7c2777bcc34dSvdStat1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GdMerged7c2777bcc34dSvdStat1 {{ rtf: {=bool:?}, ebf: {=bool:?}, bsy: {=bool:?} }}",
                self.rtf(),
                self.ebf(),
                self.bsy()
            )
        }
    }
    #[doc = "Guard time and prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtpr(pub u32);
    impl Gtpr {
        #[doc = "Prescaler value"]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Guard time value"]
        #[must_use]
        #[inline(always)]
        pub const fn gt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Guard time value"]
        #[inline(always)]
        pub const fn set_gt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Gtpr {
        #[inline(always)]
        fn default() -> Gtpr {
            Gtpr(0)
        }
    }
    impl core::fmt::Debug for Gtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtpr")
                .field("psc", &self.psc())
                .field("gt", &self.gt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gtpr {{ psc: {=u8:?}, gt: {=u8:?} }}", self.psc(), self.gt())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Parity error"]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error"]
        #[must_use]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub const fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise error flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise error flag"]
        #[inline(always)]
        pub const fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error"]
        #[must_use]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub const fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected"]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected"]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read data register not empty"]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete"]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete"]
        #[inline(always)]
        pub const fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmit data register empty"]
        #[must_use]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty"]
        #[inline(always)]
        pub const fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LIN break detection flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lbd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection flag"]
        #[inline(always)]
        pub const fn set_lbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS flag"]
        #[inline(always)]
        pub const fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("idle", &self.idle())
                .field("rxne", &self.rxne())
                .field("tc", &self.tc())
                .field("txe", &self.txe())
                .field("lbd", &self.lbd())
                .field("cts", &self.cts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ pe: {=bool:?}, fe: {=bool:?}, ne: {=bool:?}, ore: {=bool:?}, idle: {=bool:?}, rxne: {=bool:?}, tc: {=bool:?}, txe: {=bool:?}, lbd: {=bool:?}, cts: {=bool:?} }}",
                self.pe(),
                self.fe(),
                self.ne(),
                self.ore(),
                self.idle(),
                self.rxne(),
                self.tc(),
                self.txe(),
                self.lbd(),
                self.cts()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        First = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        Second = 0x01,
    }
    impl Cpha {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpha {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpha {
        #[inline(always)]
        fn from(val: u8) -> Cpha {
            Cpha::from_bits(val)
        }
    }
    impl From<Cpha> for u8 {
        #[inline(always)]
        fn from(val: Cpha) -> u8 {
            Cpha::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpol {
        #[doc = "Steady low value on CK pin outside transmission window"]
        Low = 0x0,
        #[doc = "Steady high value on CK pin outside transmission window"]
        High = 0x01,
    }
    impl Cpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpol {
        #[inline(always)]
        fn from(val: u8) -> Cpol {
            Cpol::from_bits(val)
        }
    }
    impl From<Cpol> for u8 {
        #[inline(always)]
        fn from(val: Cpol) -> u8 {
            Cpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Irlp {
        #[doc = "Normal mode"]
        Normal = 0x0,
        #[doc = "Low-power mode"]
        LowPower = 0x01,
    }
    impl Irlp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Irlp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Irlp {
        #[inline(always)]
        fn from(val: u8) -> Irlp {
            Irlp::from_bits(val)
        }
    }
    impl From<Irlp> for u8 {
        #[inline(always)]
        fn from(val: Irlp) -> u8 {
            Irlp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lbdl {
        #[doc = "10-bit break detection"]
        Bit10 = 0x0,
        #[doc = "11-bit break detection"]
        Bit11 = 0x01,
    }
    impl Lbdl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lbdl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lbdl {
        #[inline(always)]
        fn from(val: u8) -> Lbdl {
            Lbdl::from_bits(val)
        }
    }
    impl From<Lbdl> for u8 {
        #[inline(always)]
        fn from(val: Lbdl) -> u8 {
            Lbdl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum M0 {
        #[doc = "1 start bit, 8 data bits, n stop bits"]
        Bit8 = 0x0,
        #[doc = "1 start bit, 9 data bits, n stop bits"]
        Bit9 = 0x01,
    }
    impl M0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> M0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for M0 {
        #[inline(always)]
        fn from(val: u8) -> M0 {
            M0::from_bits(val)
        }
    }
    impl From<M0> for u8 {
        #[inline(always)]
        fn from(val: M0) -> u8 {
            M0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ps {
        #[doc = "Even parity"]
        Even = 0x0,
        #[doc = "Odd parity"]
        Odd = 0x01,
    }
    impl Ps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ps {
        #[inline(always)]
        fn from(val: u8) -> Ps {
            Ps::from_bits(val)
        }
    }
    impl From<Ps> for u8 {
        #[inline(always)]
        fn from(val: Ps) -> u8 {
            Ps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rwu {
        #[doc = "Receiver in active mode"]
        Active = 0x0,
        #[doc = "Receiver in mute mode"]
        Mute = 0x01,
    }
    impl Rwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rwu {
        #[inline(always)]
        fn from(val: u8) -> Rwu {
            Rwu::from_bits(val)
        }
    }
    impl From<Rwu> for u8 {
        #[inline(always)]
        fn from(val: Rwu) -> u8 {
            Rwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stop {
        #[doc = "1 stop bit"]
        Stop1 = 0x0,
        #[doc = "0.5 stop bits"]
        Stop0p5 = 0x01,
        #[doc = "2 stop bits"]
        Stop2 = 0x02,
        #[doc = "1.5 stop bits"]
        Stop1p5 = 0x03,
    }
    impl Stop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stop {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stop {
        #[inline(always)]
        fn from(val: u8) -> Stop {
            Stop::from_bits(val)
        }
    }
    impl From<Stop> for u8 {
        #[inline(always)]
        fn from(val: Stop) -> u8 {
            Stop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wake {
        #[doc = "USART wakeup on idle line"]
        IdleLine = 0x0,
        #[doc = "USART wakeup on address mark"]
        AddressMark = 0x01,
    }
    impl Wake {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wake {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wake {
        #[inline(always)]
        fn from(val: u8) -> Wake {
            Wake::from_bits(val)
        }
    }
    impl From<Wake> for u8 {
        #[inline(always)]
        fn from(val: Wake) -> u8 {
            Wake::to_bits(val)
        }
    }
}
