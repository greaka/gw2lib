use crate::utils::*;
use rest_client::*;
use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum GameTypes {
    Activity,
    Dungeon,
    Pve,
    Pvp,
    PvpLobby,
    Wvw,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum ArmorSlot {
    Boots,
    Coat,
    Gloves,
    Helm,
    HelmAquatic,
    Leggings,
    Shoulders,
}

#[derive(Deserialize)]
pub enum WeightClass {
    Heavy,
    Medium,
    Light,
    Clothing,
}

#[derive(Deserialize)]
pub enum InfusionType {
    Enrichment,
    Infusion,
}

#[derive(Deserialize)]
pub struct InfusionSlot {
    flags: Vec<InfusionType>,
    item_id: Option<u64>,
}

#[derive(Deserialize)]
pub enum AttributeType {
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

#[derive(Deserialize)]
pub struct Attribute {
    attribute: AttributeType,
    modifier: u16,
}

#[derive(Deserialize)]
pub struct Buff {
    skill_id: u64,
    description: String,
}

#[derive(Deserialize)]
pub struct InfixUpgrade {
    id: u64,
    attributes: Vec<Attribute>,
    buff: Option<Buff>,
}

#[derive(Deserialize)]
pub struct Upgrades {
    infusion_slots: Vec<InfusionSlot>,
    infix_upgrade: Option<InfixUpgrade>,
    suffix_item_id: Option<u64>,
    secondary_suffix_item_id: String,
    stat_choices: Option<Vec<u64>>,
}

#[derive(Deserialize)]
pub struct ArmorDetails {
    #[serde(rename = "type")]
    slot_type: ArmorSlot,
    weight_class: WeightClass,
    defense: u16,
    #[serde(flatten)]
    upgrades: Upgrades,
}

#[derive(Deserialize)]
pub struct BackItemDetails {
    #[serde(flatten)]
    upgrades: Upgrades,
}

#[derive(Deserialize)]
pub struct BagDetails {
    size: u8,
    no_sell_or_sort: bool,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ConsumableDetails {
    #[serde(rename = "type")]
    consumable_type: ConsumableType,
    description: Option<String>,
    duration_ms: Option<u64>,
    unlock_type: Option<UnlockType>,
    color_id: Option<u64>,
    recipe_id: Option<u64>,
    extra_recipe_ids: Option<Vec<u64>>,
    guild_upgrade_id: Option<u64>,
    apply_count: Option<u8>,
    name: Option<String>,
    icon: Option<String>,
    skins: Option<Vec<u64>>,
}

#[derive(Deserialize)]
pub enum ContainerType {
    Default,
    GiftBox,
    Immediate,
    OpenUI,
}

#[derive(Deserialize)]
pub struct ContainerDetails {
    #[serde(rename = "type")]
    container_type: ContainerType,
}

#[derive(Deserialize)]
pub enum GatheringToolsType {
    Foraging,
    Logging,
    Mining,
}

#[derive(Deserialize)]
pub struct GatheringToolsDetails {
    #[serde(rename = "type")]
    gathering_tools_type: GatheringToolsType,
}

#[derive(Deserialize)]
pub enum GizmoType {
    Default,
    ContainerKey,
    RentableContractNpc,
    UnlimitedConsumable,
}

#[derive(Deserialize)]
pub struct GizmoDetails {
    #[serde(rename = "type")]
    gizmo_type: GizmoType,
    guild_upgrade_id: Option<u64>,
}

#[derive(Deserialize)]
pub struct MiniatureDetails {
    minipet_id: u64,
}

#[derive(Deserialize)]
pub enum SalvageKitType {
    Salvage,
}

#[derive(Deserialize)]
pub struct SalvageKitDetails {
    #[serde(rename = "type")]
    salvage_type: SalvageKitType,
    charges: u8,
}

#[derive(Deserialize)]
pub enum TrinketType {
    Accessory,
    Amulet,
    Ring,
}

#[derive(Deserialize)]
pub struct TrinketDetails {
    #[serde(rename = "type")]
    trinket_type: TrinketType,
    #[serde(flatten)]
    upgrades: Upgrades,
}

#[derive(Deserialize)]
pub enum UpgradeComponentType {
    Default,
    Gem,
    Rune,
    Sigil,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum InfusionUpgradeFlags {
    Enrichment,
    Infusion,
    Defense,
    Offense,
    Utility,
    Agony,
}

#[derive(Deserialize)]
pub struct UpgradeComponentDetails {
    #[serde(rename = "type")]
    upgrade_component_type: UpgradeComponentType,
    suffix: String,
    infix_upgrade: InfixUpgrade,
    bonuses: Option<Vec<String>>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum DamageType {
    Fire,
    Ice,
    Lightning,
    Physical,
    Choking,
}

#[derive(Deserialize)]
pub struct WeaponDetails {
    #[serde(rename = "type")]
    weapon_type: WeaponType,
    damage_type: DamageType,
    min_power: u16,
    max_power: u16,
    defense: u16,
    #[serde(flatten)]
    upgrades: Upgrades,
}

#[derive(Deserialize)]
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

#[rest("https://api.guildwars2.com/v2/items/{}?lang={}&v=2019-04-22T00:00:00Z")]
#[rest(
    "https://api.guildwars2.com/v2/items?ids={}&lang={}&v=2019-04-22T00:00:00Z",
    vec
)]
#[derive(Deserialize)]
pub struct Item {
    id: u64,
    chat_link: String,
    name: String,
    icon: String,
    description: Option<String>,
    #[serde(rename = "type")]
    item_type: ItemType,
    rarity: Rarity,
    level: u8,
    vendor_value: u64,
    default_skin: Option<u64>,
    flags: Vec<Flags>,
    game_types: Vec<GameTypes>,
    restrictions: Vec<Restrictions>,
    details: Option<Details>,
}

pub fn get_item(
    item_id: impl std::fmt::Display,
    lang: Language,
) -> Result<Box<Item>, Box<std::error::Error>> {
    Item::get(vec![item_id.to_string(), lang.to_string()])
}

pub fn get_items(
    item_ids: impl IntoIterator<Item = impl std::fmt::Display>,
    lang: Language,
) -> Result<Vec<Box<Item>>, Box<std::error::Error>> {
    let mut items = item_ids
        .into_iter()
        .fold(String::new(), |acc, x| format!("{},{}", acc, x));
    items = (&items[1..]).to_owned();
    Item::gets(vec![items, lang.to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_item() {
        get_item(19723, Language::En).unwrap();
    }

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

    #[allow(unused_must_use)]
    #[test]
    fn test_get_items() {
        get_items(
            vec![
                19723, 80248, 77474, 85371, 19993, 20316, 69478, 38506, 48879, 67027, 77958, 24691,
                30699,
            ],
            Language::En,
        )
        .unwrap();
    }
}
