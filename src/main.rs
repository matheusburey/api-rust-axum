mod controlles;
mod model;
mod repository;

use axum::{routing::get, Router};
use controlles::{create_person, delete_person, find_person, get_all_persons, update_person};
use dotenv::dotenv;
use repository::PostgresRepository;

use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let repo = PostgresRepository::connect(env::var("DB_URL").expect("DB_URL")).await;

    let repo_arch = Arc::new(repo);

    let app = Router::new()
        .route("/pessoas", get(get_all_persons).post(create_person))
        .route(
            "/pessoas/:id",
            get(find_person).patch(update_person).delete(delete_person),
        )
        .with_state(repo_arch);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
