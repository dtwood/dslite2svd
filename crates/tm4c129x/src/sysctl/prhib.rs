#[doc = "Reader of register PRHIB"]
pub type R = crate::R<u32, super::PRHIB>;
#[doc = "Reader of field `R0`"]
pub type R0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Peripheral Ready"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new((self.bits & 0x01) != 0)
    }
}
