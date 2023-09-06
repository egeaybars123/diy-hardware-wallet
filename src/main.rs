extern crate dotenv;

use dotenv::dotenv;
use ethers::{prelude::*, utils};
use std::env;
//use std::time::Duration;

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

    let nonce = provider
        .get_transaction_count(wallet.address(), None)
        .await?;
    //println!("Nonce: {}", nonce);

    //add typed transaction and tx.sig_hash() which will send the hash to the stm32 for signing.
    //prepare tx.
    let price_gas = provider.get_gas_price().await?;
    let divided = price_gas.checked_div(U256::from(100)).unwrap();
    let suggested_increase = divided.checked_mul(U256::from(10)).unwrap();
    //println!("{:}", price_gas);

    let tx = TransactionRequest::new()
        .nonce(nonce)
        .to("0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e".parse::<Address>()?)
        .value(U256::from(utils::parse_ether(0.0001)?))
        .gas_price(price_gas + suggested_increase)
        .gas(21000)
        .chain_id(Chain::Sepolia);

   /*  let mut port = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(5000))
        .open()
        .expect("Failed to open port");

    let binding = tx.sighash();
    let hash = binding.as_bytes(); //could be useful while sending data over UART.

    port.write(hash).expect("Hash write failed");

    let mut serial_buf: Vec<u8> = vec![0; 64];
    port.read(serial_buf.as_mut_slice()).expect("Found no data!"); */

    let binding = tx.sighash();
    let mut sig = wallet.sign_hash(binding)?;

    let sig_r = sig.r;
    let sig_s = sig.s;

    //println!("{:?}", sig.v);
    sig.v = to_eip155_v(sig.v as u8 - 27, 11155111);
    println!("{:?}", sig.recovery_id());
    
    let mut recid_count = 0;
    while recid_count < 4 {
        let mut sig_ready = Signature{r: sig_r, s: sig_s, v: recid_count};
        println!("{}", recid_count);

        sig_ready.v = to_eip155_v(recid_count as u8, 11155111);
        let signed_raw_tx = tx.rlp_signed(&sig_ready);
        let send_tx_result = provider.send_raw_transaction(signed_raw_tx).await;

        match send_tx_result {
            Ok(_) => {
                println!("{:}", "TX sent successfully");
                break;
            },
            Err(_) => {
                println!("{:}", "false rec id");
                recid_count += 1;
                
            }
        };
    }
    Ok(())
}
