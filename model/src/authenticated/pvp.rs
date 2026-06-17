use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint, PagedEndpoint, TimeStamp};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpResult {
    pub wins: u32,
    pub losses: u32,
    pub desertions: u32,
    pub byes: u32,
    pub forfeits: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpStats {
    pub pvp_rank: u32,
    pub pvp_rank_points: u32,
    pub pvp_rank_rollovers: u32,
    pub aggregate: PvpResult,
    pub professions: HashMap<String, PvpResult>,
    pub ladders: HashMap<String, PvpResult>,
}

impl Endpoint for PvpStats {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/pvp/stats";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for PvpStats {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpGameScores {
    pub red: u32,
    pub blue: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpGame {
    pub id: String,
    pub map_id: u32,
    pub started: TimeStamp,
    pub ended: TimeStamp,
    pub result: String,
    pub team: String,
    pub profession: String,
    pub rating_type: String,
    pub rating_change: Option<i32>,
    pub season: Option<String>,
    pub scores: PvpGameScores,
}

impl Endpoint for PvpGame {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/pvp/games";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl PagedEndpoint for PvpGame {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpStandingResult {
    pub total_points: u32,
    pub division: u32,
    pub tier: u32,
    pub points: u32,
    pub repeats: u32,
    pub rating: Option<u32>,
    pub decay: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PvpStanding {
    pub current: PvpStandingResult,
    pub best: PvpStandingResult,
    pub season_id: String,
}

pub type PvpStandings = Vec<PvpStanding>;

impl Endpoint for PvpStandings {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/pvp/standings";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for PvpStandings {}
