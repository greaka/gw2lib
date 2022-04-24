pub mod itemstats;
pub mod recipes;
pub mod skins;

use serde::{Deserialize, Serialize};

use crate::{
    game_mechanics::skills::SkillId,
    guild::upgrades::GuildUpgradeId,
    items::{itemstats::StatsId, recipes::RecipeId, skins::SkinId},
    misc::{colors::ColorId, minis::MiniPetId},
    BulkEndpoint, Endpoint,
};

pub type ItemId = u32;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum ItemType {
    Armor,
    Back,
    Bag,
    Consumable,
    Container,
    CraftingMaterial,
    Gathering,
    Gizmo,
    MiniPet,
    Tool,
    Trait,
    Trinket,
    Trophy,
    UpgradeComponent,
    Weapon,
    Key,
    Qux,
    Quux,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Rarity {
    Junk,
    Basic,
    Fine,
    Masterwork,
    Rare,
    Exotic,
    Ascended,
    Legendary,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Flags {
    AccountBindOnUse,
    AccountBound,
    Attuned,
    BulkConsume,
    DeleteWarning,
    HideSuffix,
    Infused,
    MonsterOnly,
    NoMysticForge,
    NoSalvage,
    NoSell,
    NotUpgradeable,
    NoUnderwater,
    SoulbindOnAcquire,
    SoulBindOnUse,
    Tonic,
    Unique,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum GameTypes {
    Activity,
    Dungeon,
    Pve,
    Pvp,
    PvpLobby,
    Wvw,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Restrictions {
    Asura,
    Charr,
    Human,
    Norn,
    Sylvari,
    Elementalist,
    Engineer,
    Guardian,
    Mesmer,
    Necromancer,
    Ranger,
    Thief,
    Warrior,
    Female,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum ArmorSlot {
    Boots,
    Coat,
    Gloves,
    Helm,
    HelmAquatic,
    Leggings,
    Shoulders,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum WeightClass {
    Heavy,
    Medium,
    Light,
    Clothing,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum InfusionType {
    Enrichment,
    Infusion,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct InfusionSlot {
    pub flags: Vec<InfusionType>,
    pub item_id: Option<ItemId>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum AttributeType {
    AgonyResistance,
    BoonDuration,
    ConditionDamage,
    ConditionDuration,
    CritDamage,
    Healing,
    Power,
    Precision,
    Toughness,
    Vitality,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub attribute: AttributeType,
    pub modifier: u16,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Buff {
    pub skill_id: SkillId,
    pub description: Option<String>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct InfixUpgrade {
    pub id: StatsId,
    pub attributes: Vec<Attribute>,
    pub buff: Option<Buff>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Upgrades {
    pub attribute_adjustment: f32,
    pub infusion_slots: Vec<InfusionSlot>,
    pub infix_upgrade: Option<InfixUpgrade>,
    pub suffix_item_id: Option<ItemId>,
    pub secondary_suffix_item_id: Option<ItemId>,
    pub stat_choices: Option<Vec<StatsId>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmorDetails {
    #[serde(rename = "type")]
    pub _type: ArmorSlot,
    pub weight_class: WeightClass,
    pub defense: u16,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackItemDetails {
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BagDetails {
    pub size: u8,
    pub no_sell_or_sort: bool,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum ConsumableType {
    AppearanceChange,
    Booze,
    ContractNpc,
    Currency,
    Food,
    Generic,
    Halloween,
    Immediate,
    MountRandomUnlock,
    RandomUnlock,
    Transmutation,
    Unlock,
    UpgradeRemoval,
    Utility,
    TeleportToFriend,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum UnlockType {
    BagSlot,
    BankTab,
    Champion,
    CollectibleCapacity,
    Content,
    CraftingRecipe,
    Dye,
    GliderSkin,
    GearLoadoutTab,
    BuildLibrarySlot,
    BuildLoadoutTab,
    Minipet,
    Ms,
    Outfit,
    RandomUnlock,
    SharedSlot,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumableDetails {
    #[serde(rename = "type")]
    pub _type: ConsumableType,
    pub description: Option<String>,
    pub duration_ms: Option<u64>,
    pub unlock_type: Option<UnlockType>,
    pub color_id: Option<ColorId>,
    pub recipe_id: Option<RecipeId>,
    pub extra_recipe_ids: Option<Vec<RecipeId>>,
    pub guild_upgrade_id: Option<GuildUpgradeId>,
    pub apply_count: Option<u8>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub skins: Option<Vec<u64>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum ContainerType {
    Default,
    GiftBox,
    Immediate,
    OpenUI,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContainerDetails {
    #[serde(rename = "type")]
    pub _type: ContainerType,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum GatheringToolsType {
    Foraging,
    Logging,
    Mining,
    Foo,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatheringToolsDetails {
    #[serde(rename = "type")]
    pub _type: GatheringToolsType,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum GizmoType {
    Default,
    ContainerKey,
    RentableContractNpc,
    UnlimitedConsumable,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GizmoDetails {
    #[serde(rename = "type")]
    pub _type: GizmoType,
    pub guild_upgrade_id: Option<GuildUpgradeId>,
    pub vendor_ids: Option<Vec<u64>>, // TODO: figure out if this is resolvable
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiniatureDetails {
    pub minipet_id: MiniPetId,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum SalvageKitType {
    Salvage,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SalvageKitDetails {
    #[serde(rename = "type")]
    pub _type: SalvageKitType,
    pub charges: u8,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum TrinketType {
    Accessory,
    Amulet,
    Ring,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrinketDetails {
    #[serde(rename = "type")]
    pub _type: TrinketType,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum UpgradeComponentType {
    Default,
    Gem,
    Rune,
    Sigil,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum UpgradeComponentFlags {
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
    HeavyArmor,
    MediumArmor,
    LightArmor,
    Trinket,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum InfusionUpgradeFlags {
    Enrichment,
    Infusion,
    Defense,
    Offense,
    Utility,
    Agony,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpgradeComponentDetails {
    #[serde(rename = "type")]
    pub _type: UpgradeComponentType,
    pub flags: Vec<UpgradeComponentFlags>,
    pub infusion_upgrade_flags: Vec<InfusionUpgradeFlags>,
    pub suffix: String,
    pub attribute_adjustment: f32,
    pub infix_upgrade: InfixUpgrade,
    pub bonuses: Option<Vec<String>>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum WeaponType {
    Axe,
    Dagger,
    Mace,
    Pistol,
    Scepter,
    Sword,
    Focus,
    Shield,
    Torch,
    Warhorn,
    Greatsword,
    Hammer,
    LongBow,
    Rifle,
    ShortBow,
    Staff,
    Harpoon,
    Speargun,
    Trident,
    LargeBundle,
    SmallBundle,
    Toy,
    ToyTwoHanded,
    None,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum DamageType {
    Fire,
    Ice,
    Lightning,
    Physical,
    Choking,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeaponDetails {
    #[serde(rename = "type")]
    pub _type: WeaponType,
    pub damage_type: DamageType,
    pub min_power: u16,
    pub max_power: u16,
    pub defense: u16,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "details")]
pub enum Details {
    Armor(ArmorDetails),
    Back(BackItemDetails),
    Bag(BagDetails),
    Consumable(ConsumableDetails),
    Container(ContainerDetails),
    Gathering(GatheringToolsDetails),
    Gizmo(GizmoDetails),
    MiniPet(MiniatureDetails),
    Tool(SalvageKitDetails),
    Trinket(TrinketDetails),
    UpgradeComponent(UpgradeComponentDetails),
    Weapon(WeaponDetails),
    CraftingMaterial,
    //Trait,
    Trophy,
    Key,
    Qux,
    Quux,
}

impl From<Details> for ItemType {
    fn from(d: Details) -> Self {
        match d {
            Details::Armor(_) => ItemType::Armor,
            Details::Back(_) => ItemType::Back,
            Details::Bag(_) => ItemType::Bag,
            Details::Consumable(_) => ItemType::Consumable,
            Details::Container(_) => ItemType::Container,
            Details::Gathering(_) => ItemType::Gathering,
            Details::Gizmo(_) => ItemType::Gizmo,
            Details::MiniPet(_) => ItemType::MiniPet,
            Details::Tool(_) => ItemType::Tool,
            Details::Trinket(_) => ItemType::Trinket,
            Details::UpgradeComponent(_) => ItemType::UpgradeComponent,
            Details::Weapon(_) => ItemType::Weapon,
            Details::CraftingMaterial => ItemType::CraftingMaterial,
            //Details::Trait => ItemType::Trait,
            Details::Trophy => ItemType::Trophy,
            Details::Key => ItemType::Key,
            Details::Qux => ItemType::Qux,
            Details::Quux => ItemType::Quux,
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub chat_link: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub rarity: Rarity,
    pub level: u8,
    pub vendor_value: u64,
    pub default_skin: Option<SkinId>,
    pub flags: Vec<Flags>,
    pub game_types: Vec<GameTypes>,
    pub restrictions: Vec<Restrictions>,
    #[serde(flatten)]
    pub details: Details,
}

impl_id!(Item, ItemId);
impl Endpoint for Item {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/items";
    const VERSION: &'static str = "2021-03-21T00:00:00.000Z";
}

impl BulkEndpoint for Item {
    const ALL: bool = false;
    const PAGING: bool = true;
}
