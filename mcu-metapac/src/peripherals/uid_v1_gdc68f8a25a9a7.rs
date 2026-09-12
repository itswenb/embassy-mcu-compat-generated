#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GD32 工厂写入的唯一设备标识"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdUidFe175d2bf68b {
    ptr: *mut u8,
}
unsafe impl Send for GdUidFe175d2bf68b {}
unsafe impl Sync for GdUidFe175d2bf68b {}
impl GdUidFe175d2bf68b {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn uid(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
