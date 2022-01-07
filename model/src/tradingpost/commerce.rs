use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingDetails {
    pub listings:   u64,
    pub unit_price: u64,
    pub quantity:   u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listings {
    pub id:    ItemId,
    pub buys:  Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

impl Endpoint for Listings {
    fn url() -> &'static str {
        "v2/commerce/listings"
    }
}

impl BulkEndpoint for Listings {
    type IdType = ItemId;

    const ALL: bool = false;
    const PAGING: bool = true;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity:   u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id:          ItemId,
    pub whitelisted: bool,
    pub buys:        PriceDetails,
    pub sells:       PriceDetails,
}

impl Endpoint for Prices {
    fn url() -> &'static str {
        "v2/commerce/prices"
    }
}

impl BulkEndpoint for Prices {
    type IdType = ItemId;

    const ALL: bool = false;
    const PAGING: bool = true;
}
