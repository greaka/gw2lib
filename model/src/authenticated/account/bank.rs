use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    items,
    items::{itemstats::StatsId, skins::SkinId, AttributeType, ItemId},
    misc::colors::ColorId,
    Endpoint, FixedEndpoint,
};

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct BankItemStats {
    pub id: StatsId,
    pub attributes: BTreeMap<AttributeType, u16>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct BankItem {
    pub id: ItemId,
    pub count: u32,
    pub charges: Option<u32>,
    pub skin: Option<SkinId>,
    pub dyes: Option<ColorId>,
    pub upgrades: Option<Vec<ItemId>>,
    pub upgrade_slot_indices: Option<Vec<u32>>,
    pub infusions: Option<Vec<ItemId>>,
    pub binding: Option<String>,
    pub bound_to: Option<String>,
    pub stats: Option<BankItemStats>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Bank(pub Vec<Option<BankItem>>);

impl Endpoint for Bank {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/materials";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Bank {}
