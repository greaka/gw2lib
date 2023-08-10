#![cfg(feature = "blocking")]

use gw2lib::{
    model::misc::raids::{Raid, RaidId, Wing},
    Requester,
};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<RaidId> = client.ids::<Raid, _>().unwrap();
}

#[test]
#[ignore]
fn forsaken_thicket_wings() {
    let client = crate::setup::setup();
    let _: Raid = client.single("forsaken_thicket".to_string()).unwrap();
}

#[test]
#[ignore]
fn bastion_of_the_penitent_wings() {
    let client = crate::setup::setup();
    let _: Raid = client
        .single("bastion_of_the_penitent".to_string())
        .unwrap();
}

#[test]
#[ignore]
fn hall_of_chains_wings() {
    let client = crate::setup::setup();
    let _: Raid = client.single("hall_of_chains".to_string()).unwrap();
}

#[test]
#[ignore]
fn the_key_of_ahdashim_wings() {
    let client = crate::setup::setup();
    let _: Raid = client.single("the_key_of_ahdashim".to_string()).unwrap();
}
