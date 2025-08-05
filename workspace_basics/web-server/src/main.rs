use axum::{Json, extract::Path, routing::get, Router};
use shared_utils::User;
use database::Database;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/user/{id}", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn get_user(Path(id): Path<u64>) -> Json<Option<User>> {
    let db = Database::new();
    let user = db.get_user(id).await;
    Json(user)
}
