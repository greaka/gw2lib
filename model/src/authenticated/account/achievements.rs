use serde::{Deserialize, Serialize};

use crate::{achievements::AchievementId, Endpoint, FixedEndpoint};

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountAchievement {
    /// The achievement id.
    id: AchievementId,
    /// This attribute contains an array of numbers, giving more specific
    /// information on the progress for the achievement. The meaning of each
    /// value varies with each achievement. Bits start at zero. If an
    /// achievement is done, the in-progress bits are not displayed.
    bits: Option<Vec<u8>>,
    /// The player's current progress towards the achievement
    current: Option<u32>,
    /// The amount needed to complete the achievement.
    max: Option<u32>,
    /// Whether or not the achievement is done.
    done: bool,
    /// The number of times the achievement has been completed if the
    /// achievement is repeatable.
    repeated: Option<u32>,
    // The API exposes this as (boolean, optional), where a lack of presence is regarded as being
    // unlocked. I consider this equivalent to the below. Please critique in review.
    /// Whether or not the achievement is unlocked. Note that if this property
    /// does not exist, the achievement is unlocked as well.
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
