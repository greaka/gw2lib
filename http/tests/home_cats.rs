#![cfg(feature = "blocking")]

use gw2lib::{
    model::home_instance::{Cat, CatId},
    Requester,
};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Cat> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = crate::setup::setup();
    let _: Vec<CatId> = client.ids::<Cat, _>().unwrap();
}

#[test]
fn guardian_cat() {
    let client = crate::setup::setup();
    let _: Cat = client.single(15).unwrap();
}
