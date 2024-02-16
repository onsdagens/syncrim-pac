#[doc = "Register `MTIME_LO` reader"]
pub type R = crate::R<MTIME_LO_SPEC>;
#[doc = "Field `MTIME_LO` reader - Timer Low 32 bits"]
pub type MTIME_LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Low 32 bits"]
    #[inline(always)]
    pub fn mtime_lo(&self) -> MTIME_LO_R {
        MTIME_LO_R::new(self.bits)
    }
}
#[doc = "Timer Low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime_lo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_LO_SPEC;
impl crate::RegisterSpec for MTIME_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_lo::R`](R) reader structure"]
impl crate::Readable for MTIME_LO_SPEC {}
#[doc = "`reset()` method sets MTIME_LO to value 0"]
impl crate::Resettable for MTIME_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
