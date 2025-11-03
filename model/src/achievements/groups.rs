use serde::{Deserialize, Serialize};

use crate::{achievements::categories::AchievementCategoryId, Endpoint, FixedEndpoint};

pub type AchievementGroupGuid = String;
pub type AchievementGroupOrder = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementGroup {
    id: AchievementGroupGuid,
    name: String,
    description: String,
    order: AchievementGroupOrder,
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
