use serde::{Deserialize, Serialize};

use crate::{guild::GuildId, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuildSearch(pub Vec<GuildId>);

impl Endpoint for GuildSearch {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild/search";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
}

impl EndpointWithId for GuildSearch {
    type IdType = String;

    fn format_url(_id: &str) -> String {
        Self::URL.to_string()
    }

    fn extra_queries(id: &str) -> Option<String> {
        Some(format!("name={}", id))
    }
}
