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

    
    let nonce = provider.get_transaction_count(wallet.address(), None).await?;
    println!("Nonce: {}", nonce);

    let client = SignerMiddleware::new(provider.clone(), wallet.clone());

    println!("{}", client.address());

    //let balance = web3::get_balance(&provider, client.address()).await?;
    //println!("Sepolia ETH Balance: {}", utils::format_units(balance, "ether")?);


    //add typed transaction and tx.sig_hash() which will send the hash to the stm32 for signing.
    //prepare tx.
    let price_gas = provider.get_gas_price().await?;
    println!("{:}", price_gas);

    let tx = TransactionRequest::new()
    .from(wallet.address())
    .nonce(nonce)
    .to("0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e".parse::<Address>()?)
    .value(U256::from(utils::parse_ether(0.0001)?))
    .gas_price(price_gas+10000)
    .gas(21000)
    .chain_id(Chain::Sepolia); 

    let binding = tx.sighash();
    let hash = binding.as_bytes(); //could be useful while sending data over UART.
    println!("{:}", binding);
    println!("{:x?}", hash); //displays as hex

    let mut sig = wallet.sign_hash(binding)?;
    sig.v = to_eip155_v(sig.v as u8 - 27, 11155111);
    println!("{}", sig);

    let signed_raw_tx = tx.rlp_signed(&sig);
    //provider.send_raw_transaction(signed_raw_tx).await?;

    Ok(())
}
