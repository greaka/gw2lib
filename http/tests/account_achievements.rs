#![cfg(feature = "blocking")]

use gw2lib::{model::authenticated::account::achievements::AccountAchievements, Requester};

pub mod setup;

#[test]
fn eff_testing() {
    let client = setup::setup();
    let _: AccountAchievements = client.get().unwrap();
}
