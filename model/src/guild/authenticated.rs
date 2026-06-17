use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    authenticated::pvp::PvpResult, guild::upgrades::GuildUpgradeId, items::ItemId, Endpoint,
    EndpointWithId, TimeStamp,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildLogEntry {
    pub id: u32,
    pub time: TimeStamp,
    #[serde(rename = "type")]
    pub _type: String,
    pub user: Option<String>,
    pub action: Option<String>,
    pub item_id: Option<ItemId>,
    pub count: Option<u32>,
    pub upgrade_id: Option<GuildUpgradeId>,
    pub currency_id: Option<u32>,
    pub activity: Option<String>,
    pub total_participants: Option<u32>,
    pub participants: Option<Vec<String>>,
    pub quantity: Option<u32>,
    pub item_name: Option<String>,
    pub motd: Option<String>,
    pub kick_reason: Option<String>,
}

pub type GuildLog = Vec<GuildLogEntry>;

impl Endpoint for GuildLog {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildLog {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/log", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildMember {
    pub name: String,
    pub rank: String,
    pub joined: TimeStamp,
    pub wvw_member: bool,
}

pub type GuildMembers = Vec<GuildMember>;

impl Endpoint for GuildMembers {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildMembers {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/members", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildRank {
    pub id: String,
    pub order: u32,
    pub permissions: Vec<String>,
    pub icon: String,
}

pub type GuildRanks = Vec<GuildRank>;

impl Endpoint for GuildRanks {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildRanks {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/ranks", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildStashItem {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildStash {
    pub upgrade_id: GuildUpgradeId,
    pub size: u32,
    pub coins: u64,
    pub note: String,
    pub inventory: Vec<Option<GuildStashItem>>,
}

pub type GuildStashes = Vec<GuildStash>;

impl Endpoint for GuildStashes {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildStashes {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/stash", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildUpgradeNeed {
    pub upgrade_id: GuildUpgradeId,
    pub count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildTreasuryItem {
    pub item_id: ItemId,
    pub count: u32,
    pub needed_by: Vec<GuildUpgradeNeed>,
}

pub type GuildTreasury = Vec<GuildTreasuryItem>;

impl Endpoint for GuildTreasury {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildTreasury {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/treasury", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildTeamMember {
    pub name: String,
    pub role: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildTeam {
    pub id: u32,
    pub members: Vec<GuildTeamMember>,
    pub name: String,
    pub aggregate: PvpResult,
    pub ladders: HashMap<String, PvpResult>,
    pub games: Vec<serde_json::Value>,
    pub seasons: Vec<serde_json::Value>,
}

pub type GuildTeams = Vec<GuildTeam>;

impl Endpoint for GuildTeams {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildTeams {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/teams", id)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuildCompletedUpgrades(pub Vec<GuildUpgradeId>);

impl Endpoint for GuildCompletedUpgrades {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildCompletedUpgrades {
    type IdType = String;

    fn format_url(id: &str) -> String {
        format!("v2/guild/{}/upgrades", id)
    }
}
