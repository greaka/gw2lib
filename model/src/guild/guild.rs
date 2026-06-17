use serde::{Deserialize, Serialize};

use crate::{misc::colors::ColorId, BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblemLayer {
    pub id: u32,
    pub colors: Vec<ColorId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblem {
    pub background: GuildEmblemLayer,
    pub foreground: GuildEmblemLayer,
    pub flags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub level: Option<u32>,
    pub motd: Option<String>,
    pub influence: Option<u32>,
    pub aetherium: Option<u32>,
    pub resonance: Option<u32>,
    pub favor: Option<u32>,
    pub member_count: Option<u32>,
    pub member_capacity: Option<u32>,
    pub emblem: Option<GuildEmblem>,
}

impl Endpoint for Guild {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for Guild {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildPermission {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Endpoint for GuildPermission {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild/permissions";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildPermission {
    type IdType = String;
}
impl BulkEndpoint for GuildPermission {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuildSearchResult(pub Vec<String>);

impl Endpoint for GuildSearchResult {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild/search";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for GuildSearchResult {}
