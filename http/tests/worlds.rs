#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::misc::worlds::World};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<World> = client.all().unwrap();
}
