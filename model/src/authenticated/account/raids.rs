use crate::{Authenticated, Endpoint, FixedEndpoint, misc::raids::EventId};

pub type RaidEvent = Vec<EventId>;

impl Endpoint for RaidEvent {
    type Authenticated = Authenticated;

    const LOCALE: bool = false;
    const URL: &'static str = "v2/account/raids";
    const VERSION: &'static str = "2023-08-02T00:00:00.000Z";
}

impl FixedEndpoint for RaidEvent {}
