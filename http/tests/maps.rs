#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::maps::Map};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Map> = client.all().unwrap();
    let _: Map = client.try_single(161).unwrap();
}
