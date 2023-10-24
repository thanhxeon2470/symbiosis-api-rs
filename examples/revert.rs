use symbiosis_api::api::revert::{Revert, RevertResponse};
use symbiosis_api::cores::query::Query;
use symbiosis_api::symbiosis::Symbiosis;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let endpoint = Revert::builder()
        .transaction_hash(Default::default())
        .build()?;

    // Call the endpoint. The return type decides how to represent the value.
    let revert: RevertResponse = endpoint.query(&client).await?;

    println!("{:?}", revert);

    anyhow::Ok(())
}
