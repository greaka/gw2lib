use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct BackstoryAnswer {
    pub id: String,
    pub title: String,
    pub description: String,
    pub journal: String,
    pub question: u32,
    pub professions: Option<Vec<String>>,
    pub races: Option<Vec<String>>,
}

impl Endpoint for BackstoryAnswer {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/backstory/answers";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for BackstoryAnswer {
    type IdType = String;
}
impl BulkEndpoint for BackstoryAnswer {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct BackstoryQuestion {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub answers: Vec<String>,
    pub order: u32,
    pub races: Option<Vec<String>>,
    pub professions: Option<Vec<String>>,
}

impl Endpoint for BackstoryQuestion {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/backstory/questions";
    const VERSION: &'static str = "2021-01-11T00:00:00.000Z";
}
impl EndpointWithId for BackstoryQuestion {
    type IdType = u32;
}
impl BulkEndpoint for BackstoryQuestion {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
