use crate::{
    app::interface::UserRepository,
    infra::clients::{auth_client, db_client, session_client},
};
use brize_auth::config::Expiry;
use poem::{handler, http::StatusCode, web::Form, Error, Response, Result};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[handler]
pub async fn handle_login(Form(req): Form<LoginRequest>) -> Result<Response> {
    // Pass auth check
    auth_client()
        .await
        .verify_credentials(&req.email, &req.password)
        .await
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

    // Get user_id
    let user = db_client()
        .await
        .get_user_by_email(&req.email)
        .await
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;

    // Start session
    let session = session_client()
        .await
        .start_session(&user.user_id, Expiry::Month(1))
        .await
        .map_err(|_| Error::from_status(StatusCode::BAD_GATEWAY))?;

    // Send response
    let res = Response::builder()
        .header(
            "Set-Cookie",
            format!(
                "session_id={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                session.session_id
            ),
        )
        .header(
            "Set-Cookie",
            format!(
                "csrf_token={}; Path=/; Secure; SameSite=Strict",
                session.csrf_token
            ),
        )
        .header("Location", "/usr/dashboard")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{post, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_login() {
        // build route
        let path = "/usr/login";
        let ep = Route::new().at(path, post(handle_login));
        let test_client = TestClient::new(ep);

        // set test creds, this matches the seeder
        let email = "seed_user@gmail.com";
        let password = "seeder_password";
        let form_data = [("email", email), ("password", password)];

        // run test
        let resp = test_client
            .post(path)
            .content_type("application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await;

        // assert results
        resp.assert_status(StatusCode::FOUND);

        // TODO select from session table with the returned id
    }
}
