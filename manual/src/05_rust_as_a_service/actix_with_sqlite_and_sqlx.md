# Actix with SqLite and SQLX

> We're going to use `sqlite` because I don't want to ask everyone to install a local PostreSQL server!

First, a little setup:

```bash
cargo new actix_sqlx
cd actix_sqlx
cargo add sqlx -F runtime-tokio -F sqlite
cargo add dotenv
cargo add serde -F derive
cargo add actix-web
cargo install sqlx-cli
```

Next, we'll create a `.env` file with the connection string:

```env
DATABASE_URL="sqlite:hello_db.db"
```

Now we'll have sqlx make the database for us:

```bash
sqlx database create
```

And add an initial migration:

```bash
sqlx migrate add initial
```

If you look, a `migrations` folder has been created with a timestamped file inside. Edit that file to look like this:

```sql
-- Create a messages table
CREATE TABLE IF NOT EXISTS messages
(
    id          INTEGER PRIMARY KEY NOT NULL,
    message     TEXT                NOT NULL
);

--- Insert some test messages
INSERT INTO messages (id, message) VALUES (1, 'Hello World!');
INSERT INTO messages (id, message) VALUES (2, 'Hello Galaxy!');
INSERT INTO messages (id, message) VALUES (3, 'Hello Universe!');
```

And apply the migration:

```bash
sqlx migrate run
```

Now we can write some code!

```rust
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
```