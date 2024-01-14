use std::collections::HashMap;

use crate::errors::Error;

#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if !params.contains_key("limit") || !params.contains_key("offset") {
        return Err(Error::MissingParameters);
    }

    Ok(Pagination {
        limit: Some(
            params
                .get("limit")
                .unwrap()
                .parse()
                .map_err(Error::ParseInt)?,
        ),
        offset: params
            .get("offset")
            .unwrap()
            .parse()
            .map_err(Error::ParseInt)?,
    })
}
