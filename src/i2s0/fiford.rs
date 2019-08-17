#[doc = "Reader of register FIFORD"]
pub type R = crate::R<u32, super::FIFORD>;
#[doc = "Reader of field `RXDATA`"]
pub type RXDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Received data from the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
