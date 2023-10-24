use symbiosis_api::{
    api::routes::{self, AvailableRoutesResponse},
    cores::query::Query,
    symbiosis::Symbiosis,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    // Create a simple endpoint.
    let endpoint = routes::GetAvailableRoutes;

    // Call the endpoint. The return type decides how to represent the value.
    let routes: Vec<AvailableRoutesResponse> = endpoint.query(&client).await?;

    println!("{:?}", routes);

    anyhow::Ok(())
}
