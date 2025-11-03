use crate::{achievements::AchievementId, authenticated::account::Access, Endpoint, FixedEndpoint};
use serde::{Deserialize,Serialize};

pub type AchievementCategoryId = u32;
pub type AchievementCategoryOrder = u32;

// I would consider this nomenclature asinine if it weren't literally how it was documented.
/// The type of daily achievement.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DailyAchievementType {
    PvE,
    PvP,
    WvW,
    SpecialEvent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AchievementAccessCondition {
    HasAccess,
    NoAccess,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementAccess {
    /// A Guild Wars 2 campaign.
    product: Access,
    /// The condition if a account can or cannot see this daily achievement.
    condition: AchievementAccessCondition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementDetails {
    /// The ID of an achievement.
    id: AchievementId,
    /// Describes if a daily requires a Guild Wars 2 campaign or not.
    required_access: Option<AchievementAccess>,
    /// The type of daily achievement.
    flags: Vec<DailyAchievementType>,
    /// The inclusive level range for a daily achievement. Available on achievements that are level locked.
    level: Option<[u8; 2]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementCategory {
    /// The category's ID.
    id: AchievementCategoryId,
    /// The category name.
    name: String,
    /// The category description.
    description: String,
    /// A number describing where to sort this category among other the other categories in its group. Lowest numbers go first, highest numbers go last.
    order: AchievementCategoryOrder,
    /// A URL to an image for the icon of the category.
    icon: String,
    /// An array containing a number of achievement IDs that this category contains.
    achievements: Vec<AchievementDetails>,
    /// A list of achievements that will be active tomorrow. This is only available on some daily categories.
    tomorrow: Option<Vec<AchievementDetails>>,
}

pub type AchievementCategories = Vec<AchievementCategory>;

impl Endpoint for AchievementCategories {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements/categories";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for AchievementCategories {}
