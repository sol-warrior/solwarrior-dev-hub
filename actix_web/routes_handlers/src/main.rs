use std::sync::Mutex;

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
struct MultiplyRequest {
    a: u32,
    b: u32,
}

#[derive(Serialize)]
struct MultiplyError {
    error: &'static str,
}

#[derive(Serialize)]
struct MultiplyResponse {
    product: u32,
}

struct AppStateCounterState {
    counter: Mutex<u32>,
}

#[derive(Serialize)]
struct CounterResponse {
    count: u32,
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

#[post("/mulitply")]
async fn multiply(req: web::Json<MultiplyRequest>) -> impl Responder {
    let MultiplyRequest { a, b } = req.into_inner(); // we can destruct like this

    if a == 0 || b == 0 {
        return HttpResponse::BadRequest().json(MultiplyError {
            error: "value_must_not_be_zero",
        });
    }

    HttpResponse::Ok().json(MultiplyResponse { product: a * b })
}

#[get("/counter")]
async fn counter(data: web::Data<AppStateCounterState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().json(CounterResponse { count: *counter })
}

// GET /counter
// → returns { "count": N }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let count = web::Data::new(AppStateCounterState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(count.clone())
            .service(counter)
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
