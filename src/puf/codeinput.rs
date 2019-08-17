#[doc = "Writer for register CODEINPUT"]
pub type W = crate::W<u32, super::CODEINPUT>;
#[doc = "Register CODEINPUT `reset()`'s with value 0"]
impl crate::ResetValue for super::CODEINPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CODEIN`"]
pub struct CODEIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CODEIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - AC/KC input data"]
    #[inline(always)]
    pub fn codein(&mut self) -> CODEIN_W {
        CODEIN_W { w: self }
    }
}
