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
    pub init_config: extern "C" fn(RStr) -> RResult<BoxedConfig<'static>, RString>,
    pub init_context: extern "C" fn(&BoxedConfig, RStr) -> RResult<BoxedContext<'static>, RString>,
    pub init_hash_context:
        for<'a> extern "C" fn(
            &'a BoxedConfig,
            RStr<'a>,
        )
            -> BorrowingFfiFuture<'a, RResult<BoxedContext<'static>, RString>>,

    pub invoke: for<'a> extern "C" fn(
        &'a BoxedConfig,
        &'a BoxedContext,
        RStr<'a>,
        RStr<'a>,
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
    #[error("InitConfigAndContext error")]
    InitConfigAndContext(String),
}

pub struct NeonLibGlobals {
    pub lib: NeonLib_Ref,
    pub config: BoxedConfig<'static>,
    pub context: BoxedContext<'static>,
}

impl From<RString> for NeonLoadLibError {
    fn from(value: RString) -> Self {
        NeonLoadLibError::InitConfigAndContext(value.to_string())
    }
}

pub fn load_libraries<P>(
    directory: P,
    api_config: &str,
    context_config: &str,
) -> Result<HashMap<String, NeonLibGlobals>, NeonLoadLibError>
where
    P: AsRef<Path>,
{
    let paths = std::fs::read_dir(directory)?;
    let mut result = HashMap::new();
    for path in paths {
        let lib = NeonLib_Ref::load_from_file(&path?.path())?;
        let hash = lib.hash()();

        let config = lib.init_config()(RStr::from_str(api_config)).into_result()?;
        let context = lib.init_context()(&config, RStr::from_str(context_config)).into_result()?;

        result.insert(
            hash.into_string(),
            NeonLibGlobals {
                lib,
                config,
                context,
            },
        );
    }
    Ok(result)
}
