#![cfg(feature = "blocking")]

use gw2lib::{model::misc::worlds::World, Requester};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let worlds: Vec<World> = client.all().unwrap();
    let _: World = client.try_single(worlds.first().unwrap().id).unwrap();
}
