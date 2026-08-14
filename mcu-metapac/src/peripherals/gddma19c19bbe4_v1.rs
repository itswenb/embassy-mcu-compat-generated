#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]




# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Dma1 { ptr : * mut u8 } unsafe impl Send for Dma1 { } unsafe impl Sync for Dma1 { } impl Dma1 { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [inline (always)]
pub const fn dma_acfg (self) -> crate :: common :: Reg < regs :: DmaAcfg , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . wrapping_add (0x0300usize) as _) } } } pub mod regs { # [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct DmaAcfg (pub u32) ; impl DmaAcfg { # [must_use]
# [inline (always)]
pub const fn dma_acfg_fd_ch5en (& self) -> bool { let val = (self . 0 >> 5usize) & 0x01 ; val != 0 } # [inline (always)]
pub const fn set_dma_acfg_fd_ch5en (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize) ; } } impl Default for DmaAcfg { # [inline (always)]
fn default () -> DmaAcfg { DmaAcfg (0) } } impl core :: fmt :: Debug for DmaAcfg { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_struct ("DmaAcfg") . field ("dma_acfg_fd_ch5en" , & self . dma_acfg_fd_ch5en ()) . finish () } } # [cfg (feature = "defmt")]
impl defmt :: Format for DmaAcfg { fn format (& self , f : defmt :: Formatter) { defmt :: write ! (f , "DmaAcfg {{ dma_acfg_fd_ch5en: {=bool:?} }}" , self . dma_acfg_fd_ch5en ()) } } }