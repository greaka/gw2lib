pub mod authenticated;
pub mod game_mechanics;
pub mod items;
pub mod misc;
pub mod pvp;
pub mod tradingpost;
pub mod wvw;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    En,
    Fr,
    De,
    Es,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub type TimeStamp = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[must_use]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> ApiResult<T> {
    pub fn to_result(self) -> Result<T, ApiError> {
        Result::<T, ApiError>::from(self)
    }
}

impl<T> From<ApiResult<T>> for Result<T, ApiError> {
    fn from(val: ApiResult<T>) -> Self {
        match val {
            ApiResult::Ok(data) => Ok(data),
            ApiResult::Err(err) => Err(err),
        }
    }
}

impl<T> From<ApiResult<T>> for Result<T, Box<dyn std::error::Error>> {
    fn from(val: ApiResult<T>) -> Self {
        match val {
            ApiResult::Ok(data) => Ok(data),
            ApiResult::Err(err) => Err(Box::new(err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    text: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text)
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(&self, f)
    }
}

impl std::error::Error for ApiError {}

pub trait Endpoint: Sized {
    /// endpoint url in the format `v2/account`
    fn url() -> &'static str;
}

pub trait FixedEndpoint: Endpoint {}

pub trait BulkEndpoint: Endpoint {
    /// denominates the type of the ids of this endpoint
    type IdType;

    /// whether this endpoint supports pagination à la `page=1&page_size=200`
    const PAGING: bool;
    /// whether this endpoint supports `ids=all`
    const ALL: bool;
}
