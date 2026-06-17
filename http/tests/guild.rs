#![cfg(feature = "blocking")]

use gw2lib::{
    model::guild::{
        authenticated::{GuildLog, GuildMembers, GuildRanks},
        emblem::{EmblemBackground, EmblemForeground},
        guild::{Guild, GuildPermission},
        upgrades::GuildUpgrade,
    },
    Requester,
};

pub mod setup;

// Guild ID for "Fay Timewhisper"
const GUILD_ID: &str = "87B97E30-580E-E811-81A1-06AEA39922EA";

#[test]
fn guild_info() {
    let client = setup::setup();
    let _: Guild = client.single(GUILD_ID.to_string()).unwrap();
}

#[test]
fn guild_permissions() {
    let client = setup::setup();
    let ids: Vec<String> = client.ids::<GuildPermission, _>().unwrap();
    assert!(!ids.is_empty());
}

#[test]
fn guild_upgrades() {
    let client = setup::setup();
    let _: GuildUpgrade = client.single(38u64).unwrap();
}

#[test]
fn emblem_backgrounds() {
    let client = setup::setup();
    let _: EmblemBackground = client.single(1u32).unwrap();
}

#[test]
fn emblem_foregrounds() {
    let client = setup::setup();
    let _: EmblemForeground = client.single(1u32).unwrap();
}

#[test]
fn guild_log() {
    let client = setup::setup();
    let _: GuildLog = client.single(GUILD_ID.to_string()).unwrap();
}

#[test]
fn guild_members() {
    let client = setup::setup();
    let _: GuildMembers = client.single(GUILD_ID.to_string()).unwrap();
}

#[test]
fn guild_ranks() {
    let client = setup::setup();
    let _: GuildRanks = client.single(GUILD_ID.to_string()).unwrap();
}
