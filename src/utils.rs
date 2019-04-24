use std::fmt::{self, Debug, Display};

#[derive(Debug)]
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
