use crate::utils;
use serde::Deserialize;
use rest_client::*;

#[derive(Deserialize, PartialEq)]
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
    pub id: String,
    pub age: u64,
    pub name: String,
    pub world: u16,
    pub guilds: Vec<String>,
    pub guild_leader: Vec<String>,
    pub created: utils::TimeStamp,
    pub access: Vec<Access>,
    pub commander: bool,
    pub fractal_level: Option<u8>,
    pub daily_ap: Option<u16>,
    pub monthly_ap: Option<u16>,
    pub wvw_rank: Option<u16>,
    pub last_modified: String,
}

pub fn get_account(api_key: &str) -> Result<Box<Account>, Box<std::error::Error>> {
    Account::get(vec![api_key])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_must_use)]
    #[test]
    fn test_get_account() {
        get_account("564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015")
            .unwrap();
    }
}