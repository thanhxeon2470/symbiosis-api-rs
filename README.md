# symbiosis-api

[![Crates.io](https://img.shields.io/crates/v/symbiosis-api.svg)](https://crates.io/crates/symbiosis-api)
[![Documentation](https://docs.rs/symbiosis-api/badge.svg)](https://docs.rs/symbiosis-api/)
[![Codecov](https://codecov.io/github/phnaharris/symbiosis-api/coverage.svg?branch=main)](https://codecov.io/gh/phnaharris/symbiosis-api)
[![Build Status](https://github.com/phnaharris/symbiosis-api-rs/actions/workflows/main.yml/badge.svg)](https://github.com/phnaharris/symbiosis-api-rs/actions/workflows/main.yml)

A high-level binding for Symbiosis API, written in Rust.

Symbiosis API allows you to integrate the functionalities of the Symbiosis Protocol into your
application, platform or protocol.

By integrating the Symbiosis API, you can quickly and effectively enable decentralized
cross-chain swaps and cross-chain liquidity management for your users.

This crate uses the [reqwest] crate for a convenient, higher-level HTTP Client, for request and
response, to and from Symbiosis, and [serde] for serialize and deserialize from and to
appropriate data format.

## Examples

Let's start out swapping from ETH on Ethereum to MNT on Mantle network.

```rust
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
```

For more examples, take a look at the `examples/` directory.

This crate design based on https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is
