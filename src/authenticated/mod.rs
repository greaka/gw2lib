use rest_client::*;
use serde::Deserialize;

#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct Tokeninfo {
    id: String,
    name: String,
    permissions: Vec<Permissions>,
}

pub fn get_tokeninfo(api_key: String) -> Result<Box<Tokeninfo>, Box<std::error::Error>> {
    Tokeninfo::get(vec![api_key])
}

#[derive(Deserialize)]
pub enum Access {
    None,
    PlayForFree,
    GuildWars2,
    HeartOfThorns,
    PathOfFire,
}

#[rest("https://api.guildwars2.com/v2/account?access_token={}&v=2019-04-22T00:00:00Z")]
#[derive(Deserialize)]
pub struct Account {
    id: String,
    age: u64,
    name: String,
    world: u16,
    guilds: Vec<String>,
    guild_leader: Vec<String>,
    created: String,
    access: Vec<Access>,
    commander: bool,
    fractal_level: Option<u8>,
    daily_ap: Option<u16>,
    monthly_ap: Option<u16>,
    wvw_rank: Option<u16>,
    last_modified: String,
}

pub fn get_account(api_key: String) -> Result<Box<Account>, Box<std::error::Error>> {
    Account::get(vec![api_key])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_tokeninfo() {
        get_tokeninfo(
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015".to_owned(),
        )
        .unwrap();
    }

    #[allow(unused_must_use)]
    #[test]
    fn test_get_account() {
        get_account(
            "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015".to_owned(),
        )
        .unwrap();
    }
}
