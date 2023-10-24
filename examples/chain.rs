use symbiosis_api::{
    api::chain::{self, SymbiosisChain},
    cores::query::Query,
    symbiosis::Symbiosis,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let endpoint = chain::GetSupportedChains;

    // Call the endpoint. The return type decides how to represent the value.
    let chains: Vec<SymbiosisChain> = endpoint.query(&client).await?;

    println!("{:?}", chains);

    anyhow::Ok(())
}
