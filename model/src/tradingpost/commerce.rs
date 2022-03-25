use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListingDetails {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listings {
    pub id: ItemId,
    pub buys: Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

impl_id!(Listings, ItemId);
impl Endpoint for Listings {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/listings";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Listings {
    const ALL: bool = false;
    const PAGING: bool = true;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id: ItemId,
    pub whitelisted: bool,
    pub buys: PriceDetails,
    pub sells: PriceDetails,
}

impl_id!(Prices, ItemId);
impl Endpoint for Prices {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/commerce/prices";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Prices {
    const ALL: bool = false;
    const PAGING: bool = true;
}
