use abi_stable::std_types::{RResult, RString};
use async_ffi::LocalBorrowingFfiFuture;
use serde::{Deserialize, Serialize};

pub type RNeonResult<'a> = LocalBorrowingFfiFuture<'a, RResult<RString, RString>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeonLibError {
    pub code: u32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
