// Carol's version: pagination support
use crate::models::Todo;
use crate::db::TodoRepository;
use crate::events::EventBus;
use crate::audit::AuditLogger;

pub struct TodoService {
    repo: TodoRepository,
    events: EventBus,
    audit: AuditLogger,
}

impl TodoService {
    pub async fn list_paginated(&self, page: u32, limit: u32) -> Vec<Todo> {
        self.repo.find_paginated(page, limit).await
    }
    pub async fn create(&self, todo: Todo) -> Todo {
        self.audit.log("create", &todo.id.to_string()).await;
        self.events.publish("todo.created", &todo).await;
        self.repo.insert(todo).await
    }
}
