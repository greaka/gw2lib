use serde::{Deserialize, Serialize};

pub mod daily;
pub mod listings;
pub mod special;
pub mod weekly;

pub type WizardsVaultObjectiveId = u32;

// TODO: can I set this to u16? You can't currently have more than what, 1,500 acclaim?
pub type AstralAcclaim = u16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultObjective {
    /// The ID of the objective.
    id: WizardsVaultObjectiveId,
    title: String,
    track: String,
    acclaim: AstralAcclaim,
    // TODO: figure out if u32 is unreasonable here
    progress_current: u32,
    progress_complete: u32,
    claimed: bool,
}
