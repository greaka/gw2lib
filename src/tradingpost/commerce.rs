use crate::utils::*;
use rest_client::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingDetails {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

#[rest(
    "https://api.guildwars2.com/v2/commerce/listings/{}?v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[rest(
    "https://api.guildwars2.com/v2/commerce/listings?ids={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult",
    vec
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Listings {
    pub id: u64,
    pub buys: Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

/// ```
/// use gw2api::tradingpost::*;
///
/// get_listings(19723).unwrap();
/// ```
pub fn get_listings(
    item_id: impl Display,
) -> Result<ApiResult<Box<Listings>>, Box<dyn std::error::Error>> {
    Listings::get(vec![item_id])
}

/// ```
/// use gw2api::tradingpost::*;
///
/// get_multiple_listings(vec![19723, 30699]).unwrap();
/// ```
pub fn get_multiple_listings(
    item_ids: impl IntoIterator<Item = impl Display>,
) -> Result<ApiResult<Vec<Box<Listings>>>, Box<dyn std::error::Error>> {
    let item_ids = format_ids(item_ids);
    Listings::get(vec![item_ids])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity: u64,
}

#[rest(
    "https://api.guildwars2.com/v2/commerce/prices/{}?v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[rest(
    "https://api.guildwars2.com/v2/commerce/prices?ids={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult",
    vec
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id: u64,
    pub whitelisted: bool,
    pub buys: PriceDetails,
    pub sells: PriceDetails,
}

/// ```
/// use gw2api::tradingpost::*;
///
/// get_prices(19723).unwrap();
/// ```
pub fn get_prices(
    item_id: impl Display,
) -> Result<ApiResult<Box<Prices>>, Box<dyn std::error::Error>> {
    Prices::get(vec![item_id])
}

/// ```
/// use gw2api::tradingpost::*;
///
/// get_multiple_prices(vec![19723, 30699]).unwrap();
/// ```
pub fn get_multiple_prices(
    item_ids: impl IntoIterator<Item = impl Display>,
) -> Result<ApiResult<Vec<Box<Prices>>>, Box<dyn std::error::Error>> {
    let item_ids = format_ids(item_ids);
    Prices::get(vec![item_ids])
}

/// ```
/// use gw2api::tradingpost::*;
///
/// get_all_items().unwrap();
/// ```
pub fn get_all_items() -> Result<ApiResult<Vec<u64>>, Box<dyn std::error::Error>> {
    let new_self = reqwest::get("https://api.guildwars2.com/v2/commerce/prices")?.json()?;
    Ok(new_self)
}
