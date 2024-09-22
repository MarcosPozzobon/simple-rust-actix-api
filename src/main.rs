mod services;

use std::env;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    postgres_cliet : Pool<Postgres>
}

mod database {
    pub mod postgres_connection;
}


#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let _pool = database::postgres_connection::start_connection().await;
    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(AppState {
                    postgres_cliet: _pool.clone()
                })
            )
            .service(index)
            .configure(services::users::service::users_routes)
    }).bind(("127.0.0.1", 8080))?.run().await
}
