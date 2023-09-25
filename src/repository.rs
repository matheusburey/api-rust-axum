use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::model::{NewPerson, Person};

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub async fn connect(url: String) -> Self {
        PostgresRepository {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await
                .unwrap(),
        }
    }

    pub async fn find_person(&self, id: i32) -> Result<Option<Person>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM people WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_all_persons(&self) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM people")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_person(&self, person: NewPerson) -> Result<Person, sqlx::Error> {
        sqlx::query_as("INSERT INTO people (name, nick, birth_date, stack) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(person.name)
        .bind(person.nick)
        .bind(person.birth_date)
        .bind(person.stack)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_person(&self, id: i32, person: NewPerson) -> Result<Person, sqlx::Error> {
        sqlx::query_as(
            "UPDATE people SET name = $1, birth_date = $2, stack = $3 WHERE id = $4 RETURNING *",
        )
        .bind(person.name)
        .bind(person.birth_date)
        .bind(person.stack)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_person(&self, id: i32) -> Result<Person, sqlx::Error> {
        sqlx::query_as("DELETE FROM people WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
}
