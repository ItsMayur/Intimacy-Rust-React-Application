use sqlx::MySqlPool;

// #[derive(Serialize, Deserialize)]
// pub struct DbError {
//     error: String,
// }

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:Mayur4Mysql@127.0.0.1:3306/intemacy").await
}
