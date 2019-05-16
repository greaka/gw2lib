use rest_client::*;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub struct ListingDetails {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

#[rest("https://api.guildwars2.com/v2/commerce/listings/{}")]
#[rest("https://api.guildwars2.com/v2/commerce/listings?ids={}", vec)]
#[derive(Deserialize)]
pub struct Listings {
    pub id: u64,
    pub buys: Vec<ListingDetails>,
    pub sells: Vec<ListingDetails>,
}

pub fn get_listings(item_id: impl Display) -> Result<Box<Listings>, Box<std::error::Error>> {
    Listings::get(vec![item_id])
}

pub fn get_multiple_listings(
    item_ids: impl IntoIterator<Item = impl Display>,
) -> Result<Vec<Box<Listings>>, Box<std::error::Error>> {
    Listings::gets(item_ids)
}

#[derive(Deserialize)]
pub struct PriceDetails {
    pub unit_price: u64,
    pub quantity: u64,
}

#[rest("https://api.guildwars2.com/v2/commerce/prices/{}")]
#[rest("https://api.guildwars2.com/v2/commerce/prices?ids={}", vec)]
#[derive(Deserialize)]
pub struct Prices {
    pub id: u64,
    pub whitelisted: bool,
    pub buys: PriceDetails,
    pub sells: PriceDetails,
}

pub fn get_prices(item_id: impl Display) -> Result<Box<Prices>, Box<std::error::Error>> {
    Prices::get(vec![item_id])
}

pub fn get_multiple_prices(
    item_ids: impl IntoIterator<Item = impl Display>,
) -> Result<Vec<Box<Prices>>, Box<std::error::Error>> {
    Prices::gets(item_ids)
}

pub fn get_all_items() -> Result<Vec<u64>, Box<std::error::Error>> {
	let new_self = reqwest::get("https://api.guildwars2.com/v2/commerce/prices")?
		.json()?;
	Ok(new_self)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_listings() {
        get_listings(19723).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_multiple_listings() {
        get_multiple_listings(vec![19723, 30699]).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_prices() {
        get_prices(19723).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_multiple_prices() {
        get_multiple_prices(vec![19723, 30699]).unwrap();
    }
}