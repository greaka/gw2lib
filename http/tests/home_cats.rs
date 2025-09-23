#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::home_instance::cats::{Cat, CatId},
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
    let _: Vec<CatId> = client.ids::<Cat>().unwrap();
}

#[test]
fn guardian_cat() {
    let client = crate::setup::setup();
    let _: Cat = client.single(15).unwrap();
}
