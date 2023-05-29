use abi_stable::{
    std_types::{ROption, RResult, RStr, RVec},
    StableAbi,
};

use crate::types::{
    BoxedConfig, BoxedContext, BoxedNeonCliError, RAPIOptions, RAddress, RNeonCliResult, RPubkey,
    RTxParams, RU256,
};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
#[sabi(missing_field(panic))]
pub struct Version1 {
    pub init_config:
        extern "C" fn(RAPIOptions) -> RResult<BoxedConfig<'static>, BoxedNeonCliError<'static>>,
    pub init_context: extern "C" fn() -> BoxedContext<'static>,
    pub cancel_trx: extern "C" fn(&BoxedConfig, &BoxedContext, RPubkey) -> RNeonCliResult,
    pub collect_treasury: extern "C" fn(&BoxedConfig, &BoxedContext) -> RNeonCliResult,
    pub create_ether_account:
        extern "C" fn(&BoxedConfig, &BoxedContext, &RAddress) -> RNeonCliResult,
    pub deposit: extern "C" fn(&BoxedConfig, &BoxedContext, u64, &RAddress) -> RNeonCliResult,
    pub emulate: extern "C" fn(
        &BoxedConfig,
        &BoxedContext,
        RTxParams,
        RPubkey,
        u64,
        u64,
        RVec<RAddress>,
        RVec<RPubkey>,
    ) -> RNeonCliResult,
    pub get_ether_account_data:
        extern "C" fn(&BoxedConfig, &BoxedContext, &RAddress) -> RNeonCliResult,
    pub get_neon_elf: extern "C" fn(&BoxedConfig, &BoxedContext, ROption<RStr>) -> RNeonCliResult,
    pub get_storage_at:
        extern "C" fn(&BoxedConfig, &BoxedContext, RAddress, RU256) -> RNeonCliResult,
    #[sabi(last_prefix_field)]
    pub init_environment: extern "C" fn(
        &BoxedConfig,
        &BoxedContext,
        bool,
        bool,
        ROption<RStr>,
        ROption<RStr>,
    ) -> RNeonCliResult,
}
