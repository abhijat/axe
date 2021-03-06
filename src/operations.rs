use std::fs;

use chrono::Datelike;

use crate::options::{Options, SplitFields};
use crate::structures::{CappedDumpableHashMap, Entry, MapKey, PersistDataToDisk};

fn to_string(s: &Option<String>) -> String {
    s.as_ref()
        .map(|s| s.to_owned())
        .unwrap_or("".to_string())
}

fn build_key(entry: &Entry, split_fields: &SplitFields) -> MapKey {
    match split_fields {
        SplitFields::Author => MapKey::StringKey { key: to_string(&entry.author) },
        SplitFields::SubReddit => MapKey::StringKey { key: to_string(&entry.subreddit) },
        SplitFields::Day => MapKey::NumKey { key: entry.created_at().day() as usize },
        SplitFields::Month => MapKey::NumKey { key: entry.created_at().month() as usize },
        SplitFields::DayOfYear => MapKey::NumKey { key: entry.created_at().ordinal() as usize },
        SplitFields::DayOfWeek => MapKey::StringKey { key: entry.created_at().weekday().to_string() }
    }
}

pub fn parse_json(options: &Options) {
    let data = fs::read_to_string(&options.input_path)
        .expect(&format!("Failed to read file {}", options.input_path.to_string_lossy()));

    let mut entries: CappedDumpableHashMap<Entry> = CappedDumpableHashMap::new(
        options.max_size,
        &options.output_prefix.as_os_str().to_string_lossy(),
        Box::new(PersistDataToDisk {}),
    );

    for line in data.lines() {
        let e: Entry = serde_json::from_str(line)
            .expect(&format!("failed to parse json `{}`", line));
        entries.add(build_key(&e, &options.split_on), e);
    }

    entries.dump_to_disk();
}
