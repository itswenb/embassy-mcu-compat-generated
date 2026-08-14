#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ob { ptr : * mut u8 } unsafe impl Send for Ob { } unsafe impl Sync for Ob { } impl Ob { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn ob_wp1_0 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x08usize) as _) } } # [inline (always)]
pub const fn ob_wp1_1 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x09usize) as _) } } # [inline (always)]
pub const fn ob_wp1_2 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0cusize) as _) } } # [inline (always)]
pub const fn ob_user (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0001_0000usize) as _) } } # [inline (always)]
pub const fn ob_spc (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0001_0001usize) as _) } } # [inline (always)]
pub const fn ob_wp0_0 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0001_0008usize) as _) } } # [inline (always)]
pub const fn ob_wp0_1 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0001_0009usize) as _) } } # [inline (always)]
pub const fn ob_wp0_2 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0001_000cusize) as _) } } }