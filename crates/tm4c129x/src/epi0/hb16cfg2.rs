#[doc = "Reader of register HB16CFG2"]
pub type R = crate::R<u32, super::HB16CFG2>;
#[doc = "Writer for register HB16CFG2"]
pub type W = crate::W<u32, super::HB16CFG2>;
#[doc = "Register HB16CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::HB16CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CS1n Host Bus Sub-Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: ADMUX - AD\\[15:0\\]"]
    ADMUX = 0,
    #[doc = "1: ADNONMUX - D\\[15:0\\]"]
    AD = 1,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::ADMUX),
            1 => Val(MODE_A::AD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADMUX`"]
    #[inline(always)]
    pub fn is_admux(&self) -> bool {
        *self == MODE_A::ADMUX
    }
    #[doc = "Checks if the value of the field is `AD`"]
    #[inline(always)]
    pub fn is_ad(&self) -> bool {
        *self == MODE_A::AD
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ADMUX - AD\\[15:0\\]"]
    #[inline(always)]
    pub fn admux(self) -> &'a mut W {
        self.variant(MODE_A::ADMUX)
    }
    #[doc = "ADNONMUX - D\\[15:0\\]"]
    #[inline(always)]
    pub fn ad(self) -> &'a mut W {
        self.variant(MODE_A::AD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "CS1n Read Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDWS_A {
    #[doc = "0: Active RDn is 2 EPI clocks"]
    _2 = 0,
    #[doc = "1: Active RDn is 4 EPI clocks"]
    _4 = 1,
    #[doc = "2: Active RDn is 6 EPI clocks"]
    _6 = 2,
    #[doc = "3: Active RDn is 8 EPI clocks"]
    _8 = 3,
}
impl From<RDWS_A> for u8 {
    #[inline(always)]
    fn from(variant: RDWS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RDWS`"]
pub type RDWS_R = crate::R<u8, RDWS_A>;
impl RDWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDWS_A {
        match self.bits {
            0 => RDWS_A::_2,
            1 => RDWS_A::_4,
            2 => RDWS_A::_6,
            3 => RDWS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RDWS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == RDWS_A::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == RDWS_A::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == RDWS_A::_8
    }
}
#[doc = "Write proxy for field `RDWS`"]
pub struct RDWS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDWS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RDWS_A::_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(RDWS_A::_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(RDWS_A::_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(RDWS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "CS1n Write Wait States\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WRWS_A {
    #[doc = "0: Active WRn is 2 EPI clocks"]
    _2 = 0,
    #[doc = "1: Active WRn is 4 EPI clocks"]
    _4 = 1,
    #[doc = "2: Active WRn is 6 EPI clocks"]
    _6 = 2,
    #[doc = "3: Active WRn is 8 EPI clocks"]
    _8 = 3,
}
impl From<WRWS_A> for u8 {
    #[inline(always)]
    fn from(variant: WRWS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WRWS`"]
pub type WRWS_R = crate::R<u8, WRWS_A>;
impl WRWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRWS_A {
        match self.bits {
            0 => WRWS_A::_2,
            1 => WRWS_A::_4,
            2 => WRWS_A::_6,
            3 => WRWS_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == WRWS_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == WRWS_A::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == WRWS_A::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WRWS_A::_8
    }
}
#[doc = "Write proxy for field `WRWS`"]
pub struct WRWS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRWS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(WRWS_A::_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(WRWS_A::_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WRWS_A::_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WRWS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
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
#[doc = "Reader of field `RDCRE`"]
pub type RDCRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDCRE`"]
pub struct RDCRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCRE_W<'a> {
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
#[doc = "Reader of field `WRCRE`"]
pub type WRCRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRCRE`"]
pub struct WRCRE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRCRE_W<'a> {
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
#[doc = "Reader of field `ALEHIGH`"]
pub type ALEHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALEHIGH`"]
pub struct ALEHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALEHIGH_W<'a> {
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
#[doc = "Reader of field `RDHIGH`"]
pub type RDHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDHIGH`"]
pub struct RDHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDHIGH_W<'a> {
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
#[doc = "Reader of field `WRHIGH`"]
pub type WRHIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRHIGH`"]
pub struct WRHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WRHIGH_W<'a> {
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
#[doc = "Chip Select Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSCFG_A {
    #[doc = "0: ALE Configuration"]
    ALE = 0,
    #[doc = "1: CSn Configuration"]
    CS = 1,
    #[doc = "2: Dual CSn Configuration"]
    DCS = 2,
    #[doc = "3: ALE with Dual CSn Configuration"]
    ADCS = 3,
}
impl From<CSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CSCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSCFG`"]
pub type CSCFG_R = crate::R<u8, CSCFG_A>;
impl CSCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSCFG_A {
        match self.bits {
            0 => CSCFG_A::ALE,
            1 => CSCFG_A::CS,
            2 => CSCFG_A::DCS,
            3 => CSCFG_A::ADCS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALE`"]
    #[inline(always)]
    pub fn is_ale(&self) -> bool {
        *self == CSCFG_A::ALE
    }
    #[doc = "Checks if the value of the field is `CS`"]
    #[inline(always)]
    pub fn is_cs(&self) -> bool {
        *self == CSCFG_A::CS
    }
    #[doc = "Checks if the value of the field is `DCS`"]
    #[inline(always)]
    pub fn is_dcs(&self) -> bool {
        *self == CSCFG_A::DCS
    }
    #[doc = "Checks if the value of the field is `ADCS`"]
    #[inline(always)]
    pub fn is_adcs(&self) -> bool {
        *self == CSCFG_A::ADCS
    }
}
#[doc = "Write proxy for field `CSCFG`"]
pub struct CSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ALE Configuration"]
    #[inline(always)]
    pub fn ale(self) -> &'a mut W {
        self.variant(CSCFG_A::ALE)
    }
    #[doc = "CSn Configuration"]
    #[inline(always)]
    pub fn cs(self) -> &'a mut W {
        self.variant(CSCFG_A::CS)
    }
    #[doc = "Dual CSn Configuration"]
    #[inline(always)]
    pub fn dcs(self) -> &'a mut W {
        self.variant(CSCFG_A::DCS)
    }
    #[doc = "ALE with Dual CSn Configuration"]
    #[inline(always)]
    pub fn adcs(self) -> &'a mut W {
        self.variant(CSCFG_A::ADCS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CSBAUD`"]
pub type CSBAUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSBAUD`"]
pub struct CSBAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBAUD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CSCFGEXT`"]
pub type CSCFGEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSCFGEXT`"]
pub struct CSCFGEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCFGEXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn rdws(&self) -> RDWS_R {
        RDWS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn wrws(&self) -> WRWS_R {
        WRWS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 16 - CS1n Burst Mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CS1n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&self) -> RDCRE_R {
        RDCRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CS1n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&self) -> WRCRE_R {
        WRCRE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&self) -> ALEHIGH_R {
        ALEHIGH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&self) -> RDHIGH_R {
        RDHIGH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&self) -> WRHIGH_R {
        WRHIGH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn cscfg(&self) -> CSCFG_R {
        CSCFG_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn csbaud(&self) -> CSBAUD_R {
        CSBAUD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn cscfgext(&self) -> CSCFGEXT_R {
        CSCFGEXT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS1n Host Bus Sub-Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - CS1n Read Wait States"]
    #[inline(always)]
    pub fn rdws(&mut self) -> RDWS_W {
        RDWS_W { w: self }
    }
    #[doc = "Bits 6:7 - CS1n Write Wait States"]
    #[inline(always)]
    pub fn wrws(&mut self) -> WRWS_W {
        WRWS_W { w: self }
    }
    #[doc = "Bit 16 - CS1n Burst Mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 17 - CS1n PSRAM Configuration Register Read"]
    #[inline(always)]
    pub fn rdcre(&mut self) -> RDCRE_W {
        RDCRE_W { w: self }
    }
    #[doc = "Bit 18 - CS1n PSRAM Configuration Register Write"]
    #[inline(always)]
    pub fn wrcre(&mut self) -> WRCRE_W {
        WRCRE_W { w: self }
    }
    #[doc = "Bit 19 - CS1n ALE Strobe Polarity"]
    #[inline(always)]
    pub fn alehigh(&mut self) -> ALEHIGH_W {
        ALEHIGH_W { w: self }
    }
    #[doc = "Bit 20 - CS1n READ Strobe Polarity"]
    #[inline(always)]
    pub fn rdhigh(&mut self) -> RDHIGH_W {
        RDHIGH_W { w: self }
    }
    #[doc = "Bit 21 - CS1n WRITE Strobe Polarity"]
    #[inline(always)]
    pub fn wrhigh(&mut self) -> WRHIGH_W {
        WRHIGH_W { w: self }
    }
    #[doc = "Bits 24:25 - Chip Select Configuration"]
    #[inline(always)]
    pub fn cscfg(&mut self) -> CSCFG_W {
        CSCFG_W { w: self }
    }
    #[doc = "Bit 26 - Chip Select Baud Rate and Multiple Sub-Mode Configuration enable"]
    #[inline(always)]
    pub fn csbaud(&mut self) -> CSBAUD_W {
        CSBAUD_W { w: self }
    }
    #[doc = "Bit 27 - Chip Select Extended Configuration"]
    #[inline(always)]
    pub fn cscfgext(&mut self) -> CSCFGEXT_W {
        CSCFGEXT_W { w: self }
    }
}
