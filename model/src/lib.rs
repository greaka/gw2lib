use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod achievements;
pub mod authenticated;
pub mod daily_rewards;
pub mod game_mechanics;
pub mod guild;
pub mod home_instance;
pub mod items;
pub mod maps;
pub mod misc;
pub mod pvp;
pub mod story;
pub mod tradingpost;
pub mod wvw;

#[derive(Hash, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Hash, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub text: String,
}

pub trait Endpoint: Sized {
    const AUTHENTICATED: bool;
    const LOCALE: bool;
    const URL: &'static str;
    const VERSION: &'static str;
}

pub trait EndpointWithId: Endpoint {
    type IdType: Display;

    fn format_id(id: &Self::IdType) -> String {
        urlencoding::encode(&id.to_string()).into_owned()
    }

    fn format_url(id: &str) -> String {
        format!("{}/{}", Self::URL, id)
    }
}

pub trait FixedEndpoint: Endpoint {}

pub trait BulkEndpoint: EndpointWithId {
    const ALL: bool;

    fn id(&self) -> &Self::IdType;
}

pub trait PagedEndpoint: Endpoint {}

impl<T: BulkEndpoint> PagedEndpoint for T {}
