#![cfg(feature = "blocking")]

use gw2lib::{
    model::maps::continents::{Continent, Floor},
    Requester,
};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Continent> = client.all().unwrap();
}

#[test]
fn tyria_all_floors() {
    let client = crate::setup::setup();
    let tyria: Continent = client.single(1).unwrap();
    for floor in tyria.floors {
        let _: Floor = client.single(floor).unwrap();
    }
}

#[test]
fn mists_all_floors() {
    let client = crate::setup::setup();
    let mists: Continent = client.single(2).unwrap();
    for floor in mists.floors {
        let _: Floor = client.single(floor).unwrap();
    }
}
