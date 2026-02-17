use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

pub type GuildPermissionId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildPermission {
    pub id: GuildPermissionId,
    pub name: String,
    pub description: String,
}

impl Endpoint for GuildPermission {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/guild/permissions";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
}

impl EndpointWithId for GuildPermission {
    type IdType = GuildPermissionId;
}

impl BulkEndpoint for GuildPermission {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
