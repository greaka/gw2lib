use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingDetails {
    pub listings:   u64,
    pub unit_price: u64,
    pub quantity:   u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listings {
    pub id:    u64,
    pub buys:  Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity:   u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id:          u64,
    pub whitelisted: bool,
    pub buys:        PriceDetails,
    pub sells:       PriceDetails,
}
