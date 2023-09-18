use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuySellInfo {
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CommercePrice {
    pub id: ItemId,
    pub whitelisted: bool,
    pub buys: BuySellInfo,
    pub sells: BuySellInfo,
}

impl EndpointWithId for CommercePrice {
    type IdType = ItemId;
}

impl Endpoint for CommercePrice {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/prices";
    const VERSION: &'static str = "2023-07-01T00:00:00.000Z";
}

impl FixedEndpoint for CommercePrice {}

impl BulkEndpoint for CommercePrice {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
