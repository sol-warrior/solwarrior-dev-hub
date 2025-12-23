use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
enum Status {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: Status,
    priority: Priority,
    assignee: Option<String>,
    is_archived: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
    description: Option<String>,
    priority: Option<Priority>,
    assignee: Option<String>,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
    priority: Option<Priority>,
    assignee: Option<String>,
    is_archived: Option<bool>,
}

#[derive(Error, Debug)]
enum ServiceError {
    #[error("not found")]
    NotFound,
    #[error("invalid status transition")]
    BadTransition,
}

type TaskStore = Arc<Mutex<HashMap<Uuid, Task>>>;

async fn create_task(
    store: web::Data<TaskStore>,
    payload: web::Json<CreateTask>,
) -> impl Responder {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let task = Task {
        id,
        title: payload.title.clone(),
        description: payload.description.clone(),
        status: Status::Todo,
        priority: payload.priority.clone().unwrap_or(Priority::Medium),
        assignee: payload.assignee.clone(),
        is_archived: false,
        created_at: now,
        updated_at: now,
    };
    store.lock().unwrap().insert(id, task.clone());
    HttpResponse::Created().json(task)
}

async fn list_tasks(store: web::Data<TaskStore>) -> impl Responder {
    let tasks: Vec<Task> = store
        .lock()
        .unwrap()
        .values()
        .filter(|t| !t.is_archived)
        .cloned()
        .collect();
    HttpResponse::Ok().json(tasks)
}

async fn get_task(store: web::Data<TaskStore>, path: web::Path<Uuid>) -> impl Responder {
    match store.lock().unwrap().get(&path.into_inner()).cloned() {
        Some(t) => HttpResponse::Ok().json(t),
        None => HttpResponse::NotFound().body("task not found"),
    }
}

fn valid_transition(from: &Status, to: &Status) -> bool {
    use Status::*;
    matches!(
        (from, to),
        (Todo, Todo)
            | (Todo, InProgress)
            | (InProgress, InProgress)
            | (InProgress, Done)
            | (Done, Done)
    )
}

async fn update_task(
    store: web::Data<TaskStore>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateTask>,
) -> impl Responder {
    let id = path.into_inner();
    let mut map = store.lock().unwrap();
    let task = map.get_mut(&id);
    match task {
        Some(t) => {
            if let Some(new_status) = &payload.status {
                if !valid_transition(&t.status, new_status) {
                    return HttpResponse::BadRequest().body("invalid status transition");
                }
                t.status = new_status.clone();
            }
            if let Some(title) = &payload.title {
                t.title = title.clone();
            }
            if let Some(desc) = &payload.description {
                t.description = Some(desc.clone());
            }
            if let Some(prio) = &payload.priority {
                t.priority = prio.clone();
            }
            if let Some(assignee) = &payload.assignee {
                t.assignee = Some(assignee.clone());
            }
            if let Some(arch) = payload.is_archived {
                t.is_archived = arch;
            }
            t.updated_at = Utc::now();
            HttpResponse::Ok().json(t.clone())
        }
        None => HttpResponse::NotFound().body("task not found"),
    }
}

async fn delete_task(store: web::Data<TaskStore>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    let mut map = store.lock().unwrap();
    match map.get_mut(&id) {
        Some(t) => {
            t.is_archived = true;
            t.updated_at = Utc::now();
            HttpResponse::Ok().body("archived")
        }
        None => HttpResponse::NotFound().body("task not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let store: TaskStore = Arc::new(Mutex::new(HashMap::new()));

    println!("Starting server on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(store.clone()))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks", web::get().to(list_tasks))
            .route("/tasks/{id}", web::get().to(get_task))
            .route("/tasks/{id}", web::patch().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
