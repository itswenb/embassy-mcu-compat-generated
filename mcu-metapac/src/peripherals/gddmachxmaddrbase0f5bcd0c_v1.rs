#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [derive (Copy , Clone , Eq , PartialEq)]
pub struct DmaChxmaddrBase { ptr : * mut u8 } unsafe impl Send for DmaChxmaddrBase { } unsafe impl Sync for DmaChxmaddrBase { } impl DmaChxmaddrBase { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn dma_chmaddr_0 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0usize) as _) } } # [inline (always)]
pub const fn dma_chmaddr_1 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x14usize) as _) } } # [inline (always)]
pub const fn dma_chmaddr_2 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x28usize) as _) } } # [inline (always)]
pub const fn dma_chmaddr_3 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x3cusize) as _) } } # [inline (always)]
pub const fn dma_chmaddr_4 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x50usize) as _) } } }