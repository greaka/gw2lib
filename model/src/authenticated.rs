use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint, TimeStamp};
pub mod account;
pub mod characters;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Permissions {
    Account,
    Builds,
    Characters,
    Guilds,
    Inventories,
    Progression,
    PvP,
    TradingPost,
    Unlocks,
    Wallet,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenType {
    APIKey,
    Subtoken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtokenDetails {
    pub expires_at: TimeStamp,
    pub issued_at: TimeStamp,
    pub urls: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tokeninfo {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permissions>,
    #[serde(rename = "type")]
    pub _type: TokenType,
    #[serde(flatten)]
    details: Option<SubtokenDetails>,
}

impl Endpoint for Tokeninfo {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/tokeninfo";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl FixedEndpoint for Tokeninfo {}
