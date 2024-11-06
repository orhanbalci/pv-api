use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use quiz::{Question, Quiz};
use rand::{seq::SliceRandom, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
pub mod quiz;

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

async fn search(
    Path(term): Path<String>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Proverb>("SELECT * FROM proverb WHERE proverb like $1")
        .bind(format!("%{}%", term))
        .fetch_all(&state.pool)
        .await
    {
        Ok(proverbs) => Ok((StatusCode::OK, Json(proverbs))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn quiz(State(state): State<MyState>) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Proverb>("SELECT * FROM proverb")
        .fetch_all(&state.pool)
        .await
    {
        Ok(proverbs) => {
            let mut rng = rand::thread_rng();
            let mut quiz = Quiz::new();

            for _ in 0..10 {
                let random_index = rng.gen_range(0..proverbs.len());
                let asked_proverb = &proverbs[random_index];
                let mut options = vec![cleanup(&asked_proverb.meaning)];

                while options.len() < 4 {
                    let random_index = rng.gen_range(0..proverbs.len());
                    let random_proverb = &proverbs[random_index];
                    if !options.contains(&random_proverb.meaning) {
                        options.push(cleanup(&random_proverb.meaning));
                    }
                }

                options.shuffle(&mut rng);
                quiz.add_question(Question::new(
                    asked_proverb.proverb.clone(),
                    options,
                    cleanup(&asked_proverb.meaning),
                ));
            }

            Ok(Json(quiz))
        }
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
        .route("/proverb/search/:term", get(search))
        .route("/proverb/quiz", get(quiz))
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

fn cleanup(proverb_meaning: &str) -> String {
    remove_after_char(&remove_numbered_patterns(proverb_meaning).trim_start(), ':')
}

fn remove_numbered_patterns(input: &str) -> String {
    let re = Regex::new(r"\d+\)").unwrap(); // Matches any digit(s) followed by a closing parenthesis
    re.replace_all(input, "").to_string() // Replaces matched patterns with an empty string
}

fn remove_after_char(input: &str, delimiter: char) -> String {
    // Split at the first occurrence of `delimiter` and take the first part
    input.split(delimiter).next().unwrap_or("").to_string()
}
