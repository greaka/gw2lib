use crate::{
    Endpoint, FixedEndpoint,
    achievements::categories::AchievementCategoryId
};

use serde::{Deserialize,Serialize};

pub type AchievementGroupGuid = String;
pub type AchievementGroupOrder = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementGroup {
    /// The group's GUID.
    id: AchievementGroupGuid,
    /// The group name.
    name: String,
    /// The group description.
    description: String,
    /// A number describing where to sort this group among other groups. Lowest numbers go first, highest numbers go last.
    order: AchievementGroupOrder,
    /// An array containing a number of category IDs that this group contains.
    categories: Vec<AchievementCategoryId>,
}

pub type AchievementGroups = Vec<AchievementGroup>;

impl Endpoint for AchievementGroups {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements/groups";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for AchievementGroups {}
