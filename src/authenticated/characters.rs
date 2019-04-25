use crate::utils;

use rest_client::*;
use serde::Deserialize;

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/backstory?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Backstory {
    backstory: Vec<String>,
}

pub fn get_backstory(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Backstory>, Box<std::error::Error>> {
    Backstory::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub enum Race {
    Asura,
    Charr,
    Human,
    Norn,
    Sylvari,
}

#[derive(Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Deserialize)]
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
    name: String,
    race: Race,
    gender: Gender,
    profession: Profession,
    level: u8,
    guild: Option<String>,
    age: u64,
    created: utils::TimeStamp,
    deaths: u64,
    title: Option<u64>,
}

pub fn get_core(character_name: &str, api_key: &str) -> Result<Box<Core>, Box<std::error::Error>> {
    Core::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
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
    discipline: Discipline,
    rating: u16,
    active: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/crafting?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Crafting {
    crafting: Vec<Craft>,
}

pub fn get_crafting(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Crafting>, Box<std::error::Error>> {
    Crafting::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
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
    power: Option<u16>,
    precision: Option<u16>,
    toughness: Option<u16>,
    vitality: Option<u16>,
    condition_damage: Option<u16>,
    condition_duration: Option<u16>,
    healing: Option<u16>,
    boon_duration: Option<u16>,
}

#[derive(Deserialize)]
pub struct Stats {
    id: u64,
    attributes: Attributes,
}

#[derive(Deserialize)]
pub enum Binding {
    Character,
    Account,
}

#[derive(Deserialize)]
pub struct Equip {
    id: u64,
    slot: Slot,
    infusions: Option<Vec<u64>>,
    upgrades: Option<Vec<u64>>,
    skin: Option<u64>,
    stats: Option<Stats>,
    binding: Option<Binding>,
    charges: Option<u16>,
    bound_to: Option<String>,
    dyes: Option<Vec<Option<u64>>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/equipment?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Equipment {
    equipment: Vec<Equip>,
}

pub fn get_equipment(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Equipment>, Box<std::error::Error>> {
    Equipment::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct InventoryItem {
    id: u64,
    count: u8,
    infusions: Option<Vec<u64>>,
    upgrades: Option<Vec<u64>>,
    skin: Option<u64>,
    stats: Option<Stats>,
    binding: Option<Binding>,
    bound_to: Option<String>,
}

#[derive(Deserialize)]
pub struct InventoryBag {
    id: u64,
    size: u8,
    inventory: Vec<Option<InventoryItem>>,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/inventory?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Inventory {
    bags: Vec<InventoryBag>,
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
    heal: Option<u64>,
    utilities: Utilities,
    elite: Option<u64>,
    legends: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct SkillDataSet {
    pve: Skillset,
    pvp: Skillset,
    wvw: Skillset,
}

#[rest("https://api.guildwars2.com/v2/characters/{}/skills?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Skills {
    skills: SkillDataSet,
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
    id: u64,
    traits: TraitSet,
}

pub type Specialization = (Option<TraitLine>, Option<TraitLine>, Option<TraitLine>);

#[derive(Deserialize)]
pub struct SpecializationSet {
    pve: Specialization,
    pvp: Specialization,
    wvw: Specialization,
}

#[rest("https://api.guildwars2.com/v2/characters/{}/specializations?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Specializations {
    specializations: SpecializationSet,
}

pub fn get_specializations(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Specializations>, Box<std::error::Error>> {
    Specializations::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct TrainingSet {
    id: u64,
    spent: u16,
    done: bool,
}

#[rest(
    "https://api.guildwars2.com/v2/characters/{}/training?access_token={}&v=2019-04-22T00:00:00Z"
)]
#[derive(Deserialize)]
pub struct Training {
    training: Vec<TrainingSet>,
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
    recipes: Vec<u64>,
}

pub fn get_recipes(
    character_name: &str,
    api_key: &str,
) -> Result<Box<Recipes>, Box<std::error::Error>> {
    Recipes::get(vec![character_name, api_key])
}

#[derive(Deserialize)]
pub struct WvwAbility {
    id: u64,
    rank: u8,
}

#[derive(Deserialize)]
pub struct EquipmentPvp {
    amulet: u64,
    rune: u64,
    sigils: (u64, u64, u64, u64),
}

#[derive(Deserialize)]
pub enum Flags {
    Beta,
}

#[rest("https://api.guildwars2.com/v2/characters/{}?access_token={}&v=2019-04-22T00:00:00Z")]
#[rest("https://api.guildwars2.com/v2/characters?access_token={}&v=2019-04-22T00:00:00Z&page=0", vec)]
#[derive(Deserialize)]
pub struct Character {
    #[serde(flatten)]
    backstory: Backstory,
    #[serde(flatten)]
    core: Core,
    #[serde(flatten)]
    crafting: Crafting,
    #[serde(flatten)]
    equipment: Equipment,
    #[serde(flatten)]
    inventory: Inventory,
    #[serde(flatten)]
    recipes: Recipes,
    #[serde(flatten)]
    skills: Skills,
    #[serde(flatten)]
    specializations: Specializations,
    #[serde(flatten)]
    training: Training,

    wvw_abilities: Vec<WvwAbility>,
    equipment_pvp: EquipmentPvp,
    flags: Vec<Flags>,
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