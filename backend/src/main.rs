use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get,
    web::{self, Data},
    App, HttpServer,
};

mod routes;
use routes::*;

mod models;
use models::*;

mod utils;
use utils::*;

mod db;
use db::*;

#[get("/")]
async fn index() -> String {
    format!("Backend is working")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection()
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");

    HttpServer::new(move || {
        App::new().app_data(Data::new(database.clone())).service(
            web::scope("/api")
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                        .cookie_secure(true)
                        .build(),
                )
                .service(index)
                .service(signup)
                .service(signin),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
