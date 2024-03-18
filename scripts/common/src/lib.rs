#![allow(unused_imports)]

use dotenvy::{dotenv, dotenv_iter};
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{self, Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::Address,
    utils::get_contract_address,
};
use eyre::{bail, eyre, Context, OptionExt, Result};

use cargo_stylus::{CheckConfig, DeployConfig, KeystoreOpts, TxSendingOpts};

use std::{collections::BTreeMap, sync::Arc};
use std::{env, path::Path, str::FromStr};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

/// Your private key file path
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

#[derive(Debug)]
pub struct NetworkConfig {
    pub priv_key_path: PathBuf,
    pub rpc_url: String,
    pub wallet: Wallet<SigningKey>,
    pub client: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub rest: BTreeMap<String, String>,
}

pub async fn load_env_for(network: &str) -> Result<NetworkConfig> {
    // Calculate common prefix for given network
    let prefix = format!("{}_", network.to_uppercase());

    // Collect this network's envvar pairs from .env
    let mut vars = dotenv_iter()?
        .collect::<Result<Vec<(_, _)>, _>>()?
        .into_iter()
        .filter_map(|(k, v)| {
            k.strip_prefix(&prefix)
                .map(|k_base| (k_base.to_string(), v))
        })
        .collect::<BTreeMap<_, _>>();

    // Pull out PRIV_KEY_PATH
    #[rustfmt::skip]
    let priv_key_path: PathBuf = {
         let priv_key_path =  vars.remove(PRIV_KEY_PATH)
            .ok_or_eyre(format!("No PRIV_KEY_PATH env var set for network {}", network))?
            .into();

        let project_root = find_parent_project_root(None)?;

        make_absolute_relative_to(priv_key_path, project_root)?
    };

    // Pull out RPC_URL
    #[rustfmt::skip]
    let rpc_url: String = vars.remove(RPC_URL)
        .ok_or_eyre(format!("No RPC_URL env var set for network {}", network))?;

    // Prepare client
    let provider = Provider::<Http>::try_from(rpc_url.clone())?;
    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    Ok(NetworkConfig {
        priv_key_path,
        rpc_url,
        wallet,
        client,
        rest: vars,
    })
}

/// Reads and trims a line from a filepath
pub fn read_secret_from_file(fpath: impl AsRef<Path>) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}

pub fn find_parent_project_root(start_from: Option<PathBuf>) -> Result<PathBuf> {
    let start_from = start_from.unwrap_or(env::current_dir()?);

    //  NOTE: search upwards for `.git`
    cargo_stylus::util::discover_project_root_from_path(start_from)?
        .ok_or_eyre("Could not find project root")
}

pub fn move_to_parent_project_root() -> Result<()> {
    let parent_project_root = &find_parent_project_root(None)?;

    env::set_current_dir(parent_project_root)?;
    println!("Set cwd to {}", parent_project_root.display());

    Ok(())
}

pub fn make_absolute_relative_to(mut path: PathBuf, relative_to: PathBuf) -> Result<PathBuf> {
    if !path.is_absolute() {
        path = relative_to.join(path);
    }

    path.canonicalize()
        .wrap_err(format!("Could not canonicalize {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn testnet_env_loads() {
        let config = load_env_for("testnet").await.unwrap();
        println!("{:?}", config);
    }
}
