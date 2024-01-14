use crate::{
    profanity::check_profanity,
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, UpdateQuestion},
    },
};
use std::collections::HashMap;
use tokio::join;
use tracing::{info, instrument};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    info!("querying questions");

    let mut pagination = Pagination::default();

    if !params.is_empty() {
        info!(pagination = true);
        pagination = extract_pagination(params)?;
    }

    match store.get_questions(pagination).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn update_question(
    question_id: i32,
    store: Store,
    question: UpdateQuestion,
) -> Result<impl Reply, Rejection> {
    let question = question_check_profanity(question).await?;

    match store.update_question(question_id, question).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn delete_question(question_id: i32, store: Store) -> Result<impl Reply, Rejection> {
    match store.delete_question(question_id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", question_id),
            StatusCode::OK,
        )),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl Reply, Rejection> {
    let new_question = question_check_profanity(new_question).await?;

    match store.add_question(new_question).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}

async fn question_check_profanity(question: NewQuestion) -> Result<NewQuestion, Rejection> {
    let (title, content) = join!(
        check_profanity(question.title),
        check_profanity(question.content)
    );

    if let Err(err) = title {
        return Err(warp::reject::custom(err));
    }

    if let Err(err) = content {
        return Err(warp::reject::custom(err));
    }

    Ok(NewQuestion {
        title: title.unwrap(),
        content: content.unwrap(),
        ..question
    })
}
