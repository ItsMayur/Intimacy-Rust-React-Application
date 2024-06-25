use std::error;

use crate::routes::logging;
use actix_session::Session;
use actix_web::{
    cookie::time::util,
    post,
    web::{self, Data},
    HttpResponse, Responder,
};
use sqlx::MySqlPool;

use crate::models::*;
use crate::utils::*;

// <------------------SIGN UP ROUTE------------------>

#[post("/sign-up")]
async fn signup(db: Data<MySqlPool>, session: Session, req: web::Json<User>) -> impl Responder {
    logging("POST:/sign-up");

    // CREATED USER WITH STRUCT
    let user: User = User::create_user(
        req.username.clone(),
        req.email.clone(),
        req.first_name.clone(),
        req.last_name.clone(),
        req.age.clone(),
        req.password.clone(),
        req.profile_pic.clone(),
    );

    // INSERTED USER IN DATABASE
    let response = sqlx::query(
        "INSERT INTO users(user_id,email,username,first_name,last_name,age,password_hash,profile_pic,is_email_verified) values(?,?,?,?,?,?,?,?,?)"
    )
    .bind("thisissampleid")
    .bind(&user.email)
    .bind(&user.username)
    .bind(&user.first_name)
    .bind(&user.last_name)
    .bind(&user.age)
    .bind(&user.password)
    .bind("ProfilePic")
    .bind(false)
    .execute(&**db)
    .await;

    let token = "randomtoken".to_string();

    match response {
        Ok(id) => {
            let is_otp_sent = send_otp(db, &"thisissampleid".to_string(), &user.email).await;

            match is_otp_sent {
                Ok(res) => {
                    let session_token: Result<(), actix_session::SessionInsertError> =
                        session.insert("sesion_token", token);

                    match session_token {
                        Ok(()) => HttpResponse::Created().json(CreateUserResponse {
                            id: id.last_insert_id() as u32,
                            user: user,
                        }),
                        Err(err) => HttpResponse::InternalServerError().json(ApiError {
                            error: "Fail to add session token.Please try to login".to_string(),
                        }),
                    }
                }
                Err(err) => HttpResponse::InternalServerError().json(ApiError {
                    error: err.to_string(),
                }),
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(ApiError {
            error: err.to_string(),
        }),
    }
    // match response {
    //     Ok(id) => HttpResponse::Created().json(CreateUserResponse {
    //         id: id.last_insert_id() as u32,
    //         user: user,
    //     }),
    //     Err(err) => HttpResponse::InternalServerError().json(ApiError {
    //         error: err.to_string(),
    //     }),
    // }
}

// <------------------SIGN IN ROUTE------------------>

#[post("/sign-in")]
async fn signin(
    db: Data<MySqlPool>,
    session: Session,
    req: web::Json<LoginUser>,
) -> impl Responder {
    let response: Result<UserDbModel, sqlx::Error> = sqlx::query_as(
        "SELECT *
    FROM users
    WHERE email = ?",
    )
    .bind(&req.email)
    .fetch_one(&**db)
    .await;

    let token = "randomtoken".to_string();

    match response {
        Ok(user) => {
            if user.password_hash == req.password {
                let session_token: Result<(), actix_session::SessionInsertError> =
                    session.insert("sesion_token", token);

                match session_token {
                    Ok(()) => HttpResponse::Found().json(SignInUserResponse {
                        message: ("Signed in sucessfully").to_string(),
                    }),
                    Err(err) => HttpResponse::InternalServerError().json(ApiError {
                        error: "Fail to add session token.Please try to login".to_string(),
                    }),
                }
            } else {
                HttpResponse::InternalServerError().json(ApiError {
                    error: ("Password is incorrect").to_string(),
                })
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(ApiError {
            error: (err).to_string(),
        }),
    }
}

// #[post("/verify-email-otp")]
// async fn verifyemailotp(db<MySqlPool>,req:web::Json<>)-> impl Responder{

// }
