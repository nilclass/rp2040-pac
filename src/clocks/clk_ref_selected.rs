#[doc = "Register `CLK_REF_SELECTED` reader"]
pub struct R(crate::R<CLK_REF_SELECTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REF_SELECTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REF_SELECTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REF_SELECTED_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLK_REF_SELECTED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot).  
 The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s.  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_ref_selected](index.html) module"]
pub struct CLK_REF_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_REF_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ref_selected::R](R) reader structure"]
impl crate::Readable for CLK_REF_SELECTED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_REF_SELECTED to value 0x01"]
impl crate::Resettable for CLK_REF_SELECTED_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
