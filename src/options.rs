use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug)]
pub enum SplitFields {
    Author,
    SubReddit,
    Day,
    Month,
}

impl FromStr for SplitFields {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "author" => Ok(SplitFields::Author),
            "subreddit" => Ok(SplitFields::SubReddit),
            "day" => Ok(SplitFields::Day),
            "month" => Ok(SplitFields::Month),
            _ => Err("invalid argument".to_owned())
        }
    }
}

impl ToString for SplitFields {
    fn to_string(&self) -> String {
        match *self {
            SplitFields::Author => "author".to_owned(),
            SplitFields::SubReddit => "subreddit".to_owned(),
            SplitFields::Day => "day".to_owned(),
            SplitFields::Month => "month".to_owned(),
        }
    }
}

#[derive(Debug, StructOpt)]
/// A utility to split a reddit dataset into individual JSON files
pub struct Options {
    #[structopt(short, long, default_value = "150000")]
    /// The maximum size the hashmap will grow to before it is written to disk
    pub max_size: usize,

    #[structopt(short, long)]
    /// The attribute to split the data set on
    pub split_on: SplitFields,

    #[structopt(short, long, parse(from_os_str))]
    /// The path to the data set
    pub input_path: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    /// The parent directory where output JSON files will be written
    pub output_prefix: PathBuf,
}