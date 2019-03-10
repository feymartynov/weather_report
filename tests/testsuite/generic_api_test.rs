use crate::support::test_server_client::CLIENT;
use crate::support::shared_responses::ErrorResponse;

/// It should return not found error on wrong URL.
#[test]
fn bad_endpoint() {
    let response = CLIENT.get_json::<ErrorResponse>("/wrong", 404);
    assert_eq!(response.error, "Not found");
}

