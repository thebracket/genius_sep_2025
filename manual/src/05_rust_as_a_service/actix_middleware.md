# Actix Middleware

Actix supports the concept of "middleware" --- code that runs before and/or after a request is processed. Middleware can be used for logging, authentication, adding data to requests, etc. (Actix has built-in middleware for logging, compression and session state).

Let's build a simple middleware that checks incoming requests for a cookie:

```rust
use std::sync::{
    atomic::{AtomicI32, Ordering},
    Arc,
};

use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    get,
    middleware::{from_fn, Next},
    web, App, Error, HttpMessage, HttpResponse, HttpServer,
};

#[derive(Clone, Default)]
struct AppState {
    counter: Arc<AtomicI32>,
}

#[derive(Clone, Debug)]
struct User(String);

#[get("/")]
async fn hello(data: web::Data<AppState>, user: web::ReqData<User>) -> String {
    let counter = data.counter.fetch_add(1, Ordering::Relaxed);
    let name = &user.into_inner().0;
    format!(
        "Hello {name}! You are visitor number {counter}!"
    )
}

async fn user_cookie_middleware<B: MessageBody + 'static>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error> {
    // Look for a cookie named "User" and attach it to request extensions.
    if let Some(cookie) = req.cookie("User") {
        req.extensions_mut()
            .insert(User(cookie.value().to_string()));

        // Continue down the stack; normalize body to EitherBody
        let res = next.call(req).await?.map_into_left_body();
        Ok(res)
    } else {
        // Short-circuit with 401 if no User cookie is present
        let (req, _pl) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .body("Missing 'User' cookie")
            .map_into_right_body();
        Ok(ServiceResponse::new(req, res))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(from_fn(user_cookie_middleware))
            .service(hello)
            .app_data(web::Data::new(AppState::default()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

To test it, let's make another project. We'll need `tokio` and `reqwest`:

```bash
cargo add tokio -F full
cargo add reqwest
```

The client code:

```rust
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("URL").unwrap_or_else(|_| "http://127.0.0.1:8080/".to_string());
    let user = env::var("USER_NAME").unwrap_or_else(|_| "Alice".to_string());

    let client = reqwest::Client::builder().build()?;
    let response = client
        .get(&url)
        .header(reqwest::header::COOKIE, format!("User={}", user))
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    println!("Status: {}", status);
    println!("{}", body);

    Ok(())
}
```