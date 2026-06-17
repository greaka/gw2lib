use serde::{Deserialize, Serialize};

use crate::{
    authenticated::characters::{Binding, Stats},
    items::{skins::SkinId, ItemId},
    misc::colors::ColorId,
    Endpoint, FixedEndpoint,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SharedInventorySlot {
    pub id: ItemId,
    pub count: u32,
    pub charges: Option<u32>,
    pub skin: Option<SkinId>,
    pub upgrades: Option<Vec<ItemId>>,
    pub infusions: Option<Vec<ItemId>>,
    pub dyes: Option<Vec<Option<ColorId>>>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
}

pub type AccountInventory = Vec<Option<SharedInventorySlot>>;

impl Endpoint for AccountInventory {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/inventory";
    const VERSION: &'static str = "2023-07-01T00:00:00.000Z";
}

impl FixedEndpoint for AccountInventory {}
