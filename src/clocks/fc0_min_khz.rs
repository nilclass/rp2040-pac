#[doc = "Register `FC0_MIN_KHZ` reader"]
pub struct R(crate::R<FC0_MIN_KHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_MIN_KHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_MIN_KHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_MIN_KHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC0_MIN_KHZ` writer"]
pub struct W(crate::W<FC0_MIN_KHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC0_MIN_KHZ_SPEC>;
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
impl From<crate::W<FC0_MIN_KHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC0_MIN_KHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0_MIN_KHZ` reader - "]
pub type FC0_MIN_KHZ_R = crate::FieldReader<u32>;
#[doc = "Field `FC0_MIN_KHZ` writer - "]
pub type FC0_MIN_KHZ_W<'a, const O: u8> = crate::FieldWriter<'a, FC0_MIN_KHZ_SPEC, 25, O, u32>;
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_min_khz(&self) -> FC0_MIN_KHZ_R {
        FC0_MIN_KHZ_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_min_khz(&mut self) -> FC0_MIN_KHZ_W<0> {
        FC0_MIN_KHZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_min_khz](index.html) module"]
pub struct FC0_MIN_KHZ_SPEC;
impl crate::RegisterSpec for FC0_MIN_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_min_khz::R](R) reader structure"]
impl crate::Readable for FC0_MIN_KHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc0_min_khz::W](W) writer structure"]
impl crate::Writable for FC0_MIN_KHZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC0_MIN_KHZ to value 0"]
impl crate::Resettable for FC0_MIN_KHZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
