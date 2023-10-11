use crate::model::NewPerson;
use crate::repository::PostgresRepository;

use axum::extract::{Json, Path, State};
use axum::{http::StatusCode, response::IntoResponse};
use std::sync::Arc;

type AppState = Arc<PostgresRepository>;

pub async fn get_all_persons(State(people): State<AppState>) -> impl IntoResponse {
    match people.get_all_persons().await {
        Ok(people) => Ok(Json(people)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn find_person(State(people): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match people.find_person(id).await {
        Ok(Some(person)) => Ok(Json(person)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_person(
    State(db): State<AppState>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    match db.create_person(new_person).await {
        Ok(person) => Ok((StatusCode::CREATED, Json(person))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_person(
    State(people): State<AppState>,
    Path(id): Path<i32>,
    Json(new_person): Json<NewPerson>,
) -> impl IntoResponse {
    match people.update_person(id, new_person).await {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_person(
    State(people): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match people.delete_person(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
