#[doc = "Reader of register PPPECI"]
pub type R = crate::R<u32, super::PPPECI>;
#[doc = "Reader of field `P0`"]
pub type P0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - PECI Module Present"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
}
