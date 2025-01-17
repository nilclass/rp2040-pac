#[doc = "Register `UARTPCELLID0` reader"]
pub struct R(crate::R<UARTPCELLID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPCELLID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPCELLID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPCELLID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UARTPCELLID0` reader - These bits read back as 0x0D"]
pub type UARTPCELLID0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x0D"]
    #[inline(always)]
    pub fn uartpcellid0(&self) -> UARTPCELLID0_R {
        UARTPCELLID0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID0 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartpcellid0](index.html) module"]
pub struct UARTPCELLID0_SPEC;
impl crate::RegisterSpec for UARTPCELLID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartpcellid0::R](R) reader structure"]
impl crate::Readable for UARTPCELLID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPCELLID0 to value 0x0d"]
impl crate::Resettable for UARTPCELLID0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
