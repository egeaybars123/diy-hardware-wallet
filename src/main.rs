extern crate dotenv;

use std::env;
use dotenv::dotenv;
//use ethers::prelude::*;

fn main() {
    dotenv().ok();

    let priv_key = "PRIVATE_KEY";
    let rpc_url = "RPC_URL";

    match env::var(priv_key) {
        Ok(v) => println!("${}: {}", priv_key, v),
        Err(e) => panic!("${} is not set ({})", priv_key, e)
    }

    let test = env::var(rpc_url).unwrap();
    println!("{}", test);
}
