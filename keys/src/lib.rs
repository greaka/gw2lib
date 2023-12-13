use std::str::FromStr;

use base64::{engine::general_purpose, Engine};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
pub enum ApiKey {
    ApiKey,
    Subtoken(Subtoken),
}

impl ApiKey {
    pub fn is_expired(&self) -> bool {
        match self {
            ApiKey::ApiKey => false,
            ApiKey::Subtoken(s) => s.expires < Utc::now().naive_utc(),
        }
    }

    pub fn has_permissions(&self, permissions: &[ApiKeyPermissions]) -> bool {
        match self {
            ApiKey::ApiKey => true,
            ApiKey::Subtoken(s) => permissions.iter().all(|p| s.permissions.contains(p)),
        }
    }

    pub fn has_url_permissions(&self, urls: &[&str]) -> bool {
        match self {
            ApiKey::ApiKey => true,
            ApiKey::Subtoken(Subtoken { urls: None, .. }) => true,
            ApiKey::Subtoken(Subtoken { urls: Some(u), .. }) => {
                urls.iter().all(|r| u.iter().any(|c| c == r))
            }
        }
    }

    const fn is_main_token(s: &str) -> bool {
        let b = s.as_bytes();
        s.len() == 72
            && (b[8] == b'-')
                & (b[13] == b'-')
                & (b[18] == b'-')
                & (b[23] == b'-')
                & (b[36 + 8] == b'-')
                & (b[36 + 13] == b'-')
                & (b[36 + 18] == b'-')
                & (b[36 + 23] == b'-')
    }

    fn parse_jwt(s: &str) -> Option<Subtoken> {
        let mut iter = s.splitn(3, '.');
        iter.next()?;
        let chunk = iter.next()?;
        iter.next()?;

        let bytes = general_purpose::STANDARD_NO_PAD.decode(chunk).ok()?;
        let data = String::from_utf8(bytes).ok()?;

        serde_json::from_str(data.as_str()).ok()
    }
}

impl FromStr for ApiKey {
    type Err = InvalidKeyPattern;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ApiKey::is_main_token(s) {
            return Ok(Self::ApiKey);
        }

        let token = ApiKey::parse_jwt(s).ok_or(InvalidKeyPattern)?;

        Ok(ApiKey::Subtoken(token))
    }
}

#[derive(Debug)]
pub struct InvalidKeyPattern;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Subtoken {
    #[serde(rename = "exp", with = "timestamp_parser")]
    pub expires: NaiveDateTime,
    pub permissions: Vec<ApiKeyPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Ord, PartialOrd, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyPermissions {
    Account,
    Characters,
    Progression,
    Unlocks,
    Inventories,
    Wallet,
    Pvp,
    Builds,
    Guilds,
    TradingPost,
}

mod timestamp_parser {
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(d)?;
        Ok(NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .expect("invalid or out-of-range datetime"))
    }

    pub fn serialize<S>(dt: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(dt.timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_MAIN_KEY: &str =
        "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";
    const VALID_SUB_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJWWFd4V3lxNFktM1FDYUM5bTg3WUZpZm1QT3VpS2NCOWtKNzJFSmQtdG93IiwiaWF0IjoxNzAxNTMxNjg4LCJleHAiOjE3MDIxMzYzNDAsInBlcm1pc3Npb25zIjpbImFjY291bnQiXSwidXJscyI6WyIvdjIvdG9rZW5pbmZvIl19.omGlVisJ2WBE9dtRSnMxd0S9wdsS9BADyyx-WDepg1w";
    const VALID_SUB_KEY_WITHOUT_URLS: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJWWFd4V3lxNFktM1FDYUM5bTg3WUZpZm1QT3VpS2NCOWtKNzJFSmQtdG93IiwiaWF0IjoxNzAxNTk5OTkyLCJleHAiOjE3MDE1OTk5ODQsInBlcm1pc3Npb25zIjpbImFjY291bnQiXX0.1vd0uufoiS7ZqFoOJCMfOv8fSx6fcTUOa0LKzuchalA";

    #[test]
    fn valid_main_key() {
        let key: ApiKey = VALID_MAIN_KEY.parse().unwrap();
        assert_eq!(key, ApiKey::ApiKey);
    }

    #[test]
    fn valid_subkey() {
        let key: ApiKey = VALID_SUB_KEY.parse().unwrap();
        let ApiKey::Subtoken(subkey) = key else {
            panic!("not a valid subkey");
        };
        assert_eq!(
            subkey.expires,
            NaiveDateTime::from_timestamp_opt(1702136340, 0).unwrap()
        );
        assert_eq!(subkey.permissions, vec![ApiKeyPermissions::Account]);
        assert_eq!(subkey.urls, Some(vec!["/v2/tokeninfo".to_string()]));
    }

    #[test]
    fn valid_subkey_no_urls() {
        let key: ApiKey = VALID_SUB_KEY_WITHOUT_URLS.parse().unwrap();
        let ApiKey::Subtoken(subkey) = key else {
            panic!("not a valid subkey");
        };
        assert_eq!(
            subkey.expires,
            NaiveDateTime::from_timestamp_opt(1701599984, 0).unwrap()
        );
        assert_eq!(subkey.permissions, vec![ApiKeyPermissions::Account]);
        assert_eq!(subkey.urls, None);
    }
}
