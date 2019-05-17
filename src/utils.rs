use std::fmt::{self, Debug, Display};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
    En,
    Fr,
    De,
    Es,
}

impl Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub type TimeStamp = String;

pub fn format_ids(item_ids: impl IntoIterator<Item = impl std::fmt::Display>) -> String {
    let items = item_ids
        .into_iter()
        .fold(String::new(), |acc, x| format!("{},{}", acc, x));
    (&items[1..]).to_owned()
}