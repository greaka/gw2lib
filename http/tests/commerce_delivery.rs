#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::commerce::delivery::Delivery, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Delivery = client.get().unwrap();
}
