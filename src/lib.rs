#![allow(non_camel_case_types)]

pub mod types;

use std::{collections::HashMap, path::Path};

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    std_types::{RResult, RString},
    StableAbi,
};
use thiserror::Error;

use crate::types::{BoxedConfig, BoxedContext, BoxedNeonCliError, RNeonCliResult};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NeonLib_Ref)))]
#[sabi(missing_field(panic))]
pub struct NeonLib {
    pub api_version: u32,
    pub hash: extern "C" fn() -> RString,
    pub init_config:
        extern "C" fn(RString) -> RResult<BoxedConfig<'static>, BoxedNeonCliError<'static>>,
    pub init_context: extern "C" fn(
        &BoxedConfig,
        RString,
    )
        -> RResult<BoxedContext<'static>, BoxedNeonCliError<'static>>,
    pub cancel_trx: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub collect_treasury: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub create_ether_account: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub deposit: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub emulate: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub get_ether_account_data:
        extern "C" fn(&BoxedConfig, &BoxedContext, &RString) -> RNeonCliResult,
    pub get_neon_elf: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    pub get_storage_at: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
    #[sabi(last_prefix_field)]
    pub init_environment: extern "C" fn(&BoxedConfig, &BoxedContext, RString) -> RNeonCliResult,
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

pub fn load_libraries(directory: &Path) -> Result<HashMap<String, NeonLib_Ref>, NeonLibError> {
    let paths = std::fs::read_dir(directory)?;
    let mut result = HashMap::new();
    for path in paths {
        let lib = NeonLib_Ref::load_from_file(&path?.path())?;
        let hash = lib.hash()();
        result.insert(hash.into_string(), lib);
    }
    Ok(result)
}
