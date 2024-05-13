use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger, Error, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
struct ApiResponse<T = Value> {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl ApiResponse<Value> {
    fn new(message: String) -> Self {
        ApiResponse {
            message,
            data: None,
        }
    }
}

impl<T> ApiResponse<T> {
    fn with_data(message: String, data: T) -> ApiResponse<T> {
        ApiResponse {
            message,
            data: Some(data),
        }
    }
}

async fn authenticate_user_handler(item: web::Json<AuthenticateRequest>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(ApiResponse::new(format!("User {} authenticated", item.username))))
}

async fn manage_bulk_project_handler(item: web::Json<BulkProjectRequest>) -> Result<HttpResponse, Error> {
    let project_names: Vec<String> = item.projects.iter().map(|p| p.name.clone()).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::with_data(format!("Projects managed"), project_names)))
}

async fn track_bulk_time_entry_handler(item: web::Json<BulkTimeEntryRequest>) -> Result<HttpResponse, Error> {
    let project_ids: Vec<u32> = item.time_entries.iter().map(|te| te.project_id).collect();
    Ok(HttpResponse::Ok().json(ApiResponse::with_data(format!("Time entries tracked"), project_ids)))
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
                        .json(ApiResponse::new("404 Not Found".to_string()))
                }),
            )
    })
    .bind(&server_address)?
    .run()
    .await
}