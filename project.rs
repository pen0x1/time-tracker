use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: u32,
    name: String,
    description: String,
}

lazy_static::lazy_static! {
    static ref PROJECTS: Mutex<HashMap<u32, Project>> = Mutex::new(HashMap::new());
    static ref PROJECT_ID_COUNTER: AtomicU32 = AtomicU32::new(1);
}

async fn add_project(project: web::Json<Project>) -> impl Responder {
    let id = PROJECT_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let mut new_project = project.into_inner();
    new_project.id = id;

    match PROJECTS.lock() {
        Ok(mut projects) => {
            projects.insert(id, new_project);
            log::info!("Project added successfully with id: {}", id);
            HttpResponse::Created().json("Project added successfully")
        },
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
    let mut edited_project = project.into_inner();
    edited_project.id = *id;
    
    match PROJECTS.lock() {
        Ok(mut projects) => {
            if projects.contains_key(&id) {
                projects.insert(*id, edited_project);
                log::info!("Project updated successfully for id: {}", id);
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
                log::info!("Project deleted with id: {}", id);
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
    env_logger::init();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/projects").route(web::post().to(add_project)))
            .service(
                web::resource("/projects/{id}")
                    .route(web::get().to(get_project))
                    .route(web::put().to(edit_project))
                    .route(web::delete().to(delete_project)),
            )
    })
    .bind(server_address)?
    .run()
    .await
}