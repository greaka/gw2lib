use crate::utils::*;
use rest_client::*;
use serde::{Deserialize, Serialize};

#[rest(
    "https://api.guildwars2.com/v2/build?v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Build {
    pub id: u64,
}

/// ```
/// use gw2api::misc::*;
///
/// get_build().unwrap();
/// ```
pub fn get_build() -> Result<ApiResult<Box<Build>>, Box<dyn std::error::Error>> {
    Build::get(Vec::<bool>::new())
}

pub type RGB = (u8, u8, u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDetails {
    pub brightness: u8,
    pub contrast: f32,
    pub hue: u8,
    pub saturation: f32,
    pub lightness: f32,
    pub rgb: RGB,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Hue {
    Gray,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Material {
    Vibrant,
    Leather,
    Metal,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Exclusive,
}

#[rest(
    "https://api.guildwars2.com/v2/colors/{}?lang={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub id: u64,
    pub name: String,
    pub base_rgb: RGB,
    pub cloth: MaterialDetails,
    pub leather: MaterialDetails,
    pub metal: MaterialDetails,
    pub fur: Option<MaterialDetails>,
    pub item: u64,
    pub categories: (Hue, Material, Rarity),
}

/// ```
/// use gw2api::misc::*;
/// use gw2api::utils::*;
///
/// get_color(10, Language::En).unwrap();
/// ```
pub fn get_color(
    id: u64,
    lang: Language,
) -> Result<ApiResult<Box<Color>>, Box<dyn std::error::Error>> {
    Color::get(vec![id.to_string(), lang.to_string()])
}

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
