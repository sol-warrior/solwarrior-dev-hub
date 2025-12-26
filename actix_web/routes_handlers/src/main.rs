use actix_web::{App, HttpServer, Responder, get, web};
use log::info;

async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    format!("Hello, {}!", name)
}

async fn sum2(path: web::Path<(u32, u32)>) -> impl Responder {
    let (first, second) = path.into_inner();

    format!("sum : {}", first + second)
}

#[get("/sum/{first}/{second}")]
async fn sum(path: web::Path<(u32, u32)>) -> impl Responder {
    let (first, second) = path.into_inner();
    format!("sum of {} + {} = {}", first, second, first + second)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(sum)
            .route("/hello/{name}", web::get().to(greet))
            .route("/sum2/{first}/{second}", web::get().to(sum2))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// GET /sum/5/7 → return “sum: 12”
