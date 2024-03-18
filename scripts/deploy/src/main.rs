#![allow(unused_imports)]

use ethers::{
    providers::Middleware,
    signers::Signer,
    utils::get_contract_address,
};
use eyre::{bail, eyre, OptionExt};

use cargo_stylus::{CheckConfig, DeployConfig, KeystoreOpts, TxSendingOpts};
use common::{load_env_for, move_to_parent_project_root, NetworkConfig};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load config
    let NetworkConfig {
        priv_key_path,
        rpc_url,
        wallet,
        client,
        rest: _,
    } = load_env_for("TESTNET").await?;

    // Deploy and activate contract
    let addr = wallet.address();
    let nonce = client
        .get_transaction_count(addr, None)
        .await
        .map_err(|e| eyre!("could not get nonce for address {addr}: {e}"))?;

    let expected_program_address = get_contract_address(wallet.address(), nonce);

    // NOTE: reusing `cargo_stylus::deploy::deploy`'s CLI arguments
    let cfg = DeployConfig {
        check_cfg: CheckConfig {
            endpoint: rpc_url,
            wasm_file_path: None,
            expected_program_address,
            private_key_path: Some(priv_key_path.display().to_string()),
            private_key: None,
            keystore_opts: KeystoreOpts {
                keystore_path: None,
                keystore_password_path: None,
            },
            nightly: false,
            skip_contract_size_check: false,
        },
        estimate_gas_only: false,
        mode: None,
        activate_program_address: None,
        tx_sending_opts: TxSendingOpts {
            dry_run: false,
            output_tx_data_to_dir: None,
        },
    };

    // NOTE: `cargo_stylus::deploy::deploy` uses cwd
    move_to_parent_project_root()?;
    cargo_stylus::deploy::deploy(cfg).await?;

    Ok(())
}
