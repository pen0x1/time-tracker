use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger, Error, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct AuthenticateRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct ProjectRequest {
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct TimeEntryRequest {
    project_id: u32,
    start_time: String,
    end_time: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

async fn authenticate_user_handler(item: web::Json<AuthenticateRequest>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse{message: format!("User {} authenticated", item.username)})
}

async fn manage_project_handler(item: web::Json<ProjectRequest>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse{message: format!("Project {} managed", item.name)})
}

async fn track_time_entry_handler(item: web::Json<TimeEntryRequest>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse{message: format!("Time entry for project_id {} tracked", item.project_id)})
}

fn setup_api_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
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

    println!("Starting server at: {}", server_address);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(setup_api_routes)
            .default_service(
                web::route().to(|req| async move {
                    eprintln!("Handling 404 for {:?}", req);
                    HttpResponse::build(StatusCode::NOT_FOUND)
                        .json(ApiResponse{message: "404 Not Found".to_string()})
                }),
            )
    })
    .bind(server_address)?
    .run()
    .await
}