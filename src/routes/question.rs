use crate::{
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        question::{NewQuestion, UpdateQuestion},
    },
};
use std::collections::HashMap;
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
    match store.add_question(new_question).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
