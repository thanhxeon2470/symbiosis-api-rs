use symbiosis_api::cores::query::Query;
use symbiosis_api::{api::healthcheck, symbiosis::Symbiosis};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let endpoint = healthcheck::HealthCheck;

    // Call the endpoint. The return type decides how to represent the value.
    let healthcheck: Result<String, anyhow::Error> = endpoint.query(&client).await;

    println!("{:?}", healthcheck);

    anyhow::Ok(())
}
