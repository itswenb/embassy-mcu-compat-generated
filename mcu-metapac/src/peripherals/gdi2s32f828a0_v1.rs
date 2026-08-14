#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [derive (Copy , Clone , Eq , PartialEq)]
pub struct I2s { ptr : * mut u8 } unsafe impl Send for I2s { } unsafe impl Sync for I2s { } impl I2s { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn i2s_add_ctl0 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } # [inline (always)]
pub const fn i2s_add_ctl1 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x04usize) as _) } } # [inline (always)]
pub const fn i2s_add_stat (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x08usize) as _) } } # [inline (always)]
pub const fn i2s_add_data (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0cusize) as _) } } # [inline (always)]
pub const fn i2s_add_crcpoly (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x10usize) as _) } } # [inline (always)]
pub const fn i2s_add_rcrc (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x14usize) as _) } } # [inline (always)]
pub const fn i2s_add_tcrc (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x18usize) as _) } } # [inline (always)]
pub const fn i2s_add_i2sctl (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x1cusize) as _) } } # [inline (always)]
pub const fn i2s_add_i2spsc (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x20usize) as _) } } }