use crate::utils::*;
use rest_client::*;
use serde::Deserialize;

#[rest("https://api.guildwars2.com/v2/build?v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Build {
    id: u64,
}

pub fn get_build() -> Result<Box<Build>, Box<std::error::Error>> {
    Build::get(Vec::<bool>::new())
}

pub type RGB = (u8, u8, u8);

#[derive(Deserialize)]
pub struct MaterialDetails {
    brightness: u8,
    contrast: f32,
    hue: u8,
    saturation: f32,
    lightness: f32,
    rgb: RGB,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum Material {
    Vibrant,
    Leather,
    Metal,
}

#[derive(Deserialize)]
pub enum Rarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Exclusive,
}

#[rest("https://api.guildwars2.com/v2/colors/{}?lang={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Color {
    id: u64,
    name: String,
    base_rgb: RGB,
    cloth: MaterialDetails,
    leather: MaterialDetails,
    metal: MaterialDetails,
    fur: Option<MaterialDetails>,
    #[serde(rename = "item")]
    item_id: u64,
    categories: (Hue, Material, Rarity),
}

pub fn get_color(id: u64, lang: Language) -> Result<Box<Color>, Box<std::error::Error>> {
    Color::get(vec![id.to_string(), lang.to_string()])
}

#[rest("https://api.guildwars2.com/v2/currencies/{}?lang={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Currency {
    id: u64,
    name: String,
    description: String,
    icon: String,
    order: u8,
}

pub fn get_currency(id: u64, lang: Language) -> Result<Box<Currency>, Box<std::error::Error>> {
    Currency::get(vec![id.to_string(), lang.to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_build() {
        get_build().unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_color() {
        get_color(10, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_currency() {
        get_currency(1, Language::En).unwrap();
    }
}