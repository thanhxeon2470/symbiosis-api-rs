use ethers_core::types::Chain;
use ethers_core::utils::parse_ether;
use symbiosis_api::cores::query::Query;
use symbiosis_api::{
    api::swapping,
    api::swapping::SwapResponse,
    symbiosis::Symbiosis,
    types::token::{Token, TokenAmount},
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let eth = Token::builder().build()?;
    let mnt = Token::builder().chain_id(Chain::Mantle).build()?;
    let token_amount_in = TokenAmount::builder()
        .token(eth)
        .amount(parse_ether(1)?)
        .build()?;
    let endpoint = swapping::SwappingExactIn::builder()
        .token_amount_in(token_amount_in)
        .token_out(mnt)
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
    let swapping: SwapResponse = endpoint.query(&client).await?;

    println!("{:?}", swapping);

    anyhow::Ok(())
}
