use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    En,
    Fr,
    De,
    Es,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

pub type TimeStamp = String;

pub fn format_ids(item_ids: impl IntoIterator<Item = impl std::fmt::Display>) -> String {
    let items = item_ids
        .into_iter()
        .fold(String::new(), |acc, x| format!("{},{}", acc, x));
    (&items[1..]).to_owned()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> Into<Result<T, ApiError>> for ApiResult<T> {
    fn into(self) -> Result<T, ApiError> {
        match self {
            ApiResult::Ok(data) => Ok(data),
            ApiResult::Err(err) => Err(err),
        }
    }
}

impl<T> Into<Result<T, Box<dyn std::error::Error>>> for ApiResult<T> {
    fn into(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            ApiResult::Ok(data) => Ok(data),
            ApiResult::Err(err) => Err(Box::new(err)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    text: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text)
    }
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.text)
    }
}

impl std::error::Error for ApiError {}
