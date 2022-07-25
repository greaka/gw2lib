#![cfg(feature = "blocking")]

use gw2lib::Requester;
use gw2lib::model::authenticated::account::materials::Materials;

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: Materials = client.get().unwrap();
}
