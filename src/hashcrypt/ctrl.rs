#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: SHA1 is enabled"]
    SHA1 = 1,
    #[doc = "2: SHA2-256 is enabled"]
    SHA2_256 = 2,
    #[doc = "4: AES if available (see also CRYPTCFG register for more controls)"]
    AES = 4,
    #[doc = "5: ICB-AES if available (see also CRYPTCFG register for more controls)"]
    ICB_AES = 5,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `Mode`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::DISABLED),
            1 => Val(MODE_A::SHA1),
            2 => Val(MODE_A::SHA2_256),
            4 => Val(MODE_A::AES),
            5 => Val(MODE_A::ICB_AES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == MODE_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA2_256`"]
    #[inline(always)]
    pub fn is_sha2_256(&self) -> bool {
        *self == MODE_A::SHA2_256
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == MODE_A::AES
    }
    #[doc = "Checks if the value of the field is `ICB_AES`"]
    #[inline(always)]
    pub fn is_icb_aes(&self) -> bool {
        *self == MODE_A::ICB_AES
    }
}
#[doc = "Write proxy for field `Mode`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
    #[doc = "SHA1 is enabled"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(MODE_A::SHA1)
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline(always)]
    pub fn sha2_256(self) -> &'a mut W {
        self.variant(MODE_A::SHA2_256)
    }
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(MODE_A::AES)
    }
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn icb_aes(self) -> &'a mut W {
        self.variant(MODE_A::ICB_AES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEW_HASH_AW {
    #[doc = "1: Starts a new Hash/Crypto and initializes the Digest/Result."]
    START = 1,
}
impl From<NEW_HASH_AW> for bool {
    #[inline(always)]
    fn from(variant: NEW_HASH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `New_Hash`"]
pub struct NEW_HASH_W<'a> {
    w: &'a mut W,
}
impl<'a> NEW_HASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEW_HASH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(NEW_HASH_AW::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_I_A {
    #[doc = "0: DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0,
    #[doc = "1: DMA will push in the data."]
    PUSH = 1,
}
impl From<DMA_I_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_I`"]
pub type DMA_I_R = crate::R<bool, DMA_I_A>;
impl DMA_I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_I_A {
        match self.bits {
            false => DMA_I_A::NOT_USED,
            true => DMA_I_A::PUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMA_I_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == DMA_I_A::PUSH
    }
}
#[doc = "Write proxy for field `DMA_I`"]
pub struct DMA_I_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMA_I_A::NOT_USED)
    }
    #[doc = "DMA will push in the data."]
    #[inline(always)]
    pub fn push(self) -> &'a mut W {
        self.variant(DMA_I_A::PUSH)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_O_A {
    #[doc = "0: DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED = 0,
}
impl From<DMA_O_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA_O`"]
pub type DMA_O_R = crate::R<bool, DMA_O_A>;
impl DMA_O_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DMA_O_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(DMA_O_A::NOTUSED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTUSED`"]
    #[inline(always)]
    pub fn is_notused(&self) -> bool {
        *self == DMA_O_A::NOTUSED
    }
}
#[doc = "Write proxy for field `DMA_O`"]
pub struct DMA_O_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_O_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline(always)]
    pub fn notused(self) -> &'a mut W {
        self.variant(DMA_O_A::NOTUSED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HASHSWPB`"]
pub type HASHSWPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASHSWPB`"]
pub struct HASHSWPB_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHSWPB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub fn dma_i(&self) -> DMA_I_R {
        DMA_I_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub fn dma_o(&self) -> DMA_O_R {
        DMA_O_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub fn hashswpb(&self) -> HASHSWPB_R {
        HASHSWPB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 4 - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline(always)]
    pub fn new_hash(&mut self) -> NEW_HASH_W {
        NEW_HASH_W { w: self }
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub fn dma_i(&mut self) -> DMA_I_W {
        DMA_I_W { w: self }
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub fn dma_o(&mut self) -> DMA_O_W {
        DMA_O_W { w: self }
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub fn hashswpb(&mut self) -> HASHSWPB_W {
        HASHSWPB_W { w: self }
    }
}
