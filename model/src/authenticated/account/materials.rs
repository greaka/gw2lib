use serde::{Deserialize, Serialize};

use crate::{Endpoint, FixedEndpoint};

pub type MaterialId = u32;

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct MaterialsItem {
    pub id: MaterialId,
    pub category: u32,
    pub count: u32,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Materials(pub Vec<MaterialsItem>);

impl Endpoint for Materials {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/materials";
    const VERSION: &'static str = "2022-07-25T00:00:00.000Z";
}

impl FixedEndpoint for Materials {}
