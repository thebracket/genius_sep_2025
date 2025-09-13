# Hello Actix

Let's start with a minimal HTTP server example:

```bash
cargo add actix-web
```

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hello - the hard way!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

We've bound to localhost on port 8080. If you go to `http://localhost:8080` you should see "Hello World!". If you go to `http://localhost:8080/hey` you should see "Hello - the hard way!".