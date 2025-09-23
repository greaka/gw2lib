#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::maps::Map};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Map> = client.all().unwrap();
}
