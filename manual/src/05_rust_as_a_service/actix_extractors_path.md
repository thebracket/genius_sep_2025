# Actix Extractors: Path

Let's add a little more to the web service. This time we will add a path extractor. This will allow us to extract values from the URL path.

```rust
// We added Result
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn user_list(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(user_list)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

The path is defined in the attribute macro. Notice how it maps to the types in the function signature. It's strongly typed - it won't match an URL if the types don't match.

Go to `http://localhost:8080/users/42/human` and you should see: `Welcome human, user_id 42!`