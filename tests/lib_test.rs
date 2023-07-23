use hyper::{Uri, StatusCode};
use lighthouse::fallback;

#[tokio::test]
async fn fallback_test_ok() {
    let uri: Uri = "/".parse::<Uri>().unwrap();
    let status_code: StatusCode = StatusCode::NOT_FOUND;
    let response: (StatusCode, String) = (status_code,"No route for /".to_string());
    assert_eq!(response,fallback(uri).await);
}
