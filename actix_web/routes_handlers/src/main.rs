use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Add_Request_Body {
    a: u32,
    b: u32,
}

#[derive(Serialize)]
struct Add_Response_Body {
    sum: u32,
}

#[derive(Serialize)]
struct AddError {
    error: String,
}

#[derive(Deserialize)]
struct MulitplyRequest {
    a: u32,
    b: u32,
}

#[derive(Serialize)]
struct MultiplyError {
    error: String,
}

#[derive(Serialize)]
struct MultiplyResponse {
    product: u32,
}

#[get("/hello/{name}")]
async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[get("/sum/{first}/{second}")]
async fn sum(path: web::Path<(u32, u32)>) -> impl Responder {
    let (a, b) = path.into_inner();
    HttpResponse::Ok().body(format!("sum: {}", a + b))
}

#[post("/add")]
async fn add(req_body: web::Json<Add_Request_Body>) -> impl Responder {
    let a = req_body.a;
    let b = req_body.b;

    if a > 1000 || b > 1000 {
        return HttpResponse::BadRequest().json(AddError {
            error: "too big".to_string(),
        });
    }

    HttpResponse::Ok().json(Add_Response_Body { sum: a + b })
}

#[post("/multiple")]
async fn multiply(req: web::Json<MulitplyRequest>) -> impl Responder {
    let a = req.a;
    let b = req.b;

    if a == 0 || b == 0 {
        return HttpResponse::BadRequest().json(MultiplyError {
            error: "Value can't be 0".to_string(),
        });
    }

    HttpResponse::Ok().json(MultiplyResponse { product: a * b })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(sum)
            .service(add)
            .service(multiply)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// use actix_web::{App, HttpServer, Responder, get, web};
// use log::info;

// async fn greet(path: web::Path<String>) -> impl Responder {
//     let name = path.into_inner();
//     format!("Hello, {}!", name)
// }

// async fn sum2(path: web::Path<(u32, u32)>) -> impl Responder {
//     let (first, second) = path.into_inner();

//     format!("sum : {}", first + second)
// }

// #[get("/sum/{first}/{second}")]
// async fn sum(path: web::Path<(u32, u32)>) -> impl Responder {
//     let (first, second) = path.into_inner();
//     format!("sum of {} + {} = {}", first, second, first + second)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();
//     HttpServer::new(|| {
//         App::new()
//             .service(sum)
//             .route("/hello/{name}", web::get().to(greet))
//             .route("/sum2/{first}/{second}", web::get().to(sum2))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }
