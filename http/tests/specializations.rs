#![cfg(feature = "blocking")]

use gw2lib::model::game_mechanics::specializations::{Specialization, SpecializationId};
use gw2lib::Requester;

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Specialization> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<SpecializationId> = client.ids::<Specialization, _>().unwrap();
}

#[test]
fn single() {
    let client = setup::setup();
    let _: Specialization = client.single(1).unwrap();
}
