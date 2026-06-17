use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, TimeStamp};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpTier {
    pub points: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpDivision {
    pub name: String,
    pub flags: Vec<String>,
    pub large_icon: String,
    pub small_icon: String,
    pub pip_icon: String,
    pub tiers: Vec<PvpTier>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpSeason {
    pub id: String,
    pub name: String,
    pub start: TimeStamp,
    pub end: TimeStamp,
    pub active: bool,
    pub divisions: Vec<PvpDivision>,
    pub leaderboards: Option<serde_json::Value>,
}

impl Endpoint for PvpSeason {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/pvp/seasons";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for PvpSeason {
    type IdType = String;
}
impl BulkEndpoint for PvpSeason {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
