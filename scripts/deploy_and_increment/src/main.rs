#![allow(unused_imports)]

use ethers::{
    prelude::abigen,
    types::Address,
};
use eyre::{bail, eyre, Result, OptionExt};

use cargo_stylus::common::{self, Config};

const INCREMENT_NUMBER_KEY: &str = "increment_number";

#[tokio::main]
async fn main() -> Result<()> {
    // Load config
    let Config {
        client,
        additional_variables,
    } = common::load_config_for("TESTNET").await?;

    // Extract `increment_number` from the additional variables in `Stylus.toml`
    let number = additional_variables
        .get(INCREMENT_NUMBER_KEY)
        .ok_or_eyre("Missing INCREMENT_NUMBER for selected network")?
        .parse::<i32>()?;

    // Generate type from parent contract ABI
    // NOTE: need to run `cargo stylus export-abi --json --output target/abi.json` beforehand
    abigen!(Counter, "../../target/abi.json");

    // Deploy program and get its address
    let program_address = common::deploy("TESTNET").await?;
    println!("Program deployed at {:x}", program_address);

    // Construct and interact with the contract
    let counter = Counter::new(program_address, client);

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    counter.set_number(number.into()).send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num,);

    Ok(())
}
