#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]

# [derive (Copy , Clone , Eq , PartialEq)]
pub struct DmaChxctlBase { ptr : * mut u8 } unsafe impl Send for DmaChxctlBase { } unsafe impl Sync for DmaChxctlBase { } impl DmaChxctlBase { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn dma_chctl_0 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0usize) as _) } } # [inline (always)]
pub const fn dma_chctl_1 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x14usize) as _) } } # [inline (always)]
pub const fn dma_chctl_2 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x28usize) as _) } } # [inline (always)]
pub const fn dma_chctl_3 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x3cusize) as _) } } # [inline (always)]
pub const fn dma_chctl_4 (self) -> crate :: common :: Reg < u32 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x50usize) as _) } } }