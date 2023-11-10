//!# IGDB API Rust
//!
//! This is a wrapper for the IGDB REST API. It contains all the protocol buffers compiled using PROST which makes it typesafe.
//!
//!
//! ## Usage
//! ```rust
//! use igdb_api_rust::*;
//! use igdb_api_rust::client::Client;
//! use igdb_api_rust::apicalypse_builder::ApicalypseBuilder;
//! use igdb_api_rust::igdb::GameResult;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Default trait will get the credentials from the env vars: IGDB_API_ID and IGDB_API_SECRET
//!     // Otherwise you can use the "new" method to supply them in your own way.
//!     let mut client = Client::new("test","test");
//!
//!     let query = ApicalypseBuilder::default().filter("id > 1337")
//!                                             .limit(55)
//!                                             .offset(66)
//!                                             .fields("*")
//!                                             .exclude("id,name")
//!                                             .sort("id desc");
//!
//!     // IF you prefer you can use the request_raw method.
//!     if let Ok(game_result) = client.request::<GameResult>(&query).await {
//!         // Do something with the game results.
//!     }
//!
//!     // The generic "GameResult" is required for knowing what endpoint it uses.
//!     if let Ok(game_result_count) = client.request_count::<GameResult>(&query).await {
//!         // Do something with the game count.
//!     }
//! }
//! ```



pub mod apicalypse_builder;
pub mod client;
pub mod igdb;
