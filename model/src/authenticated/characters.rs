use crate::{items::itemstats::StatsId, misc::titles::TitleId, TimeStamp};

use crate::{
    game_mechanics::{skills::SkillId, specializations::SpecializationId, traits::TraitId},
    items::{recipes::RecipeId, skins::SkinId, ItemId},
    misc::colors::ColorId,
    pvp::amulets::AmuletId,
    wvw::abilities::AbilityId,
};
use serde::{Deserialize, Serialize};

pub type Age = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Backstory {
    pub backstory: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Core {
    pub name:       String,
    pub race:       Race,
    pub gender:     Gender,
    pub profession: Profession,
    pub level:      u8,
    pub guild:      Option<String>,
    pub age:        Age,
    pub created:    TimeStamp,
    pub deaths:     u32,
    pub title:      Option<TitleId>,
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
    pub rating:     u16,
    pub active:     bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crafting {
    pub crafting: Vec<Craft>,
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
    pub power:              Option<u16>,
    pub precision:          Option<u16>,
    pub toughness:          Option<u16>,
    pub vitality:           Option<u16>,
    pub condition_damage:   Option<u16>,
    pub condition_duration: Option<u16>,
    pub healing:            Option<u16>,
    pub boon_duration:      Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub id:         StatsId,
    pub attributes: Attributes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Binding {
    Character,
    Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equip {
    pub id:        ItemId,
    pub slot:      Slot,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades:  Option<Vec<ItemId>>,
    pub skin:      Option<SkinId>,
    pub stats:     Option<Stats>,
    pub binding:   Option<Binding>,
    pub charges:   Option<u16>,
    pub bound_to:  Option<String>,
    pub dyes:      Option<Vec<Option<ColorId>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Equipment {
    pub equipment: Vec<Equip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id:        ItemId,
    pub count:     u8,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades:  Option<Vec<ItemId>>,
    pub skin:      Option<SkinId>,
    pub stats:     Option<Stats>,
    pub binding:   Option<Binding>,
    pub bound_to:  Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryBag {
    pub id:        ItemId,
    pub size:      u8,
    pub inventory: Vec<Option<InventoryItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub bags: Vec<Option<InventoryBag>>,
}

pub type Utilities = (Option<SkillId>, Option<SkillId>, Option<SkillId>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Skillset {
    pub heal:      Option<SkillId>,
    pub utilities: Utilities,
    pub elite:     Option<SkillId>,
    // TODO: legends enum
    pub legends:   Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDataSet {
    pub pve: Skillset,
    pub pvp: Skillset,
    pub wvw: Skillset,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Skills {
    pub skills: SkillDataSet,
}

pub type TraitSet = (Option<TraitId>, Option<TraitId>, Option<TraitId>);

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitLine {
    pub id:     SpecializationId,
    pub traits: TraitSet,
}

pub type Specialization = (Option<TraitLine>, Option<TraitLine>, Option<TraitLine>);

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecializationSet {
    pub pve: Specialization,
    pub pvp: Specialization,
    pub wvw: Specialization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specializations {
    pub specializations: SpecializationSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingSet {
    // TODO: training id
    pub id:    u64,
    pub spent: u16,
    pub done:  bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Training {
    pub training: Vec<TrainingSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipes {
    pub recipes: Vec<RecipeId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WvwAbility {
    pub id:   AbilityId,
    pub rank: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquipmentPvp {
    pub amulet: Option<AmuletId>,
    pub rune:   Option<ItemId>,
    pub sigils: (
        Option<ItemId>,
        Option<ItemId>,
        Option<ItemId>,
        Option<ItemId>,
    ),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Flags {
    Beta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    #[serde(flatten)]
    pub backstory:       Backstory,
    #[serde(flatten)]
    pub core:            Core,
    #[serde(flatten)]
    pub crafting:        Crafting,
    #[serde(flatten)]
    pub equipment:       Equipment,
    #[serde(flatten)]
    pub inventory:       Inventory,
    #[serde(flatten)]
    pub recipes:         Recipes,
    #[serde(flatten)]
    pub skills:          Skills,
    #[serde(flatten)]
    pub specializations: Specializations,
    #[serde(flatten)]
    pub training:        Training,

    pub wvw_abilities: Vec<WvwAbility>,
    pub equipment_pvp: EquipmentPvp,
    pub flags:         Vec<Flags>,
}
