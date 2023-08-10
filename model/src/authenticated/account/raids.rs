use crate::{Endpoint, FixedEndpoint};
use crate::misc::raids::{EventId};

pub type RaidEvent = Vec<EventId>;

impl Endpoint for RaidEvent {
    const AUTHENTICATED: bool = true;
    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/raids";
    const VERSION: &'static str = "2023-08-02T00:00:00.000Z";
}

impl FixedEndpoint for RaidEvent {}
