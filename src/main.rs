use actix_web::{HttpServer, web, App, HttpResponse};
use actix_web::middleware::Logger;
use user::user_controller::user_config;
use log::LevelFilter;
use crate::config::rbatis_config::{InitDb,rb};

mod user;
mod config;
mod result;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = fast_log::init(fast_log::config::Config::new().console());

    rb.link_opt("mysql://root:123@localhost:3306/acitx-test", InitDb::db_option()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(user_config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}