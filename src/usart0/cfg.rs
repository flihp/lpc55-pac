#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    DISABLED,
    #[doc = "Enabled. The USART is enabled for operation."]
    ENABLED,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        match variant {
            ENABLE_A::DISABLED => false,
            ENABLE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
#[doc = "Possible values of the field `DATALEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATALEN_A {
    #[doc = "7 bit Data length."]
    BIT_7,
    #[doc = "8 bit Data length."]
    BIT_8,
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    BIT_9,
}
impl From<DATALEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALEN_A) -> Self {
        match variant {
            DATALEN_A::BIT_7 => 0,
            DATALEN_A::BIT_8 => 1,
            DATALEN_A::BIT_9 => 2,
        }
    }
}
#[doc = "Reader of field `DATALEN`"]
pub type DATALEN_R = crate::R<u8, DATALEN_A>;
impl DATALEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATALEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATALEN_A::BIT_7),
            1 => Val(DATALEN_A::BIT_8),
            2 => Val(DATALEN_A::BIT_9),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_7`"]
    #[inline(always)]
    pub fn is_bit_7(&self) -> bool {
        *self == DATALEN_A::BIT_7
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == DATALEN_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_9`"]
    #[inline(always)]
    pub fn is_bit_9(&self) -> bool {
        *self == DATALEN_A::BIT_9
    }
}
#[doc = "Write proxy for field `DATALEN`"]
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATALEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "7 bit Data length."]
    #[inline(always)]
    pub fn bit_7(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_7)
    }
    #[doc = "8 bit Data length."]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_8)
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    #[inline(always)]
    pub fn bit_9(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `PARITYSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSEL_A {
    #[doc = "No parity."]
    NO_PARITY,
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY,
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY,
}
impl From<PARITYSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITYSEL_A) -> Self {
        match variant {
            PARITYSEL_A::NO_PARITY => 0,
            PARITYSEL_A::EVEN_PARITY => 2,
            PARITYSEL_A::ODD_PARITY => 3,
        }
    }
}
#[doc = "Reader of field `PARITYSEL`"]
pub type PARITYSEL_R = crate::R<u8, PARITYSEL_A>;
impl PARITYSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PARITYSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PARITYSEL_A::NO_PARITY),
            2 => Val(PARITYSEL_A::EVEN_PARITY),
            3 => Val(PARITYSEL_A::ODD_PARITY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY`"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITYSEL_A::NO_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == PARITYSEL_A::EVEN_PARITY
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == PARITYSEL_A::ODD_PARITY
    }
}
#[doc = "Write proxy for field `PARITYSEL`"]
pub struct PARITYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITYSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No parity."]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::NO_PARITY)
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::EVEN_PARITY)
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::ODD_PARITY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `STOPLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPLEN_A {
    #[doc = "1 stop bit."]
    BIT_1,
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    BITS_2,
}
impl From<STOPLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPLEN_A) -> Self {
        match variant {
            STOPLEN_A::BIT_1 => false,
            STOPLEN_A::BITS_2 => true,
        }
    }
}
#[doc = "Reader of field `STOPLEN`"]
pub type STOPLEN_R = crate::R<bool, STOPLEN_A>;
impl STOPLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPLEN_A {
        match self.bits {
            false => STOPLEN_A::BIT_1,
            true => STOPLEN_A::BITS_2,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_1`"]
    #[inline(always)]
    pub fn is_bit_1(&self) -> bool {
        *self == STOPLEN_A::BIT_1
    }
    #[doc = "Checks if the value of the field is `BITS_2`"]
    #[inline(always)]
    pub fn is_bits_2(&self) -> bool {
        *self == STOPLEN_A::BITS_2
    }
}
#[doc = "Write proxy for field `STOPLEN`"]
pub struct STOPLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn bit_1(self) -> &'a mut W {
        self.variant(STOPLEN_A::BIT_1)
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline(always)]
    pub fn bits_2(self) -> &'a mut W {
        self.variant(STOPLEN_A::BITS_2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MODE32K`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE32K_A {
    #[doc = "Disabled. USART uses standard clocking."]
    DISABLED,
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    ENABLED,
}
impl From<MODE32K_A> for bool {
    #[inline(always)]
    fn from(variant: MODE32K_A) -> Self {
        match variant {
            MODE32K_A::DISABLED => false,
            MODE32K_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `MODE32K`"]
pub type MODE32K_R = crate::R<bool, MODE32K_A>;
impl MODE32K_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE32K_A {
        match self.bits {
            false => MODE32K_A::DISABLED,
            true => MODE32K_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE32K_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MODE32K_A::ENABLED
    }
}
#[doc = "Write proxy for field `MODE32K`"]
pub struct MODE32K_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE32K_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE32K_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. USART uses standard clocking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE32K_A::DISABLED)
    }
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MODE32K_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `LINMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINMODE_A {
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    DISABLED,
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    ENABLED,
}
impl From<LINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LINMODE_A) -> Self {
        match variant {
            LINMODE_A::DISABLED => false,
            LINMODE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LINMODE`"]
pub type LINMODE_R = crate::R<bool, LINMODE_A>;
impl LINMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINMODE_A {
        match self.bits {
            false => LINMODE_A::DISABLED,
            true => LINMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINMODE_A::ENABLED
    }
}
#[doc = "Write proxy for field `LINMODE`"]
pub struct LINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINMODE_A::DISABLED)
    }
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINMODE_A::ENABLED)
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
#[doc = "Possible values of the field `CTSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSEN_A {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    DISABLED,
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    ENABLED,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        match variant {
            CTSEN_A::DISABLED => false,
            CTSEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `CTSEN`"]
pub type CTSEN_R = crate::R<bool, CTSEN_A>;
impl CTSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLED,
            true => CTSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CTSEN`"]
pub struct CTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLED)
    }
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLED)
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
#[doc = "Possible values of the field `SYNCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN_A {
    #[doc = "Asynchronous mode."]
    ASYNCHRONOUS_MODE,
    #[doc = "Synchronous mode."]
    SYNCHRONOUS_MODE,
}
impl From<SYNCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN_A) -> Self {
        match variant {
            SYNCEN_A::ASYNCHRONOUS_MODE => false,
            SYNCEN_A::SYNCHRONOUS_MODE => true,
        }
    }
}
#[doc = "Reader of field `SYNCEN`"]
pub type SYNCEN_R = crate::R<bool, SYNCEN_A>;
impl SYNCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN_A {
        match self.bits {
            false => SYNCEN_A::ASYNCHRONOUS_MODE,
            true => SYNCEN_A::SYNCHRONOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == SYNCEN_A::ASYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == SYNCEN_A::SYNCHRONOUS_MODE
    }
}
#[doc = "Write proxy for field `SYNCEN`"]
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::ASYNCHRONOUS_MODE)
    }
    #[doc = "Synchronous mode."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::SYNCHRONOUS_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `CLKPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPOL_A {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE,
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        match variant {
            CLKPOL_A::FALLING_EDGE => false,
            CLKPOL_A::RISING_EDGE => true,
        }
    }
}
#[doc = "Reader of field `CLKPOL`"]
pub type CLKPOL_R = crate::R<bool, CLKPOL_A>;
impl CLKPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::FALLING_EDGE,
            true => CLKPOL_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CLKPOL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CLKPOL_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `CLKPOL`"]
