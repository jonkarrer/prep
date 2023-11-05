use crate::{
    application::repository::RecipeRepository,
    configuration::{get_configuration, Settings},
    domain::Recipe,
    infra::MySqlGateway,
};
use poem::{
    handler,
    web::{Json, Path},
    Result,
};

#[handler]
pub async fn recipe_by_id(recipe_id: Path<String>) -> Result<Json<Recipe>> {
    let Settings { database, .. } = get_configuration();
    let repo = MySqlGateway::new(&database).await;

    let recipe = repo.select_by_id(&recipe_id).await?;

    Ok(Json(recipe))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poem::{get, test::TestClient, Route};

    #[tokio::test]
    async fn test_route_get_recipe_by_id() {
        let app = Route::new().at("/recipe/:id", get(recipe_by_id));
        let test_client = TestClient::new(app);

        let resp = test_client
            .get("/recipe/e947f008-2835-4e6f-9b80-edeb1ce096c9")
            .send()
            .await;

        resp.assert_status_is_ok();

        let json: Recipe = resp.json().await.value().deserialize();
        assert_eq!(json.recipe_title, "Gingerbread");
    }
}
