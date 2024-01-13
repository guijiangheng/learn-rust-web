use tracing::{error, instrument};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
pub enum Error {
    Parse(std::num::ParseIntError),
    MissingParameters,
    DatabaseQuery,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Parse(ref err) => write!(fmt, "Cannot parse parameter {}", err),
            Error::MissingParameters => write!(fmt, "Missing parameters"),
            Error::DatabaseQuery => write!(fmt, "Database query error"),
        }
    }
}

impl Reject for Error {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        error!("{}", error.to_string());
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        error!("CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        error!("{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        error!("Requested route not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
