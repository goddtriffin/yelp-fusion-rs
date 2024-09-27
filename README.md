# yelp-fusion-rs

[![Version](https://img.shields.io/crates/v/yelp-fusion-rs)](https://crates.io/crates/yelp-fusion-rs)
[![Docs](https://docs.rs/yelp-fusion-rs/badge.svg)](https://docs.rs/yelp-fusion-rs)

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
    let business_search_payload: BusinessSearchPayload = BusinessSearchPayload::builder()
            .coordinates(Coordinates::new(37.772_484, -122.396_68))
            .radius(1609)
            .categories(vec![String::from("mexican"), String::from("sandwiches")])
            .limit(50)
            .price(HashSet::from([
              PriceType::OneDollar, 
              PriceType::TwoDollar, 
              PriceType::ThreeDollar, 
              PriceType::FourDollar,
            ]))
            .open_now(false)
            .build()
            .unwrap();
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

Project is under active maintenance - even if there are no recent commits!
Please submit an issue / bug request if the library needs updating for any reason!

### Feature Requests

#### Implement the rest of the features: Business, Event, Category endpoints

Currently, I only have a use-case for Yelp Fusion API's Business Search endpoint,
so I haven't prioritized developing the rest of the Business, Event, and Category endpoints.

I fully intend to implement all of those features so that this library can do everything the Yelp Fusion API allows.

If you have a dire need for any of those endpoints, please ping me via an issue on Github and I'll know to prioritize that work.
If you're feeling extra adventurous and/or REALLY need those endpoints implemented, please send a pull request :)

### Commands

- `make lint`
- `make test`
- `make fix`

## Credits

Made with 🤬 and 🥲 by [Todd Everett Griffin](https://www.toddgriffin.me/).
