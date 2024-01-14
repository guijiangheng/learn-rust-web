use reqwest::Response;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;

use crate::errors::{APILayerError, Error};

#[derive(Deserialize)]
pub struct APILayerErrorDetail {
    message: String,
}

#[derive(Deserialize)]
struct BadWord {
    original: String,
    word: String,
    deviations: i32,
    info: i32,
    #[serde(rename = "replacedLen")]
    replaced_len: i32,
}

#[derive(Deserialize)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i32,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, Error> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", "CPfbRgUYd9uiPMO9sDZivF5QKebjHNkO")
        .body(content)
        .send()
        .await
        .map_err(Error::MiddlewareReqwestAPI)?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            return Err(Error::APILayerClient(transform_error(res).await));
        } else {
            return Err(Error::APILayerServer(transform_error(res).await));
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(err) => Err(Error::ReqwestAPI(err)),
    }
}

async fn transform_error(res: Response) -> APILayerError {
    APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APILayerErrorDetail>().await.unwrap().message,
    }
}
