use crate::{authenticated::account::wizards_vault::{AstralAcclaim, WizardsVaultObjective},
    Endpoint, FixedEndpoint, items::ItemId};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultDailies {
    // TODO: figure out if u32 is unreasonable here
    ///  The current progress to the meta achievement for the daily.
    meta_progress_current: u32,
    /// The threshold for the meta progress to be 'complete', and the meta reward claimable.
    meta_progress_complete: u32,
    /// The ID of the item you receive for claiming the meta reward.
    meta_reward_item_id: ItemId,
    /// The amount of Astral Acclaim you receive for claiming the meta reward.
    meta_reward_astral: AstralAcclaim,
    /// Whether the account has claimed the meta reward.
    meta_reward_claimed: bool,
    /// An array of objects detailing each daily objective.
    objectives: Vec<WizardsVaultObjective>,
}

impl Endpoint for WizardsVaultDailies {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wizardsvault/daily";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for WizardsVaultDailies {}
