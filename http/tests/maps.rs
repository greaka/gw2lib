#![cfg(feature = "blocking")]

use gw2lib::{model::maps::Map, Requester};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Map> = client.all().unwrap();
}
