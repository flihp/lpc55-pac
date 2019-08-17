#[doc = "Reader of register MONRXDAT"]
pub type R = crate::R<u32, super::MONRXDAT>;
#[doc = "Reader of field `MONRXDAT`"]
pub type MONRXDAT_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `MONSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONSTART_A {
    #[doc = "No start detected. The Monitor function has not detected a Start event on the I2C bus."]
    NO_START_DETECTED,
    #[doc = "Start detected. The Monitor function has detected a Start event on the I2C bus."]
    START_DETECTED,
}
impl From<MONSTART_A> for bool {
    #[inline(always)]
    fn from(variant: MONSTART_A) -> Self {
        match variant {
            MONSTART_A::NO_START_DETECTED => false,
            MONSTART_A::START_DETECTED => true,
        }
    }
}
#[doc = "Reader of field `MONSTART`"]
pub type MONSTART_R = crate::R<bool, MONSTART_A>;
impl MONSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONSTART_A {
        match self.bits {
            false => MONSTART_A::NO_START_DETECTED,
            true => MONSTART_A::START_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_DETECTED`"]
    #[inline(always)]
    pub fn is_no_start_detected(&self) -> bool {
        *self == MONSTART_A::NO_START_DETECTED
    }
    #[doc = "Checks if the value of the field is `START_DETECTED`"]
    #[inline(always)]
    pub fn is_start_detected(&self) -> bool {
        *self == MONSTART_A::START_DETECTED
    }
}
#[doc = "Possible values of the field `MONRESTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRESTART_A {
    #[doc = "No repeated start detected. The Monitor function has not detected a Repeated Start event on the I2C bus."]
    NOT_DETECTED,
    #[doc = "Repeated start detected. The Monitor function has detected a Repeated Start event on the I2C bus."]
    DETECTED,
}
impl From<MONRESTART_A> for bool {
    #[inline(always)]
    fn from(variant: MONRESTART_A) -> Self {
        match variant {
            MONRESTART_A::NOT_DETECTED => false,
            MONRESTART_A::DETECTED => true,
        }
    }
}
#[doc = "Reader of field `MONRESTART`"]
pub type MONRESTART_R = crate::R<bool, MONRESTART_A>;
impl MONRESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRESTART_A {
        match self.bits {
            false => MONRESTART_A::NOT_DETECTED,
            true => MONRESTART_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == MONRESTART_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == MONRESTART_A::DETECTED
    }
}
#[doc = "Possible values of the field `MONNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONNACK_A {
    #[doc = "Acknowledged. The data currently being provided by the Monitor function was acknowledged by at least one master or slave receiver."]
    ACKNOWLEDGED,
    #[doc = "Not acknowledged. The data currently being provided by the Monitor function was not acknowledged by any receiver."]
    NOT_ACKNOWLEDGED,
}
impl From<MONNACK_A> for bool {
    #[inline(always)]
    fn from(variant: MONNACK_A) -> Self {
        match variant {
            MONNACK_A::ACKNOWLEDGED => false,
            MONNACK_A::NOT_ACKNOWLEDGED => true,
        }
    }
}
#[doc = "Reader of field `MONNACK`"]
pub type MONNACK_R = crate::R<bool, MONNACK_A>;
impl MONNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONNACK_A {
        match self.bits {
            false => MONNACK_A::ACKNOWLEDGED,
            true => MONNACK_A::NOT_ACKNOWLEDGED,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_acknowledged(&self) -> bool {
        *self == MONNACK_A::ACKNOWLEDGED
    }
    #[doc = "Checks if the value of the field is `NOT_ACKNOWLEDGED`"]
    #[inline(always)]
    pub fn is_not_acknowledged(&self) -> bool {
        *self == MONNACK_A::NOT_ACKNOWLEDGED
    }
}
impl R {
    #[doc = "Bits 0:7 - Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub fn monrxdat(&self) -> MONRXDAT_R {
        MONRXDAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Monitor Received Start."]
    #[inline(always)]
    pub fn monstart(&self) -> MONSTART_R {
        MONSTART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Monitor Received Repeated Start."]
    #[inline(always)]
    pub fn monrestart(&self) -> MONRESTART_R {
        MONRESTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Monitor Received NACK."]
    #[inline(always)]
    pub fn monnack(&self) -> MONNACK_R {
        MONNACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
