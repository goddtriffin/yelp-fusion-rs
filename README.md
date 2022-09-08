# yelp-fusion-rs

[![Version](https://img.shields.io/crates/v/yelp-fusion-rs)](https://crates.io/crates/yelp-fusion-rs)
[![Docs](https://docs.rs/yelp-fusion-rs/badge.svg)](https://docs.rs/yelp-fusion-rs)
[![License](https://img.shields.io/crates/l/yelp-fusion-rs)](https://crates.io/crates/yelp-fusion-rs)

A Rust library for the Yelp Fusion API.

## Features

- [ ] Business Endpoints (TODO)
  - [X] [Business Search](https://www.yelp.com/developers/documentation/v3/business_search)
  - [ ] [Phone Search](https://www.yelp.com/developers/documentation/v3/business_search_phone)
  - [ ] [Transaction Search](https://www.yelp.com/developers/documentation/v3/transaction_search)
  - [ ] [Business Details](https://www.yelp.com/developers/documentation/v3/business)
  - [ ] [Business Match](https://www.yelp.com/developers/documentation/v3/business_match)
  - [ ] [Reviews](https://www.yelp.com/developers/documentation/v3/business_reviews)
  - [ ] [Autocomplete](https://www.yelp.com/developers/documentation/v3/autocomplete)
- [ ] Event Endpoints (TODO)
  - [ ] [Event Lookup](https://www.yelp.com/developers/documentation/v3/event)
  - [ ] [Event Search](https://www.yelp.com/developers/documentation/v3/event_search)
  - [ ] [Featured Event](https://www.yelp.com/developers/documentation/v3/featured_event)
- [ ] Category Endpoints (TODO)
  - [ ] [All Categories](https://www.yelp.com/developers/documentation/v3/all_categories)
  - [ ] [Category Details](https://www.yelp.com/developers/documentation/v3/category)

## Examples

Search for businesses based on custom criteria!

`YELP_FUSION_API_KEY=<api_key> cargo run --example business_search`

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    let yelp_fusion_api_key = env::var("YELP_FUSION_API_KEY").expect("no Yelp Fusion API key");

    let yelp_fusion_client: YelpFusion = YelpFusion::new(yelp_fusion_api_key, None);
    let business_search_payload: BusinessSearchPayload = BusinessSearchPayload::new(
        None,
        None,
        Some(Coordinates::new(37.7724847, -122.3966801)),
        Some(1609),
        Some(vec![String::from("mexican"), String::from("sandwiches")]),
        None,
        Some(50),
        None,
        None,
        Some(HashSet::from([
            PriceType::OneDollar,
            PriceType::TwoDollar,
            PriceType::ThreeDollar,
            PriceType::FourDollar,
        ])),
        Some(false),
        None,
        None,
    );
    let business_search_response: BusinessSearchResponse = yelp_fusion_client
        .business_search(business_search_payload)
        .await?;
    
    println!(
        "{}",
        serde_json::to_string(&business_search_response).unwrap()
    );
    Ok(())
}
```

For more examples, check out the [examples](https://github.com/goddtriffin/yelp-fusion-rs/blob/main/examples) directory.

## Developers

Built with: `Rust 1.63`.

Project is under active maintenance - even if there are no recent commits!
Please submit an issue / bug request if you the library needs updating for any reason!

### Feature Requests

#### Implement the rest of the features: Business, Event, Category endpoints

Currently, I only have a use-case for Yelp Fusion API's Business Search endpoint,
so I haven't prioritized developing the rest of the Business, Event, and Category endpoints.

I fully intend to implement all of those features so that this library can do everything the Yelp Fusion API allows.

If you have a dire need for any of those endpoints, please ping me via an issue on Github and I'll know to prioritize that work.
If you're feeling extra adventurous and/or REALLY need those endpoints implemented, please send a pull request :)

#### Builder pattern

Currently, the only way to create instances of each struct is by using ::new().
By implementing the Builder pattern on each struct, less work has to be done on the library users' side as they don't 
have to throw in many None values for each optional field they don't want to use.
Not only would this make the library more ergonomic to use, but it would vastly improve readability (specifically at 
each struct initialization point).

This hasn't been prioritized yet as I am currently satisfied with ::new() for my use cases.
Pull requests are welcome!

### Commands

- `make lint`
  - Lints the codebase via `cargo fmt`.
- `make test`
  - Tests the codebase via:
    - `cargo fmt`
    - `cargo check`
    - `cargo clippy` (with insanely strict defaults)
    - `cargo test`.

## Credits

Made with 🤬 and 🥲 by [Todd Everett Griffin](https://www.toddgriffin.me/).

`yelp-fusion-rs` is open source under the [MIT License](https://github.com/goddtriffin/yelp-fusion-rs/blob/main/LICENSE).
