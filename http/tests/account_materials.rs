#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::materials::Materials, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Materials = client.get().unwrap();
}
