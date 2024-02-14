#[doc = "Register `INTERRUPT_ENABLE` reader"]
pub type R = crate::R<INTERRUPT_ENABLE_SPEC>;
#[doc = "Register `INTERRUPT_ENABLE` writer"]
pub type W = crate::W<INTERRUPT_ENABLE_SPEC>;
#[doc = "Field `INTERRUPT_ENABLE` reader - GPIO Pin Interrupt Enable"]
pub type INTERRUPT_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT_ENABLE` writer - GPIO Pin Interrupt Enable"]
pub type INTERRUPT_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO Pin Interrupt Enable"]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> INTERRUPT_ENABLE_R {
        INTERRUPT_ENABLE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO Pin Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_enable(&mut self) -> INTERRUPT_ENABLE_W<INTERRUPT_ENABLE_SPEC> {
        INTERRUPT_ENABLE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO Pin Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENABLE to value 0"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
