use ethers::{prelude::*, utils};

//type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn get_balance(
    provider: &Provider<Http>,
    to_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let balance = provider.get_balance(to_addr, None).await?;
    Ok(balance)
}

pub fn _rlp_tx(to_addr: &str, value: f64) -> Result<Bytes, Box<dyn std::error::Error>> {
    let to_addr = to_addr.parse::<Address>()?;
    let tx = TransactionRequest::new()
        .to(to_addr)
        .value(U256::from(utils::parse_ether(value)?));

    Ok(tx.rlp_unsigned())
}
