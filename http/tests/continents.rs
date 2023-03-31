#![cfg(feature = "blocking")]

use gw2lib::{model::maps::continents::Continent, Requester};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Continent> = client.all().unwrap();
}

