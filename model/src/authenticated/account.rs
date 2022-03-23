use serde::{Deserialize, Serialize};

pub use crate::misc::worlds::WorldId;
use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum Access {
    None,
    PlayForFree,
    GuildWars2,
    HeartOfThorns,
    PathOfFire,
    EndOfDragons,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl Endpoint for Account {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl FixedEndpoint for Account {}
