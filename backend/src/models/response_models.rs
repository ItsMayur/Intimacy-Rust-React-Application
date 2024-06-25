use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::user_models::User;

#[derive(Serialize, Deserialize, FromRow)]
pub struct ApiError {
    pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: u32,
    pub user: User,
}
#[derive(Serialize, Deserialize)]
pub struct SignInUserResponse {
    pub message: String,
}
