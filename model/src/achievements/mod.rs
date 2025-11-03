use serde::{Deserialize, Serialize};

use crate::{
    items::{skins::SkinId, ItemId},
    maps::continents::{MasteryPointId, MasteryPointRegion},
    misc::{minis::MiniPetId, titles::TitleId},
    Endpoint, FixedEndpoint,
};

pub mod categories;
pub mod groups;

pub type AchievementId = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementFlags {
    Pvp,
    CategoryDisplay,
    MoveToTop,
    IgnoreNearlyComplete,
    Repeatable,
    Hidden,
    RequiresUnlock,
    RepairOnLogin,
    Daily,
    Weekly,
    Monthly,
    Permanent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementType {
    Default,
    ItemSet,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTier {
    count: u32,
    points: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementCoinsReward {
    count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementItemReward {
    id: ItemId,
    count: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementMasteryReward {
    id: MasteryPointId,
    region: MasteryPointRegion,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementTitleReward {
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
    id: Option<u32>,
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementItemBit {
    id: Option<ItemId>,
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementMinipetBit {
    id: Option<MiniPetId>,
    text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementSkinBit {
    id: Option<SkinId>,
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
    id: AchievementId,
    icon: Option<String>,
    name: String,
    description: String,
    requirement: String,
    locked_text: String,
    #[serde(rename = "type")]
    _type: AchievementType,
    flags: Vec<AchievementFlags>,
    tiers: Vec<AchievementTier>,
    prerequisites: Vec<AchievementId>,
    rewards: Vec<AchievementReward>,
    bits: Option<Vec<AchievementBit>>,
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
