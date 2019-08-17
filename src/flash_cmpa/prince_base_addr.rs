#[doc = "Reader of register PRINCE_BASE_ADDR"]
pub type R = crate::R<u32, super::PRINCE_BASE_ADDR>;
#[doc = "Writer for register PRINCE_BASE_ADDR"]
pub type W = crate::W<u32, super::PRINCE_BASE_ADDR>;
#[doc = "Register PRINCE_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PRINCE_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR0_PRG`"]
pub type ADDR0_PRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR0_PRG`"]
pub struct ADDR0_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR0_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDR1_PRG`"]
pub type ADDR1_PRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR1_PRG`"]
pub struct ADDR1_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR1_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADDR2_PRG`"]
pub type ADDR2_PRG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR2_PRG`"]
pub struct ADDR2_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR2_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LOCK_REG0`"]
pub type LOCK_REG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCK_REG0`"]
pub struct LOCK_REG0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `LOCK_REG1`"]
pub type LOCK_REG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCK_REG1`"]
pub struct LOCK_REG1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `LOCK_REG2`"]
pub type LOCK_REG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOCK_REG2`"]
pub struct LOCK_REG2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `REG0_ERASE_CHECK_EN`"]
pub type REG0_ERASE_CHECK_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG0_ERASE_CHECK_EN`"]
pub struct REG0_ERASE_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG0_ERASE_CHECK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `REG1_ERASE_CHECK_EN`"]
pub type REG1_ERASE_CHECK_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG1_ERASE_CHECK_EN`"]
pub struct REG1_ERASE_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG1_ERASE_CHECK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `REG2_ERASE_CHECK_EN`"]
pub type REG2_ERASE_CHECK_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG2_ERASE_CHECK_EN`"]
pub struct REG2_ERASE_CHECK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG2_ERASE_CHECK_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr0_prg(&self) -> ADDR0_PRG_R {
        ADDR0_PRG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr1_prg(&self) -> ADDR1_PRG_R {
        ADDR1_PRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub fn addr2_prg(&self) -> ADDR2_PRG_R {
        ADDR2_PRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg0(&self) -> LOCK_REG0_R {
        LOCK_REG0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg1(&self) -> LOCK_REG1_R {
        LOCK_REG1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg2(&self) -> LOCK_REG2_R {
        LOCK_REG2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg0_erase_check_en(&self) -> REG0_ERASE_CHECK_EN_R {
        REG0_ERASE_CHECK_EN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg1_erase_check_en(&self) -> REG1_ERASE_CHECK_EN_R {
        REG1_ERASE_CHECK_EN_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg2_erase_check_en(&self) -> REG2_ERASE_CHECK_EN_R {
        REG2_ERASE_CHECK_EN_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr0_prg(&mut self) -> ADDR0_PRG_W {
        ADDR0_PRG_W { w: self }
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr1_prg(&mut self) -> ADDR1_PRG_W {
        ADDR1_PRG_W { w: self }
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub fn addr2_prg(&mut self) -> ADDR2_PRG_W {
        ADDR2_PRG_W { w: self }
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg0(&mut self) -> LOCK_REG0_W {
        LOCK_REG0_W { w: self }
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg1(&mut self) -> LOCK_REG1_W {
        LOCK_REG1_W { w: self }
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg2(&mut self) -> LOCK_REG2_W {
        LOCK_REG2_W { w: self }
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg0_erase_check_en(&mut self) -> REG0_ERASE_CHECK_EN_W {
        REG0_ERASE_CHECK_EN_W { w: self }
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg1_erase_check_en(&mut self) -> REG1_ERASE_CHECK_EN_W {
        REG1_ERASE_CHECK_EN_W { w: self }
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg2_erase_check_en(&mut self) -> REG2_ERASE_CHECK_EN_W {
        REG2_ERASE_CHECK_EN_W { w: self }
    }
}
