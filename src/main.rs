#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/cover/{address}")]
async fn get_cover(
    pool: web::Data<DbPool>,
    address: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let address = address.into_inner();
    let conn = pool.get().expect("Unable to get a database connection from the pool");

    let cover = web::block(move || actions::find_cover_by_address(address.to_string(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(cover) = cover {
        Ok(HttpResponse::Ok().json(cover))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No cover found with address: {}", address));
        Ok(res)
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    HttpServer::new(|| {
        App::new()
        .data(pool.clone())
        .wrap(middleware::Logger::default())
        .service(hello)
        .service(echo)
        .service(get_cover)
        .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
