use crate::{
    errors::Error,
    store::Store,
    types::{
        pagination::extract_pagination,
        question::{Question, QuestionId},
    },
};
use std::collections::HashMap;
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if params.is_empty() {
        let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();
        return Ok(warp::reply::json(&questions));
    }

    let pagination = extract_pagination(params)?;
    let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();
    let questions = &questions[pagination.start..pagination.end];

    Ok(warp::reply::json(&questions))
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}

pub async fn delete_question(id: String, store: Store) -> Result<impl Reply, Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => (),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }

    Ok(warp::reply::with_status("Question deleted", StatusCode::OK))
}

pub async fn add_question(store: Store, question: Question) -> Result<impl Reply, Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}
