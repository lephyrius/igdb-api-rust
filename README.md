# IGDB API Rust

This is a wrapper for the IGDB REST API. It contains all the protocol buffers compiled using PROST which makes it typesafe.

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/lephyrius/igdb-api-rust#license)
[![Crates.io](https://img.shields.io/crates/v/igdb-api-rust.svg)](https://crates.io/crates/igdb-api-rust)
[![Downloads](https://img.shields.io/crates/d/igdb-api-rust.svg)](https://crates.io/crates/igdb-api-rust)
[![Docs](https://docs.rs/igdb-api-rust/badge.svg)](https://docs.rs/igdb-api-rust/latest/igdb_api_rust/)
[![dependency status](https://deps.rs/repo/github/lephyrius/igdb-api-rust/status.svg)](https://deps.rs/repo/github/lephyrius/igdb-api-rust)
[![CI](https://github.com/lephyrius/igdb-api-rust/workflows/Rust/badge.svg)](https://github.com/lephyrius/igdb-api-rust/actions)

## Usage
```rust
use igdb_api_rust::*;

fn main() {
    // Default trait will get the credentials from the env vars: IGDB_API_ID and IGDB_API_SECRET
    // Otherwise you can use the "new" method to supply them in your own way.
    let client = Client::default();     
    
    let query = ApicalypseBuilder::default().filter("id > 1337")
                                            .limit(55)
                                            .offset(66)
                                            .fields("*")
                                            .exclude("id,name")
                                            .sort("id desc");
    // IF you prefer you can use the request_raw method.
    if let Ok(game_result) = client.request::<GameResult>(query) {
        // Do something with the game results.
    }

    // The generic "GameResult" is required for knowing what endpoint it uses.
    if let Ok(game_result_count) = client.request_count::<GameResult>(query) {
        // Do something with the game count.
    }
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.