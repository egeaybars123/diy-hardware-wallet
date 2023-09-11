use clap::Parser;

#[derive(Parser)]
#[command(author="Ege")]
#[command(long_about="DIY Hardware Wallet project for Ethereum - Arduino implementation (STM32 soon)")]
#[command(version="0.1")]
pub struct Cli {
    ///Ethereum address you want to send Ether to.
    #[arg(long)]
    pub address: String,
    
    ///Value of transaction (in Ether)
    #[arg(long)]
    pub value: String,
}
