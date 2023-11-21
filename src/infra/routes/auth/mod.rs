mod login;
mod logout;
mod register;

use crate::infra::middleware::AuthGuard;
use login::*;
use logout::*;
use poem::{endpoint::StaticFileEndpoint, get, EndpointExt, Route};
use register::*;

pub fn use_auth_routes() -> Route {
    Route::new()
        .at(
            "/",
            get(StaticFileEndpoint::new("src/web/pages/auth/auth.html")).post(handle_register),
        )
        .at("/register", get(handle_register_ui).post(handle_register))
        .at("/login", get(handle_login_ui).post(handle_login))
        .at(
            "/logout",
            get(handle_logout_ui).post(handle_logout).with(AuthGuard),
        )
}
