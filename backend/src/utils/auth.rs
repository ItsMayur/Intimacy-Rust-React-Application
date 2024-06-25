use std::error::Error;

use actix_web::web::Data;
use lettre::{
    transport::smtp::{authentication::Credentials, response},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use sqlx::MySqlPool;

use crate::models;

pub async fn send_otp(
    db: Data<MySqlPool>,
    user_id: &String,
    email: &String,
) -> Result<(), Box<dyn Error>> {
    let otp = "1234";

    let db_otp = sqlx::query("INSERT INTO otps(user_id,otp) values(?,?)")
        .bind(&user_id)
        .bind(&otp)
        .execute(&**db)
        .await;

    let smtp_credentials = Credentials::new(
        "its.mayur718@gmail.com".to_string(),
        "cfgbgfgrrkygjkep".to_string(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .build();

    let email = Message::builder()
        .from("its.mayur718@gmail.com".parse()?)
        .to(email.parse()?)
        .subject("Subject")
        .body((otp).to_string())?;

    let sended_mail = mailer.send(email).await?;

    Ok(())
}

// pub async fn verify_otp(user_id: &String, otp: &String) -> Result<bool, Error> {
//     let response: Result<models::OtpDbModel, sqlx::Error> = sqlx::query(
//         "SELECT *
//         FROM otps
//         WHERE user_id=? && otp = ?",
//     )
//     .bind(user_id)
//     .bind(otp)
//     .execute(&**db)
//     .await;
// }
