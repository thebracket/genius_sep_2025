use actix_web::{get, web, App, HttpServer, Result};
use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, FromRow, Serialize)]
struct Message {
    id: i64,
    message: String,
}

#[get("/")]
async fn list_messages(data: web::Data<Pool<Sqlite>>) -> Result<web::Json<Vec<Message>>> {
    // Fetch all messages from the database
    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(data.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(messages))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the .env file and obtain the database URL
    let _ = dotenvy::dotenv().is_ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await
        .expect("Failed to create pool.");
    
    println!("Listening on http://127.0.0.1:8080");
    let my_pool = pool.clone();
    HttpServer::new(move || {
        App::new()
            .service(list_messages)
            .app_data(web::Data::new(my_pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}