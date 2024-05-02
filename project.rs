use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct Project {
    id: u32,
    name: String,
    description: String,
}

lazy_static::lazy_static! {
    static ref PROJECTS: Mutex<HashMap<u32, Project>> = Mutex::new(HashMap::new());
}

async fn add_project(project: web::Json<Project>) -> impl Responder {
    let mut projects = PROJECTS.lock().unwrap();
    projects.insert(project.id, Project {
        id: project.id,
        name: project.name.clone(),
        description: project.description.clone(),
    });
    HttpResponse::Created().json(&*project)
}

async fn get_project(info: web::Path<u32>) -> impl Responder {
    let projects = PROJECTS.lock().unwrap();
    if let Some(project) = projects.get(&info) {
        HttpResponse::Ok().json(project)
    } else {
        HttpResponse::NotFound().body("Project not found")
    }
}

async fn edit_project(id: web::Path<u32>, project: web::Json<Project>) -> impl Responder {
    let mut projects = PROJECTS.lock().unwrap();
    if projects.contains_key(&id) {
        projects.insert(*id, Project {
            id: *id,
            name: project.name.clone(),
            description: project.description.clone(),
        });
        HttpResponse::Ok().json(&*project)
    } else {
        HttpResponse::NotFound().body("Project not found for updating")
    }
}

async fn delete_project(id: web::Path<u32>) -> impl Responder {
    let mut projects = PROJECTS.lock().unwrap();
    if projects.remove(&id).is_some() {
        HttpResponse::Ok().body("Project deleted")
    } else {
        HttpResponse::NotFound().body("Project not found for deletion")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/projects", web::post().to(add_project))
            .route("/projects/{id}", web::get().to(get_project))
            .route("/projects/{id}", web::put().to(edit_project))
            .route("/projects/{id}", web::delete().to(delete_project))
    })
    .bind(server_address)?
    .run()
    .await
}