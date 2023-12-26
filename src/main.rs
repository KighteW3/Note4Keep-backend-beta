use std::sync::Arc;

use axum::{extract::Extension, routing::post, Router};
use dotenv::dotenv;

use crate::db::connect::connect_db;
use crate::db::connect::DbState;
use crate::handlers::{
    notes::{create_note, delete_notes, delete_spec_note, get_notes, some_note, spec_note},
    users::{create_user, list_users, log_in},
};
use crate::utils::check_integrity::check_integrity;

use std::env;

pub mod auth;
pub mod db;
pub mod handlers;
pub mod utils;

type StateExtension = axum::extract::Extension<Arc<DbState>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    check_integrity();

    let db_state = Arc::new(DbState {
        db: connect_db().await,
    });

    let app = Router::new()
        .route("/api/users", post(list_users))
        .route("/api/notes", post(get_notes))
        .route("/api/users/create-user", post(create_user))
        .route("/api/users/login", post(log_in))
        .route("/api/notes/create-note", post(create_note))
        .route("/api/notes/some-note", post(some_note))
        .route("/api/notes/spec-note", post(spec_note))
        .route("/api/notes/delete-spec-note", post(delete_spec_note))
        .route("/api/notes/delete-notes", post(delete_notes))
        .layer(Extension(db_state));

    let mut bind_to = String::new();

    let ip = "0.0.0.0";

    let port = if let Ok(res) = env::var("PORT") {
        res
    } else {
        "3000".to_string()
    };

    bind_to.push_str(ip);
    bind_to.push_str(":");
    bind_to.push_str(&port);

    println!("The server is open on {}:{}", ip, port);

    let listener = tokio::net::TcpListener::bind(bind_to).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
