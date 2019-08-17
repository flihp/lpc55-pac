#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    pub random_number: RANDOM_NUMBER,
    #[doc = "0x04 - This register contains a random 32 bit number which is pre-computed"]
    pub encrypted_number: ENCRYPTED_NUMBER,
    #[doc = "0x08 - no description available"]
    pub counter_val: COUNTER_VAL,
    #[doc = "0x0c - no description available"]
    pub counter_cfg: COUNTER_CFG,
    #[doc = "0x10 - no description available"]
    pub online_test_cfg: ONLINE_TEST_CFG,
    #[doc = "0x14 - no description available"]
    pub online_test_val: ONLINE_TEST_VAL,
    #[doc = "0x18 - no description available"]
    pub misc_cfg: MISC_CFG,
    _reserved7: [u8; 4056usize],
    #[doc = "0xff4 - Powerdown mode (standard but certainly useless here)"]
    pub powerdown: POWERDOWN,
    _reserved8: [u8; 4usize],
    #[doc = "0xffc - IP identifier"]
    pub moduleid: MODULEID,
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [random_number](random_number) module"]
pub type RANDOM_NUMBER = crate::Reg<u32, _RANDOM_NUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RANDOM_NUMBER;
#[doc = "`read()` method returns [random_number::R](random_number::R) reader structure"]
impl crate::Readable for RANDOM_NUMBER {}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub mod random_number;
#[doc = "This register contains a random 32 bit number which is pre-computed\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [encrypted_number](encrypted_number) module"]
pub type ENCRYPTED_NUMBER = crate::Reg<u32, _ENCRYPTED_NUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENCRYPTED_NUMBER;
#[doc = "`read()` method returns [encrypted_number::R](encrypted_number::R) reader structure"]
impl crate::Readable for ENCRYPTED_NUMBER {}
#[doc = "This register contains a random 32 bit number which is pre-computed"]
pub mod encrypted_number;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [counter_val](counter_val) module"]
pub type COUNTER_VAL = crate::Reg<u32, _COUNTER_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER_VAL;
#[doc = "`read()` method returns [counter_val::R](counter_val::R) reader structure"]
impl crate::Readable for COUNTER_VAL {}
#[doc = "`write(|w| ..)` method takes [counter_val::W](counter_val::W) writer structure"]
impl crate::Writable for COUNTER_VAL {}
#[doc = "no description available"]
pub mod counter_val;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [counter_cfg](counter_cfg) module"]
pub type COUNTER_CFG = crate::Reg<u32, _COUNTER_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER_CFG;
#[doc = "`read()` method returns [counter_cfg::R](counter_cfg::R) reader structure"]
impl crate::Readable for COUNTER_CFG {}
#[doc = "`write(|w| ..)` method takes [counter_cfg::W](counter_cfg::W) writer structure"]
impl crate::Writable for COUNTER_CFG {}
#[doc = "no description available"]
pub mod counter_cfg;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [online_test_cfg](online_test_cfg) module"]
pub type ONLINE_TEST_CFG = crate::Reg<u32, _ONLINE_TEST_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ONLINE_TEST_CFG;
#[doc = "`read()` method returns [online_test_cfg::R](online_test_cfg::R) reader structure"]
impl crate::Readable for ONLINE_TEST_CFG {}
#[doc = "`write(|w| ..)` method takes [online_test_cfg::W](online_test_cfg::W) writer structure"]
impl crate::Writable for ONLINE_TEST_CFG {}
#[doc = "no description available"]
pub mod online_test_cfg;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [online_test_val](online_test_val) module"]
pub type ONLINE_TEST_VAL = crate::Reg<u32, _ONLINE_TEST_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ONLINE_TEST_VAL;
#[doc = "`read()` method returns [online_test_val::R](online_test_val::R) reader structure"]
impl crate::Readable for ONLINE_TEST_VAL {}
#[doc = "`write(|w| ..)` method takes [online_test_val::W](online_test_val::W) writer structure"]
impl crate::Writable for ONLINE_TEST_VAL {}
#[doc = "no description available"]
pub mod online_test_val;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc_cfg](misc_cfg) module"]
pub type MISC_CFG = crate::Reg<u32, _MISC_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CFG;
#[doc = "`read()` method returns [misc_cfg::R](misc_cfg::R) reader structure"]
impl crate::Readable for MISC_CFG {}
#[doc = "`write(|w| ..)` method takes [misc_cfg::W](misc_cfg::W) writer structure"]
impl crate::Writable for MISC_CFG {}
#[doc = "no description available"]
pub mod misc_cfg;
#[doc = "Powerdown mode (standard but certainly useless here)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [powerdown](powerdown) module"]
pub type POWERDOWN = crate::Reg<u32, _POWERDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERDOWN;
#[doc = "`read()` method returns [powerdown::R](powerdown::R) reader structure"]
impl crate::Readable for POWERDOWN {}
#[doc = "`write(|w| ..)` method takes [powerdown::W](powerdown::W) writer structure"]
impl crate::Writable for POWERDOWN {}
#[doc = "Powerdown mode (standard but certainly useless here)"]
pub mod powerdown;
#[doc = "IP identifier\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [moduleid](moduleid) module"]
pub type MODULEID = crate::Reg<u32, _MODULEID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODULEID;
#[doc = "`read()` method returns [moduleid::R](moduleid::R) reader structure"]
impl crate::Readable for MODULEID {}
#[doc = "IP identifier"]
pub mod moduleid;
