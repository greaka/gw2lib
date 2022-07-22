#![cfg(feature = "blocking")]

use gw2lib::Requester;
use gw2lib_model::misc::worlds::World;

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<World> = client.all().unwrap();
}
