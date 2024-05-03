use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: u32,
    name: String,
    description: String,
}

lazy_static::lazy_static! {
    static ref PROJECTS: Mutex<HashMap<u32, Project>> = Mutex::new(HashMap::new());
}

async fn add_project(project: web::Json<Project>) -> impl Responder {
    match PROJECTS.lock() {
        Ok(mut projects) => {
            projects.insert(project.id, project.into_inner());
            HttpResponse::Created().json("Project added successfully")
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to acquire lock"),
    }
}

async fn get_project(info: web::Path<u32>) -> impl Responder {
    match PROJECTS.lock() {
        Ok(projects) => {
            if let Some(project) = projects.get(&info) {
                HttpResponse::Ok().json(project)
            } else {
                HttpResponse::NotFound().body("Project not found")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to acquire lock"),
    }
}

async fn edit_project(id: web::Path<u32>, project: web::Json<Project>) -> impl Responder {
    match PROJECTS.lock() {
        Ok(mut projects) => {
            if projects.contains_key(&id) {
                projects.insert(*id, project.into_inner());
                HttpResponse::Ok().json("Project updated successfully")
            } else {
                HttpResponse::NotFound().body("Project not found for updating")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to acquire lock"),
    }
}

async fn delete_project(id: web::Path<u32>) -> impl Responder {
    match PROJECTS.lock() {
        Ok(mut projects) => {
            if projects.remove(&id).is_some() {
                HttpResponse::Ok().body("Project deleted")
            } else {
                HttpResponse::NotFound().body("Project not found for deletion")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to acquire lock"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/projects")
                    .route(web::post().to(add_project))
            )
            .service(
                web::resource("/projects/{id}")
                    .route(web::get().to(get_project))
                    .route(web::put().to(edit_project))
                    .route(web::delete().to(delete_project))
            )
    })
    .bind(server_address)?
    .run()
    .await
}
