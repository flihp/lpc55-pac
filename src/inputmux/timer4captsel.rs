#[doc = "Reader of register TIMER4CAPTSEL[%s]"]
pub type R = crate::R<u32, super::TIMER4CAPTSEL>;
#[doc = "Writer for register TIMER4CAPTSEL[%s]"]
pub type W = crate::W<u32, super::TIMER4CAPTSEL>;
#[doc = "Register TIMER4CAPTSEL[%s]
`reset()`'s with value 0x1f"]
impl crate::ResetValue for super::TIMER4CAPTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `CAPTSEL`"]
pub type CAPTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPTSEL`"]
pub struct CAPTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CAPTSEL_R {
        CAPTSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER%s capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&mut self) -> CAPTSEL_W {
        CAPTSEL_W { w: self }
    }
}
