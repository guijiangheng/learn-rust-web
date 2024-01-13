use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};
use tracing::error;

use crate::{
    errors::Error,
    types::{
        answer::{Answer, AnswerId, NewAnswer},
        pagination::Pagination,
        question::{NewQuestion, Question, QuestionId, UpdateQuestion},
    },
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

fn to_question(row: PgRow) -> Question {
    Question {
        id: QuestionId(row.get("id")),
        title: row.get("title"),
        content: row.get("content"),
        tags: row.get("tags"),
    }
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(err) => panic!("Couldn't establish DB connection: {}", err),
        };

        Store { connection: pool }
    }

    pub async fn get_questions(&self, pagination: Pagination) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(pagination.limit)
            .bind(pagination.offset)
            .map(to_question)
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(err) => {
                error!("{:?}", err);
                Err(Error::DatabaseQuery)
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags)
            VALUES ($1, $2, $3)
            RETURNING id, title, content, tags
        ",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(to_question)
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(err) => {
                error!("{:?}", err);
                Err(Error::DatabaseQuery)
            }
        }
    }

    pub async fn update_question(
        &self,
        question_id: i32,
        question: UpdateQuestion,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3
            WHERE id = $4,
            RETURNING id, title, content, tags
        ",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(to_question)
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                error!("{:?}", e);
                Err(Error::DatabaseQuery)
            }
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(err) => {
                error!("{:?}", err);
                Err(Error::DatabaseQuery)
            }
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query("INSERT INTO answers VALUES (content, question_id) VALUES ($1, $2)")
            .bind(new_answer.content)
            .bind(new_answer.question_id.0)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(answer) => Ok(answer),
            Err(err) => {
                error!("{:?}", err);
                Err(Error::DatabaseQuery)
            }
        }
    }
}
