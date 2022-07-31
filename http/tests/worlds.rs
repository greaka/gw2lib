#![cfg(feature = "blocking")]

use gw2lib::{model::misc::worlds::World, Requester};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<World> = client.all().unwrap();
}
