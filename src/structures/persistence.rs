use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use serde::Serialize;

use crate::structures::MapKey;

pub trait PersistData<V> {
    fn persist(&self, prefix: &str, data: &HashMap<MapKey, HashSet<V>>);
}

pub struct PersistDataToDisk {}

impl<V> PersistData<V> for PersistDataToDisk
    where V: Serialize {
    fn persist(&self, prefix: &str, data: &HashMap<MapKey, HashSet<V>>) {
        fs::create_dir_all(prefix).expect("failed to create data path");

        for (k, v) in data.iter() {
            let path = Path::new(prefix).join(format!("{}.json", k));
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .expect("failed to open file");

            for entry in v {
                let j = serde_json::to_string(&entry).expect("failed to serialize struct to JSON");
                writeln!(file, "{}", j).expect("Failed to write to file");
            }
        }
    }
}


