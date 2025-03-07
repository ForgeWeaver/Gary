// Modified from: https://github.com/orca-so/whirlpools/tree/main/examples/rust-sdk/whirlpool_repositioning_bot
// Original: https://github.com/orca-so/whirlpools

use clap::Parser;
use cli::Args;
use colored::Colorize;
use dotenv::dotenv;
use gary::{
    StdResult, cli, position_manager,
    utils::{env_util, pool_util},
    wallet,
};
use orca_whirlpools::{WhirlpoolsConfigInput, set_funder, set_whirlpools_config_address};
use orca_whirlpools_client::get_position_address;
use pool_util::{
    display_position_balances, display_wallet_balances, fetch_mint, fetch_position, fetch_whirlpool,
};
use position_manager::run_position_manager;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use std::{env, str::FromStr};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> StdResult {
    let args = Args::parse();
    let config_addr: WhirlpoolsConfigInput = if env_util::is_main() {
        dotenv().ok();
        WhirlpoolsConfigInput::SolanaMainnet
    } else {
        dotenv::from_filename(".env.dev").ok();
        WhirlpoolsConfigInput::SolanaDevnet
    };
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env.dev");
    let rpc = RpcClient::new(rpc_url.to_string());
    set_whirlpools_config_address(config_addr)
        .expect("Failed to set Whirlpools config address for specified network.");
    let wallet = wallet::load_wallet()?;
    set_funder(wallet.pubkey()).expect("Failed to set funder address.");

    let position_mint_address = Pubkey::from_str(&args.position_mint_address)
        .expect("Invalid position mint address provided.");

    println!(
        "\n\
        ====================\n\
        @ Whirlpool LP BOT \n\
        ====================\n"
    );
    println!("Configuration:");
    println!(
        "  Position Mint Address: {}\n  Threshold: {:.2}bps\n  Interval: {} seconds\n  Priority Fee Tier: {:?}\n  Slippage tolerance bps: {:?}\n",
        args.position_mint_address,
        args.threshold,
        args.interval,
        args.priority_fee_tier,
        args.slippage_tolerance_bps
    );
    println!("-------------------------------------\n");

    let (position_address, _) =
        get_position_address(&position_mint_address).expect("Failed to derive position address.");
    let position_result = fetch_position(&rpc, &position_address).await;
    let mut position = match position_result {
        Ok(pos) => pos,
        Err(e) => {
            eprintln!("{}", format!("Error fetching position: {}. Ensure the position mint address is correct and exists.", e).red());
            return Err(e);
        }
    };
    let whirlpool = fetch_whirlpool(&rpc, &position.whirlpool)
        .await
        .expect("Failed to fetch Whirlpool data.");
    let token_mint_a = fetch_mint(&rpc, &whirlpool.token_mint_a)
        .await
        .expect("Failed to fetch Token Mint A data.");
    let token_mint_b = fetch_mint(&rpc, &whirlpool.token_mint_b)
        .await
        .expect("Failed to fetch Token Mint B data.");

    display_wallet_balances(
        &rpc,
        &wallet.pubkey(),
        &whirlpool.token_mint_a,
        &whirlpool.token_mint_b,
    )
    .await
    .expect("Failed to display wallet balances.");

    display_position_balances(
        &rpc,
        &position,
        &whirlpool.token_mint_a,
        &whirlpool.token_mint_b,
        token_mint_a.decimals,
        token_mint_b.decimals,
        args.slippage_tolerance_bps,
    )
    .await
    .expect("Failed to display position balances.");

    loop {
        if let Err(err) = run_position_manager(
            &rpc,
            &args,
            &wallet,
            &mut position,
            &token_mint_a,
            &token_mint_b,
        )
        .await
        {
            eprintln!("{}", format!("Error: {}", err).to_string().red());
        }
        sleep(Duration::from_secs(args.interval)).await;
    }
}
