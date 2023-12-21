use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub date_joined: Option<String>,
    pub job: Option<String>,
    pub friends: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub uid: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub time_from: String,
    pub time_to: String,
    pub date: String,
    pub is_completed: bool,
    pub sub_todo: Option<Vec<SubTodoSchema>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubTodoSchema {
    pub id: String,
    pub title: String,
    pub description: String,
    pub time_from: String,
    pub time_to: String,
    pub icon: String,
    pub color: String,
    pub task_type: String,
}
