#![allow(non_camel_case_types)]

pub mod types;

use std::{collections::HashMap, path::Path};

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    std_types::{RResult, RString},
    StableAbi,
};
use async_ffi::LocalBorrowingFfiFuture;
use thiserror::Error;

use crate::types::{BoxedConfig, BoxedContext, BoxedNeonError, RNeonResult};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NeonLib_Ref)))]
#[sabi(missing_field(panic))]
pub struct NeonLib {
    pub api_version: u32,
    pub hash: extern "C" fn() -> RString,
    pub init_config:
        extern "C" fn(RString) -> RResult<BoxedConfig<'static>, BoxedNeonError<'static>>,
    pub init_context: extern "C" fn(
        &BoxedConfig,
        RString,
    ) -> RResult<BoxedContext<'static>, BoxedNeonError<'static>>,
    pub init_hash_context: for<'a> extern "C" fn(
        &'a BoxedConfig,
        RString,
    ) -> LocalBorrowingFfiFuture<
        'a,
        RResult<BoxedContext<'static>, BoxedNeonError<'static>>,
    >,

    pub cancel_trx:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub collect_treasury:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub create_ether_account:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub deposit:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub emulate:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub get_ether_account_data:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub get_neon_elf:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    pub get_storage_at:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
    #[sabi(last_prefix_field)]
    pub init_environment:
        for<'a> extern "C" fn(&'a BoxedConfig, &'a BoxedContext, RString) -> RNeonResult<'a>,
}

impl RootModule for NeonLib_Ref {
    abi_stable::declare_root_module_statics! {NeonLib_Ref}

    const BASE_NAME: &'static str = "neon-interface";
    const NAME: &'static str = "neon-interface";
    const VERSION_STRINGS: abi_stable::sabi_types::VersionStrings = package_version_strings!();
}

#[derive(Error, Debug)]
pub enum NeonLibError {
    #[error("abi_stable library error")]
    LibraryError(#[from] LibraryError),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

pub fn load_libraries<P>(directory: P) -> Result<HashMap<String, NeonLib_Ref>, NeonLibError>
where
    P: AsRef<Path>,
{
    let paths = std::fs::read_dir(directory)?;
    let mut result = HashMap::new();
    for path in paths {
        let lib = NeonLib_Ref::load_from_file(&path?.path())?;
        let hash = lib.hash()();
        result.insert(hash.into_string(), lib);
    }
    Ok(result)
}
