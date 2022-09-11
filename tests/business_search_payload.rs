use std::collections::HashSet;
use yelp_fusion_rs::endpoints::{BusinessSearchPayload, BusinessSearchPayloadError};
use yelp_fusion_rs::models::{Attribute, Coordinates, PriceType, SortBy};

#[test]
fn test_constructor_only_required_fields() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        );
    assert!(business_search_payload_result.is_ok());
}

#[test]
fn test_constructor_all_fields() {
    // Some fields aren't set due to certain pairs not being allowed to be set at the same time.
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            Some(String::from("restaurants")),
            None,
            Some(Coordinates::new(37.772_484, -122.396_68)),
            Some(1609),
            Some(vec![String::from("mexican"), String::from("sandwiches")]),
            Some(String::from("en_US")),
            Some(50),
            Some(0),
            Some(SortBy::default()),
            Some(HashSet::from([
                PriceType::OneDollar,
                PriceType::TwoDollar,
                PriceType::ThreeDollar,
                PriceType::FourDollar,
            ])),
            Some(true),
            None,
            Some(HashSet::from([Attribute::HotAndNew])),
        );
    assert!(business_search_payload_result.is_ok());
}

#[test]
fn test_constructor_location_and_coordinates_set_error() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            None,
            Some(String::from("Chicago, IL")),
            Some(Coordinates::new(37.772_484, -122.396_68)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
    match business_search_payload_result {
        Ok(_) => panic!("Returned a BusinessSearchPayload!"),
        Err(e) => match e {
            BusinessSearchPayloadError::BothLocationAndLatLongSet => (),
            BusinessSearchPayloadError::RadiusTooLarge(_) => panic!("Returned RadiusTooLarge!"),
            BusinessSearchPayloadError::LimitTooLarge(_) => panic!("Returned LimitTooLarge!"),
            BusinessSearchPayloadError::BothOpenNowAndOpenAtSet => {
                panic!("Returned BothOpenNowAndOpenAtSet!")
            }
        },
    }
}

#[test]
fn test_constructor_radius_too_large_error() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            None,
            None,
            None,
            Some(69_420),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
    match business_search_payload_result {
        Ok(_) => panic!("Returned a BusinessSearchPayload!"),
        Err(e) => match e {
            BusinessSearchPayloadError::BothLocationAndLatLongSet => {
                panic!("Returned BothLocationAndLatLongSet!")
            }
            BusinessSearchPayloadError::RadiusTooLarge(radius) => assert_eq!(69_420, radius),
            BusinessSearchPayloadError::LimitTooLarge(_) => panic!("Returned LimitTooLarge!"),
            BusinessSearchPayloadError::BothOpenNowAndOpenAtSet => {
                panic!("Returned BothOpenNowAndOpenAtSet!")
            }
        },
    }
}

#[test]
fn test_constructor_limit_too_large_error() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            None,
            None,
            None,
            None,
            None,
            None,
            Some(69),
            None,
            None,
            None,
            None,
            None,
            None,
        );
    match business_search_payload_result {
        Ok(_) => panic!("Returned a BusinessSearchPayload!"),
        Err(e) => match e {
            BusinessSearchPayloadError::BothLocationAndLatLongSet => {
                panic!("Returned BothLocationAndLatLongSet!")
            }
            BusinessSearchPayloadError::RadiusTooLarge(_) => panic!("Returned RadiusTooLarge!"),
            BusinessSearchPayloadError::LimitTooLarge(limit) => assert_eq!(69, limit),
            BusinessSearchPayloadError::BothOpenNowAndOpenAtSet => {
                panic!("Returned BothOpenNowAndOpenAtSet!")
            }
        },
    }
}

#[test]
fn test_constructor_both_open_now_and_open_at_set_error() {
    let business_search_payload_result: Result<BusinessSearchPayload, BusinessSearchPayloadError> =
        BusinessSearchPayload::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            Some(1_662_917_069),
            None,
        );
    match business_search_payload_result {
        Ok(_) => panic!("Returned a BusinessSearchPayload!"),
        Err(e) => match e {
            BusinessSearchPayloadError::BothLocationAndLatLongSet => {
                panic!("Returned BothLocationAndLatLongSet!")
            }
            BusinessSearchPayloadError::RadiusTooLarge(_) => panic!("Returned RadiusTooLarge!"),
            BusinessSearchPayloadError::LimitTooLarge(_) => panic!("Returned LimitTooLarge!"),
            BusinessSearchPayloadError::BothOpenNowAndOpenAtSet => (),
        },
    }
}
