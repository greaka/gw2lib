#![cfg(feature = "blocking")]

use gw2lib::Requester;
use gw2lib_model::guild::{
    emblem::{GuildEmblemBackground, GuildEmblemForeground},
    permissions::GuildPermission,
    search::GuildSearch,
    upgrades::GuildUpgrade,
    Guild,
};
use uuid::uuid;

pub mod setup;

#[test]
fn by_id() {
    let client = setup::setup();
    let gid = uuid!("4BBB52AA-D768-4FC6-8EDE-C299F2822F0F");
    let g: Guild = client.single(gid).unwrap();
    assert_eq!(g.id, gid);
}

#[test]
fn emblem_backgrounds() {
    let client = setup::setup();
    let _: Vec<GuildEmblemBackground> = client.all().unwrap();
}

#[test]
fn emblem_foregrounds() {
    let client = setup::setup();
    let _: Vec<GuildEmblemForeground> = client.all().unwrap();
}

#[test]
fn permissions() {
    let client = setup::setup();
    let _: Vec<GuildPermission> = client.all().unwrap();
}

#[test]
fn search() {
    let client = setup::setup();
    let ids: GuildSearch = client.single("Arenanet".to_owned()).unwrap();
    assert_eq!(
        ids.0.first().unwrap(),
        &uuid!("4BBB52AA-D768-4FC6-8EDE-C299F2822F0F")
    )
}

#[test]
fn upgrades() {
    let client = setup::setup();
    let _: Vec<GuildUpgrade> = client.all().unwrap();
}
