use axum::extract::{Json, Path, State};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Serialize)]
struct Person {
    id: i32,
    name: String,
    nick: String,
    stack: Option<Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct NewPerson {
    id: i32,
    name: String,
    nick: String,
    stack: Option<Vec<String>>,
}

type AppState = Arc<Mutex<HashMap<i32, Person>>>;

async fn search_people(state: State<AppState>) -> impl IntoResponse {
    let State(people) = state;
    (StatusCode::OK, "Busca pessoas")
}

async fn find_person(State(people): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let my_people = people.lock().await;
    match my_people.get(&id) {
        Some(person) => Ok(Json(person.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_person(
    State(people): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    let id = new_person.id;
    let person = Person {
        id,
        name: new_person.name,
        nick: new_person.nick,
        stack: new_person.stack,
    };
    people.lock().await.insert(id, person.clone());
    (StatusCode::CREATED, Json(person))
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::OK, "Conta pessoas")
}

#[tokio::main]
async fn main() {
    let mut people: HashMap<i32, Person> = HashMap::new();

    let person = Person {
        id: 1,
        name: "João".to_string(),
        nick: "kakashi".to_string(),
        stack: None,
    };
    people.insert(1, person);

    let app_state = Arc::new(Mutex::new(people));

    let app = Router::new()
        .route("/pessoas", get(search_people).post(create_person))
        .route("/pessoas/:id", get(find_person))
        .route("/conta-pessoas", get(count_people))
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
