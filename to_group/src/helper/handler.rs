use axum::http::StatusCode;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_document, Document},
    Collection,
};

use super::{
    app_error::AppError,
    response_model::{
        LoginResponseModel, TodoListResponse, TodoResponse, TodoUpdateRequest, UserResponse,
        UserUpdateRequest,
    },
    schema::{TodoSchema, UserSchema},
};

pub async fn create_user_helper(
    user_collection: Collection<UserSchema>,
    user: UserSchema,
) -> Result<LoginResponseModel, AppError> {
    let created_user = user_collection
        .insert_one(user, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not insert user"))?;
    let obj_id = created_user.inserted_id.as_object_id().unwrap();

    let filter: Document = doc! {"_id":obj_id};

    let user_schema = user_collection
        .find_one(filter, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get todo 1"))?;
    if user_schema.is_some() {
        let user = user_schema.unwrap();

        Ok(LoginResponseModel {
            full_name: user.full_name,
            email: user.email,
            id: user.id.unwrap().to_string(),
            token: user.token,
        })
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Could not get todo 2",
        ))
    }
}

pub async fn create_todo_helper(
    todo_collection: Collection<TodoSchema>,
    todo: TodoSchema,
) -> Result<TodoResponse, AppError> {
    let created_todo = todo_collection
        .insert_one(todo, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not insert todo"))?;

    let obj_id = created_todo.inserted_id.as_object_id().unwrap();
    let query = doc! {"_id":obj_id};
    let todo_schema = todo_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))?;
    if todo_schema.is_some() {
        Ok(TodoResponse {
            result: todo_schema.unwrap(),
        })
    } else {
        Err(AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))
    }
}

pub async fn delete_user_helper(
    user_collection: Collection<UserSchema>,
    uid: &String,
) -> Result<(), AppError> {
    let query = doc! {"_id":ObjectId::parse_str(uid).unwrap()};

    user_collection
        .delete_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not delete user"))?;
    Ok(())
}

pub async fn delete_todo_helper(
    todo_collection: Collection<TodoSchema>,
    uid: &String,
) -> Result<(), AppError> {
    let query = doc! {"_id":ObjectId::parse_str(uid).unwrap()};

    todo_collection
        .delete_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not delete todo"))?;
    Ok(())
}

pub async fn get_user_todos_helper(
    todo_collection: Collection<TodoSchema>,
    uid: &String,
) -> Result<TodoListResponse, AppError> {
    let query = doc! {"uid":ObjectId::parse_str(uid).unwrap()};
    let mut cursor = todo_collection
        .find(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))?;

    let mut json_result: Vec<TodoSchema> = Vec::new();
    while let Some(todo) = cursor
        .try_next()
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))?
    {
        json_result.push(todo);
    }

    Ok(TodoListResponse {
        length: json_result.len(),
        result: json_result,
    })
}

pub async fn get_user_details_helper(
    user_collection: Collection<UserSchema>,
    uid: &String,
) -> Result<UserResponse, AppError> {
    let query = doc! {"_id":ObjectId::parse_str(uid).unwrap()};
    let res = user_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details"))?;
    match res {
        Some(doc) => Ok(UserResponse { result: doc }),
        None => Err(AppError::new(StatusCode::BAD_REQUEST, "Could not get todo")),
    }
}

pub async fn get_user_by_email_helper(
    user_collection: Collection<UserSchema>,
    email: &String,
) -> Result<UserSchema, AppError> {
    let query = doc! {"email":email};
    let res = user_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details"))?;
    match res {
        Some(doc) => Ok(doc),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "User does not exist",
        )),
    }
}

pub async fn get_user_by_token_helper(
    user_collection: Collection<UserSchema>,
    token: &String,
) -> Result<UserSchema, AppError> {
    let query = doc! {"token":token};
    let res = user_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details"))?;
    match res {
        Some(doc) => Ok(doc),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "User does not exist",
        )),
    }
}

pub async fn update_user_details_helper(
    uid: &String,
    user_collection: Collection<UserSchema>,
    update_user: UserUpdateRequest,
) -> Result<UserResponse, AppError> {
    let query = doc! {"_id":ObjectId::parse_str(uid).unwrap()};
    let doc = to_document(&update_user)
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details, 1"))?;

    let doc = doc! {
        "$set": doc,
    };

    user_collection
        .update_one(query.clone(), doc, None)
        .await
        .map_err(|err| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                format!("Could not get user details, 2, {:?}", err),
            )
        })?;

    let user_schema = user_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user 3"))?;
    if user_schema.is_some() {
        let user = user_schema.unwrap();

        Ok(UserResponse { result: user })
    } else {
        Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Could not get user 4",
        ))
    }
}

pub async fn update_user_todo_helper(
    todo_collection: Collection<TodoSchema>,
    uid: &String,
    update_todo: TodoUpdateRequest,
) -> Result<TodoResponse, AppError> {
    let query = doc! {"_id":ObjectId::parse_str(uid).unwrap()};
    let doc = to_document(&update_todo)
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details"))?;

    todo_collection
        .update_one(query.clone(), doc, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get user details"))?;

    let todo_schema = todo_collection
        .find_one(query, None)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))?;
    if todo_schema.is_some() {
        Ok(TodoResponse {
            result: todo_schema.unwrap(),
        })
    } else {
        Err(AppError::new(StatusCode::BAD_REQUEST, "Could not get todo"))
    }
}
