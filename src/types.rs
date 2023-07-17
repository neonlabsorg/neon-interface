use abi_stable::{
    std_types::{RArc, RBox, RResult, RString},
    DynTrait, StableAbi,
};
use async_ffi::BorrowingFfiFuture;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug, Clone))]
pub struct ConfigOpaque;

pub type BoxedConfig<'borr> = DynTrait<'borr, RArc<()>, ConfigOpaque>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send))]
pub struct ContextOpaque;

pub type BoxedContext<'borr> = DynTrait<'borr, RBox<()>, ContextOpaque>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Debug, Display))]
pub struct NeonErrorOpaque;

pub type BoxedNeonError<'borr> = DynTrait<'borr, RBox<()>, NeonErrorOpaque>;

pub type RNeonResult<'a> = BorrowingFfiFuture<'a, RResult<RString, BoxedNeonError<'static>>>;
