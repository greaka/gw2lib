use crate::utils;

use rest_client::*;
use serde::Deserialize;

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/backstory?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Backstory {
    pub backstory: Vec<String>,
}

pub fn get_backstory(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Backstory>, Box<std::error::Error>> {
    Backstory::get(vec![character_name, api_key])
}

#[derive(Deserialize, PartialEq)]
pub enum Race {
    Asura,
    Charr,
    Human,
    Norn,
    Sylvari,
}

#[derive(Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Deserialize, PartialEq)]
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

#[rest("https://api.guildwars2.com/v2/characters/{}/core?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Core {
    pub name: String,
    pub race: Race,
    pub gender: Gender,
    pub profession: Profession,
    pub level: u8,
    pub guild: Option<String>,
    pub age: u64,
    pub created: utils::TimeStamp,
    pub deaths: u64,
    pub title: Option<u64>,
}

pub fn get_core(character_name: &str, api_key: &str) -> Result<Box<Core>, Box<std::error::Error>> {
    Core::get(vec![character_name, api_key])
}

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize)]
pub struct Craft {
    pub discipline: Discipline,
    pub rating: u16,
    pub active: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/crafting?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Crafting {
    pub crafting: Vec<Craft>,
}

pub fn get_crafting(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Crafting>, Box<std::error::Error>> {
    Crafting::get(vec![character_name, api_key])
}

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Stats {
    pub id: u64,
    pub attributes: Attributes,
}

#[derive(Deserialize, PartialEq)]
pub enum Binding {
    Character,
    Account,
}

#[derive(Deserialize)]
pub struct Equip {
    pub id: u64,
    pub slot: Slot,
    pub infusions: Option<Vec<u64>>,
    pub upgrades: Option<Vec<u64>>,
    pub skin: Option<u64>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub charges: Option<u16>,
    pub bound_to: Option<String>,
    pub dyes: Option<Vec<Option<u64>>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/equipment?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Equipment {
    pub equipment: Vec<Equip>,
}

pub fn get_equipment(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Equipment>, Box<std::error::Error>> {
    Equipment::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct InventoryItem {
    pub id: u64,
    pub count: u8,
    pub infusions: Option<Vec<u64>>,
    pub upgrades: Option<Vec<u64>>,
    pub skin: Option<u64>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub bound_to: Option<String>,
}

#[derive(Deserialize)]
pub struct InventoryBag {
    pub id: u64,
    pub size: u8,
    pub inventory: Vec<Option<InventoryItem>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/inventory?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Inventory {
    pub bags: Vec<Option<InventoryBag>>,
}

pub fn get_inventory(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Inventory>, Box<std::error::Error>> {
    Inventory::get(vec![character_name, api_key])
}

pub type Utilities = (Option<u64>, Option<u64>, Option<u64>);

#[derive(Deserialize)]
pub struct Skillset {
    pub heal: Option<u64>,
    pub utilities: Utilities,
    pub elite: Option<u64>,
    pub legends: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct SkillDataSet {
    pub pve: Skillset,
    pub pvp: Skillset,
    pub wvw: Skillset,
}

#[rest("https://api.guildwars2.com/v2/characters/{}/skills?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Skills {
    pub skills: SkillDataSet,
}

pub fn get_skills(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Skills>, Box<std::error::Error>> {
    Skills::get(vec![character_name, api_key])
}

pub type TraitSet = (Option<u64>, Option<u64>, Option<u64>);

#[derive(Deserialize)]
pub struct TraitLine {
    pub id: u64,
    pub traits: TraitSet,
}

pub type Specialization = (Option<TraitLine>, Option<TraitLine>, Option<TraitLine>);

#[derive(Deserialize)]
pub struct SpecializationSet {
    pub pve: Specialization,
    pub pvp: Specialization,
    pub wvw: Specialization,
}

#[rest("https://api.guildwars2.com/v2/characters/{}/specializations?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Specializations {
    pub specializations: SpecializationSet,
}

pub fn get_specializations(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Specializations>, Box<std::error::Error>> {
    Specializations::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct TrainingSet {
    pub id: u64,
    pub spent: u16,
    pub done: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/training?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Training {
    pub training: Vec<TrainingSet>,
}

pub fn get_training(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Training>, Box<std::error::Error>> {
    Training::get(vec![character_name, api_key])
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/recipes?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Recipes {
    pub recipes: Vec<u64>,
}

pub fn get_recipes(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Recipes>, Box<std::error::Error>> {
    Recipes::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct WvwAbility {
    pub id: u64,
    pub rank: u8,
}

#[derive(Deserialize)]
pub struct EquipmentPvp {
    pub amulet: Option<u64>,
    pub rune: Option<u64>,
    pub sigils: (Option<u64>, Option<u64>, Option<u64>, Option<u64>),
}

#[derive(Deserialize, PartialEq)]
pub enum Flags {
    Beta,
}

#[rest("https://api.guildwars2.com/v2/characters/{}?access_token={}&v=2019-04-22T00:00:00Z")]
#[rest(
    "https://api.guildwars2.com/v2/characters?access_token={}&v=2019-04-22T00:00:00Z&page=0",
    vec
)]
#[derive(Deserialize)]
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

pub fn get_character(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Character>, Box<std::error::Error>> {
    Character::get(vec![character_name, api_key])
}

pub fn get_all_characters(api_key: &str) -> Result<Vec<Box<Character>>, Box<std::error::Error>> {
    Character::gets(vec![api_key])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_backstory() {
        get_backstory(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_core() {
        get_core(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_crafting() {
        get_crafting(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_equipment() {
        get_equipment(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_skills() {
        get_skills(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_specializations() {
        get_specializations(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_training() {
        get_training(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_recipes() {
        get_recipes(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_character() {
        get_character(
            "Eff Testing Ele",
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_all_characters() {
        get_all_characters(
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015",
        )
        .unwrap();
    }
}