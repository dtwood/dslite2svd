#[doc = "Reader of register DSLPPWRCFG"]
pub type R = crate::R<u32, super::DSLPPWRCFG>;
#[doc = "Writer for register DSLPPWRCFG"]
pub type W = crate::W<u32, super::DSLPPWRCFG>;
#[doc = "Register DSLPPWRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DSLPPWRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SRAM Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRAMPM_A {
    #[doc = "0: Active Mode"]
    NRM = 0,
    #[doc = "1: Standby Mode"]
    SBY = 1,
    #[doc = "3: Low Power Mode"]
    LP = 3,
}
impl From<SRAMPM_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMPM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRAMPM`"]
pub type SRAMPM_R = crate::R<u8, SRAMPM_A>;
impl SRAMPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRAMPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRAMPM_A::NRM),
            1 => Val(SRAMPM_A::SBY),
            3 => Val(SRAMPM_A::LP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NRM`"]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == SRAMPM_A::NRM
    }
    #[doc = "Checks if the value of the field is `SBY`"]
    #[inline(always)]
    pub fn is_sby(&self) -> bool {
        *self == SRAMPM_A::SBY
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == SRAMPM_A::LP
    }
}
#[doc = "Write proxy for field `SRAMPM`"]
pub struct SRAMPM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut W {
        self.variant(SRAMPM_A::NRM)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn sby(self) -> &'a mut W {
        self.variant(SRAMPM_A::SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(SRAMPM_A::LP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Flash Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHPM_A {
    #[doc = "0: Active Mode"]
    NRM = 0,
    #[doc = "2: Low Power Mode"]
    SLP = 2,
}
impl From<FLASHPM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHPM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASHPM`"]
pub type FLASHPM_R = crate::R<u8, FLASHPM_A>;
impl FLASHPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASHPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLASHPM_A::NRM),
            2 => Val(FLASHPM_A::SLP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NRM`"]
    #[inline(always)]
    pub fn is_nrm(&self) -> bool {
        *self == FLASHPM_A::NRM
    }
    #[doc = "Checks if the value of the field is `SLP`"]
    #[inline(always)]
    pub fn is_slp(&self) -> bool {
        *self == FLASHPM_A::SLP
    }
}
#[doc = "Write proxy for field `FLASHPM`"]
pub struct FLASHPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn nrm(self) -> &'a mut W {
        self.variant(FLASHPM_A::NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn slp(self) -> &'a mut W {
        self.variant(FLASHPM_A::SLP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn srampm(&self) -> SRAMPM_R {
        SRAMPM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn flashpm(&self) -> FLASHPM_R {
        FLASHPM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn srampm(&mut self) -> SRAMPM_W {
        SRAMPM_W { w: self }
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn flashpm(&mut self) -> FLASHPM_W {
        FLASHPM_W { w: self }
    }
}
