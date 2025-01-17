#[doc = "Register `UARTDMACR` reader"]
pub struct R(crate::R<UARTDMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTDMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTDMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTDMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTDMACR` writer"]
pub struct W(crate::W<UARTDMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTDMACR_SPEC>;
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
impl From<crate::W<UARTDMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTDMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_R = crate::BitReader;
#[doc = "Field `RXDMAE` writer - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
pub type RXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, UARTDMACR_SPEC, O>;
#[doc = "Field `TXDMAE` reader - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_R = crate::BitReader;
#[doc = "Field `TXDMAE` writer - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
pub type TXDMAE_W<'a, const O: u8> = crate::BitWriter<'a, UARTDMACR_SPEC, O>;
#[doc = "Field `DMAONERR` reader - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
pub type DMAONERR_R = crate::BitReader;
#[doc = "Field `DMAONERR` writer - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
pub type DMAONERR_W<'a, const O: u8> = crate::BitWriter<'a, UARTDMACR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RXDMAE_W<0> {
        RXDMAE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TXDMAE_W<1> {
        TXDMAE_W::new(self)
    }
    #[doc = "Bit 2 - DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn dmaonerr(&mut self) -> DMAONERR_W<2> {
        DMAONERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register, UARTDMACR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartdmacr](index.html) module"]
pub struct UARTDMACR_SPEC;
impl crate::RegisterSpec for UARTDMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartdmacr::R](R) reader structure"]
impl crate::Readable for UARTDMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartdmacr::W](W) writer structure"]
impl crate::Writable for UARTDMACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTDMACR to value 0"]
impl crate::Resettable for UARTDMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
