use std::collections::HashSet;
use std::env;
use yelp_fusion_rs::endpoints::{BusinessSearchPayload, BusinessSearchResponse};
use yelp_fusion_rs::error::Error;
use yelp_fusion_rs::models::{Coordinates, PriceType};
use yelp_fusion_rs::yelp_fusion::YelpFusion;

#[tokio::test]
async fn test() {
    // create client
    let yelp_fusion_api_key = env::var("YELP_FUSION_API_KEY").expect("no Yelp Fusion API key");
    let yelp_fusion_client: YelpFusion = YelpFusion::new(yelp_fusion_api_key, None);

    // create payload
    let business_search_payload: BusinessSearchPayload = BusinessSearchPayload::new(
        None,
        None,
        Some(Coordinates::new(37.772_484, -122.396_68)),
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

    // search for businesses
    let business_search_result: Result<BusinessSearchResponse, Error> = yelp_fusion_client
        .business_search(business_search_payload)
        .await;
    assert!(business_search_result.is_ok());
}
