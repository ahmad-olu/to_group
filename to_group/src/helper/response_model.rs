use serde::{Deserialize, Serialize};

use super::schema::{SubTodoSchema, TodoSchema, UserSchema};

// ---------------- request

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
    pub date_joined: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoRequest {
    pub title: String,
    pub description: String,
    pub time_from: String,
    pub time_to: String,
    pub date: String,
    pub is_completed: bool,
    pub sub_todo: Option<Vec<SubTodoSchema>>,
}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SubTodoRequest {
//     pub id: String,
//     pub title: String,
//     pub description: String,
//     pub time_from: String,
//     pub time_to: String,
//     pub icon: String,
//     pub color: String,
//     pub task_type: String,
// }

// ----------------response

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponseModel {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub result: UserSchema,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoListResponse {
    pub length: usize,
    pub result: Vec<TodoSchema>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoResponse {
    pub result: TodoSchema,
}

// ----------------request update
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friends: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_todo: Option<Vec<SubTodoUpdateRequest>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubTodoUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}
