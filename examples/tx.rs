use symbiosis_api::api::tx::{GetBatchTx, GetSingleTx, TxHashWithChainId, TxResponse};
use symbiosis_api::cores::query::Query;
use symbiosis_api::symbiosis::Symbiosis;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create the client.
    let client = Symbiosis::new("");

    let txs = vec![
        TxHashWithChainId::builder()
            .transaction_hash(
                "0x1af1019ee3875b938779bd3a867924193a574e0375d86c36da60aecdac1f5c9e"
                    .parse()
                    .unwrap(),
            )
            .build()?,
        TxHashWithChainId::builder()
            .transaction_hash(
                "0x13db41d1b4c6a84f4152a5672d5a1daa990d6cf225eaa83f3bbd2b882d87fbca"
                    .parse()
                    .unwrap(),
            )
            .build()?,
        TxHashWithChainId::builder()
            .transaction_hash(
                "0x94aff34674871da54149c5001b75e810a54b0b9eb0f0567a23168b8ceef3055a"
                    .parse()
                    .unwrap(),
            )
            .build()?,
        TxHashWithChainId::builder()
            .transaction_hash(
                "0xe0c0e93ba0e67ee88eadec555e8ac32f3ba6f36f94b1dfde53ab1c3a8b6c8f02"
                    .parse()
                    .unwrap(),
            )
            .build()?,
    ];

    // Create a simple endpoint.
    let endpoint = GetSingleTx::builder().tx(txs[0].clone()).build()?;
    // Call the endpoint. The return type decides how to represent the value.
    let single_tx: TxResponse = endpoint.query(&client).await?;
    println!("single {:?}", single_tx);

    // Create a simple endpoint.
    let endpoint = GetBatchTx::builder().txs(txs).build()?;
    // Call the endpoint. The return type decides how to represent the value.
    let batch_tx: Vec<TxResponse> = endpoint.query(&client).await?;
    println!("batch {:?}", batch_tx);

    anyhow::Ok(())
}
