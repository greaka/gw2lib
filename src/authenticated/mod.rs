use rest_client::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Permissions {
    Account,
    Builds,
    Characters,
    Guilds,
    Inventories,
    Progression,
    PvP,
    TradingPost,
    Unlocks,
    Wallet
}

#[rest("https://api.guildwars2.com/v2/tokeninfo?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Tokeninfo {
    id: String,
    name: String,
    permission: Vec<Permissions>,
}
