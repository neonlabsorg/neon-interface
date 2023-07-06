#![allow(non_camel_case_types)]

pub mod types;
pub mod version1;

use std::{collections::HashMap, path::Path};

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    std_types::RString,
    StableAbi,
};
use thiserror::Error;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NeonLib_Ref)))]
#[sabi(missing_field(panic))]
pub struct NeonLib {
    pub api_version: u32,
    pub hash: extern "C" fn() -> RString,
    #[sabi(last_prefix_field)]
    pub version1: version1::Version1_Ref,
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
