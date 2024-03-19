#![allow(unused_imports)]

use eyre::{bail, eyre, OptionExt};

use common::{self, Config};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    common::deploy_on("TESTNET").await?;

    Ok(())
}
