extern crate dotenv;

use std::env;
use dotenv::dotenv;
use ethers::{utils, prelude::*};

//send to this address: 0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    dotenv().ok();

    let priv_key = env::var("PRIVATE_KEY").unwrap();
    let rpc_url = env::var("RPC_URL").unwrap();

    //let signer = priv_key.parse::<LocalWallet>().unwrap();
    //let address = signer.address();
    //println!("{}", address);

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = priv_key
    .parse::<LocalWallet>()?
    .with_chain_id(Chain::Sepolia);

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    println!("{}", client.address());

    let balance = provider.get_balance(client.address(), None).await?;
    println!("{}", balance);

    let to_addr = "0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e".parse::<Address>()?;
    let tx = TransactionRequest::new()
        .to(to_addr)
        .value(U256::from(utils::parse_ether(0.01)?));
    println!("{:?}", tx.rlp_unsigned());
    Ok(())
}
