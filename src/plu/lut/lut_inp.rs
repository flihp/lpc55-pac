#[doc = "Reader of register LUT_INP%s"]
pub type R = crate::R<u32, super::LUT_INP>;
#[doc = "Writer for register LUT_INP%s"]
pub type W = crate::W<u32, super::LUT_INP>;
#[doc = "Register LUT_INP%s `reset()`'s with value 0"]
impl crate::ResetValue for super::LUT_INP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `LUT_INP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_INP_A {
    #[doc = "The PLU primary inputs 0."]
    PLU_INPUTS0,
    #[doc = "The PLU primary inputs 1."]
    PLU_INPUTS1,
    #[doc = "The PLU primary inputs 2."]
    PLU_INPUTS2,
    #[doc = "The PLU primary inputs 3."]
    PLU_INPUTS3,
    #[doc = "The PLU primary inputs 4."]
    PLU_INPUTS4,
    #[doc = "The PLU primary inputs 5."]
    PLU_INPUTS5,
    #[doc = "The output of LUT0."]
    LUT_OUTPUTS0,
    #[doc = "The output of LUT1."]
    LUT_OUTPUTS1,
    #[doc = "The output of LUT2."]
    LUT_OUTPUTS2,
    #[doc = "The output of LUT3."]
    LUT_OUTPUTS3,
    #[doc = "The output of LUT4."]
    LUT_OUTPUTS4,
    #[doc = "The output of LUT5."]
    LUT_OUTPUTS5,
    #[doc = "The output of LUT6."]
    LUT_OUTPUTS6,
    #[doc = "The output of LUT7."]
    LUT_OUTPUTS7,
    #[doc = "The output of LUT8."]
    LUT_OUTPUTS8,
    #[doc = "The output of LUT9."]
    LUT_OUTPUTS9,
    #[doc = "The output of LUT10."]
    LUT_OUTPUTS10,
    #[doc = "The output of LUT11."]
    LUT_OUTPUTS11,
    #[doc = "The output of LUT12."]
    LUT_OUTPUTS12,
    #[doc = "The output of LUT13."]
    LUT_OUTPUTS13,
    #[doc = "The output of LUT14."]
    LUT_OUTPUTS14,
    #[doc = "The output of LUT15."]
    LUT_OUTPUTS15,
    #[doc = "The output of LUT16."]
    LUT_OUTPUTS16,
    #[doc = "The output of LUT17."]
    LUT_OUTPUTS17,
    #[doc = "The output of LUT18."]
    LUT_OUTPUTS18,
    #[doc = "The output of LUT19."]
    LUT_OUTPUTS19,
    #[doc = "The output of LUT20."]
    LUT_OUTPUTS20,
    #[doc = "The output of LUT21."]
    LUT_OUTPUTS21,
    #[doc = "The output of LUT22."]
    LUT_OUTPUTS22,
    #[doc = "The output of LUT23."]
    LUT_OUTPUTS23,
    #[doc = "The output of LUT24."]
    LUT_OUTPUTS24,
    #[doc = "The output of LUT25."]
    LUT_OUTPUTS25,
    #[doc = "state(0)."]
    STATE0,
    #[doc = "state(1)."]
    STATE1,
    #[doc = "state(2)."]
    STATE2,
    #[doc = "state(3)."]
    STATE3,
}
impl From<LUT_INP_A> for u8 {
    #[inline(always)]
    fn from(variant: LUT_INP_A) -> Self {
        match variant {
            LUT_INP_A::PLU_INPUTS0 => 0,
            LUT_INP_A::PLU_INPUTS1 => 1,
            LUT_INP_A::PLU_INPUTS2 => 2,
            LUT_INP_A::PLU_INPUTS3 => 3,
            LUT_INP_A::PLU_INPUTS4 => 4,
            LUT_INP_A::PLU_INPUTS5 => 5,
            LUT_INP_A::LUT_OUTPUTS0 => 6,
            LUT_INP_A::LUT_OUTPUTS1 => 7,
            LUT_INP_A::LUT_OUTPUTS2 => 8,
            LUT_INP_A::LUT_OUTPUTS3 => 9,
            LUT_INP_A::LUT_OUTPUTS4 => 10,
            LUT_INP_A::LUT_OUTPUTS5 => 11,
            LUT_INP_A::LUT_OUTPUTS6 => 12,
            LUT_INP_A::LUT_OUTPUTS7 => 13,
            LUT_INP_A::LUT_OUTPUTS8 => 14,
            LUT_INP_A::LUT_OUTPUTS9 => 15,
            LUT_INP_A::LUT_OUTPUTS10 => 16,
            LUT_INP_A::LUT_OUTPUTS11 => 17,
            LUT_INP_A::LUT_OUTPUTS12 => 18,
            LUT_INP_A::LUT_OUTPUTS13 => 19,
            LUT_INP_A::LUT_OUTPUTS14 => 20,
            LUT_INP_A::LUT_OUTPUTS15 => 21,
            LUT_INP_A::LUT_OUTPUTS16 => 22,
            LUT_INP_A::LUT_OUTPUTS17 => 23,
            LUT_INP_A::LUT_OUTPUTS18 => 24,
            LUT_INP_A::LUT_OUTPUTS19 => 25,
            LUT_INP_A::LUT_OUTPUTS20 => 26,
            LUT_INP_A::LUT_OUTPUTS21 => 27,
            LUT_INP_A::LUT_OUTPUTS22 => 28,
            LUT_INP_A::LUT_OUTPUTS23 => 29,
            LUT_INP_A::LUT_OUTPUTS24 => 30,
            LUT_INP_A::LUT_OUTPUTS25 => 31,
            LUT_INP_A::STATE0 => 32,
            LUT_INP_A::STATE1 => 33,
            LUT_INP_A::STATE2 => 34,
            LUT_INP_A::STATE3 => 35,
        }
    }
}
#[doc = "Reader of field `LUT_INP`"]
pub type LUT_INP_R = crate::R<u8, LUT_INP_A>;
impl LUT_INP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LUT_INP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LUT_INP_A::PLU_INPUTS0),
            1 => Val(LUT_INP_A::PLU_INPUTS1),
            2 => Val(LUT_INP_A::PLU_INPUTS2),
            3 => Val(LUT_INP_A::PLU_INPUTS3),
            4 => Val(LUT_INP_A::PLU_INPUTS4),
            5 => Val(LUT_INP_A::PLU_INPUTS5),
            6 => Val(LUT_INP_A::LUT_OUTPUTS0),
            7 => Val(LUT_INP_A::LUT_OUTPUTS1),
            8 => Val(LUT_INP_A::LUT_OUTPUTS2),
            9 => Val(LUT_INP_A::LUT_OUTPUTS3),
            10 => Val(LUT_INP_A::LUT_OUTPUTS4),
            11 => Val(LUT_INP_A::LUT_OUTPUTS5),
            12 => Val(LUT_INP_A::LUT_OUTPUTS6),
            13 => Val(LUT_INP_A::LUT_OUTPUTS7),
            14 => Val(LUT_INP_A::LUT_OUTPUTS8),
            15 => Val(LUT_INP_A::LUT_OUTPUTS9),
            16 => Val(LUT_INP_A::LUT_OUTPUTS10),
            17 => Val(LUT_INP_A::LUT_OUTPUTS11),
            18 => Val(LUT_INP_A::LUT_OUTPUTS12),
            19 => Val(LUT_INP_A::LUT_OUTPUTS13),
            20 => Val(LUT_INP_A::LUT_OUTPUTS14),
            21 => Val(LUT_INP_A::LUT_OUTPUTS15),
            22 => Val(LUT_INP_A::LUT_OUTPUTS16),
            23 => Val(LUT_INP_A::LUT_OUTPUTS17),
            24 => Val(LUT_INP_A::LUT_OUTPUTS18),
            25 => Val(LUT_INP_A::LUT_OUTPUTS19),
            26 => Val(LUT_INP_A::LUT_OUTPUTS20),
            27 => Val(LUT_INP_A::LUT_OUTPUTS21),
            28 => Val(LUT_INP_A::LUT_OUTPUTS22),
            29 => Val(LUT_INP_A::LUT_OUTPUTS23),
            30 => Val(LUT_INP_A::LUT_OUTPUTS24),
            31 => Val(LUT_INP_A::LUT_OUTPUTS25),
            32 => Val(LUT_INP_A::STATE0),
            33 => Val(LUT_INP_A::STATE1),
            34 => Val(LUT_INP_A::STATE2),
            35 => Val(LUT_INP_A::STATE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS0`"]
    #[inline(always)]
    pub fn is_plu_inputs0(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS0
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS1`"]
    #[inline(always)]
    pub fn is_plu_inputs1(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS1
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS2`"]
    #[inline(always)]
    pub fn is_plu_inputs2(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS2
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS3`"]
    #[inline(always)]
    pub fn is_plu_inputs3(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS3
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS4`"]
    #[inline(always)]
    pub fn is_plu_inputs4(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS4
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS5`"]
    #[inline(always)]
    pub fn is_plu_inputs5(&self) -> bool {
        *self == LUT_INP_A::PLU_INPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS0`"]
    #[inline(always)]
    pub fn is_lut_outputs0(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS0
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS1`"]
    #[inline(always)]
    pub fn is_lut_outputs1(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS1
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS2`"]
    #[inline(always)]
    pub fn is_lut_outputs2(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS2
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS3`"]
    #[inline(always)]
    pub fn is_lut_outputs3(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS3
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS4`"]
    #[inline(always)]
    pub fn is_lut_outputs4(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS4
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS5`"]
    #[inline(always)]
    pub fn is_lut_outputs5(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS6`"]
    #[inline(always)]
    pub fn is_lut_outputs6(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS6
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS7`"]
    #[inline(always)]
    pub fn is_lut_outputs7(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS7
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS8`"]
    #[inline(always)]
    pub fn is_lut_outputs8(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS8
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS9`"]
    #[inline(always)]
    pub fn is_lut_outputs9(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS9
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS10`"]
    #[inline(always)]
    pub fn is_lut_outputs10(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS10
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS11`"]
    #[inline(always)]
    pub fn is_lut_outputs11(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS11
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS12`"]
    #[inline(always)]
    pub fn is_lut_outputs12(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS12
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS13`"]
    #[inline(always)]
    pub fn is_lut_outputs13(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS13
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS14`"]
    #[inline(always)]
    pub fn is_lut_outputs14(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS14
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS15`"]
    #[inline(always)]
    pub fn is_lut_outputs15(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS15
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS16`"]
    #[inline(always)]
    pub fn is_lut_outputs16(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS16
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS17`"]
    #[inline(always)]
    pub fn is_lut_outputs17(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS17
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS18`"]
    #[inline(always)]
    pub fn is_lut_outputs18(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS18
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS19`"]
    #[inline(always)]
    pub fn is_lut_outputs19(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS19
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS20`"]
    #[inline(always)]
    pub fn is_lut_outputs20(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS20
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS21`"]
    #[inline(always)]
    pub fn is_lut_outputs21(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS21
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS22`"]
    #[inline(always)]
    pub fn is_lut_outputs22(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS22
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS23`"]
    #[inline(always)]
    pub fn is_lut_outputs23(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS23
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS24`"]
    #[inline(always)]
    pub fn is_lut_outputs24(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS24
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS25`"]
    #[inline(always)]
    pub fn is_lut_outputs25(&self) -> bool {
        *self == LUT_INP_A::LUT_OUTPUTS25
    }
    #[doc = "Checks if the value of the field is `STATE0`"]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        *self == LUT_INP_A::STATE0
    }
    #[doc = "Checks if the value of the field is `STATE1`"]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        *self == LUT_INP_A::STATE1
    }
    #[doc = "Checks if the value of the field is `STATE2`"]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        *self == LUT_INP_A::STATE2
    }
    #[doc = "Checks if the value of the field is `STATE3`"]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        *self == LUT_INP_A::STATE3
    }
}
#[doc = "Write proxy for field `LUT_INP`"]
pub struct LUT_INP_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LUT_INP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The PLU primary inputs 0."]
    #[inline(always)]
    pub fn plu_inputs0(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS0)
    }
    #[doc = "The PLU primary inputs 1."]
    #[inline(always)]
    pub fn plu_inputs1(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS1)
    }
    #[doc = "The PLU primary inputs 2."]
    #[inline(always)]
    pub fn plu_inputs2(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS2)
    }
    #[doc = "The PLU primary inputs 3."]
    #[inline(always)]
    pub fn plu_inputs3(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS3)
    }
    #[doc = "The PLU primary inputs 4."]
    #[inline(always)]
    pub fn plu_inputs4(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS4)
    }
    #[doc = "The PLU primary inputs 5."]
    #[inline(always)]
    pub fn plu_inputs5(self) -> &'a mut W {
        self.variant(LUT_INP_A::PLU_INPUTS5)
    }
    #[doc = "The output of LUT0."]
    #[inline(always)]
    pub fn lut_outputs0(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS0)
    }
    #[doc = "The output of LUT1."]
    #[inline(always)]
    pub fn lut_outputs1(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS1)
    }
    #[doc = "The output of LUT2."]
    #[inline(always)]
    pub fn lut_outputs2(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS2)
    }
    #[doc = "The output of LUT3."]
    #[inline(always)]
    pub fn lut_outputs3(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS3)
    }
    #[doc = "The output of LUT4."]
    #[inline(always)]
    pub fn lut_outputs4(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS4)
    }
    #[doc = "The output of LUT5."]
    #[inline(always)]
    pub fn lut_outputs5(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS5)
    }
    #[doc = "The output of LUT6."]
    #[inline(always)]
    pub fn lut_outputs6(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS6)
    }
    #[doc = "The output of LUT7."]
    #[inline(always)]
    pub fn lut_outputs7(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS7)
    }
    #[doc = "The output of LUT8."]
    #[inline(always)]
    pub fn lut_outputs8(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS8)
    }
    #[doc = "The output of LUT9."]
    #[inline(always)]
    pub fn lut_outputs9(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS9)
    }
    #[doc = "The output of LUT10."]
    #[inline(always)]
    pub fn lut_outputs10(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS10)
    }
    #[doc = "The output of LUT11."]
    #[inline(always)]
    pub fn lut_outputs11(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS11)
    }
    #[doc = "The output of LUT12."]
    #[inline(always)]
    pub fn lut_outputs12(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS12)
    }
    #[doc = "The output of LUT13."]
    #[inline(always)]
    pub fn lut_outputs13(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS13)
    }
    #[doc = "The output of LUT14."]
    #[inline(always)]
    pub fn lut_outputs14(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS14)
    }
    #[doc = "The output of LUT15."]
    #[inline(always)]
    pub fn lut_outputs15(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS15)
    }
    #[doc = "The output of LUT16."]
    #[inline(always)]
    pub fn lut_outputs16(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS16)
    }
    #[doc = "The output of LUT17."]
    #[inline(always)]
    pub fn lut_outputs17(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS17)
    }
    #[doc = "The output of LUT18."]
    #[inline(always)]
    pub fn lut_outputs18(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS18)
    }
    #[doc = "The output of LUT19."]
    #[inline(always)]
    pub fn lut_outputs19(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS19)
    }
    #[doc = "The output of LUT20."]
    #[inline(always)]
    pub fn lut_outputs20(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS20)
    }
    #[doc = "The output of LUT21."]
    #[inline(always)]
    pub fn lut_outputs21(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS21)
    }
    #[doc = "The output of LUT22."]
    #[inline(always)]
    pub fn lut_outputs22(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS22)
    }
    #[doc = "The output of LUT23."]
    #[inline(always)]
    pub fn lut_outputs23(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS23)
    }
    #[doc = "The output of LUT24."]
    #[inline(always)]
    pub fn lut_outputs24(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS24)
    }
    #[doc = "The output of LUT25."]
    #[inline(always)]
    pub fn lut_outputs25(self) -> &'a mut W {
        self.variant(LUT_INP_A::LUT_OUTPUTS25)
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut W {
        self.variant(LUT_INP_A::STATE0)
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut W {
        self.variant(LUT_INP_A::STATE1)
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut W {
        self.variant(LUT_INP_A::STATE2)
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut W {
        self.variant(LUT_INP_A::STATE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0."]
    #[inline(always)]
    pub fn lut_inp(&self) -> LUT_INP_R {
        LUT_INP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0."]
    #[inline(always)]
    pub fn lut_inp(&mut self) -> LUT_INP_W {
        LUT_INP_W { w: self }
    }
}
