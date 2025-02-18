use mockito::Matcher;
use pushover::requests::message::{Limits, LimitsResponse};
use pushover::{Error, API};

#[test]
fn test_sync_client_returns_pushover_error() {
    let mut server = mockito::Server::new();

    let _m = server.mock("GET", Matcher::Any)
        .with_body("{\"status\":0, \"request\":\"request_number\", \"errors\": [\"Error 1\", \"Error 2\"]}")
        .create();

    let request = Limits::new("token");
    let response = API::new().base_url(&server.url()).send(&request);

    match response.expect_err("Expected error") {
        Error::PushoverError {
            status,
            errors,
            request,
        } => {
            assert_eq!(status, 0);
            assert_eq!(errors, vec!["Error 1", "Error 2"]);
            assert_eq!(request, "request_number");
        }
        _ => panic!("Did not receive PushoverError"),
    }
}

#[test]
fn test_sync_client_does_not_return_error() {
    let mut server = mockito::Server::new();

    let _m = server.mock("GET", Matcher::Any)
        .with_body("{\"status\":1, \"request\":\"request_number\", \"limit\": 1, \"remaining\": 2, \"reset\": 3}")
        .create();

    let request = Limits::new("token");
    let response = API::new().base_url(&server.url()).send(&request);

    match response {
        Ok(LimitsResponse {
            request,
            limit,
            remaining,
            reset,
        }) => {
            assert_eq!(request, "request_number");
            assert_eq!(limit, 1);
            assert_eq!(remaining, 2);
            assert_eq!(reset, 3);
        }
        _ => panic!("Received error"),
    }
}

#[test]
fn test_async_client_returns_pushover_error() {
    let mut server = mockito::Server::new();

    let _m = server.mock("GET", Matcher::Any)
        .with_body("{\"status\":0, \"request\":\"request_number\", \"errors\": [\"Error 1\", \"Error 2\"]}")
        .create();

    let api = API::new().base_url(&server.url());

    let request = Limits::new("token");
    let response = tokio_test::block_on(api.send_async(&request));

    match response.expect_err("Expected error") {
        Error::PushoverError {
            status,
            errors,
            request,
        } => {
            assert_eq!(status, 0);
            assert_eq!(errors, vec!["Error 1", "Error 2"]);
            assert_eq!(request, "request_number");
        }
        _ => panic!("Did not receive PushoverError"),
    }
}

#[test]
fn test_async_client_does_not_return_error() {
    let mut server = mockito::Server::new();

    let _m = server.mock("GET", Matcher::Any)
        .with_body("{\"status\":1, \"request\":\"request_number\", \"limit\": 1, \"remaining\": 2, \"reset\": 3}")
        .create();

    let api = API::new().base_url(&server.url());

    let request = Limits::new("token");
    let response = tokio_test::block_on(api.send_async(&request));

    match response {
        Ok(LimitsResponse {
            request,
            limit,
            remaining,
            reset,
        }) => {
            assert_eq!(request, "request_number");
            assert_eq!(limit, 1);
            assert_eq!(remaining, 2);
            assert_eq!(reset, 3);
        }
        _ => panic!("Received error"),
    }
}
