#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::materials::AccountMaterials, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: AccountMaterials = client.get().unwrap();
}
