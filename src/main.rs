
#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(features = "export-abi")]
pub use stylus_hello_world::main;
