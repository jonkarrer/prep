use crate::{
    app::{
        configs::DbConfig,
        interface::{Database, RecipeRepository, UserRepository},
    },
    domain::entity::{Direction, Ingredient, PasswordResetToken, Recipe, RecipeArgs, Tag, User},
};
use anyhow::{Context, Result};
use serde_json::Value;
use sqlx::{mysql::MySqlPool, FromRow};

impl Database<MySqlPool> {
    pub async fn new(config: &DbConfig) -> Database<MySqlPool> {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with database");

        Database { pool }
    }
}

#[async_trait::async_trait]
impl RecipeRepository for Database<MySqlPool> {
    async fn create_recipe_from_args(&self, recipe: RecipeArgs, user_id: &str) -> Result<String> {
        let mut transaction = self
            .pool
            .begin()
            .await
            .expect("transaction failed to start");

        let recipe_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO recipes (recipe_id, user_id, recipe_title, servings)
            VALUES (?,?,?,?)
            "#,
        )
        .bind(&recipe_id)
        .bind(user_id)
        .bind(recipe.title)
        .bind(recipe.servings)
        .execute(&self.pool)
        .await?;

        for ingredient in recipe.ingredients {
            sqlx::query(
                r#"
                INSERT INTO ingredients (recipe_id, ingredient_name, amount, unit)
                VALUES (?,?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(ingredient.name)
            .bind(ingredient.amount)
            .bind(ingredient.unit)
            .execute(&mut *transaction)
            .await?;
        }

        for direction in recipe.directions {
            sqlx::query(
                r#"
                INSERT INTO directions (recipe_id, direction_details, step_order)
                VALUES (?,?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(direction.details)
            .bind(direction.step_order)
            .execute(&mut *transaction)
            .await?;
        }

        for tag_name in recipe.tags {
            sqlx::query(
                r#"
                INSERT INTO tags (recipe_id, tag_name)
                VALUES (?,?)
                "#,
            )
            .bind(&recipe_id)
            .bind(tag_name)
            .execute(&mut *transaction)
            .await?;
        }

        transaction
            .commit()
            .await
            .expect("Failed to commit transaction");

        Ok(recipe_id)
    }

    async fn select_by_id(&self, recipe_id: &str) -> Result<Recipe> {
        #[derive(FromRow)]
        struct RecipeDetails {
            pub recipe_id: String,
            pub recipe_title: String,
            pub servings: f32,
            pub favorite: bool,
        }
        let RecipeDetails {
            recipe_title,
            recipe_id,
            servings,
            favorite,
        } = sqlx::query_as(
            r#"
            SELECT recipe_id, recipe_title, servings, favorite
            FROM recipes
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by id")?;

        let ingredients: Vec<Ingredient> = sqlx::query_as(
            r#"
            SELECT ingredient_id, ingredient_name, amount, unit
            FROM ingredients
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select ingredients by id")?;

        let directions: Vec<Direction> = sqlx::query_as(
            r#"
            SELECT direction_id, step_order, direction_details
            FROM directions
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")?;

        let tags: Vec<Tag> = sqlx::query_as(
            r#"
            SELECT tag_id, tag_name
            FROM tags
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe_id)
        .fetch_all(&self.pool)
        .await
        .context("Failed to select directions by id")?;

        let recipe = Recipe {
            recipe_id,
            recipe_title,
            servings,
            favorite,
            ingredients,
            directions,
            tags,
        };

        Ok(recipe)
    }

    async fn update(&self, new_recipe: Recipe, recipe_id: &str) -> Result<()> {
        let blob: Value = serde_json::to_value(new_recipe)?;

        sqlx::query(
            r#"
            UPDATE recipes
            SET recipe = ?
            WHERE id = ?;
            "#,
        )
        .bind(blob)
        .bind(recipe_id)
        .execute(&self.pool)
        .await
        .context("Failed to update recipe")?;

        Ok(())
    }

    async fn delete(&self, recipe_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM recipes 
            WHERE id = ?
            "#,
        )
        .bind(recipe_id)
        .execute(&self.pool)
        .await
        .context("Failed to delete recipe")?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for Database<MySqlPool> {
    async fn create_user(&self, email: &str, credentials_id: &str) -> Result<String> {
        let user_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO users (user_id, email, credential_id)
            VALUES (?,?,?)
            "#,
        )
        .bind(&user_id)
        .bind(email)
        .bind(credentials_id)
        .execute(&self.pool)
        .await
        .context("Failed to create user in database")?;

        Ok(user_id)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user: User = sqlx::query_as(
            r#"
            SELECT user_id, email, profile_pic_url, role
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find user_id by email in database")?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        let user: User = sqlx::query_as(
            r#"
            SELECT user_id, email, profile_pic_url, role
            FROM users
            WHERE user_id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find user_id by email in database")?;

        Ok(user)
    }

    async fn insert_password_reset_token(
        &self,
        token: &PasswordResetToken,
        user_id: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users 
            SET password_reset_token = ?, password_reset_expiry = ?
            WHERE user_id = ?
            "#,
        )
        .bind(token.password_reset_token.as_str())
        .bind(token.password_reset_expiry)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .context("Failed to update reset password details")?;

        Ok(())
    }

    async fn get_password_reset_token(&self, user_id: &str) -> Result<PasswordResetToken> {
        sqlx::query_as(
            r#"
            SELECT password_reset_token, password_reset_expiry
            FROM users
            WHERE user_id = ?
            "#,
        )
        .bind(&user_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to select recipe details by id")
    }

    async fn update_email(&self, new_email: &str, user_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET email = ?
            WHERE user_id = ?
            "#,
        )
        .bind(new_email)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::helper::get_test_recipe_args;

    #[tokio::test]
    async fn test_recipe_repository() {
        let config = DbConfig::default();
        let repo = Database::new(&config).await;

        let recipe_args = get_test_recipe_args();
        let recipe_id = repo
            .create_recipe_from_args(recipe_args, "test_user_id")
            .await
            .unwrap();
        let recipe = repo.select_by_id(&recipe_id).await.unwrap();
        assert_eq!(&recipe.recipe_title, "Oatmeal");
    }
}
