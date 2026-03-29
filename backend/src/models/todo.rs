use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
    pub priority: Priority,
    pub assignee: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus { Pending, InProgress, Done, Archived }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority { Low, Medium, High, Critical }
