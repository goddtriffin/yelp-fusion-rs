//! A Rust library for the Yelp Fusion API.
//!
//! ## Example
//!
//! `YELP_FUSION_API_KEY=<api_key> cargo run --example business_search`
//!
//! ```rust no_run
//! use std::collections::HashSet;
//! use std::env;
//! use yelp_fusion_rs::endpoints::{BusinessSearchPayload, BusinessSearchResponse};
//! use yelp_fusion_rs::error::Error;
//! use yelp_fusion_rs::models::{Coordinates, PriceType};
//! use yelp_fusion_rs::yelp_fusion::YelpFusionClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let yelp_fusion_api_key = env::var("YELP_FUSION_API_KEY").expect("no Yelp Fusion API key");
//!
//!     let yelp_fusion_client: YelpFusionClient = YelpFusionClient::new(yelp_fusion_api_key, None);
//!     let business_search_payload: BusinessSearchPayload = BusinessSearchPayload::builder()
//!         .coordinates(Coordinates::new(37.772_484, -122.396_68))
//!         .radius(1609)
//!         .categories(vec![String::from("mexican"), String::from("sandwiches")])
//!         .limit(50)
//!         .price(HashSet::from([
//!             PriceType::OneDollar,
//!             PriceType::TwoDollar,
//!             PriceType::ThreeDollar,
//!             PriceType::FourDollar,
//!         ]))
//!         .open_now(false)
//!         .build()
//!         .unwrap();
//!     let business_search_response: BusinessSearchResponse = yelp_fusion_client
//!         .business_search(&business_search_payload)
//!         .await?;
//!
//!     println!(
//!         "{}",
//!         serde_json::to_string(&business_search_response).unwrap()
//!     );
//!     Ok(())
//! }
//! ```
//!
//! For more examples, check out the `examples` directory within the repository.

extern crate core;

pub mod endpoints;
pub mod error;
pub mod models;
pub mod yelp_fusion;
