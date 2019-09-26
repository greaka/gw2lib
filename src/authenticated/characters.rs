use crate::utils::*;
use crate::misc::titles::TitleId;
use crate::items::itemstats::StatsId;

use rest_client::*;
use serde::{Deserialize, Serialize};
use crate::items::ItemId;
use crate::items::skins::SkinId;
use crate::misc::colors::ColorId;
use crate::game_mechanics::skills::SkillId;
use crate::game_mechanics::traits::TraitId;
use crate::game_mechanics::specializations::SpecializationId;
use crate::items::recipes::RecipeId;
use crate::wvw::abilities::AbilityId;
use crate::pvp::amulets::AmuletId;

pub type Age = u64;

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/backstory?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Backstory {
    pub backstory: Vec<String>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_backstory(
/// "Eff Testing Ele",
/// "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_backstory(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Backstory>>, Box<dyn std::error::Error>> {
    Backstory::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Race {
    Asura,
    Charr,
    Human,
    Norn,
    Sylvari,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Profession {
    Elementalist,
    Engineer,
    Guardian,
    Mesmer,
    Necromancer,
    Ranger,
    Revenant,
    Thief,
    Warrior,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/core?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Core {
    pub name: String,
    pub race: Race,
    pub gender: Gender,
    pub profession: Profession,
    pub level: u8,
    pub guild: Option<String>,
    pub age: Age,
    pub created: TimeStamp,
    pub deaths: u32,
    pub title: Option<TitleId>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_core(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_core(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Core>>, Box<dyn std::error::Error>> {
    Core::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Discipline {
    Armorsmith,
    Artificer,
    Chef,
    Huntsman,
    Jeweler,
    Leatherworker,
    Scribe,
    Tailor,
    Weaponsmith,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Craft {
    pub discipline: Discipline,
    pub rating: u16,
    pub active: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/crafting?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Crafting {
    pub crafting: Vec<Craft>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_crafting(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_crafting(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Crafting>>, Box<dyn std::error::Error>> {
    Crafting::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Slot {
    HelmAquatic,
    Backpack,
    Coat,
    Boots,
    Gloves,
    Helm,
    Leggings,
    Shoulders,
    Accessory1,
    Accessory2,
    Ring1,
    Ring2,
    Amulet,
    WeaponAquaticA,
    WeaponAquaticB,
    WeaponA1,
    WeaponA2,
    WeaponB1,
    WeaponB2,
    Sickle,
    Axe,
    Pick,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attributes {
    pub power: Option<u16>,
    pub precision: Option<u16>,
    pub toughness: Option<u16>,
    pub vitality: Option<u16>,
    pub condition_damage: Option<u16>,
    pub condition_duration: Option<u16>,
    pub healing: Option<u16>,
    pub boon_duration: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub id: StatsId,
    pub attributes: Attributes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Binding {
    Character,
    Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equip {
    pub id: ItemId,
    pub slot: Slot,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades: Option<Vec<ItemId>>,
    pub skin: Option<SkinId>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub charges: Option<u16>,
    pub bound_to: Option<String>,
    pub dyes: Option<Vec<Option<ColorId>>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/equipment?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub equipment: Vec<Equip>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_equipment(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_equipment(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Equipment>>, Box<dyn std::error::Error>> {
    Equipment::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: ItemId,
    pub count: u8,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades: Option<Vec<ItemId>>,
    pub skin: Option<SkinId>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub bound_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryBag {
    pub id: ItemId,
    pub size: u8,
    pub inventory: Vec<Option<InventoryItem>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/inventory?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub bags: Vec<Option<InventoryBag>>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_inventory(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_inventory(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Inventory>>, Box<dyn std::error::Error>> {
    Inventory::get(vec![character_name, api_key])
}

pub type Utilities = (Option<SkillId>, Option<SkillId>, Option<SkillId>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Skillset {
    pub heal: Option<SkillId>,
    pub utilities: Utilities,
    pub elite: Option<SkillId>,
    // TODO: legends enum
    pub legends: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDataSet {
    pub pve: Skillset,
    pub pvp: Skillset,
    pub wvw: Skillset,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/skills?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    pub skills: SkillDataSet,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_skills(
/// "Eff Testing Ele",
/// "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_skills(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Skills>>, Box<dyn std::error::Error>> {
    Skills::get(vec![character_name, api_key])
}

pub type TraitSet = (Option<TraitId>, Option<TraitId>, Option<TraitId>);

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitLine {
    pub id: SpecializationId,
    pub traits: TraitSet,
}

pub type Specialization = (Option<TraitLine>, Option<TraitLine>, Option<TraitLine>);

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecializationSet {
    pub pve: Specialization,
    pub pvp: Specialization,
    pub wvw: Specialization,
}

#[rest("https://api.guildwars2.com/v2/characters/{}/specializations?access_token={}&v=2019-04-22T00:00:00Z", wrapper = "ApiResult")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Specializations {
    pub specializations: SpecializationSet,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_specializations(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_specializations(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Specializations>>, Box<dyn std::error::Error>> {
    Specializations::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingSet {
    // TODO: training id
    pub id: u64,
    pub spent: u16,
    pub done: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/training?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    pub training: Vec<TrainingSet>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_training(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_training(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Training>>, Box<dyn std::error::Error>> {
    Training::get(vec![character_name, api_key])
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/recipes?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipes {
    pub recipes: Vec<RecipeId>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_recipes(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_recipes(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Recipes>>, Box<dyn std::error::Error>> {
    Recipes::get(vec![character_name, api_key])
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WvwAbility {
    pub id: AbilityId,
    pub rank: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentPvp {
    pub amulet: Option<AmuletId>,
    pub rune: Option<ItemId>,
    pub sigils: (Option<ItemId>, Option<ItemId>, Option<ItemId>, Option<ItemId>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Flags {
    Beta,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}?access_token={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[rest(
    "https://api.guildwars2.com/v2/characters?access_token={}&v=2019-04-22T00:00:00Z&page=0",
    vec,
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    #[serde(flatten)]
    pub backstory: Backstory,
    #[serde(flatten)]
    pub core: Core,
    #[serde(flatten)]
    pub crafting: Crafting,
    #[serde(flatten)]
    pub equipment: Equipment,
    #[serde(flatten)]
    pub inventory: Inventory,
    #[serde(flatten)]
    pub recipes: Recipes,
    #[serde(flatten)]
    pub skills: Skills,
    #[serde(flatten)]
    pub specializations: Specializations,
    #[serde(flatten)]
    pub training: Training,

    pub wvw_abilities: Vec<WvwAbility>,
    pub equipment_pvp: EquipmentPvp,
    pub flags: Vec<Flags>,
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_character(
///     "Eff Testing Ele",
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_character(
    character_name: &str,
    api_key: &str,
) -> Result<ApiResult<Box<Character>>, Box<dyn std::error::Error>> {
    Character::get(vec![character_name, api_key])
}

/// ```
/// use gw2api::authenticated::characters::*;
///
/// get_all_characters(
///     "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
/// )
/// .unwrap();
/// ```
pub fn get_all_characters(
    api_key: &str,
) -> Result<ApiResult<Vec<Box<Character>>>, Box<dyn std::error::Error>> {
    Character::get(vec![api_key])
}
