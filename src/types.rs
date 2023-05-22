use abi_stable::{
    std_types::{RBox, ROption, RResult, RString, RVec},
    DynTrait, StableAbi,
};
use ethnum::U256;
use solana_sdk::pubkey::Pubkey;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug, Display))]
pub struct ConfigOpaque;

pub type BoxedConfig<'borr> = DynTrait<'borr, RBox<()>, ConfigOpaque>;

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
