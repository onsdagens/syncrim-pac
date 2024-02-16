#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mtime_lo: MTIME_LO,
    mtime_hi: MTIME_HI,
    mtime_comp_lo: MTIME_COMP_LO,
    mtime_comp_hi: MTIME_COMP_HI,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Low 32 bits"]
    #[inline(always)]
    pub const fn mtime_lo(&self) -> &MTIME_LO {
        &self.mtime_lo
    }
    #[doc = "0x04 - Timer Hi 32 bits"]
    #[inline(always)]
    pub const fn mtime_hi(&self) -> &MTIME_HI {
        &self.mtime_hi
    }
    #[doc = "0x08 - Timer Comp Low 32 bits"]
    #[inline(always)]
    pub const fn mtime_comp_lo(&self) -> &MTIME_COMP_LO {
        &self.mtime_comp_lo
    }
    #[doc = "0x0c - Timer Comp Hi 32 bits"]
    #[inline(always)]
    pub const fn mtime_comp_hi(&self) -> &MTIME_COMP_HI {
        &self.mtime_comp_hi
    }
}
#[doc = "MTIME_LO (r) register accessor: Timer Low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_lo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_lo`]
module"]
pub type MTIME_LO = crate::Reg<mtime_lo::MTIME_LO_SPEC>;
#[doc = "Timer Low 32 bits"]
pub mod mtime_lo;
#[doc = "MTIME_HI (r) register accessor: Timer Hi 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_hi::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_hi`]
module"]
pub type MTIME_HI = crate::Reg<mtime_hi::MTIME_HI_SPEC>;
#[doc = "Timer Hi 32 bits"]
pub mod mtime_hi;
#[doc = "MTIME_COMP_LO (rw) register accessor: Timer Comp Low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_comp_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime_comp_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_comp_lo`]
module"]
pub type MTIME_COMP_LO = crate::Reg<mtime_comp_lo::MTIME_COMP_LO_SPEC>;
#[doc = "Timer Comp Low 32 bits"]
pub mod mtime_comp_lo;
#[doc = "MTIME_COMP_HI (rw) register accessor: Timer Comp Hi 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_comp_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime_comp_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime_comp_hi`]
module"]
pub type MTIME_COMP_HI = crate::Reg<mtime_comp_hi::MTIME_COMP_HI_SPEC>;
#[doc = "Timer Comp Hi 32 bits"]
pub mod mtime_comp_hi;
