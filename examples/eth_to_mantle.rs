use ethers_core::types::{Address, Chain, U256};
use symbiosis_api::{
    types::{
        request::{Request, SwappingExactInBody},
        Token, TokenAmount,
    },
    SymbiosisApi,
};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()?)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let symbiosis = SymbiosisApi::builder().with_partner_id("chat3").build();

    let address: Address = "0x9e661c4fe9b0aec733840bdc76cc6a8bd68d6882".parse()?;

    let eth_mainnet = Token::builder().build();
    let eth_mainnet_amount = TokenAmount::new(eth_mainnet, U256::from(1000000000000000u64)); // 0.001 ETH
    let mnt_mantle = Token::builder().chain_id(Chain::Mantle).build();
    let req = Request::SwappingExactIn(
        SwappingExactInBody::builder()
            .token_amount_in(eth_mainnet_amount)
            .token_out(mnt_mantle)
            .from(address)
            .to(address)
            .build(),
    );

    let resp = symbiosis.post_v1_swapping_exact_in(req)?.send().await?;
    println!("response: {:?}", resp);
    anyhow::Ok(())
}
