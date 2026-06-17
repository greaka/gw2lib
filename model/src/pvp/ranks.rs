use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpRankLevel {
    pub min_rank: u32,
    pub max_rank: u32,
    pub points: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpRank {
    pub id: u32,
    pub finisher_id: u32,
    pub name: String,
    pub icon: String,
    pub min_rank: u32,
    pub max_rank: u32,
    pub levels: Vec<PvpRankLevel>,
}

impl Endpoint for PvpRank {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/pvp/ranks";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for PvpRank {
    type IdType = u32;
}
impl BulkEndpoint for PvpRank {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
