#[doc = "Register `GPIO_HI_OUT_CLR` writer"]
pub struct W(crate::W<GPIO_HI_OUT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HI_OUT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_HI_OUT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HI_OUT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HI_OUT_CLR` writer - Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &amp;= ~wdata`"]
pub type GPIO_HI_OUT_CLR_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_HI_OUT_CLR_SPEC, 6, O>;
impl W {
    #[doc = "Bits 0:5 - Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT &amp;= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hi_out_clr(&mut self) -> GPIO_HI_OUT_CLR_W<0> {
        GPIO_HI_OUT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI output value clear  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_out_clr](index.html) module"]
pub struct GPIO_HI_OUT_CLR_SPEC;
impl crate::RegisterSpec for GPIO_HI_OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out_clr::W](W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OUT_CLR to value 0"]
impl crate::Resettable for GPIO_HI_OUT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
