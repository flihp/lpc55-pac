#[doc = "Reader of register FIFOWR"]
pub type R = crate::R<u32, super::FIFOWR>;
#[doc = "Writer for register FIFOWR"]
pub type W = crate::W<u32, super::FIFOWR>;
#[doc = "Register FIFOWR `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXDATA`"]
pub struct TXDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Possible values of the field `TXSSEL0_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL0_N_AW {
    #[doc = "SSEL0 asserted."]
    ASSERTED,
    #[doc = "SSEL0 not asserted."]
    NOT_ASSERTED,
}
impl From<TXSSEL0_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL0_N_AW) -> Self {
        match variant {
            TXSSEL0_N_AW::ASSERTED => false,
            TXSSEL0_N_AW::NOT_ASSERTED => true,
        }
    }
}
#[doc = "Write proxy for field `TXSSEL0_N`"]
pub struct TXSSEL0_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL0_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL0_N_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SSEL0 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::ASSERTED)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL0_N_AW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `TXSSEL1_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL1_N_AW {
    #[doc = "SSEL1 asserted."]
    ASSERTED,
    #[doc = "SSEL1 not asserted."]
    NOT_ASSERTED,
}
impl From<TXSSEL1_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL1_N_AW) -> Self {
        match variant {
            TXSSEL1_N_AW::ASSERTED => false,
            TXSSEL1_N_AW::NOT_ASSERTED => true,
        }
    }
}
#[doc = "Write proxy for field `TXSSEL1_N`"]
pub struct TXSSEL1_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL1_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL1_N_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SSEL1 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::ASSERTED)
    }
    #[doc = "SSEL1 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL1_N_AW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `TXSSEL2_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL2_N_AW {
    #[doc = "SSEL2 asserted."]
    ASSERTED,
    #[doc = "SSEL2 not asserted."]
    NOT_ASSERTED,
}
impl From<TXSSEL2_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL2_N_AW) -> Self {
        match variant {
            TXSSEL2_N_AW::ASSERTED => false,
            TXSSEL2_N_AW::NOT_ASSERTED => true,
        }
    }
}
#[doc = "Write proxy for field `TXSSEL2_N`"]
pub struct TXSSEL2_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL2_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL2_N_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SSEL2 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::ASSERTED)
    }
    #[doc = "SSEL2 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL2_N_AW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `TXSSEL3_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL3_N_AW {
    #[doc = "SSEL3 asserted."]
    ASSERTED,
    #[doc = "SSEL3 not asserted."]
    NOT_ASSERTED,
}
impl From<TXSSEL3_N_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSSEL3_N_AW) -> Self {
        match variant {
            TXSSEL3_N_AW::ASSERTED => false,
            TXSSEL3_N_AW::NOT_ASSERTED => true,
        }
    }
}
#[doc = "Write proxy for field `TXSSEL3_N`"]
pub struct TXSSEL3_N_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSSEL3_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSSEL3_N_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SSEL3 asserted."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::ASSERTED)
    }
    #[doc = "SSEL3 not asserted."]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut W {
        self.variant(TXSSEL3_N_AW::NOT_ASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOT_AW {
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    NOT_DEASSERTED,
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    DEASSERTED,
}
impl From<EOT_AW> for bool {
    #[inline(always)]
    fn from(variant: EOT_AW) -> Self {
        match variant {
            EOT_AW::NOT_DEASSERTED => false,
            EOT_AW::DEASSERTED => true,
        }
    }
}
#[doc = "Write proxy for field `EOT`"]
pub struct EOT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SSEL not deasserted. This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline(always)]
    pub fn not_deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::NOT_DEASSERTED)
    }
    #[doc = "SSEL deasserted. This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline(always)]
    pub fn deasserted(self) -> &'a mut W {
        self.variant(EOT_AW::DEASSERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOF_AW {
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    NOT_EOF,
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    EOF,
}
impl From<EOF_AW> for bool {
    #[inline(always)]
    fn from(variant: EOF_AW) -> Self {
        match variant {
            EOF_AW::NOT_EOF => false,
            EOF_AW::EOF => true,
        }
    }
}
#[doc = "Write proxy for field `EOF`"]
pub struct EOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data not EOF. This piece of data transmitted is not treated as the end of a frame."]
    #[inline(always)]
    pub fn not_eof(self) -> &'a mut W {
        self.variant(EOF_AW::NOT_EOF)
    }
    #[doc = "Data EOF. This piece of data is treated as the end of a frame, causing the Frame_delay time to be inserted before subsequent data is transmitted."]
    #[inline(always)]
    pub fn eof(self) -> &'a mut W {
        self.variant(EOF_AW::EOF)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `RXIGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNORE_AW {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE,
}
impl From<RXIGNORE_AW> for bool {
    #[inline(always)]
    fn from(variant: RXIGNORE_AW) -> Self {
        match variant {
            RXIGNORE_AW::READ => false,
            RXIGNORE_AW::IGNORE => true,
        }
    }
}
#[doc = "Write proxy for field `RXIGNORE`"]
pub struct RXIGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIGNORE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIGNORE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::READ)
    }
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(RXIGNORE_AW::IGNORE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:15 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TXDATA_W {
        TXDATA_W { w: self }
    }
    #[doc = "Bit 16 - Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel0_n(&mut self) -> TXSSEL0_N_W {
        TXSSEL0_N_W { w: self }
    }
    #[doc = "Bit 17 - Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel1_n(&mut self) -> TXSSEL1_N_W {
        TXSSEL1_N_W { w: self }
    }
    #[doc = "Bit 18 - Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel2_n(&mut self) -> TXSSEL2_N_W {
        TXSSEL2_N_W { w: self }
    }
    #[doc = "Bit 19 - Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn txssel3_n(&mut self) -> TXSSEL3_N_W {
        TXSSEL3_N_W { w: self }
    }
    #[doc = "Bit 20 - End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub fn eot(&mut self) -> EOT_W {
        EOT_W { w: self }
    }
    #[doc = "Bit 21 - End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub fn eof(&mut self) -> EOF_W {
        EOF_W { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub fn rxignore(&mut self) -> RXIGNORE_W {
        RXIGNORE_W { w: self }
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
