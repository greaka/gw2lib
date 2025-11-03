use serde::{Deserialize, Serialize};

use crate::items::ItemId;

pub mod daily;
pub mod listings;
pub mod special;
pub mod weekly;

pub type WizardsVaultObjectiveId = u32;
pub type AstralAcclaim = u16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultObjective {
    id: WizardsVaultObjectiveId,
    title: String,
    track: String,
    acclaim: AstralAcclaim,
    progress_current: u32,
    progress_complete: u32,
    claimed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultPeriodicCommon {
    meta_progress_current: u32,
    meta_progress_complete: u32,
    meta_reward_item_id: ItemId,
    meta_reward_astral: AstralAcclaim,
    meta_reward_claimed: bool,
}
