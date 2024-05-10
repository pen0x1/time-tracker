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
struct BulkProjectRequest {
    projects: Vec<ProjectRequest>,
}

#[derive(Deserialize)]
struct ProjectRequest {
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct BulkTimeEntryRequest {
    time_entries: Vec<TimeEntryRequest>,
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

async fn manage_bulk_project_handler(item: web::Json<BulkProjectRequest>) -> HttpResponse {
    let project_names: Vec<String> = item.projects.iter().map(|p| p.name.clone()).collect();
    HttpResponse::Ok().json(ApiResponse{message: format!("Projects {:?} managed", project_names)})
}

async fn track_bulk_time_entry_handler(item: web::Json<BulkTimeEntryRequest>) -> HttpResponse {
    let project_ids: Vec<u32> = item.time_entries.iter().map(|te| te.project_id).collect();
    HttpResponse::Ok().json(ApiResponse{message: format!("Time entries for project_ids {:?} tracked", project_ids)})
}

fn setup_api_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/api")
            .route("/authenticate", web::post().to(authenticate_user_handler))
            .route("/projects/bulk", web::post().to(manage_bulk_project_handler))
            .route("/time_entries/bulk", web::post().to(track_bulk_time_entry_handler)),
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