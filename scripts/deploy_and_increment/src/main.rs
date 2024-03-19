#![allow(unused_imports)]

use ethers::{
    prelude::abigen,
    types::Address,
};
use eyre::{bail, eyre, OptionExt};

use common::{self, Config};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load config
    let Config {
        client,
        additional_variables: _,
    } = common::load_config_for("TESTNET").await?;

    // Generate type from parent contract ABI
    // NOTE: need to run `cargo stylus export-abi --json --output target/abi.json` beforehand
    abigen!(Counter, "../../target/abi.json");

    // Deploy program and get its adress
    let program_address = common::deploy_on("TESTNET").await?;
    println!("Program at {:x}", program_address);

    // Construct and interact with the contract
    let counter = Counter::new(program_address, client);

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    let _ = counter.increment().send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    Ok(())
}
