use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::env;

use crate::domain::ApiResponse;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AiGeneratedRecipe {
    pub title: String,
    pub ingredients: Vec<String>,
    pub directions: Vec<String>,
    pub servings: f32,
}

pub fn convert_to_recipe(content: &str) -> AiGeneratedRecipe {
    serde_json::from_str(&content).expect("Failed to parse dat")
}

pub fn generate_recipe(recipe_name: &str) -> Result<AiGeneratedRecipe> {
    dotenvy::dotenv().expect("Could not find .env");
    // URL
    let url = "https://api.openai.com/v1/chat/completions";

    // Headers
    let content_type = "application/json";
    let authorization =
        "Bearer ".to_string() + &env::var("OPEN_API_TOKEN").expect("Env key not found");

    // JSON payload
    let payload = format!(
        r#"
   {{
       "model": "gpt-3.5-turbo",
       "messages": [
           {{
               "role": "system",
               "content": "You are a concise recipe book. Provide recipes straight to the point in JSON format with only ingredients, directions, servings and title"
           }},
           {{
            "role": "user",
            "content": "{}"
           }}
       ],
       "temperature": 0.3
   }}
   "#,
        recipe_name
    );

    // Send the request
    let response = ureq::post(url)
        .set("Content-Type", content_type)
        .set("Authorization", authorization.as_str())
        .send_string(payload.as_str())?;

    // Handle the response
    let body: ApiResponse = response.into_json()?;

    let recipe = convert_to_recipe(body.choices[0].message.content.as_str());

    Ok(recipe)
}
