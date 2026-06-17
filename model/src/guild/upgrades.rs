pub type GuildUpgradeId = u64;

use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildUpgradeCost {
    #[serde(rename = "type")]
    pub _type: String,
    pub count: u32,
    pub name: String,
    pub item_id: Option<ItemId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildUpgrade {
    pub id: GuildUpgradeId,
    pub name: String,
    pub description: String,
    pub build_time: u32,
    pub icon: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub required_level: u32,
    pub experience: u32,
    pub prerequisites: Vec<GuildUpgradeId>,
    pub costs: Vec<GuildUpgradeCost>,
    pub bag_max_items: Option<u32>,
    pub bag_max_coins: Option<u64>,
}

impl Endpoint for GuildUpgrade {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/guild/upgrades";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for GuildUpgrade {
    type IdType = GuildUpgradeId;
}
impl BulkEndpoint for GuildUpgrade {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
