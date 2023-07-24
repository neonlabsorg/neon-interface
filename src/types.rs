use abi_stable::{
    std_types::{RBox, RResult, RString},
    DynTrait, StableAbi,
};
use async_ffi::BorrowingFfiFuture;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send, Debug))]
pub struct ConfigOpaque;

pub type BoxedConfig<'borr> = DynTrait<'borr, RBox<()>, ConfigOpaque>;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(Sync, Send))]
pub struct ContextOpaque;

pub type BoxedContext<'borr> = DynTrait<'borr, RBox<()>, ContextOpaque>;

pub type RNeonResult<'a> = BorrowingFfiFuture<'a, RResult<RString, RString>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeonLibError {
    pub code: u32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
