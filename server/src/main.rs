use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::{get, put},
    Router,
};
use sqlx::SqlitePool;

struct Pool(SqlitePool);

#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/form", put(handle_form_put))
        .route("/", get(|| async { "hello world" }))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(Arc::new(Pool(
            SqlitePool::connect(&std::env!("DATABASE_URL"))
                .await
                .unwrap(),
        )));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_form_put(State(pool): State<Arc<Pool>>, Query(data): Query<FormData>) {
    let mut conn = pool.0.acquire().await.unwrap();
    println!("Got message: {data:?}");

    sqlx::query!(
        "INSERT INTO Message VALUES (?1, ?2, ?3, ?4);",
        data.name,
        data.email,
        data.subject,
        data.message
    )
    .execute(&mut conn)
    .await
    .unwrap();
}
