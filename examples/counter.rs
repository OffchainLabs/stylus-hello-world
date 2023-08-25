//! Example on how to interact with a deployed `hello-stylus` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.
use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    abigen!(
        Counter,
        r#"[
            function number() external view returns (uint256)
            function setNumber(uint256 number) external
            function increment() external
        ]"#
    );

    const PRIV_KEY_PATH: &str = ""; // YOUR PRIVATE KEY FILE PATH HERE.
    const RPC_URL: &str = ""; // RPC URL FOR A STYLUS CHAIN ENDPOINT.
    const PROGRAM_ADDRESS: &str = ""; // DEPLOYED PROGRAM ADDRESS FOR HELLO STYLUS.

    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let address: Address = PROGRAM_ADDRESS.parse()?;

    let privkey = read_secret_from_file(PRIV_KEY_PATH)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let counter = Counter::new(address, client);
    let num = counter.number().call().await;
    println!("Counter number value = {:?}", num);

    let _ = counter.increment().send().await?.await?;
    println!("Successfully incremented counter via a tx");

    let num = counter.number().call().await;
    println!("New counter number value = {:?}", num);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
