use abi_stable::{
    std_types::{RBox, ROption, RResult, RString, RVec},
    DynTrait, StableAbi,
};
use ethnum::U256;
use solana_sdk::pubkey::Pubkey;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug))]
pub struct ConfigOpaque;

pub type BoxedConfig<'borr> = DynTrait<'borr, RBox<()>, ConfigOpaque>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType())]
pub struct ContextOpaque;

pub type BoxedContext<'borr> = DynTrait<'borr, RBox<()>, ContextOpaque>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug, Display))]
pub struct NeonCliErrorOpaque;

pub type BoxedNeonCliError<'borr> = DynTrait<'borr, RBox<()>, NeonCliErrorOpaque>;

pub type RNeonCliResult = RResult<RString, BoxedNeonCliError<'static>>;

#[repr(transparent)]
#[derive(StableAbi, Clone)]
pub struct RPubkey(pub [u8; 32]);

impl From<RPubkey> for Pubkey {
    fn from(value: RPubkey) -> Self {
        Pubkey::new_from_array(value.0)
    }
}

impl From<Pubkey> for RPubkey {
    fn from(value: Pubkey) -> Self {
        RPubkey(value.to_bytes())
    }
}

#[repr(transparent)]
#[derive(StableAbi, Clone)]
pub struct RAddress(pub [u8; 20]);

#[repr(transparent)]
#[derive(StableAbi, Clone)]
pub struct RU256(pub [u8; 32]);

impl From<U256> for RU256 {
    fn from(value: U256) -> Self {
        RU256(value.to_be_bytes())
    }
}

impl From<RU256> for U256 {
    fn from(value: RU256) -> Self {
        U256::from_be_bytes(value.0)
    }
}

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct RTxParams {
    pub from: RAddress,
    pub to: ROption<RAddress>,
    pub data: ROption<RVec<u8>>,
    pub value: ROption<RU256>,
    pub gas_limit: ROption<RU256>,
}

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct RChDbConfig {
    pub clickhouse_url: RVec<RString>,
    pub clickhouse_user: ROption<RString>,
    pub clickhouse_password: ROption<RString>,
    pub indexer_host: RString,
    pub indexer_port: RString,
    pub indexer_database: RString,
    pub indexer_user: RString,
    pub indexer_password: RString,
}

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct RAPIOptions {
    pub solana_cli_config_path: ROption<RString>,
    pub commitment: RString,
    pub json_rpc_url: RString,
    pub evm_loader: RString,
    pub keypair: RString,
    pub fee_payer: RString,
    pub db_config: RChDbConfig,
}
