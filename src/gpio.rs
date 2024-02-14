#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    enable: ENABLE,
    config: CONFIG,
    interrupt_enable: INTERRUPT_ENABLE,
    set: SET,
    clear: CLEAR,
    toggle: TOGGLE,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Pin Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x04 - GPIO Pin I/O Config"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x08 - GPIO Pin Interrupt Enable"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &INTERRUPT_ENABLE {
        &self.interrupt_enable
    }
    #[doc = "0x0c - GPIO Pin Set"]
    #[inline(always)]
    pub const fn set(&self) -> &SET {
        &self.set
    }
    #[doc = "0x10 - GPIO Pin Clear"]
    #[inline(always)]
    pub const fn clear(&self) -> &CLEAR {
        &self.clear
    }
    #[doc = "0x14 - GPIO Pin Toggle"]
    #[inline(always)]
    pub const fn toggle(&self) -> &TOGGLE {
        &self.toggle
    }
}
#[doc = "ENABLE (rw) register accessor: GPIO Pin Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO Pin Enable"]
pub mod enable;
#[doc = "CONFIG (rw) register accessor: GPIO Pin I/O Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "GPIO Pin I/O Config"]
pub mod config;
#[doc = "INTERRUPT_ENABLE (rw) register accessor: GPIO Pin Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "GPIO Pin Interrupt Enable"]
pub mod interrupt_enable;
#[doc = "SET (w) register accessor: GPIO Pin Set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "GPIO Pin Set"]
pub mod set;
#[doc = "Clear (w) register accessor: GPIO Pin Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = "GPIO Pin Clear"]
pub mod clear;
#[doc = "TOGGLE (w) register accessor: GPIO Pin Toggle\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`toggle::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toggle`]
module"]
pub type TOGGLE = crate::Reg<toggle::TOGGLE_SPEC>;
#[doc = "GPIO Pin Toggle"]
pub mod toggle;
