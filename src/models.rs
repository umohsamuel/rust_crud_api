use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    // pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct _NewUser<'a> {
    pub id: String,
    pub username: &'a str,
    pub password_hash: &'a str,
}
