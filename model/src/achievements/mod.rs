use crate::{
    Endpoint,
    FixedEndpoint,
    misc::{
        minis::MiniPetId,
        titles::TitleId,
    },
    items::{
        skins::SkinId,
        ItemId,
    },
    maps::continents::{MasteryPointId, MasteryPointRegion},
};
use serde::{Deserialize, Serialize};

pub mod categories;
pub mod groups;

pub type AchievementId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementFlags {
    /// Can only get progress in PvP or WvW
    Pvp,
    /// is a meta-achievement
    CategoryDisplay,
    /// affects in-game UI collation
    MoveToTop,
    /// doesn't appear in the "nearly complete" UI
    IgnoreNearlyComplete,
    /// can be repeated multiple times
    Repeatable,
    /// hidden achievement; must fulfil unlock requirements before making progress or showing in the hero panel
    Hidden,
    /// must fulfil unlock requirements before making progress but will show in the hero panel before unlocking
    RequiresUnlock,
    /// unknown
    RepairOnLogin,
    /// Flags an achievement as resetting daily.
    Daily,
    /// Flags an achievement as resetting weekly.
    Weekly,
    /// Flags an achievement as resetting monthly.
    Monthly,
    /// Flags an achievement as progress never resetting.
    Permanent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementType {
    /// A default achievement.
    Default,
    /// Achievement is linked to Collections
    ItemSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTier {
    /// The number of "things" (achievement-specific) that must be completed to achieve this tier.
    count: u32,
    /// The amount of AP awarded for completing this tier.
    points: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementCoinsReward {
    // u32::MAX == 4,294,967,295
    // 4,294,967,295 (copper) / 10,000 (copper to 1 gold) == 429,496.72 (gold)
    // Max gold holdable: 200,000
    // u32 it is!
    /// The number of Coins to be rewarded.
    count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementItemReward {
    /// The item ID to be rewarded.
    id: ItemId,
    /// The number of id to be rewarded.
    count: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementMasteryReward {
    /// The mastery point ID to be rewarded.
    id: MasteryPointId,
    /// The region the Mastery Point applies to.
    region: MasteryPointRegion,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTitleReward {
    /// The title id.
    id: TitleId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementReward {
    Coins(AchievementCoinsReward),
    Item(AchievementItemReward),
    Mastery(AchievementMasteryReward),
    Title(AchievementTitleReward),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTextBit {
    // TODO: figure out if None for Text always
    /// The ID of the item, mini, or skin, if applicable.
    id: Option<u32>,
    /// The text or hint of the bit.
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementItemBit {
    /// The ID of the item, mini, or skin, if applicable.
    id: Option<ItemId>,
    /// The text or hint of the bit.
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementMinipetBit {
    /// The ID of the item, mini, or skin, if applicable.
    id: Option<MiniPetId>,
    /// The text or hint of the bit.
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementSkinBit {
    /// The ID of the item, mini, or skin, if applicable.
    id: Option<SkinId>,
    /// The text or hint of the bit.
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AchievementBit {
    Text(AchievementTextBit),
    Item(AchievementItemBit),
    Minipet(AchievementMinipetBit),
    Skin(AchievementSkinBit),
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievement {
    /// The achievement id.
    id: AchievementId,
    /// The achievement icon.
    icon: Option<String>,
    /// The achieivement name.
    name: String,
    /// The achievement description.
    description: String,
    /// The achievement requirement as listed in-game.
    requirement: String,
    /// The achievement description prior to unlocking it.
    locked_text: String,
    /// The achievement type.
    #[serde(rename = "type")]
    _type: AchievementType,
    /// Achievement categories.
    flags: Vec<AchievementFlags>,
    /// Describes the achievement's tiers.
    tiers: Vec<AchievementTier>,
    /// Contains an array of achievement ids required to progress the given achievement.
    prerequisites: Vec<AchievementId>,
    /// Describes the rewards given for the achievement.
    rewards: Vec<AchievementReward>,
    // TODO: discern whether should be Option<Vec<T>> or Vec<T>, given the fact that you
    // might want a way to check other than whether or not Vec.is_empty()?
    /// Contains a number of objects, each corresponding to a bitmask value that can give further information on the progress towards the achievement.
    bits: Option<Vec<AchievementBit>>,
    /// The maximum number of AP that can be rewarded by an achievement flagged as Repeatable.
    point_cap: Option<u32>,
}

pub type Achievements = Vec<Achievement>;

impl Endpoint for Achievements {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for Achievements {}
