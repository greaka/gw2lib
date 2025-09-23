#![cfg(feature = "blocking")]

use gw2lib::{
    Requester,
    model::home_instance::nodes::{Node, NodeId},
};

pub mod setup;

#[test]
fn all() {
    let client = crate::setup::setup();
    let _: Vec<Node> = client.all().unwrap();
}

#[test]
fn ids() {
    let client = crate::setup::setup();
    let _: Vec<NodeId> = client.ids::<Node>().unwrap();
}

#[test]
fn advanced_cloth_rack() {
    let client = crate::setup::setup();
    let _: Node = client.single("advanced_cloth_rack".to_string()).unwrap();
}
