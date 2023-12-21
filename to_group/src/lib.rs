pub mod app_state;
pub mod helper;
pub mod middleware;
pub mod router;
pub mod routes;

use app_state::AppState;
use axum::Router;
use router::create_route;

pub async fn run(app_state: AppState) {
    let app: Router = create_route(app_state).await;
    let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(address, app).await.unwrap();
}

// pub async fn run() {
//     let app: Router = Router::new()
//         .route("/", get(|| async { "Hello, World!" }))
//         .route("/login", post(login))
//         .route("/logout", post(logout))
//         .route("/users", post(create_user))
//         .route("/users", patch(update_user))
//         .route("/todo", get(get_all_todo).post(create_todo))
//         .route("/todo/:todo_id", get(get_one_todo).patch(update_todo))
//         ;

//     let address = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(address, app).await.unwrap();
// }
