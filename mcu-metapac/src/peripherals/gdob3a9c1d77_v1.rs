#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ob { ptr : * mut u8 } unsafe impl Send for Ob { } unsafe impl Sync for Ob { } impl Ob { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn ob_spc (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } # [inline (always)]
pub const fn op_byte_0 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } # [inline (always)]
pub const fn ob_user (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x02usize) as _) } } # [inline (always)]
pub const fn op_byte_1 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x02usize) as _) } } # [inline (always)]
pub const fn ob_data0 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x04usize) as _) } } # [inline (always)]
pub const fn op_byte_2 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x04usize) as _) } } # [inline (always)]
pub const fn ob_data1 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x06usize) as _) } } # [inline (always)]
pub const fn op_byte_3 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x06usize) as _) } } # [inline (always)]
pub const fn ob_wp0 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x08usize) as _) } } # [inline (always)]
pub const fn op_byte_4 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x08usize) as _) } } # [inline (always)]
pub const fn ob_wp1 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0ausize) as _) } } # [inline (always)]
pub const fn op_byte_5 (self) -> crate :: common :: Reg < u8 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0ausize) as _) } } }