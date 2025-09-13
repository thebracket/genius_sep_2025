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
