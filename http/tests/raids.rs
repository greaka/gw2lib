#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::misc::raids::{Raid, RaidId},
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Raid> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<RaidId> = client.ids::<Raid>().unwrap();
}

#[test]
fn forsaken_thicket_wings() {
    let client = setup::setup();
    let _: Raid = client.single("forsaken_thicket".to_string()).unwrap();
}

#[test]
fn bastion_of_the_penitent_wings() {
    let client = setup::setup();
    let _: Raid = client
        .single("bastion_of_the_penitent".to_string())
        .unwrap();
}

#[test]
fn hall_of_chains_wings() {
    let client = setup::setup();
    let _: Raid = client.single("hall_of_chains".to_string()).unwrap();
}

#[test]
fn the_key_of_ahdashim_wings() {
    let client = setup::setup();
    let _: Raid = client.single("the_key_of_ahdashim".to_string()).unwrap();
}
