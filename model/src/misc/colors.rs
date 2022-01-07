use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint};

pub type RGB = (u8, u8, u8);
pub type ColorId = u16;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialDetails {
    pub brightness: u8,
    pub contrast:   f32,
    pub hue:        u8,
    pub saturation: f32,
    pub lightness:  f32,
    pub rgb:        RGB,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub id:         ColorId,
    pub name:       String,
    pub base_rgb:   RGB,
    pub cloth:      MaterialDetails,
    pub leather:    MaterialDetails,
    pub metal:      MaterialDetails,
    pub fur:        Option<MaterialDetails>,
    pub item:       u64,
    pub categories: (Hue, Material, Rarity),
}

impl Endpoint for Color {
    fn url() -> &'static str {
        "v2/colors"
    }
}

impl BulkEndpoint for Color {
    type IdType = ColorId;

    const ALL: bool = true;
    const PAGING: bool = true;
}
