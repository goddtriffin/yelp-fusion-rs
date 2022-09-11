use std::collections::HashSet;
use yelp_fusion_rs::endpoints::{
    BusinessSearchPayload, BusinessSearchPayloadBuilder, BusinessSearchPayloadError,
};
use yelp_fusion_rs::models::{Attribute, Coordinates, PriceType, SortBy};

#[test]
fn test_only_required_fields() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayloadBuilder::default().build();
    assert!(business_search_payload_result.is_ok());
}

#[test]
fn test_all_fields() {
    // Some fields aren't set due to certain pairs not being allowed to be set at the same time.
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayloadBuilder::default()
            .term(String::from("restaurants"))
            .coordinates(Coordinates::new(37.772_484, -122.396_68))
            .radius(1609)
            .categories(vec![String::from("mexican"), String::from("sandwiches")])
            .locale(String::from("en_US"))
            .limit(50)
            .offset(0)
            .sort_by(SortBy::default())
            .price(HashSet::from([
                PriceType::OneDollar,
                PriceType::TwoDollar,
                PriceType::ThreeDollar,
                PriceType::FourDollar,
            ]))
            .open_now(true)
            .attributes(HashSet::from([Attribute::HotAndNew]))
            .build();
    assert!(business_search_payload_result.is_ok());
}

#[test]
fn test_all_fields_reciprocal() {
    // This tests the builder fields that weren't used in the earlier test "test_all_fields".
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayloadBuilder::default()
            .location(String::from("Fergus, Ontario"))
            .open_at(1_662_917_069)
            .build();
    assert!(business_search_payload_result.is_ok());
}
