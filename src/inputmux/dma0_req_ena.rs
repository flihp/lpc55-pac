#[doc = "Reader of register DMA0_REQ_ENA"]
pub type R = crate::R<u32, super::DMA0_REQ_ENA>;
#[doc = "Writer for register DMA0_REQ_ENA"]
pub type W = crate::W<u32, super::DMA0_REQ_ENA>;
#[doc = "Register DMA0_REQ_ENA `reset()`'s with value 0x007f_ffff"]
impl crate::ResetValue for super::DMA0_REQ_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x007f_ffff
    }
}
#[doc = "Reader of field `REQ_ENA`"]
pub type REQ_ENA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REQ_ENA`"]
pub struct REQ_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x007f_ffff) | ((value as u32) & 0x007f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> REQ_ENA_R {
        REQ_ENA_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&mut self) -> REQ_ENA_W {
        REQ_ENA_W { w: self }
    }
}
