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