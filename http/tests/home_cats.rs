#![cfg(feature = "blocking")]

use gw2lib::{
    model::home_instance::cats::{Cat, CatId},
    Requester,
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Cat> = client.all().unwrap();
    let _: Cat = client.try_single(15).unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<CatId> = client.ids::<Cat>().unwrap();
    let _: Vec<CatId> = client.try_ids::<Cat>().unwrap();
}

#[test]
fn guardian_cat() {
    let client = setup::setup();
    let _: Cat = client.single(15).unwrap();
    let _: Cat = client.try_single(15).unwrap();
}
