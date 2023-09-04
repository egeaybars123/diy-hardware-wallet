use ethers::{prelude::*, utils};

//type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn _get_balance(
    provider: &Provider<Http>,
    to_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let balance = provider.get_balance(to_addr, None).await?;
    Ok(balance)
}

pub fn _create_tx(
    from: Address,
    nonce: U256,
    to: String,
    value: U256,
    gas_price: U256,
    gas: U256,
    chain_id: Chain,
) -> Result<TransactionRequest, Box<dyn std::error::Error>> {
    let tx = TransactionRequest::new()
    .from(from)
    .nonce(nonce)
    .to(to.parse::<Address>()?)
    .value(U256::from(utils::parse_ether(value)?))
    .gas_price(gas_price+10000)
    .gas(gas)
    .chain_id(chain_id);

    Ok(tx)
}

//data that will be sent to cold wallet, ready for signature.
pub fn _data_for_sig(tx: TransactionRequest) -> H256 {
    let sig_hash = tx.sighash();
    return sig_hash;
}
