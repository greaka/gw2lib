use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, TimeStamp};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwTeamScore {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwTeamVecScore {
    pub red: Vec<u32>,
    pub blue: Vec<u32>,
    pub green: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwMatch {
    pub id: String,
    pub start_time: TimeStamp,
    pub end_time: TimeStamp,
    pub scores: WvwTeamScore,
    pub worlds: WvwTeamScore,
    pub all_worlds: WvwTeamVecScore,
    pub deaths: WvwTeamScore,
    pub kills: WvwTeamScore,
    pub victory_points: WvwTeamScore,
    pub skirmishes: Vec<serde_json::Value>,
    pub maps: Vec<serde_json::Value>,
}

impl Endpoint for WvwMatch {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/wvw/matches";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for WvwMatch {
    type IdType = String;
}
impl BulkEndpoint for WvwMatch {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
