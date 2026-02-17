use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type GuildEmblemBackgroundId = u32;
pub type GuildEmblemForegroundId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblemBackground {
    pub id: GuildEmblemBackgroundId,
    pub layers: Vec<String>,
}

impl Endpoint for GuildEmblemBackground {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/emblem/backgrounds";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
}

impl EndpointWithId for GuildEmblemBackground {
    type IdType = GuildEmblemBackgroundId;
}

impl BulkEndpoint for GuildEmblemBackground {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildEmblemForeground {
    pub id: GuildEmblemForegroundId,
    pub layers: Vec<String>,
}

impl Endpoint for GuildEmblemForeground {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/emblem/foregrounds";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
}

impl EndpointWithId for GuildEmblemForeground {
    type IdType = GuildEmblemForegroundId;
}

impl BulkEndpoint for GuildEmblemForeground {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
