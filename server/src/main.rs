use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::put,
    Router,
};
use axum_extra::routing::SpaRouter;
use sqlx::SqlitePool;

#[derive(Clone)]
struct Pool(SqlitePool);

#[derive(Debug, serde::Deserialize, Clone)]
struct Config<'s> {
    asset_path: &'s str,
}

#[derive(Debug, serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let config = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    let pool = Arc::new(Pool(
        SqlitePool::connect(&std::env!("DATABASE_URL"))
            .await
            .unwrap(),
    ));

    let app = Router::new()
        .route("/api/v1/form", put(handle_form_put))
        .merge(SpaRouter::new("/", config.asset_path).index_file("index.html"))
        .with_state(pool);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
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
