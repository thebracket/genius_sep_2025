# Actix and JSON

Adding JSON support is nice and easy. Start off by adding `serde` with the "derive" feature:

```bash
cargo add serde -F derive
```

Then you can add a Serde-supported struct and return it as JSON:

```rust
#[derive(serde::Serialize)]
struct Test {
    name: String,
}

#[get("/hello/{name}")]
async fn hello_name(path: web::Path<String>) -> Result<web::Json<Test>> {
    let name = path.into_inner();
    Ok(web::Json(Test { name }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(user_list)
            .service(hello_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```