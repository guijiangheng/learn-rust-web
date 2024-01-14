use tracing::{error, instrument, warn};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

#[derive(Debug)]
pub enum Error {
    ParseInt(std::num::ParseIntError),
    MissingParameters,
    DatabaseQuery,
    ReqwestAPI(reqwest::Error),
    MiddlewareReqwestAPI(reqwest_middleware::Error),
    APILayerClient(APILayerError),
    APILayerServer(APILayerError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseInt(ref err) => write!(fmt, "Cannot parse parameter {}", err),
            Error::MissingParameters => write!(fmt, "Missing parameters"),
            Error::DatabaseQuery => write!(fmt, "Database query error"),
            Error::ReqwestAPI(ref err) => write!(fmt, "External API error: {}", err),
            Error::MiddlewareReqwestAPI(ref err) => write!(fmt, "External API error: {}", err),
            Error::APILayerClient(ref err) => write!(fmt, "API layer client error: {}", err),
            Error::APILayerServer(ref err) => write!(fmt, "API layer server error: {}", err),
        }
    }
}

impl Reject for Error {}
impl Reject for APILayerError {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::DatabaseQuery) = r.find() {
        error!("Database query error");
        Ok(warp::reply::with_status(
            Error::DatabaseQuery.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(Error::ReqwestAPI(err)) = r.find() {
        error!("{}", err);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::MiddlewareReqwestAPI(err)) = r.find() {
        error!("{}", err);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::APILayerClient(err)) = r.find() {
        error!("{}", err);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(Error::APILayerServer(err)) = r.find() {
        error!("{}", err);
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(err) = r.find::<CorsForbidden>() {
        error!("CORS forbidden error: {}", err);
        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(err) = r.find::<BodyDeserializeError>() {
        error!("Cannot deserizalize request body: {}", err);
        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(err) = r.find::<Error>() {
        error!("{}", err);
        Ok(warp::reply::with_status(
            err.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        warn!("Requested route not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
