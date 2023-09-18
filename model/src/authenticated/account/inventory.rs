use serde::{Deserialize, Serialize};

use crate::{
    authenticated::characters::Binding,
    items::{skins::SkinId, ItemId},
    Endpoint, FixedEndpoint,
};

pub type AccountInventory = Vec<Option<AccountInventoryItem>>;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Eq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct AccountInventoryItem {
    pub id: ItemId,
    pub count: u8,
    pub charges: Option<u8>,
    pub upgrades: Option<Vec<ItemId>>,
    pub skin: Option<SkinId>,
    pub infusions: Option<Vec<ItemId>>,
    pub binding: Option<Binding>,
}

impl Endpoint for AccountInventory {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/inventory";
    const VERSION: &'static str = "2023-07-01T00:00:00.000Z";
}

impl FixedEndpoint for AccountInventory {}
