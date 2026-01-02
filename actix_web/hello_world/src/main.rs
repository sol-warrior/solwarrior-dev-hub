use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/name")]
async fn name(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Hi {}", req_body))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn hi() -> impl Responder {
    HttpResponse::Ok().body("You are warrior! Never give up.")
}

async fn index() -> impl Responder {
    "Hello world!"
}

async fn warrior() -> impl Responder{
    "Welcome to the warrior world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .service(hello)
                .route("/index.html", web::get().to(index))
                .route("hi", web::get().to(hi))
                .route("/hey", web::get().to(manual_hello)),
                .route("/warrior", web::get().to(warrior))
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .service(name)
//             .route("hi", web::get().to(hi))
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
