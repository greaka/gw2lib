use crate::utils::*;
use rest_client::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GameTypes {
    Activity,
    Dungeon,
    Pve,
    Pvp,
    PvpLobby,
    Wvw,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ArmorSlot {
    Boots,
    Coat,
    Gloves,
    Helm,
    HelmAquatic,
    Leggings,
    Shoulders,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum WeightClass {
    Heavy,
    Medium,
    Light,
    Clothing,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InfusionType {
    Enrichment,
    Infusion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfusionSlot {
    pub flags: Vec<InfusionType>,
    pub item_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub attribute: AttributeType,
    pub modifier: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buff {
    pub skill_id: u64,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfixUpgrade {
    pub id: u64,
    pub attributes: Vec<Attribute>,
    pub buff: Option<Buff>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Upgrades {
    pub infusion_slots: Vec<InfusionSlot>,
    pub infix_upgrade: Option<InfixUpgrade>,
    pub suffix_item_id: Option<u64>,
    pub secondary_suffix_item_id: String,
    pub stat_choices: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArmorDetails {
    #[serde(rename = "type")]
    pub slot_type: ArmorSlot,
    pub weight_class: WeightClass,
    pub defense: u16,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackItemDetails {
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BagDetails {
    pub size: u8,
    pub no_sell_or_sort: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnlockType {
    BagSlot,
    BankTab,
    Champion,
    CollectibleCapacity,
    Content,
    CraftingRecipe,
    Dye,
    GliderSkin,
    Minipet,
    Ms,
    Outfit,
    RandomUnlock,
    SharedSlot,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConsumableDetails {
    #[serde(rename = "type")]
    pub consumable_type: ConsumableType,
    pub description: Option<String>,
    pub duration_ms: Option<u64>,
    pub unlock_type: Option<UnlockType>,
    pub color_id: Option<u64>,
    pub recipe_id: Option<u64>,
    pub extra_recipe_ids: Option<Vec<u64>>,
    pub guild_upgrade_id: Option<u64>,
    pub apply_count: Option<u8>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub skins: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ContainerType {
    Default,
    GiftBox,
    Immediate,
    OpenUI,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContainerDetails {
    #[serde(rename = "type")]
    pub container_type: ContainerType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GatheringToolsType {
    Foraging,
    Logging,
    Mining,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatheringToolsDetails {
    #[serde(rename = "type")]
    pub gathering_tools_type: GatheringToolsType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GizmoType {
    Default,
    ContainerKey,
    RentableContractNpc,
    UnlimitedConsumable,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GizmoDetails {
    #[serde(rename = "type")]
    pub gizmo_type: GizmoType,
    pub guild_upgrade_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MiniatureDetails {
    pub minipet_id: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SalvageKitType {
    Salvage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SalvageKitDetails {
    #[serde(rename = "type")]
    pub salvage_type: SalvageKitType,
    pub charges: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TrinketType {
    Accessory,
    Amulet,
    Ring,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrinketDetails {
    #[serde(rename = "type")]
    pub trinket_type: TrinketType,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UpgradeComponentType {
    Default,
    Gem,
    Rune,
    Sigil,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum InfusionUpgradeFlags {
    Enrichment,
    Infusion,
    Defense,
    Offense,
    Utility,
    Agony,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpgradeComponentDetails {
    #[serde(rename = "type")]
    pub upgrade_component_type: UpgradeComponentType,
    pub flags: Vec<UpgradeComponentFlags>,
    pub infusion_upgrade_flags: Vec<InfusionUpgradeFlags>,
    pub suffix: String,
    pub infix_upgrade: InfixUpgrade,
    pub bonuses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DamageType {
    Fire,
    Ice,
    Lightning,
    Physical,
    Choking,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeaponDetails {
    #[serde(rename = "type")]
    pub weapon_type: WeaponType,
    pub damage_type: DamageType,
    pub min_power: u16,
    pub max_power: u16,
    pub defense: u16,
    #[serde(flatten)]
    pub upgrades: Upgrades,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
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
}

#[rest(
    "https://api.guildwars2.com/v2/items/{}?lang={}&v=2019-04-22T00:00:00Z",
    wrapper = "ApiResult"
)]
#[rest(
    "https://api.guildwars2.com/v2/items?ids={}&lang={}&v=2019-04-22T00:00:00Z",
    vec,
    wrapper = "ApiResult"
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: u64,
    pub chat_link: String,
    pub name: String,
    pub icon: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub rarity: Rarity,
    pub level: u8,
    pub vendor_value: u64,
    pub default_skin: Option<u64>,
    pub flags: Vec<Flags>,
    pub game_types: Vec<GameTypes>,
    pub restrictions: Vec<Restrictions>,
    pub details: Option<Details>,
}

/// ```
/// use gw2api::items::items::*;
/// use gw2api::utils::*;
///
/// get_item(19723, Language::En).unwrap();
/// ```
pub fn get_item(
    item_id: impl std::fmt::Display,
    lang: Language,
) -> Result<ApiResult<Box<Item>>, Box<std::error::Error>> {
    Item::get(vec![item_id.to_string(), lang.to_string()])
}

/// ```
/// use gw2api::items::items::*;
/// use gw2api::utils::*;
///
/// get_items(
///     vec![
///         19723, 80248, 77474, 85371, 19993, 20316, 69478, 38506, 48879, 67027, 77958, 24691,
///         30699,
///     ],
///     Language::En,
/// )
/// .unwrap();
/// ```
pub fn get_items(
    item_ids: impl IntoIterator<Item = impl std::fmt::Display>,
    lang: Language,
) -> Result<ApiResult<Vec<Box<Item>>>, Box<std::error::Error>> {
    let item_ids = format_ids(item_ids);
    Item::get(vec![item_ids, lang.to_string()])
}
#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_armor() {
        get_item(80248, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_back() {
        get_item(77474, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_bag() {
        get_item(85371, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_consumable() {
        get_item(19993, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_container() {
        get_item(20316, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_gathering() {
        get_item(69478, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_gizmo() {
        get_item(38506, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_mini() {
        get_item(48879, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_salvage() {
        get_item(67027, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_trinket() {
        get_item(77958, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_upgrade() {
        get_item(24691, Language::En).unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item_weapon() {
        get_item(30699, Language::En).unwrap();
    }
}
