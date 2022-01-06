pub type RecipeId = u32;
pub use crate::authenticated::characters::Discipline;
use crate::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RecipeFlag {
    AutoLearned,
    LearnedFromItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub item_id: ItemId,
    pub count:   u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildIngredient {
    pub upgrade_id: u32,
    pub count:      u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    id:                RecipeId,
    #[serde(rename = "type")]
    _type:             RecipeType,
    output_item_id:    ItemId,
    output_item_count: u16,
    time_to_craft_ms:  u16,
    disciplines:       Vec<Discipline>,
    min_rating:        u16,
    flags:             Vec<RecipeFlag>,
    ingredients:       Vec<Ingredient>,
    guild_ingredients: Option<Vec<GuildIngredient>>,
    output_upgrade_id: Option<u32>,
    chat_link:         String,
}
