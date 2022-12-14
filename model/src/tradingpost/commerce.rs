use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct ListingDetails {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Listings {
    pub id: ItemId,
    pub buys: Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

impl EndpointWithId for Listings {
    type IdType = ItemId;
}
impl Endpoint for Listings {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/listings";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Listings {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Prices {
    pub id: ItemId,
    pub whitelisted: bool,
    pub buys: PriceDetails,
    pub sells: PriceDetails,
}

impl EndpointWithId for Prices {
    type IdType = ItemId;
}
impl Endpoint for Prices {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/prices";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Prices {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
