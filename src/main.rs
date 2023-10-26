mod application;
mod domain;
mod infra;

use application::{generate_recipe, RecipeRepository};
use domain::Recipe;
use infra::{DatabaseConfig, MySqlGateway};
use poem::{
    get, handler, listener::TcpListener, web::Json, web::Path, IntoResponse, Result, Route, Server,
};

#[handler]
async fn make_recipe(Path(name): Path<String>) -> Result<String> {
    println!("request accepted");
    let recipe: Recipe = generate_recipe(name.as_str()).unwrap();

    let db_config = DatabaseConfig {
        host: "localhost:3306".to_string(),
        password: "my-secret-pw".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
    };

    let repo = MySqlGateway::new(&db_config).await;
    repo.insert(recipe, "jon@gmail").await?;

    Ok(String::from("Recipe inserted"))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("starting server");
    let app = Route::new().at("/make/recipe/:name", get(make_recipe));
    Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
