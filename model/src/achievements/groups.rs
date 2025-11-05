use serde::{Deserialize, Serialize};

use crate::{
    achievements::categories::AchievementCategoryId, BulkEndpoint, Endpoint, EndpointWithId,
    FixedEndpoint,
};

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

impl EndpointWithId for AchievementGroup {
    type IdType = AchievementGroupGuid;
}

impl Endpoint for AchievementGroup {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements/groups";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for AchievementGroup {}

impl BulkEndpoint for AchievementGroup {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
