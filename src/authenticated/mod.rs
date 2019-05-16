
use rest_client::*;
use serde::{Serialize, Deserialize};
pub mod account;
pub mod characters;
pub mod commerce;
pub mod pvp;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
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
    Wallet,
}

#[rest("https://api.guildwars2.com/v2/tokeninfo?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Serialize, Deserialize)]
pub struct Tokeninfo {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permissions>,
}

pub fn get_tokeninfo(api_key: &str) -> Result<Box<Tokeninfo>, Box<std::error::Error>> {
    Tokeninfo::get(vec![api_key])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_tokeninfo() {
        get_tokeninfo("564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015")
            .unwrap();
    }
}
