mod auth;
mod dash;
mod recipe;
mod user;
use poem::{endpoint::StaticFilesEndpoint, Route};

pub fn router() -> Route {
    Route::new()
        .nest("/recipe", recipe::use_recipe_routes())
        .nest("/auth", auth::use_auth_routes())
        .nest("/usr", user::use_user_routes())
        .nest("/dash", dash::use_dash_routes())
        .nest(
            "/",
            StaticFilesEndpoint::new("/srv/web").index_file("index.html"),
        )
}
