use serde::{Deserialize, Serialize};

use crate::{BulkEndpoint, Endpoint, EndpointWithId, NoAuthentication};

pub type NodeId = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Node {
    pub id: NodeId,
}

impl EndpointWithId for Node {
    type IdType = NodeId;
}
impl Endpoint for Node {
    type Authenticated = NoAuthentication;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/home/nodes";
    const VERSION: &'static str = "2023-08-14T00:00:00.000Z";
}

impl BulkEndpoint for Node {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
