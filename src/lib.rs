//! A high-level binding for Symbiosis API, written in Rust.
//!
//! Symbiosis API allows you to integrate the functionalities of the Symbiosis Protocol into your
//! application, platform or protocol.
//!
//! By integrating the Symbiosis API, you can quickly and effectively enable decentralized
//! cross-chain swaps and cross-chain liquidity management for your users.
//!
//! This crate uses the [reqwest] crate for a convenient, higher-level HTTP Client, for request and
//! response, to and from Symbiosis, and [serde] for serialize and deserialize from and to
//! appropriate data format.
//!
//! # Examples
//!
//! Let's start out swapping from ETH on Ethereum to MNT on Mantle network.
//!
//! ```no_run
//! use ethers_core::types::Chain;
//! use ethers_core::utils::parse_ether;
//! use symbiosis_api::cores::query::Query;
//! use symbiosis_api::{
//!     api::swapping,
//!     api::swapping::SwapResponse,
//!     symbiosis::Symbiosis,
//!     types::token::{Token, TokenAmount},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     // Create the client.
//!     let client = Symbiosis::new("");
//!
//!     // Create a simple endpoint.
//!     let eth = Token::builder().build()?;
//!     let mnt = Token::builder().chain_id(Chain::Mantle).build()?;
//!     let token_amount_in = TokenAmount::builder()
//!         .token(eth)
//!         .amount(parse_ether(1)?)
//!         .build()?;
//!     let endpoint = swapping::SwappingExactIn::builder()
//!         .token_amount_in(token_amount_in)
//!         .token_out(mnt)
//!         .from(
//!             "0xe99E80EE4792395b2F639eE0661150D2b6B1996d"
//!                 .parse()
//!                 .unwrap(),
//!         )
//!         .to("0xe99E80EE4792395b2F639eE0661150D2b6B1996d"
//!             .parse()
//!             .unwrap())
//!         .build()?;
//!
//!     // Call the endpoint. The return type decides how to represent the value.
//!     let swapping: SwapResponse = endpoint.query(&client).await?;
//!
//!     println!("{:?}", swapping);
//!
//!     anyhow::Ok(())
//! }
//! ```
//!
//! For more examples, take a look at the `examples/` directory.
//!
//! This crate design based on https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is

#![deny(missing_docs)]
#![warn(missing_debug_implementations, rust_2018_idioms, rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples, rustdoc::private_doc_tests)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unused)]

/// API module, contains list of API supported by Symbiosis.
pub mod api;
/// Core module, contains some helpful generic traits for endpoint and request.
pub mod cores;
/// Symbiosis client module, contains information about symbiosis connection.
pub mod symbiosis;
/// Types module, contains some helpers type associated with the data Symbiosis provided.
pub mod types;
