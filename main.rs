use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

async fn authenticate_user_handler() -> HttpResponse {
    HttpResponse::Ok().body("User authenticated")
}

async fn manage_project_handler() -> HttpResponse {
    HttpResponse::Ok().body("Project managed")
}

async fn track_time_entry_handler() -> HttpResponse {
    HttpResponse::Ok().body("Time entry tracked")
}

fn setup_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/authenticate", web::post().to(authenticate_user_handler))
            .route("/project", web::post().to(manage_project_handler))
            .route("/time", web::post().to(track_time_entry_handler)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_address = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Starting server at: {}", &server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(setup_api_routes)
    })
    .bind(server_address)?
    .run()
    .await
}