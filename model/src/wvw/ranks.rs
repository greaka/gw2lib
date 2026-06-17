use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwRank {
    pub id: u32,
    pub title: String,
    pub min_rank: u32,
}

impl Endpoint for WvwRank {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/wvw/ranks";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for WvwRank {
    type IdType = u32;
}
impl BulkEndpoint for WvwRank {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
