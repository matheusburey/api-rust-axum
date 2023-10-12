mod controlles;
mod model;
mod repository;

use controlles::*;
use repository::PostgresRepository;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::{env, sync::Arc};

async fn app(repo_arch: Arc<PostgresRepository>) -> Router {
    Router::new()
        .route("/pessoas", get(get_all_persons).post(create_person))
        .route(
            "/pessoas/:id",
            get(find_person).patch(update_person).delete(delete_person),
        )
        .with_state(repo_arch)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let repo = PostgresRepository::connect(env::var("DB_URL").expect("DB_URL")).await;
    let repo_arch = Arc::new(repo);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app(repo_arch).await.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::model::NewPersonTest;

    use super::*;

    use ::axum_test::TestServer;
    use axum::http::StatusCode;
    use time::macros::date;

    async fn create_server() -> TestServer {
        dotenv().ok();
        let repo = PostgresRepository::connect(env::var("DB_URL_TEST").expect("DB_URL_TEST")).await;
        let repo_arch = Arc::new(repo);
        TestServer::new(app(repo_arch).await.into_make_service()).unwrap()
    }

    #[tokio::test]
    async fn get_all_persons_test() {
        let server = create_server().await;

        let response = server.get("/pessoas").await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_person_test() {
        let server = create_server().await;

        let people = NewPersonTest {
            name: "João".to_string(),
            nick: "joao".to_string(),
            birth_date: date!(2000 - 1 - 1),
            stack: Some(vec!["Java".to_string(), "Python".to_string()]),
        };

        let response = server.post("/pessoas").json(&people).await;
        println!("{:?}", response);
        assert_eq!(response.status_code(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn find_person_test() {
        let server = create_server().await;

        let response = server.get("/pessoas/1").await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn update_person_test() {
        let server = create_server().await;

        let people = NewPersonTest {
            name: "João".to_string(),
            nick: "kakashi".to_string(),
            birth_date: date!(2000 - 8 - 19),
            stack: Some(vec![
                "Java".to_string(),
                "Python".to_string(),
                "Rust".to_string(),
            ]),
        };

        let response = server.patch("/pessoas/1").json(&people).await;
        println!("{:?}", response.text());
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn delete_person_test() {
        let server = create_server().await;

        let response = server.delete("/pessoas/1").await;
        println!("{:?}", response);
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
