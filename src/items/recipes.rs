pub use crate::authenticated::characters::Discipline;
use crate::utils::*;
use rest_client::*;
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
    pub item_id: u32,
    pub count: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildIngredient {
    pub upgrade_id: u32,
    pub count: u16,
}

#[rest(
    "https://api.guildwars2.com/v2/recipes/{}?lang={}&v=2019-09-25T00:00:00Z",
    wrapper = "ApiResult"
)]
#[rest(
    "https://api.guildwars2.com/v2/recipes?ids={}&lang={}&v=2019-09-25T00:00:00Z",
    vec,
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    id: u64,
    #[serde(rename = "type")]
    _type: RecipeType,
    output_item_id: u32,
    output_item_count: u16,
    time_to_craft_ms: u16,
    disciplines: Vec<Discipline>,
    min_rating: u16,
    flags: Vec<RecipeFlag>,
    ingredients: Vec<Ingredient>,
    guild_ingredients: Option<Vec<GuildIngredient>>,
    output_upgrade_id: Option<u16>,
    chat_link: String,
}

/// ```
/// use gw2api::items::recipes::*;
/// use gw2api::utils::*;
///
/// get_recipe(13269, Language::En).unwrap();
/// ```
pub fn get_recipe(
    item_id: impl std::fmt::Display,
    lang: Language,
) -> Result<ApiResult<Box<Recipe>>, Box<dyn std::error::Error>> {
    Recipe::get(&[item_id.to_string(), lang.to_string()])
}

/// ```
/// use gw2api::items::recipes::*;
/// use gw2api::utils::*;
///
/// get_recipes(
///     vec![
///          7823, 7824, 7825, 7826, 7827, 7828, 7829, 7830, 7831, 7832, 7833, 7834, 7835, 7836, 7837, 7838, 7839, 7840, 7841, 7842, 7843, 7844, 7845, 7846, 7847, 7848, 7849, 7850, 7851, 7852, 7853, 7854, 7855, 7856,
///     ],
///     Language::En,
/// )
/// .unwrap();
/// ```
pub fn get_recipes(
    item_ids: impl IntoIterator<Item = impl std::fmt::Display>,
    lang: Language,
) -> Result<ApiResult<Vec<Box<Recipe>>>, Box<dyn std::error::Error>> {
    let item_ids = format_ids(item_ids);
    Recipe::get(&[item_ids, lang.to_string()])
}

/// ```
/// use gw2api::items::recipes::*;
///
/// get_all_recipes().unwrap();
/// ```
pub fn get_all_recipes() -> Result<ApiResult<Vec<u64>>, Box<dyn std::error::Error>> {
    let new_self = reqwest::get("https://api.guildwars2.com/v2/items")?.json()?;
    Ok(new_self)
}
