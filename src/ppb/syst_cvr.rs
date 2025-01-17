#[doc = "Register `SYST_CVR` reader"]
pub struct R(crate::R<SYST_CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CVR` writer"]
pub struct W(crate::W<SYST_CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CVR_SPEC>;
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
impl From<crate::W<SYST_CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT` reader - Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
pub type CURRENT_R = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
pub type CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, SYST_CVR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<0> {
        CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [syst_cvr](index.html) module"]
pub struct SYST_CVR_SPEC;
impl crate::RegisterSpec for SYST_CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_cvr::R](R) reader structure"]
impl crate::Readable for SYST_CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](W) writer structure"]
impl crate::Writable for SYST_CVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_CVR to value 0"]
impl crate::Resettable for SYST_CVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
