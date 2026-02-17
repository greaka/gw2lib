use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

pub type GuildUpgradeId = u64;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GuildUpgradeType {
    AccumulatingCurrency,
    BankBag,
    Boost,
    Claimable,
    Consumable,
    Decoration,
    GuildHall,
    GuildHallExpedition,
    Hub,
    Queue,
    Unlock,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GuildUpgradeCostType {
    Item,
    Collectible,
    Currency,
    Coins,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildUpgradeCost {
    #[serde(rename = "type")]
    pub cost_type: GuildUpgradeCostType,
    pub name: Option<String>,
    pub count: u32,
    pub item_id: Option<ItemId>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct GuildUpgrade {
    pub id: GuildUpgradeId,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub upgrade_type: GuildUpgradeType,
    pub icon: String,
    pub build_time: u32,
    pub required_level: u32,
    pub experience: u32,
    pub prerequisites: Vec<GuildUpgradeId>,
    pub bag_max_items: Option<u32>,
    pub bag_max_coins: Option<u32>,
    #[serde(default)]
    pub cost: Vec<GuildUpgradeCost>,
}

impl Endpoint for GuildUpgrade {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/guild/upgrades";
    const VERSION: &'static str = "2026-02-17T00:00:00.000Z";
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
