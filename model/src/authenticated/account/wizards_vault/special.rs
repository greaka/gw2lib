
use crate::{authenticated::account::wizards_vault::WizardsVaultObjective,
    Endpoint, FixedEndpoint};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WizardsVaultSpecials {
    /// An array of objects detailing each special objective.
    objectives: Vec<WizardsVaultObjective>,
}

impl Endpoint for WizardsVaultSpecials {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/wizardsvault/special";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}

impl FixedEndpoint for WizardsVaultSpecials {}
