#![cfg(feature = "blocking")]

use gw2lib::{Requester, model::authenticated::commerce::delivery::Delivery};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Delivery = client.get().unwrap();
    let _: Delivery = client.try_get().unwrap();
}
