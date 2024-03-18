//! Example on how to interact with a deployed `stylus-hello-world` program using stylus scripts.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

#![allow(unused_imports)]

use ethers::{
    prelude::abigen,
    types::Address,
};
use eyre::{bail, eyre, OptionExt};

use common::{load_env_for, NetworkConfig};

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load config
    let NetworkConfig {
        priv_key_path: _,
        rpc_url: _,
        wallet: _,
        client,
        rest,
    } = load_env_for("TESTNET").await?;

    // Generate type from parent contract ABI
    // NOTE: need to run `cargo stylus export-abi --json --output target/abi.json` beforehand
    abigen!(Counter, "../../target/abi.json");

    // Get and parse STYLUS_PROGRAM_ADDRESS (from the extra envvars for selected network)
    // NOTE: need to manually put that in `.env` after deployment
    let program_address: Address = rest
        .get(STYLUS_PROGRAM_ADDRESS)
        .ok_or_eyre("Missing STYLUS_PROGRAM_ADDRESS for selected network")?
        .parse()?;

    println!("Program from at {:x}", program_address);

    // Construct and interact with contract
    let counter = Counter::new(program_address, client);

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    let _ = counter.increment().send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    Ok(())
}
