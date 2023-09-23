use axum::extract::{Json, Path, State};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use repository::PostgresRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::Date;

mod repository;

time::serde::format_description!(date_ftm, Date, "[year]-[month]-[day]");

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct Person {
    id: i32,
    name: String,
    nick: String,
    #[serde(with = "date_ftm")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
pub struct NewPerson {
    name: String,
    nick: String,
    #[serde(with = "date_ftm")]
    birth_date: Date,
    stack: Option<Vec<String>>,
}

type AppState = Arc<PostgresRepository>;

async fn search_people(State(people): State<AppState>, search: String) -> impl IntoResponse {
    match people.search_people(search).await {
        Ok(people) => Ok(Json(people)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn find_person(State(people): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match people.find_person(id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    match people.create_person(new_person).await {
        Ok(person) => Ok((StatusCode::CREATED, Json(person))),
        Err(e) => {
            println!("{}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn count_people(State(people): State<AppState>) -> impl IntoResponse {
    match people.count_people().await {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let repo =
        PostgresRepository::connect("postgres://postgres:root@localhost:5432/postgres".to_string())
            .await;

    let repo_arch = Arc::new(repo);

    let app = Router::new()
        .route("/pessoas", get(search_people).post(create_person))
        .route("/pessoas/:id", get(find_person))
        .route("/conta-pessoas", get(count_people))
        .with_state(repo_arch);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
