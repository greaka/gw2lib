use crate::utils::*;
use rest_client::*;
use serde::{Deserialize, Serialize};

#[rest(
    "https://api.guildwars2.com/v2/currencies/{}?lang={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub order: u8,
}

/// ```
/// use gw2api::misc::*;
/// use gw2api::utils::*;
///
/// get_currency(1, Language::En).unwrap();
/// ```
pub fn get_currency(
    id: u64,
    lang: Language,
) -> Result<ApiResult<Box<Currency>>, Box<dyn std::error::Error>> {
    Currency::get(vec![id.to_string(), lang.to_string()])
}
