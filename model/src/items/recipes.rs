pub type RecipeId = u32;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

pub use crate::authenticated::characters::Discipline;
use crate::{
    guild::upgrades::GuildUpgradeId, items::ItemId, misc::currencies::CurrencyId, BulkEndpoint,
    Endpoint, EndpointWithId,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum RecipeType {
    Axe,
    Dagger,
    Focus,
    Greatsword,
    Hammer,
    Harpoon,
    LongBow,
    Mace,
    Pistol,
    Rifle,
    Scepter,
    Shield,
    ShortBow,
    Speargun,
    Staff,
    Sword,
    Torch,
    Trident,
    Warhorn,
    Boots,
    Coat,
    Gloves,
    Helm,
    Leggings,
    Shoulders,
    Amulet,
    Earring,
    Ring,
    Dessert,
    Feast,
    IngredientCooking,
    Meal,
    Seasoning,
    Snack,
    Soup,
    Food,
    Component,
    Inscription,
    Insignia,
    LegendaryComponent,
    Refinement,
    RefinementEctoplasm,
    RefinementObsidian,
    GuildConsumable,
    GuildDecoration,
    GuildConsumableWvw,
    Backpack,
    Bag,
    Bulk,
    Consumable,
    Dye,
    Potion,
    UpgradeComponent,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum RecipeFlag {
    AutoLearned,
    LearnedFromItem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Ingredient {
    Currency { id: CurrencyId, count: u16 },
    Item { id: ItemId, count: u16 },
    GuildUpgrade { id: GuildUpgradeId, count: u16 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Recipe {
    pub id: RecipeId,
    #[serde(rename = "type")]
    pub _type: RecipeType,
    pub output_item_id: ItemId,
    pub output_item_count: u16,
    pub time_to_craft_ms: u16,
    pub disciplines: BTreeSet<Discipline>,
    pub min_rating: u16,
    pub flags: BTreeSet<RecipeFlag>,
    pub ingredients: Vec<Ingredient>,
    pub output_upgrade_id: Option<u32>,
    pub chat_link: String,
}

impl EndpointWithId for Recipe {
    type IdType = RecipeId;
}
impl Endpoint for Recipe {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/recipes";
    const VERSION: &'static str = "2023-03-20T13:00:00.000Z";
}

impl BulkEndpoint for Recipe {
    const ALL: bool = false;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
