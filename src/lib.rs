#![allow(non_camel_case_types)]

pub mod types;

use std::{collections::HashMap, path::Path};

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    std_types::{RResult, RStr, RString},
    StableAbi,
};
use async_ffi::BorrowingFfiFuture;
use thiserror::Error;

use crate::types::{BoxedConfig, BoxedContext, RNeonResult};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NeonLib_Ref)))]
#[sabi(missing_field(panic))]
pub struct NeonLib {
    pub hash: extern "C" fn() -> RString,
    pub get_version: extern "C" fn() -> RString,
    pub init_config: extern "C" fn(&RStr) -> RResult<BoxedConfig<'static>, RString>,
    pub init_context: extern "C" fn(&BoxedConfig, &RStr) -> RResult<BoxedContext<'static>, RString>,
    pub init_hash_context:
        for<'a> extern "C" fn(
            &'a BoxedConfig,
            &'a RStr,
        )
            -> BorrowingFfiFuture<'a, RResult<BoxedContext<'static>, RString>>,

    pub invoke: for<'a> extern "C" fn(
        &'a BoxedConfig,
        &'a BoxedContext,
        &'a RStr,
        &'a RStr,
    ) -> RNeonResult<'a>,
}

impl RootModule for NeonLib_Ref {
    abi_stable::declare_root_module_statics! {NeonLib_Ref}

    const BASE_NAME: &'static str = "neon-interface";
    const NAME: &'static str = "neon-interface";
    const VERSION_STRINGS: abi_stable::sabi_types::VersionStrings = package_version_strings!();
}

#[derive(Error, Debug)]
pub enum NeonLoadLibError {
    #[error("abi_stable library error")]
    LibraryError(#[from] LibraryError),
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

pub fn load_libraries<P>(directory: P) -> Result<HashMap<String, NeonLib_Ref>, NeonLoadLibError>
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
