# igdb-api-rust
IGDB API Rust

This is a wrapper for the IGDB REST API. It contains all the protocol buffers compiled using PROST which makes it typesafe. 


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