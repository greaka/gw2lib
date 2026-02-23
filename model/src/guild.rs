use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    guild::emblem::{GuildEmblemBackgroundId, GuildEmblemForegroundId},
    misc::colors::ColorId,
    Endpoint, EndpointWithId,
};

pub mod emblem;
pub mod permissions;
pub mod search;
pub mod upgrades;

pub type GuildId = Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GuildEmblemFlag {
    FlipBackgroundHorizontal,
    FlipBackgroundVertical,
    FlipForegroundHorizontal,
    FlipForegroundVertical,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblemBackLayer {
    pub id: GuildEmblemBackgroundId,
    pub colors: Vec<ColorId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblemForeLayer {
    pub id: GuildEmblemForegroundId,
    pub colors: Vec<ColorId>,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblem {
    pub background: GuildEmblemBackLayer,
    pub foreground: GuildEmblemForeLayer,
    pub flags: Vec<GuildEmblemFlag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Guild {
    pub id: GuildId,
    pub name: String,
    pub tag: String,
    pub emblem: Option<GuildEmblem>,
}

impl Endpoint for Guild {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
}

impl EndpointWithId for Guild {
    type IdType = GuildId;
}
