use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod authenticated;
pub mod game_mechanics;
pub mod guild;
pub mod items;
pub mod misc;
pub mod pvp;
pub mod tradingpost;
pub mod wvw;

#[derive(Hash, Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Language {
    En,
    Fr,
    De,
    Es,
    Zh,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Fr => "fr",
            Language::De => "de",
            Language::Es => "es",
            Language::Zh => "zh",
        }
    }
}

impl From<&str> for Language {
    fn from(v: &str) -> Self {
        match v {
            "en" => Language::En,
            "fr" => Language::Fr,
            "de" => Language::De,
            "es" => Language::Es,
            "zh" => Language::Zh,
            _ => Language::En,
        }
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
    /// whether this endpoint requires authentication
    const AUTHENTICATED: bool;

    /// whether this endpoint supports the language parameter
    const LOCALE: bool;

    /// endpoint url in the format `v2/account`
    /// ### Remarks
    /// Among other things, this URL is used to fetch ids.
    /// `v2/characters/My Character/core` still requires `v2/characters` to be
    /// set here. For special cases like characters, override the fetch url
    /// of single items here: [EndpointWithId::format_url]
    const URL: &'static str;

    /// version of the endpoint to request
    const VERSION: &'static str;
}

pub trait EndpointWithId: Endpoint {
    type IdType: Display;

    fn format_url(id: &str) -> String {
        format!("{}/{}", Self::URL, id)
    }
}

pub trait FixedEndpoint: Endpoint {}

pub trait BulkEndpoint: EndpointWithId {
    fn id(&self) -> &Self::IdType;

    /// whether this endpoint supports `ids=all`
    const ALL: bool;
    /// whether this endpoint supports pagination à la `page=1&page_size=200`
    const PAGING: bool;
}
