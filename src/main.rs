extern crate dotenv;

use dotenv::dotenv;
use std::env;
use ethers::{prelude::*, utils};

mod web3;

//send to this address: 0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e

//hardware-wallet --to 0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e --value 0.01 -> unsigned tx (rlp encoded) -> stm32 signing tx -> broadcast
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let priv_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_url = env::var("RPC_URL").unwrap();

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = priv_key
        .parse::<LocalWallet>()?
        .with_chain_id(Chain::Sepolia);

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    println!("{}", client.address());

    let balance = web3::get_balance(provider, client.address()).await?;
    println!("Sepolia ETH Balance: {}", utils::format_units(balance, "ether")?);

    let serialized_tx = web3::rlp_tx("0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e", 0.01)?;
    println!("{:?}", serialized_tx);

    Ok(())
}
