use serde::{Deserialize, Serialize};

use crate::{misc::colors::ColorId, BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MountDyeSlot {
    pub color_id: ColorId,
    pub material: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MountSkin {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub dye_slots: Vec<MountDyeSlot>,
    pub mount: String,
}

impl Endpoint for MountSkin {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/mounts/skins";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for MountSkin {
    type IdType = u32;
}
impl BulkEndpoint for MountSkin {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MountType {
    pub id: String,
    pub name: String,
    pub default_skin: u32,
    pub skins: Vec<u32>,
    pub skills: Option<Vec<serde_json::Value>>,
}

impl Endpoint for MountType {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/mounts/types";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for MountType {
    type IdType = String;
}
impl BulkEndpoint for MountType {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
