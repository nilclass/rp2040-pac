#[doc = "Register `VTOR` reader"]
pub struct R(crate::R<VTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VTOR` writer"]
pub struct W(crate::W<VTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VTOR_SPEC>;
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
impl From<crate::W<VTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBLOFF` reader - Bits \\[31:8\\]
of the indicate the vector table offset address."]
pub type TBLOFF_R = crate::FieldReader<u32>;
#[doc = "Field `TBLOFF` writer - Bits \\[31:8\\]
of the indicate the vector table offset address."]
pub type TBLOFF_W<'a, const O: u8> = crate::FieldWriter<'a, VTOR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 8:31 - Bits \\[31:8\\]
of the indicate the vector table offset address."]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Bits \\[31:8\\]
of the indicate the vector table offset address."]
    #[inline(always)]
    #[must_use]
    pub fn tbloff(&mut self) -> TBLOFF_W<8> {
        TBLOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The VTOR holds the vector table offset address.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [vtor](index.html) module"]
pub struct VTOR_SPEC;
impl crate::RegisterSpec for VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vtor::R](R) reader structure"]
impl crate::Readable for VTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vtor::W](W) writer structure"]
impl crate::Writable for VTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
