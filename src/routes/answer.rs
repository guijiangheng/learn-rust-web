use crate::{store::Store, types::answer::NewAnswer};
use warp::{http::StatusCode, reject::Rejection, reply::Reply};

pub async fn add_answer(store: Store, new_answer: NewAnswer) -> Result<impl Reply, Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(err) => Err(warp::reject::custom(err)),
    }
}
