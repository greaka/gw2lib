use serde::{Deserialize, Serialize};

use crate::{items::recipes::RecipeId, Endpoint, FixedEndpoint};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountRecipes(pub Vec<RecipeId>);

impl Endpoint for AccountRecipes {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/recipes";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl FixedEndpoint for AccountRecipes {}
