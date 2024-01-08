use symbiosis_api::types::request::{Request, SwappingExactInBody};

#[test]
fn native_ethereum_to_mantle() {
    let req = Request::SwappingExactIn(SwappingExactInBody::from(value));
}
