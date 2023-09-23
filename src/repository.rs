use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{NewPerson, Person};

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

    pub async fn get_all_persons(&self) -> Vec<Person> {
        todo!()
    }

    pub async fn search_people(&self, query: String) -> Result<Vec<Person>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM people WHERE to_tsquery('people' $1) @@ search")
            .bind(query)
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

    pub async fn update_person(&self, _person: Person) -> Option<Person> {
        todo!()
    }

    pub async fn count_people(&self) -> Result<i32, sqlx::Error> {
        // sqlx::query("SELECT COUNT(id) FROM people").fetch_one(&self.pool).await;
        Ok(10)
    }

    pub async fn delete_person(&self, _id: i32) -> bool {
        todo!()
    }
}
