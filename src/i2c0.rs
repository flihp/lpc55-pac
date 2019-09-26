#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 2048usize],
    #[doc = "0x800 - Configuration for shared functions."]
    pub cfg: CFG,
    #[doc = "0x804 - Status register for Master, Slave, and Monitor functions."]
    pub stat: STAT,
    #[doc = "0x808 - Interrupt Enable Set and read register."]
    pub intenset: INTENSET,
    #[doc = "0x80c - Interrupt Enable Clear register."]
    pub intenclr: INTENCLR,
    #[doc = "0x810 - Time-out value register."]
    pub timeout: TIMEOUT,
    #[doc = "0x814 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    pub clkdiv: CLKDIV,
    #[doc = "0x818 - Interrupt Status register for Master, Slave, and Monitor functions."]
    pub intstat: INTSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x820 - Master control register."]
    pub mstctl: MSTCTL,
    #[doc = "0x824 - Master timing configuration."]
    pub msttime: MSTTIME,
    #[doc = "0x828 - Combined Master receiver and transmitter data register."]
    pub mstdat: MSTDAT,
    _reserved10: [u8; 20usize],
    #[doc = "0x840 - Slave control register."]
    pub slvctl: SLVCTL,
    #[doc = "0x844 - Combined Slave receiver and transmitter data register."]
    pub slvdat: SLVDAT,
    #[doc = "0x848 - Slave address register."]
    pub slvadr0: SLVADR0,
    #[doc = "0x84c - Slave address register."]
    pub slvadr1: SLVADR1,
    #[doc = "0x850 - Slave address register."]
    pub slvadr2: SLVADR2,
    #[doc = "0x854 - Slave address register."]
    pub slvadr3: SLVADR3,
    #[doc = "0x858 - Slave Qualification for address 0."]
    pub slvqual0: SLVQUAL0,
    _reserved17: [u8; 36usize],
    #[doc = "0x880 - Monitor receiver data register."]
    pub monrxdat: MONRXDAT,
    _reserved18: [u8; 1912usize],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "Configuration for shared functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "Interrupt Enable Set and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "Time-out value register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [timeout](timeout) module"]
pub type TIMEOUT = crate::Reg<u32, _TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT;
#[doc = "`read()` method returns [timeout::R](timeout::R) reader structure"]
impl crate::Readable for TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [timeout::W](timeout::W) writer structure"]
impl crate::Writable for TIMEOUT {}
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub mod clkdiv;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "Master control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mstctl](mstctl) module"]
pub type MSTCTL = crate::Reg<u32, _MSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTCTL;
#[doc = "`read()` method returns [mstctl::R](mstctl::R) reader structure"]
impl crate::Readable for MSTCTL {}
#[doc = "`write(|w| ..)` method takes [mstctl::W](mstctl::W) writer structure"]
impl crate::Writable for MSTCTL {}
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "Master timing configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msttime](msttime) module"]
pub type MSTTIME = crate::Reg<u32, _MSTTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTTIME;
#[doc = "`read()` method returns [msttime::R](msttime::R) reader structure"]
impl crate::Readable for MSTTIME {}
#[doc = "`write(|w| ..)` method takes [msttime::W](msttime::W) writer structure"]
impl crate::Writable for MSTTIME {}
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "Combined Master receiver and transmitter data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mstdat](mstdat) module"]
pub type MSTDAT = crate::Reg<u32, _MSTDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTDAT;
#[doc = "`read()` method returns [mstdat::R](mstdat::R) reader structure"]
impl crate::Readable for MSTDAT {}
#[doc = "`write(|w| ..)` method takes [mstdat::W](mstdat::W) writer structure"]
impl crate::Writable for MSTDAT {}
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "Slave control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvctl](slvctl) module"]
pub type SLVCTL = crate::Reg<u32, _SLVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVCTL;
#[doc = "`read()` method returns [slvctl::R](slvctl::R) reader structure"]
impl crate::Readable for SLVCTL {}
#[doc = "`write(|w| ..)` method takes [slvctl::W](slvctl::W) writer structure"]
impl crate::Writable for SLVCTL {}
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "Combined Slave receiver and transmitter data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvdat](slvdat) module"]
pub type SLVDAT = crate::Reg<u32, _SLVDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVDAT;
#[doc = "`read()` method returns [slvdat::R](slvdat::R) reader structure"]
impl crate::Readable for SLVDAT {}
#[doc = "`write(|w| ..)` method takes [slvdat::W](slvdat::W) writer structure"]
impl crate::Writable for SLVDAT {}
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvadr0](slvadr0) module"]
pub type SLVADR0 = crate::Reg<u32, _SLVADR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVADR0;
#[doc = "`read()` method returns [slvadr0::R](slvadr0::R) reader structure"]
impl crate::Readable for SLVADR0 {}
#[doc = "`write(|w| ..)` method takes [slvadr0::W](slvadr0::W) writer structure"]
impl crate::Writable for SLVADR0 {}
#[doc = "Slave address register."]
pub mod slvadr0;
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvadr1](slvadr1) module"]
pub type SLVADR1 = crate::Reg<u32, _SLVADR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVADR1;
#[doc = "`read()` method returns [slvadr1::R](slvadr1::R) reader structure"]
impl crate::Readable for SLVADR1 {}
#[doc = "`write(|w| ..)` method takes [slvadr1::W](slvadr1::W) writer structure"]
impl crate::Writable for SLVADR1 {}
#[doc = "Slave address register."]
pub mod slvadr1;
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvadr2](slvadr2) module"]
pub type SLVADR2 = crate::Reg<u32, _SLVADR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVADR2;
#[doc = "`read()` method returns [slvadr2::R](slvadr2::R) reader structure"]
impl crate::Readable for SLVADR2 {}
#[doc = "`write(|w| ..)` method takes [slvadr2::W](slvadr2::W) writer structure"]
impl crate::Writable for SLVADR2 {}
#[doc = "Slave address register."]
pub mod slvadr2;
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvadr3](slvadr3) module"]
pub type SLVADR3 = crate::Reg<u32, _SLVADR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVADR3;
#[doc = "`read()` method returns [slvadr3::R](slvadr3::R) reader structure"]
impl crate::Readable for SLVADR3 {}
#[doc = "`write(|w| ..)` method takes [slvadr3::W](slvadr3::W) writer structure"]
impl crate::Writable for SLVADR3 {}
#[doc = "Slave address register."]
pub mod slvadr3;
#[doc = "Slave Qualification for address 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slvqual0](slvqual0) module"]
pub type SLVQUAL0 = crate::Reg<u32, _SLVQUAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVQUAL0;
#[doc = "`read()` method returns [slvqual0::R](slvqual0::R) reader structure"]
impl crate::Readable for SLVQUAL0 {}
#[doc = "`write(|w| ..)` method takes [slvqual0::W](slvqual0::W) writer structure"]
impl crate::Writable for SLVQUAL0 {}
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "Monitor receiver data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [monrxdat](monrxdat) module"]
pub type MONRXDAT = crate::Reg<u32, _MONRXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MONRXDAT;
#[doc = "`read()` method returns [monrxdat::R](monrxdat::R) reader structure"]
impl crate::Readable for MONRXDAT {}
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
#[doc = "Peripheral identification register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u32, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "Peripheral identification register."]
pub mod id;
