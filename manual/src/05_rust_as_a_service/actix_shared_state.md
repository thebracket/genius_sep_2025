# Actix Shared State

You can use the shared state systems we discussed earlier. You can also make use of Actix's built-in shared state system. This is a simple way to share state between handlers.

You can define any type that is cloneable and sendable across threads as your shared state.

```rust
use std::sync::{atomic::{AtomicI32, Ordering}, Arc};

use actix_web::{get, web, App, HttpServer};

#[derive(Clone, Default)]
struct AppState {
    counter: Arc<AtomicI32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> String {
    let counter = data.counter.fetch_add(1, Ordering::Relaxed);
    format!("Hello Visitor! You are visitor number {counter}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .app_data(web::Data::new(AppState::default()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

The "cloneable" requirement is actually a superpower. You can use `Arc` to wrap your state, and then each handler gets its own reference to the state---without actual cloning other than the reference count. This is useful for shared data, database connection pools, shared caches, etc.

> It also works great with handles for actors and channels that are designed to be cloned!