use anyhow::Result;
use brize_auth::entity::Session;

use crate::{
    app::interface::RecipeRepository,
    domain::entity::{Recipe, RecipeArgs, RecipeCard},
};

pub async fn create_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_args: RecipeArgs,
    user_id: &str,
) -> Result<()> {
    repo.create_recipe_from_args(recipe_args, user_id).await?;
    Ok(())
}

pub async fn get_single_recipe<T: RecipeRepository>(repo: &T, recipe_id: &str) -> Result<Recipe> {
    repo.select_recipe_by_id(&recipe_id).await
}

pub async fn get_all_recipe_details_for_user<T: RecipeRepository>(
    repo: &T,
    user_id: &str,
) -> Result<Vec<RecipeCard>> {
    let recipes = repo.select_all_recipes_details(&user_id).await?;

    let mut recipe_cards = Vec::new();
    for r in recipes {
        let tags = repo.select_tags_for_recipe(&r.recipe_id).await?;

        recipe_cards.push(RecipeCard::from(r, tags));
    }

    Ok(recipe_cards)
}

pub async fn delete_recipe<T: RecipeRepository>(
    repo: &T,
    session: &Session,
    recipe_id: &str,
    csrf_token: &str,
) -> Result<()> {
    match session.match_csrf_token(csrf_token) {
        true => repo.delete_recipe(recipe_id).await,
        false => Err(anyhow::anyhow!("Unauthorized")),
    }
}

pub async fn modify_recipe<T: RecipeRepository>(
    repo: &T,
    recipe_id: &str,
    recipe_args: RecipeArgs,
    user_id: &str,
) -> Result<()> {
    repo.update_recipe(recipe_args, recipe_id, user_id).await?;
    Ok(())
}

pub fn validate_recipe_args(recipe: &RecipeArgs) -> bool {
    let title = recipe.title.as_str();

    if title.len() == 0 || recipe.ingredients.len() == 0 || recipe.directions.len() == 0 {
        return false;
    }

    for dir in recipe.directions.iter() {
        if dir.details.len() == 0 {
            return false;
        }
    }

    for ing in recipe.ingredients.iter() {
        let ingredient_name = ing.name.as_str();

        if ingredient_name.len() == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        app::{
            clients::db_client,
            helper::{get_random_recipe_id, get_test_recipe_args, get_test_user_id},
        },
        domain::entity::{DirectionArgs, IngredientArgs},
    };

    #[tokio::test]
    async fn test_action_get_single_recipe() {
        let recipe_id = get_random_recipe_id().await.unwrap();
        let repo = db_client().await;

        let recipe = get_single_recipe(&repo, &recipe_id).await.unwrap();

        assert!(recipe.recipe_title.len() != 0)
    }

    #[tokio::test]
    async fn test_action_get_all_recipe_details_for_user() {
        let user_id = get_test_user_id().await;
        let repo = db_client().await;

        let recipes = get_all_recipe_details_for_user(&repo, &user_id)
            .await
            .unwrap();

        assert!(recipes.len() != 0)
    }

    #[tokio::test]
    async fn test_action_create_recipe() {
        let repo = db_client().await;
        let recipe_args = get_test_recipe_args();
        let user_id = get_test_user_id().await;

        create_recipe(&repo, recipe_args, &user_id).await.unwrap();
    }

    #[test]
    fn test_action_validate_recipe_args() {
        let mut correct_recipe_args = RecipeArgs {
            title: "Oatmeal".to_string(),
            servings: 2.0,
            favorite: true,
            tags: vec!["vegan".to_string()],
            ingredients: vec![
                IngredientArgs {
                    name: "oats".to_string(),
                    amount: 2.0,
                    unit: "cups".to_string(),
                },
                IngredientArgs {
                    name: "milk".to_string(),
                    amount: 2.0,
                    unit: "cups".to_string(),
                },
            ],
            directions: vec![
                DirectionArgs {
                    details: "boil and stir".to_string(),
                    step_order: 1,
                },
                DirectionArgs {
                    details: "enjoy and stir".to_string(),
                    step_order: 1,
                },
            ],
        };

        // Correct args passes
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(result);

        // No title fails
        correct_recipe_args.title = "".to_string();
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // No ingredients fails
        correct_recipe_args.title = "some_title".to_string();
        correct_recipe_args.ingredients = vec![];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // Empty ingredients fails
        correct_recipe_args.ingredients = vec![
            IngredientArgs {
                name: "".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
            IngredientArgs {
                name: "".to_string(),
                amount: 2.0,
                unit: "cups".to_string(),
            },
        ];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);
        correct_recipe_args.ingredients = vec![IngredientArgs {
            name: "oats".to_string(),
            amount: 2.0,
            unit: "cups".to_string(),
        }];

        // No directions fails
        correct_recipe_args.directions = vec![];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);

        // Empty directions fails
        correct_recipe_args.directions = vec![
            DirectionArgs {
                details: "".to_string(),
                step_order: 1,
            },
            DirectionArgs {
                details: "".to_string(),
                step_order: 1,
            },
        ];
        let result = validate_recipe_args(&correct_recipe_args);
        assert!(!result);
    }
}
