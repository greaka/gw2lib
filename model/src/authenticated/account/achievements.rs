use serde::{Deserialize, Serialize};

use crate::{achievements::AchievementId, Endpoint, FixedEndpoint};

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountAchievement {
    id: AchievementId,
    bits: Option<Vec<u8>>,
    current: Option<u32>,
    max: Option<u32>,
    done: bool,
    repeated: Option<u32>,
    #[serde(default = "default_true")]
    unlocked: bool,
}

pub type AccountAchievements = Vec<AccountAchievement>;

impl Endpoint for AccountAchievements {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/achievements";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for AccountAchievements {}
