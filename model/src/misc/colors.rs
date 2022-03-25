use either::Either;
use serde::{Deserialize, Serialize};

use crate::{items::ItemId, BulkEndpoint, Endpoint};

pub type RGB = (u8, u8, u8);
pub type ColorId = u16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialDetails {
    pub brightness: i8,
    pub contrast: f32,
    pub hue: u16,
    pub saturation: f32,
    pub lightness: f32,
    pub rgb: RGB,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Material {
    Vibrant,
    Leather,
    Metal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Rarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Exclusive,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub id: ColorId,
    pub name: String,
    pub base_rgb: RGB,
    pub cloth: MaterialDetails,
    pub leather: MaterialDetails,
    pub metal: MaterialDetails,
    pub fur: Option<MaterialDetails>,
    /// is only None for Dye Remover
    pub item: Option<ItemId>,
    /// is only `Right` for Dye Remover
    #[serde(with = "either::serde_untagged")]
    pub categories: Either<(Hue, Material, Rarity), [(); 0]>,
}

impl_id!(Color, ColorId);
impl Endpoint for Color {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/colors";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}

impl BulkEndpoint for Color {
    const ALL: bool = true;
    const PAGING: bool = true;
}
