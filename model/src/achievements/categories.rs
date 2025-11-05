use serde::{Deserialize, Serialize};

use crate::{achievements::AchievementId, authenticated::account::Access, BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};

pub type AchievementCategoryId = u32;
pub type AchievementCategoryOrder = u32;

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
    product: Access,
    condition: AchievementAccessCondition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementDetails {
    id: AchievementId,
    required_access: Option<AchievementAccess>,
    flags: Option<Vec<DailyAchievementType>>,
    level: Option<[u8; 2]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AchievementCategory {
    id: AchievementCategoryId,
    name: String,
    description: String,
    order: AchievementCategoryOrder,
    icon: String,
    achievements: Vec<AchievementDetails>,
    tomorrow: Option<Vec<AchievementDetails>>,
}

impl Endpoint for AchievementCategory {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/achievements/categories";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl EndpointWithId for AchievementCategory {
    type IdType = AchievementCategoryId;
}

impl FixedEndpoint for AchievementCategory {}

impl BulkEndpoint for AchievementCategory {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
