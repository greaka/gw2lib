#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::home_instance::nodes::{Node, NodeId},
};

pub mod setup;

#[test]
fn all() {
    let client = setup::setup();
    let _: Vec<Node> = client.all().unwrap();
    let _: Node = client.try_single("advanced_cloth_rack".into()).unwrap();
}

#[test]
fn ids() {
    let client = setup::setup();
    let _: Vec<NodeId> = client.ids::<Node>().unwrap();
    let _: Vec<NodeId> = client.try_ids::<Node>().unwrap();
}

#[test]
fn advanced_cloth_rack() {
    let client = setup::setup();
    let _: Node = client.single("advanced_cloth_rack".into()).unwrap();
    let _: Node = client.try_single("advanced_cloth_rack".into()).unwrap();
}
