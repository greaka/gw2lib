use serde::{Deserialize, Serialize};

use crate::{
    game_mechanics::{skills::SkillId, specializations::SpecializationId, traits::TraitId},
    items::{itemstats::StatsId, recipes::RecipeId, skins::SkinId, ItemId},
    misc::{colors::ColorId, titles::TitleId},
    pvp::amulets::AmuletId,
    wvw::abilities::AbilityId,
    BulkEndpoint, Endpoint, EndpointWithId, TimeStamp,
};

pub type Age = u64;
pub type CharacterId = String;
pub type BackStoryId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Backstory {
    pub backstory: Vec<BackStoryId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Race {
    Asura,
    Charr,
    Human,
    Norn,
    Sylvari,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Core {
    pub name: CharacterId,
    pub race: Race,
    pub gender: Gender,
    pub profession: Profession,
    pub level: u8,
    pub guild: Option<String>,
    pub age: Age,
    pub created: TimeStamp,
    pub last_modified: TimeStamp,
    pub deaths: u32,
    pub title: Option<TitleId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Craft {
    pub discipline: Discipline,
    pub rating: u16,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Crafting {
    pub crafting: Vec<Craft>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Attributes {
    pub power: Option<u16>,
    pub precision: Option<u16>,
    pub crit_damage: Option<u16>,
    pub toughness: Option<u16>,
    pub vitality: Option<u16>,
    pub condition_damage: Option<u16>,
    pub condition_duration: Option<u16>,
    pub healing: Option<u16>,
    pub boon_duration: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stats {
    pub id: StatsId,
    pub attributes: Attributes,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Binding {
    Character,
    Account,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Location {
    Equipped,
    Armory,
    EquippedFromLegendaryArmory,
    LegendaryArmory,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Equip {
    pub id: ItemId,
    pub slot: Option<Slot>,
    /// only present in character.equipment
    pub count: Option<usize>,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades: Option<Vec<ItemId>>,
    pub skin: Option<SkinId>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub location: Location,
    pub charges: Option<u16>,
    pub bound_to: Option<String>,
    pub dyes: Option<Vec<Option<ColorId>>>,
    /// only present in character.equipment
    pub tabs: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Equipment {
    pub equipment: Vec<Equip>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryItem {
    pub id: ItemId,
    pub count: u8,
    pub charges: Option<u8>,
    pub infusions: Option<Vec<ItemId>>,
    pub upgrades: Option<Vec<ItemId>>,
    pub upgrade_slot_indices: Option<Vec<usize>>,
    pub skin: Option<SkinId>,
    pub stats: Option<Stats>,
    pub binding: Option<Binding>,
    pub bound_to: Option<String>,
    pub dyes: Option<Vec<ColorId>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryBag {
    pub id: ItemId,
    pub size: u8,
    pub inventory: Vec<Option<InventoryItem>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub bags: Vec<Option<InventoryBag>>,
}

pub type Utilities = [Option<SkillId>; 3];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Skillset {
    pub heal: Option<SkillId>,
    pub utilities: Utilities,
    pub elite: Option<SkillId>,
    // TODO: legends enum
    pub legends: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillDataSet {
    pub pve: Skillset,
    pub pvp: Skillset,
    pub wvw: Skillset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Skills {
    pub skills: SkillDataSet,
}

pub type TraitSet = [Option<TraitId>; 3];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraitLine {
    pub id: Option<SpecializationId>,
    pub traits: Option<TraitSet>,
}

pub type Specialization = [Option<TraitLine>; 3];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecializationSet {
    pub pve: Specialization,
    pub pvp: Specialization,
    pub wvw: Specialization,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Specializations {
    pub specializations: SpecializationSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrainingSet {
    // TODO: training id
    pub id: u64,
    pub spent: u16,
    pub done: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Training {
    pub training: Vec<TrainingSet>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipes {
    pub recipes: Vec<RecipeId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WvwAbility {
    pub id: AbilityId,
    pub rank: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentPvp {
    pub amulet: Option<AmuletId>,
    pub rune: Option<ItemId>,
    pub sigils: (
        Option<ItemId>,
        Option<ItemId>,
        Option<ItemId>,
        Option<ItemId>,
    ),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Flags {
    Beta,
}

pub type PetId = u16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BuildPets {
    pub terrestrial: [Option<PetId>; 2],
    pub aquatic: [Option<PetId>; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildLegends {
    pub legends: LegendSlots,
    pub aquatic_legends: LegendSlots,
}

pub type LegendId = String;
pub type LegendSlots = [Option<LegendId>; 2];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildTemplate {
    pub name: Option<String>,
    pub profession: Option<Profession>,
    pub specializations: [TraitLine; 3],
    pub skills: Skillset,
    pub aquatic_skills: Skillset,
    pub pets: Option<BuildPets>,
    pub legends: Option<LegendSlots>,
    pub aquatic_legends: Option<LegendSlots>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildTab {
    /// this index starts at 1
    pub tab: usize,
    pub is_active: bool,
    pub build: BuildTemplate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentTab {
    /// this index starts at 1
    pub tab: usize,
    pub name: String,
    pub is_active: bool,
    pub equipment: Vec<Equip>,
    pub equipment_pvp: Option<EquipmentPvp>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Character {
    pub backstory: Vec<BackStoryId>,
    #[serde(flatten)]
    pub core: Core,
    pub crafting: Vec<Craft>,
    #[serde(default)]
    pub equipment: Vec<Equip>,
    #[serde(default)]
    pub bags: Vec<Option<InventoryBag>>,
    #[serde(default)]
    pub recipes: Vec<RecipeId>,
    #[serde(default)]
    pub training: Vec<TrainingSet>,
    #[serde(default)]
    pub build_tabs: Vec<BuildTab>,
    pub build_tabs_unlocked: Option<usize>,
    pub active_build_tab: Option<usize>,
    #[serde(default)]
    pub equipment_tabs: Vec<EquipmentTab>,
    pub equipment_tabs_unlocked: Option<usize>,
    pub active_equipment_tab: Option<usize>,
    #[serde(default)]
    pub wvw_abilities: Vec<WvwAbility>,
    pub flags: Vec<Flags>,
}

impl EndpointWithId<CharacterId> for Character {
    fn id(&self) -> &CharacterId {
        &self.core.name
    }
}

impl Endpoint for Character {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/characters";
    const VERSION: &'static str = "2022-06-14T00:00:00.000Z";
}

impl BulkEndpoint for Character {
    const ALL: bool = true;
    const PAGING: bool = true;
}

impl EndpointWithId<CharacterId> for Core {
    fn id(&self) -> &CharacterId {
        &self.name
    }

    fn format_url(host: &str, id: &CharacterId) -> String {
        format!("{}/{}/{}/core", host, Self::URL, id)
    }
}

impl Endpoint for Core {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/characters";
    const VERSION: &'static str = "2022-06-14T00:00:00.000Z";
}
