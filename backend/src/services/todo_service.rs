// Dave's version: subtask support
use crate::models::Todo;
use crate::db::TodoRepository;
use crate::events::EventBus;
use crate::audit::AuditLogger;
use uuid::Uuid;

pub struct TodoService {
    repo: TodoRepository,
    events: EventBus,
    audit: AuditLogger,
}

impl TodoService {
    pub async fn list(&self) -> Vec<Todo> { self.repo.find_all().await }
    pub async fn create_subtask(&self, parent_id: Uuid, subtask: Todo) -> Todo {
        self.audit.log("create_subtask", &parent_id.to_string()).await;
        self.events.publish("todo.subtask_created", &subtask).await;
        self.repo.insert_subtask(parent_id, subtask).await
    }
}
