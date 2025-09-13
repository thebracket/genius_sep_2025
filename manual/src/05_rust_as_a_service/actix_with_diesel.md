# Actix with Diesel

A lot of people prefer using an ORM (Object Relational Mapper) instead of writing raw SQL. Diesel is a popular ORM in the Rust ecosystem.

We'll keep this example intentionally short and mirror the SQLX sample: a tiny Actix app that reads rows from a `messages` table in a SQLite database and returns them as JSON.

> Note: With Diesel + SQLite, the database URL is just a file path (not a `sqlite:` URL). We’ll also avoid migrations here by creating the table at startup to keep things concise.

First, create the project and add dependencies:

```bash
cargo new actix_diesel
cd actix_diesel
cargo add actix-web
cargo add dotenvy
cargo add serde -F derive
cargo add diesel -F sqlite -F r2d2
```

Add a `.env` file with the database file name (or skip this and let it default to `hello_db.db`):

```env
DATABASE_URL=hello_db.db
```

Now write the code:

```rust
use actix_web::{get, web, App, HttpServer, Result};
use diesel::{
    r2d2::{self, ConnectionManager},
    sql_types::{Integer, Text},
    SqliteConnection,
};
use diesel::{prelude::*, sql_query};
use serde::Serialize;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, QueryableByName, Serialize)]
struct Message {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    message: String,
}

#[get("/")]
async fn list_messages(pool: web::Data<Pool>) -> Result<web::Json<Vec<Message>>> {
    let mut conn = pool
        .get()
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let rows = sql_query("SELECT id, message FROM messages")
        .load::<Message>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(web::Json(rows))
}

fn init_db(pool: &Pool) {
    let mut conn = pool.get().expect("failed to get DB connection");
    // Create table and seed a few rows if missing
    sql_query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY NOT NULL,
            message TEXT NOT NULL
        );
        "#,
    )
    .execute(&mut conn)
    .expect("failed to create table");

    sql_query(
        "INSERT OR IGNORE INTO messages (id, message) VALUES \
         (1, 'Hello World!'), (2, 'Hello Galaxy!'), (3, 'Hello Universe!');",
    )
    .execute(&mut conn)
    .expect("failed to seed data");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read the .env file and obtain the database URL
    let _ = dotenvy::dotenv().is_ok();
    // With Diesel+SQLite the URL is just a file path
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "hello_db.db".to_string());

    // Build a small r2d2 pool for Diesel
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("failed to build pool");

    // Ensure schema exists and seed data
    init_db(&pool);

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
```

Run it:

```bash
cargo run
```

And test it:

```bash
curl http://127.0.0.1:8080/
```

That should return JSON similar to the SQLX example, with three seeded messages.
