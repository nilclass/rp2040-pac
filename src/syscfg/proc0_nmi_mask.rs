#[doc = "Register `PROC0_NMI_MASK` reader"]
pub struct R(crate::R<PROC0_NMI_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC0_NMI_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC0_NMI_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC0_NMI_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC0_NMI_MASK` writer"]
pub struct W(crate::W<PROC0_NMI_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC0_NMI_MASK_SPEC>;
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
impl From<crate::W<PROC0_NMI_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC0_NMI_MASK_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PROC0_NMI_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor core 0 NMI source mask  
 Set a bit high to enable NMI from that IRQ  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc0_nmi_mask](index.html) module"]
pub struct PROC0_NMI_MASK_SPEC;
impl crate::RegisterSpec for PROC0_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc0_nmi_mask::R](R) reader structure"]
impl crate::Readable for PROC0_NMI_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc0_nmi_mask::W](W) writer structure"]
impl crate::Writable for PROC0_NMI_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROC0_NMI_MASK to value 0"]
impl crate::Resettable for PROC0_NMI_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
