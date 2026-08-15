#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]

# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cmp { ptr : * mut u8 } unsafe impl Send for Cmp { } unsafe impl Sync for Cmp { } impl Cmp { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn cmp0_cs (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0usize) as _) } } # [inline (always)]
pub const fn cmp1_cs (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x04usize) as _) } } }