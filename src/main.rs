use serde::Serialize;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use actix_web::{App, HttpServer, Responder, middleware, web};

#[derive(Serialize)]
struct RespObj {
    status: bool,
    time: String,
    data: &'static str,
}

#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String,
}

async fn index() -> impl Responder {
    let now = SystemTime::now();
    let now: DateTime<Local> = now.into();
    let now = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let body = RespObj {
        status: true,
        time: now,
        data: "Hello World",
    };

    return web::Json(body);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
        .workers(4)
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

