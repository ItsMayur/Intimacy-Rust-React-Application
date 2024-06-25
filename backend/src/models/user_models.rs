use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct OtpDbModel {
    pub user_id: String,
    pub otp: String,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct UserDbModel {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub profile_pic: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub age: i8,
    pub password_hash: String,
    pub is_email_verified: bool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub username: String,
    pub email: String,
    pub profile_pic: String,
    pub first_name: String,
    pub last_name: String,
    pub age: i8,
    pub password: String,
}

impl User {
    pub fn create_user(
        username: String,
        email: String,
        first_name: String,
        last_name: String,
        age: i8,
        password: String,
        profile_pic: String,
    ) -> Self {
        Self {
            username: username,
            email: email,
            first_name: first_name,
            last_name: last_name,
            age: age,
            password: password,
            profile_pic: profile_pic,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: String,
}
