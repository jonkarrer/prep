use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MealPlanDetails {
    pub meal_plan_id: String,
    pub meal_plan_name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MealPlan {
    pub meal_plan_id: String,
    pub meal_plan_name: String,
    pub recipes: Vec<String>,
}
