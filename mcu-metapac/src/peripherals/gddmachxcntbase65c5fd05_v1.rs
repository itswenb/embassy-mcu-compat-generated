#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]

# [derive (Copy , Clone , Eq , PartialEq)]
pub struct DmaChxcntBase { ptr : * mut u8 } unsafe impl Send for DmaChxcntBase { } unsafe impl Sync for DmaChxcntBase { } impl DmaChxcntBase { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn dma_chcnt_0 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0usize) as _) } } # [inline (always)]
pub const fn dma_chcnt_1 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x14usize) as _) } } # [inline (always)]
pub const fn dma_chcnt_2 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x28usize) as _) } } }