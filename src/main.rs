extern crate dotenv;

use dotenv::dotenv;
use ethers::{prelude::*, utils};
use std::env;
use std::time::Duration;
//use std::time::Duration;

mod web3;
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

    //add typed transaction and tx.sig_hash() which will send the hash to the stm32 for signing.
    let price_gas = provider.get_gas_price().await?;
    let divided = price_gas.checked_div(U256::from(100)).unwrap();
    let suggested_increase = divided.checked_mul(U256::from(10)).unwrap();

    let tx = TransactionRequest::new()
        .nonce(nonce)
        .to("0xC57dA14667ECf7270348dcC7FB1E6D704e82D81e".parse::<Address>()?)
        .value(U256::from(utils::parse_ether(0.0001)?))
        .gas_price(suggested_increase)
        .gas(21000)
        .chain_id(Chain::Sepolia);

    let mut binding = tx.sighash();
    let hash: &mut [u8] = binding.as_bytes_mut();
    println!("{:?}", hash);

    let mut port = serialport::new("COM3", 9600)
        .timeout(Duration::from_millis(30000))
        .open()
        .expect("Failed to open port");

    port.write(hash).expect("Hash write failed");

    //ECDSA signature length: 64 bytes.
    let mut serial_buf: Vec<u8> = vec![0; 64];

    port.read(serial_buf.as_mut_slice()).expect("No data found");

    println!("{:?}", serial_buf);

    let hardware_sig_r = U256::from_little_endian(&serial_buf[0..32]);
    let hardware_sig_s = U256::from_little_endian(&serial_buf[32..64]);

    println!("HARDWARE SIG R VALUE: {:?}", hardware_sig_r);
    println!("HARDWARE SIG S VALUE: {:?}", hardware_sig_s);
    println!();

    let mut recid_count = 0;
    while recid_count < 4 {
        let mut sig_ready = Signature{r: hardware_sig_r, s: hardware_sig_s, v: recid_count};
        //println!("{}", recid_count);

        sig_ready.v = to_eip155_v(recid_count as u8, 11155111);
        let signed_raw_tx = tx.rlp_signed(&sig_ready);
        let send_tx_result = provider.send_raw_transaction(signed_raw_tx).await;

        match send_tx_result {
            Ok(_) => {
                println!("{:}", "Transaction is broadcasted successfully.");
                break;
            },
            Err(_) => {
                recid_count += 1;
            }
        };
    }
    Ok(())
}
