use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

async fn retrieve(
    Path(id): Path<i32>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Proverb>("SELECT * FROM proverb WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

// async fn add(
//     State(state): State<MyState>,
//     Json(data): Json<TodoNew>,
// ) -> Result<impl IntoResponse, impl IntoResponse> {
//     match sqlx::query_as::<_, Proverb>("INSERT INTO todos (note) VALUES ($1) RETURNING id, note")
//         .bind(&data.note)
//         .fetch_one(&state.pool)
//         .await
//     {
//         Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
//         Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
//     }
// }

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        // .route("/todos", post(add))
        .route("/proverb/:id", get(retrieve))
        .with_state(state);

    Ok(router.into())
}

#[derive(Serialize, FromRow)]
struct Proverb {
    pub id: i32,
    pub proverb: String,
    pub meaning: String,
    pub proverb_type: String,
}
