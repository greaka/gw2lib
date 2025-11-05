use serde::{Deserialize, Serialize};

use crate::{
    authenticated::account::wizards_vault::AstralAcclaim, items::ItemId, BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint
};

pub type WizardsVaultListingId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WizardsVaultListingType {
    Featured,
    Normal,
    Legacy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultListing {
    id: WizardsVaultListingId,
    item_id: ItemId,
    item_count: u8,
    #[serde(rename = "type")]
    _type: WizardsVaultListingType,
    cost: AstralAcclaim,
    purchased: Option<u16>,
    purchase_limit: Option<u16>,
}

impl Endpoint for WizardsVaultListing {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wizardsvault/listings";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl EndpointWithId for WizardsVaultListing {
    type IdType = WizardsVaultListingId;
}

impl FixedEndpoint for WizardsVaultListing {}

impl BulkEndpoint for WizardsVaultListing {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
