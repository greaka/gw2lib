use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod authenticated;
pub mod game_mechanics;
pub mod guild;
pub mod home_instance;
pub mod items;
pub mod maps;
pub mod misc;
pub mod pvp;
pub mod tradingpost;
pub mod wvw;

// todo: restructure
// todo: non_exhaustive
// todo: upmerge endpoint changes from master

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
    /// whether this endpoint requires authentication
    type Authenticated: Authentication;

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

    fn format_id(id: &Self::IdType) -> String {
        urlencoding::encode(&id.to_string()).into_owned()
    }

    fn format_url(id: &str) -> String {
        format!("{}/{}", Self::URL, id)
    }
}

pub trait FixedEndpoint: Endpoint {}

pub trait BulkEndpoint: EndpointWithId {
    /// whether this endpoint supports `ids=all`
    const ALL: bool;

    fn id(&self) -> &Self::IdType;
}

pub trait PagedEndpoint: Endpoint {}

impl<T: BulkEndpoint> PagedEndpoint for T {}

pub struct Authenticated;
pub struct NoAuthentication;

pub trait Authentication: Send + Sync + 'static {
    const AUTHENTICATED: bool;
}
impl Authentication for Authenticated {
    const AUTHENTICATED: bool = true;
}
impl Authentication for NoAuthentication {
    const AUTHENTICATED: bool = false;
}
