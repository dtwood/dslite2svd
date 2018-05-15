#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SSIZE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "32 KB of SRAM"]
    _32KB,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SIZER::_32KB => 127,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SIZER {
        match value {
            127 => SIZER::_32KB,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32KB`"]
    #[inline]
    pub fn is_32kb(&self) -> bool {
        *self == SIZER::_32KB
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - SRAM Size"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
}
