use httpmock::prelude::*;
use pocketbase_client::admin::Admin;
use serde_json::json;

#[tokio::test]
pub async fn authenticate_admin_success() {
    let mockserver = mock_admin_login();
    let client = Admin::new(mockserver.base_url().as_str())
        .auth_with_password("", "")
        .await;
    assert!(client.is_ok());
}

#[tokio::test]
pub async fn authenticate_admin_failure() {
    let mockserver = mock_admin_login();
    let client = Admin::new(mockserver.base_url().as_str())
        .auth_with_password("wrongidentity@wrongidentity.com", "wrongpassword")
        .await;
    assert!(client.is_err());
}

fn mock_admin_login() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).json_body(json!({
            "identity": "wrongidentity@wrongidentity.com",
            "password": "wrongpassword"
        }));

        then.status(400)
            .header("content-type", "application/json")
            .json_body(json!({
                    "code": 400,
                    "message": "An error occurred while submitting the form.",
                    "data": {
                    "password": {
                    "code": "validation_required",
                    "message": "Missing required value."
                }
            }
            }));
    });

    server.mock(|when, then| {
        when
            .method(POST)
            .json_body(json!({
                "identity": "",
                "password": ""
            }))
            .path("/api/admins/auth-with-password");

        then
            .status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6InN5d2JoZWNuaDQ2cmhtMCIsInR5cGUiOiJhZG1pbiIsImV4cCI6MjIwODk4MTYwMH0.han3_sG65zLddpcX2ic78qgy7FKecuPfOpFa8Dvi5Bg",
                    "admin": {
                    "id": "b6e4b08274f34e9",
                    "created": "2022-06-22 07:13:09.735Z",
                    "updated": "2022-06-22 07:13:09.735Z",
                    "email": "test@example.com",
                    "avatar": 0
                }
            }));
    });

    server
}
