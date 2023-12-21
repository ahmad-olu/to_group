use axum::http::StatusCode;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use mongodb::{options::ClientOptions, Client};
use to_group::{
    app_state::AppState,
    helper::{
        app_error::AppError,
        schema::{TodoSchema, UserSchema},
    },
    run,
};

const USERCOLLECTION: &str = "users";
const TODOCOLLECTION: &str = "todos";
const CLUSTUER: &str = "Cluster0";

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    let database_url = dotenv!("MONGO_SERVER_URL").to_owned();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();

    let mut client_options = ClientOptions::parse(database_url)
        .await
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not connect to mongo"))?;
    client_options.app_name = Some(CLUSTUER.to_string());

    let client = Client::with_options(client_options)
        .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Could not connect to mongo"))?;
    let database = client.database(CLUSTUER);

    let app_state = AppState {
        u_collection: database.collection::<UserSchema>(USERCOLLECTION),
        t_collection: database.collection::<TodoSchema>(TODOCOLLECTION),
        jwt_secret,
    };

    run(app_state).await;

    // let user = UserSchema {
    //     id: None,
    //     full_name: "allan".to_string(),
    //     email: "allan@gmail.com".to_string(),
    //     password: "password12345".to_string(),
    //     token: "-ssndslbajbhluydf".to_string(),
    //     date_joined: "12-2-21".to_string(),
    //     job: None,
    //     friends: None,
    // };

    Ok(())
}
