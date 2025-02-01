//! Remark the master commit of our repo on the Polkadot Collectives chain.
//!
//! Call with:
//! ```bash
//! SEED="//Alice" cargo run --org "JamBrains" --repo "graymatter" --commit "abcdef1234567890abcdef1234567890abcdef123"
//! ```

use std::str::FromStr;
use subxt::{
    dynamic::{tx, Value},
    OnlineClient, PolkadotConfig,
};
use subxt_signer::{sr25519::Keypair, SecretUri};
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser)]
struct Args {
    #[clap(long, env = "RPC", default_value = "wss://polkadot-collectives-rpc.polkadot.io")]
    rpc: String,

    #[clap(short, long, env = "SEED")]
    seed: String,

    #[clap(short, long, env = "COMMIT")]
    commit: String,

    #[clap(long, env = "ORG")]
    org: String,

    #[clap(long, env = "REPO")]
    repo: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let args = Args::parse();

    let commit = parse_commit(&args.commit)?;
    let api = OnlineClient::<PolkadotConfig>::from_url(&args.rpc).await?;
    let call = tx(
        "System",
        "remark",
        vec![Value::from_bytes(
            format!(
                "{{ \"project\": \"{}/{}\", \"commit\": \"{}\" }}",
                args.org, args.repo, commit
            )
            .as_bytes(),
        )],
    );
    let uri = SecretUri::from_str(&args.seed)?;
    let keypair = Keypair::from_uri(&uri)?;

    log::info!(
        "Sending remark extrinsic from {} to {}",
        keypair.public_key().to_account_id().to_string(),
        args.rpc
    );

    let extrinsic = api
        .tx()
        .create_signed(&call, &keypair, Default::default())
        .await?;

    let mut subscription = extrinsic.submit_and_watch().await?;

    use subxt::tx::TxStatus::*;
    while let Some(status) = subscription.next().await {
        match status? {
            InBestBlock(_) => {
                log::info!("TX included in best block");
            }
            InFinalizedBlock(block) => {
                log::info!("TX finalized in block {}", block.block_hash());

                if args.rpc.contains("polkadot") && args.rpc.contains("collective") {
                    println!("https://collectives-polkadot.subscan.io/block/{:?}", block.block_hash());
                }
                break;
            }
            Validated | Broadcasted { .. } | NoLongerInBestBlock => {}
            status => {
                log::error!("Unexpected status: {:?}", status);
            }
        }
    }

    Ok(())
}

fn parse_commit(commit: &str) -> Result<String, Box<dyn std::error::Error>> {
    let commit = commit.trim_start_matches("0x");
    if commit.len() != 40 {
        return Err("COMMIT must be 40 characters long".into());
    }

    Ok(format!("0x{}", commit))
}
