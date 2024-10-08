use crate::{
    app::{action::reset_password, interface::Database},
    domain::entity::UpdatePasswordForm,
};
use brize_auth::entity::Session;
use poem::{
    http::StatusCode,
    web::{Data, Form},
    Error, Response, Result,
};
use sqlx::MySqlPool;

#[poem::handler]
pub async fn handle_password_reset(
    Data(session): Data<&Session>,
    Data(repo): Data<&Database<MySqlPool>>,
    Form(req): Form<UpdatePasswordForm>,
) -> Result<Response> {
    reset_password(session, repo, &req)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), StatusCode::INTERNAL_SERVER_ERROR))?;

    let res = Response::builder()
        .header("Location", "/usr/account")
        .status(StatusCode::FOUND)
        .finish();

    Ok(res)
}
