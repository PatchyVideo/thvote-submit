
use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};

mod comm;
mod shared;
mod models;
mod handlers;
mod services;
mod validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    let client_options = ClientOptions::parse(shared::MONGODB_URL).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("submits_v1");

    // Start http server
    HttpServer::new(move || {
        let redlock = redlock::RedLock::new(vec![comm::REDIS_ADDRESS]);
        let submit_service_v1 = services::SubmitServiceV1::new(db.clone(), redlock);
        App::new()
            .data(submit_service_v1)
            .route("/v1/character/", web::post().to(handlers::submit_character_v1))
            .route("/v1/music/", web::post().to(handlers::submit_music_v1))
            .route("/v1/cp/", web::post().to(handlers::submit_cp_v1))
            .route("/v1/work/", web::post().to(handlers::submit_work_v1))
            .route("/v1/paper/", web::post().to(handlers::submit_paper_v1))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
