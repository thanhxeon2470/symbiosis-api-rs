use ethers_core::types::Chain;
use ethers_core::utils::parse_ether;
use symbiosis_api::api::bridging::{self, BridgeResponse};
use symbiosis_api::cores::query::Query;
use symbiosis_api::{
    symbiosis::Symbiosis,
    types::token::{Token, TokenAmount},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let usdc_eth = Token::builder()
        .address("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".into())
        .decimals(6)
        .build()?;
    let token_amount_in = TokenAmount::builder()
        .token(usdc_eth)
        .amount(parse_ether(1)?)
        .build()?;
    let endpoint = bridging::BridgingExactIn::builder()
        .token_amount_in(token_amount_in)
        .chain_id_out(Chain::Boba)
        .from(
            "0xe99E80EE4792395b2F639eE0661150D2b6B1996d"
                .parse()
                .unwrap(),
        )
        .to("0xe99E80EE4792395b2F639eE0661150D2b6B1996d"
            .parse()
            .unwrap())
        .build()?;

    // Call the endpoint. The return type decides how to represent the value.
    let bridging: BridgeResponse = endpoint.query(&client).await?;

    println!("{:?}", bridging);

    anyhow::Ok(())
}
