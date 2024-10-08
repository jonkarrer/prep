use crate::{
    app::{
        action::{register_new_user, start_session_for_user, validate_user_email},
        interface::Database,
    },
    domain::constants::SESSION_COOKIE_KEY,
};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use regex::Regex;
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    confirm_password: String,
}

#[handler]
pub async fn handle_register(
    Form(req): Form<RegisterRequest>,
    Data(repo): Data<&Database<MySqlPool>>,
) -> Result<Response> {
    match validate_user_email(&req.email) {
        true => {
            if is_valid_password(&req.password, &req.confirm_password) {
                let user_id = register_new_user(&req.email, &req.password, repo)
                    .await
                    .map_err(|e| {
                        Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR)
                    })?;

                let session = start_session_for_user(&user_id.0).await.map_err(|e| {
                    Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR)
                })?;

                let mut response = Response::builder()
                    .header(
                        "Set-Cookie",
                        format!(
                            "{}={}; Path=/; HttpOnly; Secure; SameSite=Strict",
                            SESSION_COOKIE_KEY, session.session_id
                        ),
                    )
                    .header("Location", "/recipe/all")
                    .status(StatusCode::FOUND)
                    .body("Registration Successful");

                Ok(response)
            } else {
                return Err(Error::from_string(
                    "Invalid Password Entries",
                    StatusCode::BAD_REQUEST,
                ));
            }
        }
        false => {
            return Err(Error::from_string(
                "Invalid email address",
                StatusCode::BAD_REQUEST,
            ));
        }
    }
}

fn is_valid_password(password: &str, confirm_password: &str) -> bool {
    let passwords_match = password == confirm_password;
    let length_check = password.len() >= 8;
    let digit_check = Regex::new(r"\d").unwrap().is_match(password);

    length_check && digit_check && passwords_match
}

#[cfg(test)]
mod tests {
    use crate::app::clients::db_client;

    use super::*;
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    #[tokio::test]
    async fn test_route_register_user() {
        // build route
        let path = "/usr/register";
        let ep = Route::new()
            .at(path, post(handle_register))
            .with(AddData::new(db_client().await));
        let test_client = TestClient::new(ep);

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password-12";
        let form_data = [
            ("email", format!("{}12@gmail.com", &email)),
            ("password", password.to_string()),
            ("confirm_password", password.to_string()),
        ];

        // run test
        let resp = test_client
            .post(path)
            .content_type("application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await;

        // assert result
        resp.assert_text("Registration Successful").await;
    }
}
