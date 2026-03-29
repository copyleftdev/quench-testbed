use actix_web::{web, HttpResponse};
use crate::models::Todo;
use crate::services::TodoService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/todos")
            .route("", web::get().to(list_todos))
            .route("", web::post().to(create_todo))
            .route("/{id}", web::get().to(get_todo))
            .route("/{id}", web::put().to(update_todo))
            .route("/{id}", web::delete().to(delete_todo))
    );
}

async fn list_todos(svc: web::Data<TodoService>) -> HttpResponse {
    HttpResponse::Ok().json(svc.list().await)
}
async fn create_todo(svc: web::Data<TodoService>, body: web::Json<Todo>) -> HttpResponse {
    HttpResponse::Created().json(svc.create(body.into_inner()).await)
}
async fn get_todo() -> HttpResponse { HttpResponse::Ok().finish() }
async fn update_todo() -> HttpResponse { HttpResponse::Ok().finish() }
async fn delete_todo() -> HttpResponse { HttpResponse::NoContent().finish() }
