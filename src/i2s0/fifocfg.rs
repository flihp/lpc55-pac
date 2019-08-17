#[doc = "Reader of register FIFOCFG"]
pub type R = crate::R<u32, super::FIFOCFG>;
#[doc = "Writer for register FIFOCFG"]
pub type W = crate::W<u32, super::FIFOCFG>;
#[doc = "Register FIFOCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ENABLETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLETX_A {
    #[doc = "The transmit FIFO is not enabled."]
    DISABLED,
    #[doc = "The transmit FIFO is enabled."]
    ENABLED,
}
impl From<ENABLETX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLETX_A) -> Self {
        match variant {
            ENABLETX_A::DISABLED => false,
            ENABLETX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLETX`"]
pub type ENABLETX_R = crate::R<bool, ENABLETX_A>;
impl ENABLETX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLETX_A {
        match self.bits {
            false => ENABLETX_A::DISABLED,
            true => ENABLETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLETX_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLETX`"]
pub struct ENABLETX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLETX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLETX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::DISABLED)
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLERX_A {
    #[doc = "The receive FIFO is not enabled."]
    DISABLED,
    #[doc = "The receive FIFO is enabled."]
    ENABLED,
}
impl From<ENABLERX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLERX_A) -> Self {
        match variant {
            ENABLERX_A::DISABLED => false,
            ENABLERX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLERX`"]
pub type ENABLERX_R = crate::R<bool, ENABLERX_A>;
impl ENABLERX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLERX_A {
        match self.bits {
            false => ENABLERX_A::DISABLED,
            true => ENABLERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLERX_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLERX`"]
pub struct ENABLERX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLERX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLERX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::DISABLED)
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `TXI2SE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXI2SE0_A {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO,
}
impl From<TXI2SE0_A> for bool {
    #[inline(always)]
    fn from(variant: TXI2SE0_A) -> Self {
        match variant {
            TXI2SE0_A::LAST_VALUE => false,
            TXI2SE0_A::ZERO => true,
        }
    }
}
#[doc = "Reader of field `TXI2SE0`"]
pub type TXI2SE0_R = crate::R<bool, TXI2SE0_A>;
impl TXI2SE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXI2SE0_A {
        match self.bits {
            false => TXI2SE0_A::LAST_VALUE,
            true => TXI2SE0_A::ZERO,
        }
    }
    #[doc = "Checks if the value of the field is `LAST_VALUE`"]
    #[inline(always)]
    pub fn is_last_value(&self) -> bool {
        *self == TXI2SE0_A::LAST_VALUE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXI2SE0_A::ZERO
    }
}
#[doc = "Write proxy for field `TXI2SE0`"]
pub struct TXI2SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXI2SE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXI2SE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    #[inline(always)]
    pub fn last_value(self) -> &'a mut W {
        self.variant(TXI2SE0_A::LAST_VALUE)
    }
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXI2SE0_A::ZERO)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PACK48`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACK48_A {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16,
}
impl From<PACK48_A> for bool {
    #[inline(always)]
    fn from(variant: PACK48_A) -> Self {
        match variant {
            PACK48_A::BIT_24 => false,
            PACK48_A::BIT_32_16 => true,
        }
    }
}
#[doc = "Reader of field `PACK48`"]
pub type PACK48_R = crate::R<bool, PACK48_A>;
impl PACK48_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PACK48_A {
        match self.bits {
            false => PACK48_A::BIT_24,
            true => PACK48_A::BIT_32_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_24`"]
    #[inline(always)]
    pub fn is_bit_24(&self) -> bool {
        *self == PACK48_A::BIT_24
    }
    #[doc = "Checks if the value of the field is `BIT_32_16`"]
    #[inline(always)]
    pub fn is_bit_32_16(&self) -> bool {
        *self == PACK48_A::BIT_32_16
    }
}
#[doc = "Write proxy for field `PACK48`"]
pub struct PACK48_W<'a> {
    w: &'a mut W,
}
impl<'a> PACK48_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PACK48_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    #[inline(always)]
    pub fn bit_24(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_24)
    }
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    #[inline(always)]
    pub fn bit_32_16(self) -> &'a mut W {
        self.variant(PACK48_A::BIT_32_16)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `DMATX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMATX_A {
    #[doc = "DMA is not used for the transmit function."]
    DISABLED,
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl From<DMATX_A> for bool {
    #[inline(always)]
    fn from(variant: DMATX_A) -> Self {
        match variant {
            DMATX_A::DISABLED => false,
            DMATX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DMATX`"]
pub type DMATX_R = crate::R<bool, DMATX_A>;
impl DMATX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATX_A {
        match self.bits {
            false => DMATX_A::DISABLED,
            true => DMATX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATX_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMATX`"]
pub struct DMATX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMATX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is not used for the transmit function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATX_A::DISABLED)
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `DMARX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMARX_A {
    #[doc = "DMA is not used for the receive function."]
    DISABLED,
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED,
}
impl From<DMARX_A> for bool {
    #[inline(always)]
    fn from(variant: DMARX_A) -> Self {
        match variant {
            DMARX_A::DISABLED => false,
            DMARX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DMARX`"]
pub type DMARX_R = crate::R<bool, DMARX_A>;
impl DMARX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARX_A {
        match self.bits {
            false => DMARX_A::DISABLED,
            true => DMARX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARX_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMARX`"]
pub struct DMARX_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMARX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA is not used for the receive function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARX_A::DISABLED)
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `WAKETX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKETX_A {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED,
}
impl From<WAKETX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKETX_A) -> Self {
        match variant {
            WAKETX_A::DISABLED => false,
            WAKETX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `WAKETX`"]
pub type WAKETX_R = crate::R<bool, WAKETX_A>;
impl WAKETX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKETX_A {
        match self.bits {
            false => WAKETX_A::DISABLED,
            true => WAKETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKETX_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKETX`"]
pub struct WAKETX_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKETX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKETX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETX_A::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `WAKERX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKERX_A {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED,
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED,
}
impl From<WAKERX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKERX_A) -> Self {
        match variant {
            WAKERX_A::DISABLED => false,
            WAKERX_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `WAKERX`"]
pub type WAKERX_R = crate::R<bool, WAKERX_A>;
impl WAKERX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKERX_A {
        match self.bits {
            false => WAKERX_A::DISABLED,
            true => WAKERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKERX_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKERX`"]
pub struct WAKERX_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKERX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKERX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERX_A::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `EMPTYTX`"]
pub type EMPTYTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYTX`"]
pub struct EMPTYTX_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYTX_W<'a> {
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
#[doc = "Reader of field `EMPTYRX`"]
pub type EMPTYRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMPTYRX`"]
pub struct EMPTYRX_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTYRX_W<'a> {
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
#[doc = "Possible values of the field `POPDBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POPDBG_A {
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    DO_NOT_POP,
    #[doc = "A debug read will cause the FIFO to pop."]
    POP,
}
impl From<POPDBG_A> for bool {
    #[inline(always)]
    fn from(variant: POPDBG_A) -> Self {
        match variant {
            POPDBG_A::DO_NOT_POP => false,
            POPDBG_A::POP => true,
        }
    }
}
#[doc = "Reader of field `POPDBG`"]
pub type POPDBG_R = crate::R<bool, POPDBG_A>;
impl POPDBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POPDBG_A {
        match self.bits {
            false => POPDBG_A::DO_NOT_POP,
            true => POPDBG_A::POP,
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOT_POP`"]
    #[inline(always)]
    pub fn is_do_not_pop(&self) -> bool {
        *self == POPDBG_A::DO_NOT_POP
    }
    #[doc = "Checks if the value of the field is `POP`"]
    #[inline(always)]
    pub fn is_pop(&self) -> bool {
        *self == POPDBG_A::POP
    }
}
#[doc = "Write proxy for field `POPDBG`"]
pub struct POPDBG_W<'a> {
    w: &'a mut W,
}
impl<'a> POPDBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POPDBG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debug reads of the FIFO do not pop the FIFO."]
    #[inline(always)]
    pub fn do_not_pop(self) -> &'a mut W {
        self.variant(POPDBG_A::DO_NOT_POP)
    }
    #[doc = "A debug read will cause the FIFO to pop."]
    #[inline(always)]
    pub fn pop(self) -> &'a mut W {
        self.variant(POPDBG_A::POP)
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
impl R {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&self) -> ENABLETX_R {
        ENABLETX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&self) -> ENABLERX_R {
        ENABLERX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline(always)]
    pub fn txi2se0(&self) -> TXI2SE0_R {
        TXI2SE0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline(always)]
    pub fn pack48(&self) -> PACK48_R {
        PACK48_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&self) -> WAKETX_R {
        WAKETX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&self) -> WAKERX_R {
        WAKERX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&self) -> EMPTYTX_R {
        EMPTYTX_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&self) -> EMPTYRX_R {
        EMPTYRX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline(always)]
    pub fn popdbg(&self) -> POPDBG_R {
        POPDBG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&mut self) -> ENABLETX_W {
        ENABLETX_W { w: self }
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&mut self) -> ENABLERX_W {
        ENABLERX_W { w: self }
    }
    #[doc = "Bit 2 - Transmit I2S empty 0. Determines the value sent by the I2S in transmit mode if the TX FIFO becomes empty. This value is sent repeatedly until the I2S is paused, the error is cleared, new data is provided, and the I2S is un-paused."]
    #[inline(always)]
    pub fn txi2se0(&mut self) -> TXI2SE0_W {
        TXI2SE0_W { w: self }
    }
    #[doc = "Bit 3 - Packing format for 48-bit data. This relates to how data is entered into or taken from the FIFO by software or DMA."]
    #[inline(always)]
    pub fn pack48(&mut self) -> PACK48_W {
        PACK48_W { w: self }
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&mut self) -> DMATX_W {
        DMATX_W { w: self }
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&mut self) -> DMARX_W {
        DMARX_W { w: self }
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&mut self) -> WAKETX_W {
        WAKETX_W { w: self }
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&mut self) -> WAKERX_W {
        WAKERX_W { w: self }
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&mut self) -> EMPTYTX_W {
        EMPTYTX_W { w: self }
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&mut self) -> EMPTYRX_W {
        EMPTYRX_W { w: self }
    }
    #[doc = "Bit 18 - Pop FIFO for debug reads."]
    #[inline(always)]
    pub fn popdbg(&mut self) -> POPDBG_W {
        POPDBG_W { w: self }
    }
}
