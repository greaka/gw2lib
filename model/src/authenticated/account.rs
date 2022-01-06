use crate::*;
use serde::{Deserialize, Serialize};

pub use crate::misc::worlds::WorldId;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Access {
    None,
    PlayForFree,
    GuildWars2,
    HeartOfThorns,
    PathOfFire,
    EndOfDragons,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id:            String,
    pub age:           u64,
    pub name:          String,
    pub world:         WorldId,
    pub guilds:        Vec<String>,
    pub guild_leader:  Option<Vec<String>>,
    pub created:       TimeStamp,
    pub access:        Vec<Access>,
    pub commander:     bool,
    pub fractal_level: Option<u8>,
    pub daily_ap:      Option<u16>,
    pub monthly_ap:    Option<u16>,
    pub wvw_rank:      Option<u16>,
    pub last_modified: String,
}
