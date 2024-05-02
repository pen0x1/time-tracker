use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

async fn authenticate_user() -> HttpResponse {
    HttpResponse::Ok().body("User authenticated")
}

async fn manage_project() -> HttpResponse {
    HttpResponse::Ok().body("Project managed")
}

async fn track_time_entry() -> HttpResponse {
    HttpResponse::Ok().body("Time entry tracked")
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/authenticate", web::post().to(authenticate_user))
            .route("/project", web::post().to(manage_project))
            .route("/time", web::post().to(track_time_entry)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Starting server at: {}", &server_url);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind(server_url)?
    .run()
    .await
}