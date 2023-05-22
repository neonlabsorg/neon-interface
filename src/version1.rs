use abi_stable::{
    std_types::{ROption, RSlice, RStr},
    StableAbi,
};

use crate::types::{BoxedConfig, RAddress, RNeonCliResult, RPubkey, RTxParams, RU256};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
#[sabi(missing_field(panic))]
pub struct Version1 {
    pub init_config: extern "C" fn() -> BoxedConfig<'static>,
    pub cancel_trx: extern "C" fn(&BoxedConfig, &RPubkey) -> RNeonCliResult,
    pub collect_treasury: extern "C" fn(&BoxedConfig) -> RNeonCliResult,
    pub create_ether_account: extern "C" fn(&BoxedConfig, &RAddress) -> RNeonCliResult,
    pub deposit: extern "C" fn(&BoxedConfig, u64, &RAddress) -> RNeonCliResult,
    pub emulate: extern "C" fn(
        &BoxedConfig,
        RTxParams,
        RPubkey,
        u64,
        u64,
        RSlice<RAddress>,
    ) -> RNeonCliResult,
    pub get_ether_account_data: extern "C" fn(&BoxedConfig, &RAddress) -> RNeonCliResult,
    pub get_neon_elf: extern "C" fn(&BoxedConfig, ROption<RStr>) -> RNeonCliResult,
    pub get_storage_at: extern "C" fn(&BoxedConfig, RAddress, &RU256) -> RNeonCliResult,
    pub init_environment:
        extern "C" fn(&BoxedConfig, bool, bool, ROption<RStr>, ROption<RStr>) -> RNeonCliResult,
    #[sabi(last_prefix_field)]
    pub trace: extern "C" fn(
        &BoxedConfig,
        RTxParams,
        RPubkey,
        u64,
        u64,
        RSlice<RAddress>,
    ) -> RNeonCliResult,
}
