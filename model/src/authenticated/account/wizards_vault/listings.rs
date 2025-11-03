use serde::{Deserialize, Serialize};

use crate::{
    authenticated::account::wizards_vault::AstralAcclaim, items::ItemId, Endpoint, FixedEndpoint,
};

pub type WizardsVaultListingId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WizardsVaultListingType {
    /// In the hero banner at the top
    Featured,
    /// Displayed in the rewards table
    Normal,
    /// In the Legacy Vault section
    Legacy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultListing {
    /// The listing id.
    id: WizardsVaultListingId,
    /// The id of the item.
    item_id: ItemId,
    /// The quantity of the item the user receives.
    item_count: u8,
    /// Appears to be the position in the wizards vault UI.
    #[serde(rename = "type")]
    _type: WizardsVaultListingType,
    /// The quantity of Astral Acclaim to purchase.
    cost: AstralAcclaim,
    // u8 felt a little too small given that I can see some items with a purchase
    // limit of 150 even in the wiki.
    /// Not included if the reward is unlimited (e.g. the unlimited Bag of Coins
    /// (1 Gold)).
    purchased: Option<u16>,
    /// Not included if the reward is unlimited (e.g. the unlimited Bag of Coins
    /// (1 Gold)).
    purchase_limit: Option<u16>,
}

pub type WizardsVaultListings = Vec<WizardsVaultListing>;

impl Endpoint for WizardsVaultListings {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wizardsvault/listings";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for WizardsVaultListings {}
