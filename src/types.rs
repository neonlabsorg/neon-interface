use abi_stable::{
    std_types::{RBox, RResult, RString},
    DynTrait, StableAbi,
};
use async_ffi::LocalFfiFuture;

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

pub type RNeonCliResult = LocalFfiFuture<RResult<RString, BoxedNeonCliError<'static>>>;
