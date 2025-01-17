#[doc = "Register `INTERP0_POP_LANE0` reader"]
pub struct R(crate::R<INTERP0_POP_LANE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERP0_POP_LANE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERP0_POP_LANE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERP0_POP_LANE0_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<INTERP0_POP_LANE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP).  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [interp0_pop_lane0](index.html) module"]
pub struct INTERP0_POP_LANE0_SPEC;
impl crate::RegisterSpec for INTERP0_POP_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interp0_pop_lane0::R](R) reader structure"]
impl crate::Readable for INTERP0_POP_LANE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERP0_POP_LANE0 to value 0"]
impl crate::Resettable for INTERP0_POP_LANE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
