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
    pub async fn list(&self) -> Vec<Todo> { self.repo.find_all().await }
    pub async fn create(&self, todo: Todo) -> Todo {
        self.audit.log("create", &todo.id.to_string()).await;
        self.events.publish("todo.created", &todo).await;
        self.repo.insert(todo).await
    }
}
