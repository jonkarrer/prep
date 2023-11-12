use crate::{
    application::interface::{Database, RecipeRepository},
    domain::entity::{Recipe, RecipeArgs},
};
use brize_auth::entity::Session;
use poem::{
    handler,
    web::{Data, Json},
    Error, Result,
};
use sqlx::MySqlPool;

#[handler]
pub async fn handle_create_recipe(
    Json(recipe): Json<RecipeArgs>,
    Data(repo): Data<&Database<MySqlPool>>,
    Data(session): Data<&Session>,
) -> Result<Json<Recipe>> {
    let recipe_id = repo
        .create_recipe_from_args(recipe, &session.user_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    let recipe = repo
        .select_by_id(&recipe_id)
        .await
        .map_err(|e| Error::from_string(format!("{e}"), poem::http::StatusCode::BAD_GATEWAY))?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use poem::{middleware::AddData, post, test::TestClient, EndpointExt, Route};

    use super::*;
    use crate::infra::{
        database::db,
        middleware::AuthGuard,
        test_helper::{get_test_recipe_args, get_test_session},
    };

    #[tokio::test]
    async fn test_route_create_recipe() {
        // build route
        let path = "/recipe/create";
        let ep = Route::new()
            .at(path, post(handle_create_recipe))
            .with(AddData::new(db().await))
            .with(AuthGuard);

        let test_client = TestClient::new(ep);

        // get a session token
        let session = get_test_session().await.unwrap();

        // create fake recipe
        let test_recipe = get_test_recipe_args();
        let payload = serde_json::to_string(&test_recipe).unwrap();

        // run test
        let resp = test_client
            .post(path)
            .body(payload)
            .header("Cookie", format!("session_id={}", session.session_id))
            .header("X-CSRF-Token", session.csrf_token)
            .content_type("application/json")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Oatmeal");
    }
}
