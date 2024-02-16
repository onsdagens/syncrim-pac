#[doc = "Register `MTIME_HI` reader"]
pub type R = crate::R<MTIME_HI_SPEC>;
#[doc = "Field `MTIME_HI` reader - Timer Hi 32 bits"]
pub type MTIME_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Hi 32 bits"]
    #[inline(always)]
    pub fn mtime_hi(&self) -> MTIME_HI_R {
        MTIME_HI_R::new(self.bits)
    }
}
#[doc = "Timer Hi 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_HI_SPEC;
impl crate::RegisterSpec for MTIME_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_hi::R`](R) reader structure"]
impl crate::Readable for MTIME_HI_SPEC {}
#[doc = "`reset()` method sets MTIME_HI to value 0"]
impl crate::Resettable for MTIME_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
