use symbiosis_api::api::revert::{Stucked, StuckedResponse};
use symbiosis_api::cores::query::Query;
use symbiosis_api::symbiosis::Symbiosis;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let endpoint = Stucked::builder()
        .address(
            "0xe99E80EE4792395b2F639eE0661150D2b6B1996d"
                .parse()
                .unwrap(),
        )
        .build()?;

    // Call the endpoint. The return type decides how to represent the value.
    let stucked: Vec<StuckedResponse> = endpoint.query(&client).await?;

    println!("{:?}", stucked);

    anyhow::Ok(())
}