pub struct CLKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::RISING_EDGE)
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
#[doc = "Possible values of the field `SYNCMST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMST_A {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE,
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    MASTER,
}
impl From<SYNCMST_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMST_A) -> Self {
        match variant {
            SYNCMST_A::SLAVE => false,
            SYNCMST_A::MASTER => true,
        }
    }
}
#[doc = "Reader of field `SYNCMST`"]
pub type SYNCMST_R = crate::R<bool, SYNCMST_A>;
impl SYNCMST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCMST_A {
        match self.bits {
            false => SYNCMST_A::SLAVE,
            true => SYNCMST_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == SYNCMST_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == SYNCMST_A::MASTER
    }
}
#[doc = "Write proxy for field `SYNCMST`"]
pub struct SYNCMST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCMST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCMST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(SYNCMST_A::SLAVE)
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(SYNCMST_A::MASTER)
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
#[doc = "Possible values of the field `LOOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_A {
    #[doc = "Normal operation."]
    NORMAL,
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        match variant {
            LOOP_A::NORMAL => false,
            LOOP_A::LOOPBACK => true,
        }
    }
}
#[doc = "Reader of field `LOOP`"]
pub type LOOP_R = crate::R<bool, LOOP_A>;
impl LOOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::NORMAL,
            true => LOOP_A::LOOPBACK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LOOP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOOPBACK`"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LOOP_A::LOOPBACK
    }
}
#[doc = "Write proxy for field `LOOP`"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LOOP_A::NORMAL)
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LOOP_A::LOOPBACK)
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
#[doc = "Possible values of the field `OETA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OETA_A {
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    DISABLED,
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    ENABLED,
}
impl From<OETA_A> for bool {
    #[inline(always)]
    fn from(variant: OETA_A) -> Self {
        match variant {
            OETA_A::DISABLED => false,
            OETA_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `OETA`"]
pub type OETA_R = crate::R<bool, OETA_A>;
impl OETA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OETA_A {
        match self.bits {
            false => OETA_A::DISABLED,
            true => OETA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OETA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OETA_A::ENABLED
    }
}
#[doc = "Write proxy for field `OETA`"]
pub struct OETA_W<'a> {
    w: &'a mut W,
}
impl<'a> OETA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OETA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OETA_A::DISABLED)
    }
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OETA_A::ENABLED)
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
#[doc = "Possible values of the field `AUTOADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOADDR_A {
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    DISABLED,
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    ENABLED,
}
impl From<AUTOADDR_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOADDR_A) -> Self {
        match variant {
            AUTOADDR_A::DISABLED => false,
            AUTOADDR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `AUTOADDR`"]
pub type AUTOADDR_R = crate::R<bool, AUTOADDR_A>;
impl AUTOADDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOADDR_A {
        match self.bits {
            false => AUTOADDR_A::DISABLED,
            true => AUTOADDR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOADDR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOADDR_A::ENABLED
    }
}
#[doc = "Write proxy for field `AUTOADDR`"]
pub struct AUTOADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOADDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::DISABLED)
    }
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::ENABLED)
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
#[doc = "Possible values of the field `OESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OESEL_A {
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    STANDARD,
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    RS_485,
}
impl From<OESEL_A> for bool {
    #[inline(always)]
    fn from(variant: OESEL_A) -> Self {
        match variant {
            OESEL_A::STANDARD => false,
            OESEL_A::RS_485 => true,
        }
    }
}
#[doc = "Reader of field `OESEL`"]
pub type OESEL_R = crate::R<bool, OESEL_A>;
impl OESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OESEL_A {
        match self.bits {
            false => OESEL_A::STANDARD,
            true => OESEL_A::RS_485,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == OESEL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `RS_485`"]
    #[inline(always)]
    pub fn is_rs_485(&self) -> bool {
        *self == OESEL_A::RS_485
    }
}
#[doc = "Write proxy for field `OESEL`"]
pub struct OESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OESEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(OESEL_A::STANDARD)
    }
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    #[inline(always)]
    pub fn rs_485(self) -> &'a mut W {
        self.variant(OESEL_A::RS_485)
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
#[doc = "Possible values of the field `OEPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEPOL_A {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    LOW,
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    HIGH,
}
impl From<OEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: OEPOL_A) -> Self {
        match variant {
            OEPOL_A::LOW => false,
            OEPOL_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `OEPOL`"]
pub type OEPOL_R = crate::R<bool, OEPOL_A>;
impl OEPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEPOL_A {
        match self.bits {
            false => OEPOL_A::LOW,
            true => OEPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OEPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OEPOL_A::HIGH
    }
}
#[doc = "Write proxy for field `OEPOL`"]
pub struct OEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OEPOL_A::LOW)
    }
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OEPOL_A::HIGH)
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
#[doc = "Possible values of the field `RXPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPOL_A {
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD,
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl From<RXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RXPOL_A) -> Self {
        match variant {
            RXPOL_A::STANDARD => false,
            RXPOL_A::INVERTED => true,
        }
    }
}
#[doc = "Reader of field `RXPOL`"]
pub type RXPOL_R = crate::R<bool, RXPOL_A>;
impl RXPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPOL_A {
        match self.bits {
            false => RXPOL_A::STANDARD,
            true => RXPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXPOL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXPOL_A::INVERTED
    }
}
#[doc = "Write proxy for field `RXPOL`"]
pub struct RXPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXPOL_A::STANDARD)
    }
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXPOL_A::INVERTED)
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
#[doc = "Possible values of the field `TXPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOL_A {
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD,
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED,
}
impl From<TXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXPOL_A) -> Self {
        match variant {
            TXPOL_A::STANDARD => false,
            TXPOL_A::INVERTED => true,
        }
    }
}
#[doc = "Reader of field `TXPOL`"]
pub type TXPOL_R = crate::R<bool, TXPOL_A>;
impl TXPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPOL_A {
        match self.bits {
            false => TXPOL_A::STANDARD,
            true => TXPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXPOL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXPOL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TXPOL`"]
pub struct TXPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXPOL_A::STANDARD)
    }
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXPOL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub fn paritysel(&self) -> PARITYSEL_R {
        PARITYSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub fn stoplen(&self) -> STOPLEN_R {
        STOPLEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub fn mode32k(&self) -> MODE32K_R {
        MODE32K_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    pub fn linmode(&self) -> LINMODE_R {
        LINMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    pub fn syncmst(&self) -> SYNCMST_R {
        SYNCMST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub fn oeta(&self) -> OETA_R {
        OETA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    pub fn autoaddr(&self) -> AUTOADDR_R {
        AUTOADDR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    pub fn oesel(&self) -> OESEL_R {
        OESEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    pub fn rxpol(&self) -> RXPOL_R {
        RXPOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    pub fn txpol(&self) -> TXPOL_R {
        TXPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub fn paritysel(&mut self) -> PARITYSEL_W {
        PARITYSEL_W { w: self }
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub fn stoplen(&mut self) -> STOPLEN_W {
        STOPLEN_W { w: self }
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub fn mode32k(&mut self) -> MODE32K_W {
        MODE32K_W { w: self }
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    pub fn linmode(&mut self) -> LINMODE_W {
        LINMODE_W { w: self }
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W { w: self }
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W { w: self }
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    pub fn syncmst(&mut self) -> SYNCMST_W {
        SYNCMST_W { w: self }
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub fn oeta(&mut self) -> OETA_W {
        OETA_W { w: self }
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    pub fn autoaddr(&mut self) -> AUTOADDR_W {
        AUTOADDR_W { w: self }
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    pub fn oesel(&mut self) -> OESEL_W {
        OESEL_W { w: self }
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    pub fn oepol(&mut self) -> OEPOL_W {
        OEPOL_W { w: self }
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    pub fn rxpol(&mut self) -> RXPOL_W {
        RXPOL_W { w: self }
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    pub fn txpol(&mut self) -> TXPOL_W {
        TXPOL_W { w: self }
    }
}
