use serde::{Deserialize, Serialize};

use crate::TimeStamp;
pub mod account;
pub mod characters;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    APIKey,
    Subtoken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubtokenDetails {
    pub expires_at: TimeStamp,
    pub issued_at:  TimeStamp,
    pub urls:       Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tokeninfo {
    pub id:          String,
    pub name:        String,
    pub permissions: Vec<Permissions>,
    #[serde(rename = "type")]
    pub _type:       TokenType,
    #[serde(flatten)]
    details:         Option<SubtokenDetails>,
}
