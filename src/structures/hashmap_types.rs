use std::fmt::Display;
use std::hash::Hash;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde::export::fmt::Error;
use serde::export::Formatter;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialOrd, PartialEq)]
pub struct Entry {
    pub author: Option<String>,
    pub subreddit: Option<String>,
    pub title: Option<String>,
    pub permalink: Option<String>,
    pub created_utc: u64,
    pub selftext: Option<String>,
}

impl Entry {
    pub fn created_at(&self) -> NaiveDateTime {
        naive_datetime(self.created_utc)
    }
}

pub fn naive_datetime(timestamp: u64) -> NaiveDateTime {
    NaiveDateTime::from_timestamp(timestamp as i64, 0)
}

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialOrd, PartialEq)]
pub enum MapKey {
    StringKey {
        key: String,
    },
    NumKey {
        key: usize,
    },
}

impl Display for MapKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = match self {
            MapKey::StringKey { key } => { key.to_owned() }
            MapKey::NumKey { key } => { key.to_string() }
        };

        write!(f, "{}", s)
    }
}