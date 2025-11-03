use serde::{Deserialize, Serialize};

use crate::{
    authenticated::account::wizards_vault::{WizardsVaultObjective, WizardsVaultPeriodicCommon},
    Endpoint, FixedEndpoint,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultDailies {
    #[serde(flatten)]
    periodic: WizardsVaultPeriodicCommon,
    objectives: Vec<WizardsVaultObjective>,
}

impl Endpoint for WizardsVaultDailies {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wizardsvault/daily";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for WizardsVaultDailies {}
