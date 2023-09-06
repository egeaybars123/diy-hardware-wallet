use ethers::prelude::*;

//type Client = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

pub async fn _get_balance(
    provider: &Provider<Http>,
    to_addr: Address,
) -> Result<U256, Box<dyn std::error::Error>> {
    let balance = provider.get_balance(to_addr, None).await?;
    Ok(balance)
}

//data that will be sent to cold wallet, ready for signature.
pub fn _data_for_sig(tx: TransactionRequest) -> H256 {
    let sig_hash = tx.sighash();
    return sig_hash;
}
