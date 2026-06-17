use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct WvwObjective {
    pub id: String,
    pub name: Option<String>,
    pub sector_id: Option<u32>,
    #[serde(rename = "type")]
    pub _type: String,
    pub map_type: Option<String>,
    pub map_id: Option<u32>,
    pub upgrade_id: Option<u32>,
    pub coord: Option<Vec<f32>>,
    pub label_coord: Option<Vec<f32>>,
    pub marker: Option<String>,
    pub chat_link: String,
}

impl Endpoint for WvwObjective {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/wvw/objectives";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for WvwObjective {
    type IdType = String;
}
impl BulkEndpoint for WvwObjective {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
