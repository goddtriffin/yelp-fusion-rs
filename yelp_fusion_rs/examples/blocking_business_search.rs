use std::collections::HashSet;
use std::env;
use yelp_fusion_rs::endpoints::{BusinessSearchPayload, BusinessSearchResponse};
use yelp_fusion_rs::error::Error;
use yelp_fusion_rs::models::{Coordinates, PriceType};
use yelp_fusion_rs::yelp_fusion::BlockingYelpFusionClient;

fn main() -> Result<(), Error> {
    let yelp_fusion_api_key = env::var("YELP_FUSION_API_KEY").expect("no Yelp Fusion API key");

    let blocking_yelp_fusion_client: BlockingYelpFusionClient =
        BlockingYelpFusionClient::new(yelp_fusion_api_key, None);
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
        .open_now(true)
        .build()
        .unwrap();
    let business_search_response: BusinessSearchResponse =
        blocking_yelp_fusion_client.business_search(&business_search_payload)?;

    println!(
        "{}",
        serde_json::to_string(&business_search_response).unwrap()
    );
    Ok(())
}
